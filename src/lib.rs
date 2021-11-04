#![warn(missing_docs)]

//! `octopt` is a library for handling CHIP-8 configuration settings.
//!
//! CHIP-8 is a virtual machine for playing simple computer games. It has been around since 1977, and has many slightly incompatible implementations.
//!
//! Games often require specific behavior from its interpreter to run correctly, but you can't know what behavior it expects just by looking at its bytecode.
//!
//! This library contains structs and enums that represent all possible CHIP-8 options, which you can use for your CHIP-8 emulator.

pub mod color;
use color::Color;
mod ini;
use ini::OptionsIni;
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::skip_serializing_none;
use std::fmt;
use std::str::FromStr;
use std::u8;

/// If the CHIP-8 interpreter supports custom colors for visual elements, it can use these values
/// for setting them.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    /// The standard color used for active pixels on the CHIP-8 screen. For XO-CHIP, it's used for
    /// the first drawing plane.
    pub fill_color: Option<Color>,
    /// XO-CHIP only: The color used for the second drawing plane.
    pub fill_color2: Option<Color>,
    /// XO-CHIP only: The color used for when both drawing planes overlap.
    pub blend_color: Option<Color>,
    /// The standard background color of the CHIP-8 screen.
    pub background_color: Option<Color>,
    /// The color used by any visual indicator for when the sound buzzer is active.
    pub buzz_color: Option<Color>,
    /// The color used by any visual indicator for when the sound buzzer is inactive.
    pub quiet_color: Option<Color>,
}

/// The default colorscheme here is white on black, which is most common, with non-standard colors
/// for the other elements, albeit inspried by Octo's "Hot Dog" preset.
impl Default for Colors {
    fn default() -> Self {
        Self {
            fill_color: Some(Color {
                r: 255,
                g: 255,
                b: 255,
            }),
            fill_color2: Some(Color {
                r: 255,
                g: 255,
                b: 0,
            }),
            blend_color: Some(Color { r: 255, g: 0, b: 0 }),
            background_color: Some(Color { r: 0, g: 0, b: 0 }),
            buzz_color: Some(Color { r: 153, g: 0, b: 0 }),
            quiet_color: Some(Color { r: 51, g: 0, b: 0 }),
        }
    }
}

/// Represents the different touch modes supported by [Octo](https://github.com/JohnEarnest/Octo).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchMode {
    /// Do not attempt to handle touch input.
    None,
    /// Taps on the screen are treated like pressing key 6. Swipes or dragging and holding on the
    /// screen are treated like a virtual directional pad based on keys 5,8,7 and 9.
    Swipe,
    /// Treat taps and holds on the center of the screen like an invisible 4x4 hex keypad. Also
    /// supports mouse input.
    Seg16,
    /// The same as Seg16, but the virtual keys take up the entire display, rather than a square
    /// region. Also supports mouse input.
    Seg16Fill,
    /// Draw a translucent virtual gamepad around the screen. The directional pad is mapped to keys
    /// 5,8,7 and 9, and buttons A and B are mapped to keyboard keys 6 and 4, respectively.
    Gamepad,
    /// Display a 4x4 hex keypad under the screen. Also supports mouse input.
    Vip,
}

impl Default for TouchMode {
    fn default() -> Self {
        Self::None
    }
}

/// Represents the different "quirks", ie. divergent behaviors, of the CHIP-8 runtime. These are
/// the most important ones to support, as many games depend on specific settings here to run
/// properly.
///
/// In the following, "original behavior" refers to how the original CHIP-8 interpreter on the
/// COSMAC VIP operated.
///
/// All these quirks are [`Option`]s, because they can be considered to be ternary values. A `Some(true)`
/// value means that the interpreter should use the "quirky" behavior in a particular scenario. A
/// `Some(false)` value means that it should use the "default" behavior. However, a `None` value
/// means that this quirk setting was absent from the metadata, so we don't know what the game
/// requires. This also implies that the interpreter should use some default behavior. This could be
/// either because the game's creator wasn't aware of that particular quirk, or that the program that
/// exported the metadata (usually Octo) wasn't aware of it (probably because it uses the default
/// behavior for that quirk, without any option of configuring it). This is fine, because some of the
/// quirks are obscure, but we still use `Option` in these cases so we don't serialize these quirk
/// settings as `false` when we don't know that the game requires the quirk to be disabled.
///
/// Note that whether a specific behavior is considered "quirky"/"default" or not doesn't necessarily
/// mean that's the original behavior; in many cases, the original behavior is considered "quirky"
/// and requires a `true` value to enable. This is for historical reasons.
///
/// Note also that Octo doesn't support all of these quirks. This struct should support all
/// possible divergent behaviors between widely used CHIP-8 interpreters. A CHIP-8 interpreter
/// should ignore any quirks they don't recognize, or don't have any intention of supporting.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quirks {
    /// Decides the behavior of the CHIP-8 shift instructions 8XY6 (right shift) and 8XYE (left shift):
    /// * False: The value in the VY register is shifted, and the result is placed in the VX
    /// register. (Original behavior)
    /// * True: The VX register is shifted in-place, and the VY register is ignored. (CHIP48 and
    /// SUPER-CHIP behavior)
    #[serde(
        rename = "shiftQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub shift: Option<bool>,
    /// Decides the behavior of the CHIP-8 serialization FX55 (dump registers V0–VX to memory
    /// location I) and FX65 (load registers V0–VX from memory location I):
    /// * False: The value in the I register is incremented for each register loaded/stored.
    /// (Original behavior)
    /// * True: The I register is left unchanged after the operation. (SUPER-CHIP behavior)
    #[serde(
        rename = "loadStoreQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub load_store: Option<bool>,
    /// Decides the behavior of the CHIP-8 relative jump instruction BXNN (jump to address XNN,
    /// plus the value in a register):
    /// * False: The value in the V0 register is used for the offset (original behavior)
    /// * True: The value in the VX register is used, where X is the first digit in the target
    /// address XNN (CHIP48 and SUPER-CHIP behavior)
    #[serde(
        rename = "jumpQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub jump0: Option<bool>,
    /// Decides the value of the VF flag register after logical instructions 8XY1 (logical OR),
    /// 8XY2 (logical AND) and 8XY3 (logical XOR):
    /// * False: The VF flag register is unchanged by logical instructions (Octo, CHIP48 and
    /// SUPER-CHIP behavior)
    /// * True: The state of the VF flag register is undefined after logical instructions (original
    /// behavior)
    #[serde(
        rename = "logicQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub logic: Option<bool>,
    /// Decides the behavior of sprites drawn out of bounds:
    /// * False: Sprites wrap on screen edges (Octo behavior)
    /// * True: Sprites are clipped on screen edges (original, CHIP-48 and SUPER-CHIP behavior)
    #[serde(
        rename = "clipQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub clip: Option<bool>,
    /// Decides whether the CHIP-8 interpreter should wait for the rest of the current frame after
    /// each drawing operation:
    /// * False: No special behavior (CHIP-48, SUPER-CHIP and Octo behavior)
    /// * True: After a draw instruction, the CPU does no more work for the rest of the frame, ie.
    /// it waits for a "VBlank interrupt" (original behavior)
    #[serde(
        rename = "vBlankQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub vblank: Option<bool>,
    /// Decides whether arithmetic or logical instructions that have the VF register as one of the
    /// operands should set the resulting flag in the VF flag register before or after the value:
    /// * False: The resulting flags are discarded, and the result is placed in the VF register
    /// * True: The resulting value is discarded, and the flag is placed in the VF register
    /// (original behavior)
    #[serde(
        rename = "vfOrderQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub vf_order: Option<bool>,
    /// Decides what the behavior of the draw instruction should be if the given sprite height is 0
    /// (DXY0) and the interpreter is in lores (low-resolution 64x32 CHIP-8) mode:
    /// * NoOp: No operation (original behavior)
    /// * TallSprite: Draw a 16-byte sprite (DREAM 6800 behavior)
    /// * BigSprite: Draw a 16x16 pixel sprite, ie. the same behavior as in hires (high-resolution
    /// 128x64 SUPER-CHIP/XO-CHIP) mode (Octo behavior)
    #[serde(rename = "loresDXY0Quirks")]
    pub lores_dxy0: Option<LoResDxy0Behavior>,
    /// Decides whether the screen should be cleared when there is a resolution change (00FE and
    /// 00FF). Note that if this is true, then the screen should retain the current image when
    /// going from lores (low resolution) to hires (high resolution), which implies that the
    /// existing image on the screen should be scaled up 2x.
    /// * True: The screen is cleared if the resolution is changed (Octo behavior)
    /// * False: The screen retains its image if the resolution is changed (original SUPER-CHIP
    /// behavior)
    #[serde(
        rename = "resClearQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub res_clear: Option<bool>,
    /// Decides whether the delay timer should wrap around when it has counted down to 0 or not:
    /// * True: The delay timer never stops, but overflows from 0 to 255 and keeps counting (DREAM
    /// 6800 behavior)
    /// * False: The delay timer counts down to 0, and then stops (original behavior)
    #[serde(
        rename = "delayWrapQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub delay_wrap: Option<bool>,
    /// Decides the result in the VF flag register when there's a collision of sprites in hires
    /// (high resolution) mode:
    /// * True: VF is set to the number of sprite pixel ros that detected a collision (SUPER-CHIP
    /// 1.1 behavior, hires mode only)
    /// * False: VF is always set to 1 if there is a collision (original behavior)
    #[serde(
        rename = "hiresCollisionQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub hires_collision: Option<bool>,
    /// Decides whether sprites clipping at the bottom of the screen should cound as a collision.
    /// Note that this was probably a bug in the SUPER-CHIP 1.1 interpreter, and might not be
    /// required by any games. Also, this doesn't make much sense if `clip_quirks` is false.
    /// * True: VF is set if a sprite runs off the bottom of the screen (SUPER-CHIP 1.1 behavior)
    /// * False: VF is unchanged if a sprite runs off the bottom of the screen (original behavior)
    #[serde(
        rename = "clipCollisionQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub clip_collision: Option<bool>,
    /// Decides whether scrolling in lores (low-resolution) mode scrolls by half the number of
    /// pixels as in the high resolution mode. This occured in SUPER-CHIP because the low
    /// resolution display was scaled up 2x; see also the `res_clear` quirk.
    /// * True: In low resolution mode, scrolling left and right will scroll by 2 pixels rather
    /// than 4 (as in high resolution), and scrolling down (and up, with the XO-CHIP instruction)
    /// will scroll by half a pixel, since pixels are scaled upx) (SUPER-CHIP behavior)
    /// * False: Scrolling acts the same in high and low resolution mode (Octo behavior)
    #[serde(
        rename = "scrollQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub scroll: Option<bool>,
    /// Decides whether the I address register should set the VF flag register if it "overflows"
    /// from `0x0FFF` to above `0x1000`. Only one known game, _Spacefight! 2091_, relies on this
    /// quirk, which was only present in the obscure CHIP-8 interpreter for the Amiga, while at
    /// least one game (_Animal Race_) relies on the standard behavior.
    /// * True: VF is set to 1 if the I register takes a value larger than `0x0FFF` (Amiga
    /// behavior)
    /// * False: VF is not affected by the I register (original behavior)
    #[serde(
        rename = "overflowIQuirks",
        deserialize_with = "some_bool_from_int",
        default
    )]
    pub overflow_i: Option<bool>,
}

/// Returns a default where no quirks are enabled, except the ones Octo observe.
impl Default for Quirks {
    fn default() -> Self {
        Self {
            shift: Some(false),
            load_store: Some(false),
            jump0: Some(false),
            logic: Some(false),
            clip: Some(false),
            vblank: Some(false),
            vf_order: Some(false),
            lores_dxy0: Some(LoResDxy0Behavior::default()),
            res_clear: Some(true),
            delay_wrap: Some(false),
            hires_collision: Some(false),
            clip_collision: Some(false),
            scroll: Some(false),
            overflow_i: Some(false),
        }
    }
}

/// Represents the different possible behaviors of attempting to draw a sprite with 0 height with
/// the instruction DXY0 while in lores (low-resolution 64x32) mode.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoResDxy0Behavior {
    /// No operation (original behavior)
    NoOp,
    /// Draw a sprite with height 16 (DREAM 6800 behavior)
    TallSprite,
    /// Draw a 16x16 sprite, ie. the same behavior as in hires (high-resolution 128x64 SUPER-CHIP
    /// XO-CHIP) mode (Octo behavior)
    BigSprite,
}

impl Default for LoResDxy0Behavior {
    fn default() -> Self {
        Self::BigSprite
    }
}

/// Representation of Octo options.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    /// The number of CHIP-8 instructions executed per 60Hz frame, ie. the "speed" of the virtual
    /// CPU. These are all approximations of hardware limitations, because on real hardware
    /// different instructions execute in different times, but it's a conventional middle ground.
    ///
    /// Common values:
    /// * 7–15 (approximate speed of the original interpreter for the COSMAC VIP)
    /// * 20–30 (approximate speed of the SUPER-CHIP interpreters for the HP 48 calculators)
    /// * 10000 (Octo's "Ludicrous speed" setting)
    #[serde(default, deserialize_with = "some_u16_from_int_or_str")]
    pub tickrate: Option<u16>,
    /// The maximum amount of virtual memory, in bytes, that is available to the program. If the CHIP-8 program is
    /// larger than this, the interpreter should give an error.
    ///
    /// At least 512 bytes are always reserved for the CHIP-8 interpreter and unavailable to the
    /// CHIP-8 game; see the field `start_address`.
    ///
    /// This is mostly relevant when developing CHIP-8 games for real hardware, as an assertion
    /// that the game will fit in the target platform's memory. Most CHIP-8 interpreters can ignore
    /// this value without consequence.
    ///
    /// Common values:
    /// * 3216 (original interpreter for the COSMAC VIP with 4K RAM)
    /// * 3583 (SUPER-CHIP interpreter for the HP 48)
    /// * 3584 (Octo)
    /// * 65024 (XO-CHIP interpreters)
    ///
    /// Other values might be used for games for more obscure platforms, games that were designed
    /// to run on a COSMAC VIP with only 2K RAM, etc.
    #[serde(default, deserialize_with = "some_u16_from_int_or_str")]
    pub max_size: Option<u16>, // {3216, 3583, 3584, 65024}
    /// The orientation of the display.
    #[serde(default)]
    pub screen_rotation: ScreenRotation,
    /// The font style expected by the game.
    #[serde(default)]
    pub font_style: Font, // OCTO_FONT_...
    /// The touch controls this game supports.
    #[serde(default)]
    pub touch_input_mode: TouchMode, // OCTO_TOUCH_...
    /// The memory address in the virtual RAM that this game should be loaded from. On legacy
    /// hardware, the interpreter itself was loaded into the lower memory addresses, and then the
    /// game was loaded after it (usually at address `0x200`, ie. 512).
    ///
    /// Common values:
    /// * 512 (original interpreter for the COSMAC VIP, DREAM 6800, HP 48, etc)
    /// * 1536 (interpreter for the ETI-660)
    #[serde(default, deserialize_with = "some_u16_from_int_or_str")]
    pub start_address: Option<u16>,

    /// Custom colors this game would like to use, if possible. It's not important for a CHIP-8
    /// interpreter to support custom colors although not doing so might impact the creator's
    /// artistic vision, especially for XO-CHIP games that use more than two colors.
    #[serde(flatten)]
    pub colors: Colors,

    /// Specific behaviors this game expects from the interpreter in order to run properly. See
    /// [`OctoQuirks`] for specifics.
    #[serde(flatten)]
    pub quirks: Quirks,
}

/// Returns a default where no quirks are enabled, except that the [`LoResDxy0Behavior`] assumed Octo behavior..
impl Default for Options {
    fn default() -> Self {
        Self {
            tickrate: Some(500),
            max_size: Some(3584),
            screen_rotation: ScreenRotation::default(),
            font_style: Font::default(),
            touch_input_mode: TouchMode::default(),
            start_address: Some(512),
            colors: Colors::default(),
            quirks: Quirks::default(),
        }
    }
}

/// Possible orientations of the display. Note that this should only affect the visual
/// representation of the screen; draw operations still act as if the screen rotation is 0. Only
/// used by some Octo games.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ScreenRotation {
    /// Normal landscape screen display, used by 99.9999% CHIP-8 games
    Normal = 0,
    /// Portrait screen display, ie. a normal screen rotated 90 degrees clockwise
    ClockWise = 90,
    /// Upside down landscape screen display
    UpsideDown = 180,
    /// Portrait screen display, ie. a normal screen rotated 90 degrees counter-clockwise
    CounterClockWise = 270,
}

impl Default for ScreenRotation {
    fn default() -> Self {
        Self::Normal
    }
}

/// Deserializes Options from a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl FromStr for Options {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Options {
    //fn from_ini_reader(reader: std::io::Read) -> Options {
    //    let mut buffer = String::new();
    //    reader.read_to_string(&mut buffer)?;
    //    let ooi: OptionsIni = serde_ini::from_str(buffer).unwrap();
    //    Options::from(ooi)
    //}

    /// Deserializes Options from an INI string.
    pub fn from_ini(s: &str) -> Result<Self, serde_ini::de::Error> {
        Ok(Options::from(OptionsIni::from_str(s)?))
    }

    /// Serializes Options to an INI string.
    pub fn to_ini(o: Options) -> String {
        OptionsIni::to_string(&OptionsIni::from(o))
    }
}

/// Serializes Options into a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(string) => write!(f, "{}", string),
            _ => Err(fmt::Error),
        }
    }
}

// Could have used serde_aux::field_attributes::deserialize_option_number_from_string here
// but let's not pull in that dep just for this. If it had deserialize_option_bool_from_anything
// then we'd be talking.
fn some_u16_from_int_or_str<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U16OrStr<'a> {
        U16(u16),
        Str(&'a str),
    }

    Ok(match U16OrStr::deserialize(deserializer)? {
        U16OrStr::Str(v) => match v.parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        U16OrStr::U16(v) => Some(v),
    })
}

fn some_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrU8 {
        Bool(bool),
        U8(u8),
    }

    match BoolOrU8::deserialize(deserializer)? {
        BoolOrU8::Bool(v) => Ok(Some(v)),
        BoolOrU8::U8(1) => Ok(Some(true)),
        BoolOrU8::U8(0) => Ok(Some(false)),
        BoolOrU8::U8(other) => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

/// Represents the different fonts a CHIP-8 interpreter can provide.
///
/// It's not likely that many (or any) historical CHIP-8 games depend on a particular font, but it's
/// possible, and for that reason (and to make historical games look accurate) the font can be
/// overriden here _and_ you can get the sprite data for the fonts by calling [`get_font_data`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Font {
    /// The font used by [Octo](https://github.com/JohnEarnest). Its small digits are identical to
    /// SUPER-CHIP's, but the big digits are a bigger version of the small ones, rather than
    /// SUPER-CHIP's rounded big digits. Contains small digits for 0–F, as well as big digits for 0–F.
    Octo,
    /// The font used by the original CHIP-8 interpreter on the COSMAC VIP.
    /// Contains small digits for 0–F only.
    Vip,
    /// The font used by CHIP-8/CHIPOS on the DREAM 6800. Contains small
    /// digits for 0–F only.
    Dream6800,
    /// The font used by the CHIP-8 interpreter on the ETI-660. Contains small
    /// digits for 0–F only. Very similar to Dream6800.
    Eti660,
    /// The font used by SUPER-CHIP 1.1 on the HP 48. Contains small digits for
    /// 0–F (identical to Octo's), but only big digits for 0–9.
    Schip,
    /// Custom font used by the Fish'n'Chips CHIP-8 emulator. Contains small digits
    /// for 0–F and big digits (7x9 pixels) for 0–F.
    Fish,
    /// Font designed by A-KouZ1, used by in the KChip-8 and FPChip-8 emulators. Contains small
    /// digits for 0–F and big digits for 0–F.
    AKouZ1,
}

/// The default font is Octo's font, as it's the modern standard and contains all hexadecimal digits
/// in both small and large variants.
impl Default for Font {
    fn default() -> Self {
        Self::Octo
    }
}

/// Returns a tuple where the first element is an array of 16 sprites that are 5 bytes tall, where
/// each one represents the sprite data for a hexadecimal digit in a CHIP-8 font, and the other
/// optional element is a vector of sprites that are 10 bytes tall.
///
/// Note that some fonts are smaller than this, but the extra padding is still included so
/// emulators and games can use the same routines regardless of the font.
///
/// Not all fonts provide the larger sprites, as they became standard with SUPER-CHIP's high resolution mode.
/// Furthermore, the SUPER-CHIP font set itself only provides large sprites for the decimal digits
/// 0–9, not the hexadecimal A–F.
///
/// A modern CHIP-8 interpreter will put its font data (for one font) somewhere in the first 512 bytes of
/// memory, which are reserved for the interpreter, but the actual memory location doesn't matter.
/// It's common to put it at either address 0 or 80 (`0x50`).
pub fn get_font_data(font: Font) -> ([u8; 5 * 16], Option<Vec<u8>>) {
    match font {
        Font::Octo => (
            [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            Some(vec![
                0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
                0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
                0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
                0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
                0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
                0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
                0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
                0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
                0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
                0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
                0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
                0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
            ]),
        ),
        Font::Vip => (
            [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x60, 0x20, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0xA0, 0xA0, 0xF0, 0x20, 0x20, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x10, 0x10, 0x10, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xF0, 0x50, 0x70, 0x50, 0xF0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xF0, 0x50, 0x50, 0x50, 0xF0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            None,
        ),
        Font::Dream6800 => (
            [
                0xE0, 0xA0, 0xA0, 0xA0, 0xE0, // 0
                0x40, 0x40, 0x40, 0x40, 0x40, // 1
                0xE0, 0x20, 0xE0, 0x80, 0xE0, // 2
                0xE0, 0x20, 0xE0, 0x20, 0xE0, // 3
                0x80, 0xA0, 0xA0, 0xE0, 0x20, // 4
                0xE0, 0x80, 0xE0, 0x20, 0xE0, // 5
                0xE0, 0x80, 0xE0, 0xA0, 0xE0, // 6
                0xE0, 0x20, 0x20, 0x20, 0x20, // 7
                0xE0, 0xA0, 0xE0, 0xA0, 0xE0, // 8
                0xE0, 0xA0, 0xE0, 0x20, 0xE0, // 9
                0xE0, 0xA0, 0xE0, 0xA0, 0xA0, // A
                0xC0, 0xA0, 0xE0, 0xA0, 0xC0, // B
                0xE0, 0x80, 0x80, 0x80, 0xE0, // C
                0xC0, 0xA0, 0xA0, 0xA0, 0xC0, // D
                0xE0, 0x80, 0xE0, 0x80, 0xE0, // E
                0xE0, 0x80, 0xC0, 0x80, 0x80, // F
            ],
            None,
        ),
        Font::Eti660 => (
            [
                0xE0, 0xA0, 0xA0, 0xA0, 0xE0, // 0
                0x20, 0x20, 0x20, 0x20, 0x20, // 1
                0xE0, 0x20, 0xE0, 0x80, 0xE0, // 2
                0xE0, 0x20, 0xE0, 0x20, 0xE0, // 3
                0xA0, 0xA0, 0xE0, 0x20, 0x20, // 4
                0xE0, 0x80, 0xE0, 0x20, 0xE0, // 5
                0xE0, 0x80, 0xE0, 0xA0, 0xE0, // 6
                0xE0, 0x20, 0x20, 0x20, 0x20, // 7
                0xE0, 0xA0, 0xE0, 0xA0, 0xE0, // 8
                0xE0, 0xA0, 0xE0, 0x20, 0xE0, // 9
                0xE0, 0xA0, 0xE0, 0xA0, 0xA0, // A
                0x80, 0x80, 0xE0, 0xA0, 0xE0, // B
                0xE0, 0x80, 0x80, 0x80, 0xE0, // C
                0x20, 0x20, 0xE0, 0xA0, 0xE0, // D
                0xE0, 0x80, 0xE0, 0x80, 0xE0, // E
                0xE0, 0x80, 0xC0, 0x80, 0x80, // F
            ],
            None,
        ),
        Font::Schip => (
            [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            Some(vec![
                0x3C, 0x7E, 0xE7, 0xC3, 0xC3, 0xC3, 0xC3, 0xE7, 0x7E, 0x3C, // 0
                0x18, 0x38, 0x58, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, // 1
                0x3E, 0x7F, 0xC3, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xFF, 0xFF, // 2
                0x3C, 0x7E, 0xC3, 0x03, 0x0E, 0x0E, 0x03, 0xC3, 0x7E, 0x3C, // 3
                0x06, 0x0E, 0x1E, 0x36, 0x66, 0xC6, 0xFF, 0xFF, 0x06, 0x06, // 4
                0xFF, 0xFF, 0xC0, 0xC0, 0xFC, 0xFE, 0x03, 0xC3, 0x7E, 0x3C, // 5
                0x3E, 0x7C, 0xE0, 0xC0, 0xFC, 0xFE, 0xC3, 0xC3, 0x7E, 0x3C, // 6
                0xFF, 0xFF, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x60, 0x60, // 7
                0x3C, 0x7E, 0xC3, 0xC3, 0x7E, 0x7E, 0xC3, 0xC3, 0x7E, 0x3C, // 8
                0x3C, 0x7E, 0xC3, 0xC3, 0x7F, 0x3F, 0x03, 0x03, 0x3E, 0x7C, // 9
            ]),
        ),
        Font::Fish => (
            [
                0x60, 0xA0, 0xA0, 0xA0, 0xC0, // 0
                0x40, 0xC0, 0x40, 0x40, 0xE0, // 1
                0xC0, 0x20, 0x40, 0x80, 0xE0, // 2
                0xC0, 0x20, 0x40, 0x20, 0xC0, // 3
                0x20, 0xA0, 0xE0, 0x20, 0x20, // 4
                0xE0, 0x80, 0xC0, 0x20, 0xC0, // 5
                0x40, 0x80, 0xC0, 0xA0, 0x40, // 6
                0xE0, 0x20, 0x60, 0x40, 0x40, // 7
                0x40, 0xA0, 0x40, 0xA0, 0x40, // 8
                0x40, 0xA0, 0x60, 0x20, 0x40, // 9
                0x40, 0xA0, 0xE0, 0xA0, 0xA0, // A
                0xC0, 0xA0, 0xC0, 0xA0, 0xC0, // B
                0x60, 0x80, 0x80, 0x80, 0x60, // C
                0xC0, 0xA0, 0xA0, 0xA0, 0xC0, // D
                0xE0, 0x80, 0xC0, 0x80, 0xE0, // E
                0xE0, 0x80, 0xC0, 0x80, 0x80, // F
            ],
            Some(vec![
                // Note: 7x9 pixels
                0x7C, 0xC6, 0xCE, 0xDE, 0xD6, 0xF6, 0xE6, 0xC6, 0x7C, 0x00, // 0
                0x10, 0x30, 0xF0, 0x30, 0x30, 0x30, 0x30, 0x30, 0xFC, 0x00, // 1
                0x78, 0xCC, 0xCC, 0x0C, 0x18, 0x30, 0x60, 0xCC, 0xFC, 0x00, // 2
                0x78, 0xCC, 0x0C, 0x0C, 0x38, 0x0C, 0x0C, 0xCC, 0x78, 0x00, // 3
                0x0C, 0x1C, 0x3C, 0x6C, 0xCC, 0xFE, 0x0C, 0x0C, 0x1E, 0x00, // 4
                0xFC, 0xC0, 0xC0, 0xC0, 0xF8, 0x0C, 0x0C, 0xCC, 0x78, 0x00, // 5
                0x38, 0x60, 0xC0, 0xC0, 0xF8, 0xCC, 0xCC, 0xCC, 0x78, 0x00, // 6
                0xFE, 0xC6, 0xC6, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00, // 7
                0x78, 0xCC, 0xCC, 0xEC, 0x78, 0xDC, 0xCC, 0xCC, 0x78, 0x00, // 8
                0x7C, 0xC6, 0xC6, 0xC6, 0x7C, 0x18, 0x18, 0x30, 0x70, 0x00, // 9
                0x30, 0x78, 0xCC, 0xCC, 0xCC, 0xFC, 0xCC, 0xCC, 0xCC, 0x00, // A
                0xFC, 0x66, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x66, 0xFC, 0x00, // B
                0x3C, 0x66, 0xC6, 0xC0, 0xC0, 0xC0, 0xC6, 0x66, 0x3C, 0x00, // C
                0xF8, 0x6C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x00, // D
                0xFE, 0x62, 0x60, 0x64, 0x7C, 0x64, 0x60, 0x62, 0xFE, 0x00, // E
                0xFE, 0x66, 0x62, 0x64, 0x7C, 0x64, 0x60, 0x60, 0xF0, 0x00, // F
            ]),
        ),
        Font::AKouZ1 => (
            [
                0x60, 0x90, 0x90, 0x90, 0x60, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xE0, 0x10, 0x60, 0x80, 0xF0, // 2
                0xE0, 0x10, 0xE0, 0x10, 0xE0, // 3
                0x30, 0x50, 0x90, 0xF0, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xE0, // 5
                0x70, 0x80, 0xF0, 0x90, 0x60, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0x60, 0x90, 0x60, 0x90, 0x60, // 8
                0x60, 0x90, 0x70, 0x10, 0x60, // 9
                0x60, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0x70, 0x80, 0x80, 0x80, 0x70, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xE0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xE0, 0x80, 0x80, // F
            ],
            Some(vec![
                0x7E, 0xC7, 0xC7, 0xCB, 0xCB, 0xD3, 0xD3, 0xE3, 0xE3, 0x7E, // 0
                0x18, 0x38, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, // 1
                0x7E, 0xC3, 0x03, 0x03, 0x0E, 0x18, 0x30, 0x60, 0xC0, 0xFF, // 2
                0x7E, 0xC3, 0x03, 0x03, 0x1E, 0x03, 0x03, 0x03, 0xC3, 0x7E, // 3
                0x06, 0x0E, 0x1E, 0x36, 0x66, 0xC6, 0xC6, 0xFF, 0x06, 0x06, // 4
                0xFF, 0xC0, 0xC0, 0xC0, 0xFE, 0x03, 0x03, 0x03, 0xC3, 0x7E, // 5
                0x7E, 0xC3, 0xC0, 0xC0, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0x7E, // 6
                0xFF, 0x03, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
                0x7E, 0xC3, 0xC3, 0xC3, 0x7E, 0xC3, 0xC3, 0xC3, 0xC3, 0x7E, // 8
                0x7E, 0xC3, 0xC3, 0xC3, 0x7F, 0x03, 0x03, 0x03, 0xC3, 0x7E, // 9
                0x7E, 0xC3, 0xC3, 0xC3, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, // A
                0xFE, 0xC3, 0xC3, 0xC3, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, // B
                0x7E, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0x7E, // C
                0xFC, 0xC6, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC6, 0xFC, // D
                0xFF, 0xC0, 0xC0, 0xC0, 0xFE, 0xC0, 0xC0, 0xC0, 0xC0, 0xFF, // E
                0xFF, 0xC0, 0xC0, 0xC0, 0xFE, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, // F
            ]),
        ),
    }
}

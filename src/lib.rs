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
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::*;

use std::fmt;
use std::str::FromStr;
use std::u8;
/// If the CHIP-8 interpreter supports custom colors for visual elements, it can use these values
/// for setting them.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OctoColors {
    /// The standard color used for active pixels on the CHIP-8 screen. For XO-CHIP, it's used for
    /// the first drawing plane.
    pub fill_color: Color,
    /// XO-CHIP only: The color used for the second drawing plane.
    pub fill_color2: Color,
    /// XO-CHIP only: The color used for when both drawing planes overlap.
    pub blend_color: Color,
    /// The standard background color of the CHIP-8 screen.
    pub background_color: Color,
    /// The color used by any visual indicator for when the sound buzzer is active.
    pub buzz_color: Color,
    /// The color used by any visual indicator for when the sound buzzer is inactive.
    pub quiet_color: Color,
}

/// The default colorscheme here is white on black, which is most common, with non-standard colors
/// for the other elements, albeit inspried by Octo's "Hot Dog" preset.
impl Default for OctoColors {
    fn default() -> Self {
        Self {
            fill_color: Color {
                r: 255,
                g: 255,
                b: 255,
            },
            fill_color2: Color {
                r: 255,
                g: 255,
                b: 0,
            },
            blend_color: Color { r: 255, g: 0, b: 0 },
            background_color: Color { r: 0, g: 0, b: 0 },
            buzz_color: Color { r: 153, g: 0, b: 0 },
            quiet_color: Color { r: 51, g: 0, b: 0 },
        }
    }
}

/// Represents the different touch modes support by [Octo](https://github.com/JohnEarnest/Octo).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OctoTouchMode {
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

impl Default for OctoTouchMode {
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
/// Note that whether a specific behavior is considered "quirky" or "default" or not
/// doesn't necessarily mean that's the original behavior; in many cases, the original behavior is
/// considered "quirky" and requires a `true` value to enable. This is for historical reasons.
///
/// Note also that Octo doesn't support all of these quirks. This struct should support all
/// possible divergent behaviors between widely used CHIP-8 interpreters. A CHIP-8 interpreter
/// should ignore any quirks they don't recognize, or don't have any intention of supporting.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OctoQuirks {
    /// Decides the behavior of the CHIP-8 shift instructions 8XY6 (right shift) and 8XYE (left shift):
    /// * False: The value in the VY register is shifted, and the result is placed in the VX
    /// register. (Original behavior)
    /// * True: The VX register is shifted in-place, and the VY register is ignored. (CHIP48 and
    /// SUPER-CHIP behavior)
    #[serde(
        rename = "shiftQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub shift: bool,
    /// Decides the behavior of the CHIP-8 serialization FX55 (dump registers V0–VX to memory
    /// location I) and FX65 (load registers V0–VX from memory location I):
    /// * False: The value in the I register is incremented for each register loaded/stored.
    /// (Original behavior)
    /// * True: The I register is left unchanged after the operation. (SUPER-CHIP behavior)
    #[serde(
        rename = "loadStoreQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub load_store: bool,
    /// Decides the behavior of the CHIP-8 relative jump instruction BXNN (jump to address XNN,
    /// plus the value in a register):
    /// * False: The value in the V0 register is used for the offset (original behavior)
    /// * True: The value in the VX register is used, where X is the first digit in the target
    /// address XNN (CHIP48 and SUPER-CHIP behavior)
    #[serde(
        rename = "jumpQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub jump0: bool,
    /// Decides the value of the VF flag register after logical instructions 8XY1 (logical OR),
    /// 8XY2 (logical AND) and 8XY3 (logical XOR):
    /// * False: The VF flag register is unchanged by logical instructions (Octo, CHIP48 and
    /// SUPER-CHIP behavior)
    /// * True: The state of the VF flag register is undefined after logical instructions (original
    /// behavior)
    #[serde(
        rename = "logicQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub logic: bool,
    /// Decides the behavior of sprites drawn out of bounds:
    /// * False: Sprites wrap on screen edges (Octo behavior)
    /// * True: Sprites are clipped on screen edges (original, CHIP-48 and SUPER-CHIP behavior)
    #[serde(
        rename = "clipQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub clip: bool,
    /// Decides whether the CHIP-8 interpreter should wait for the rest of the current frame after
    /// each drawing operation:
    /// * False: No special behavior (CHIP-48, SUPER-CHIP and Octo behavior)
    /// * True: After a draw instruction, the CPU does no more work for the rest of the frame, ie.
    /// it waits for a "VBlank interrupt" (original behavior)
    #[serde(
        rename = "vBlankQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub vblank: bool,
    /// Decides whether arithmetic or logical instructions that have the VF register as one of the
    /// operands should set the resulting flag in the VF flag register before or after the value:
    /// * False: The resulting flags are discarded, and the result is placed in the VF register
    /// * True: The resulting value is discarded, and the flag is placed in the VF register
    /// (original behavior)
    #[serde(
        rename = "vfOrderQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    pub vf_order: bool,
    /// Decides what the behavior of the draw instruction should be if the given sprite height is 0
    /// (DXY0) and the interpreter is in lores (low-resolution 64x32 CHIP-8) mode:
    /// * NoOp: No operation (original behavior)
    /// * TallSprite: Draw a 16-byte sprite (DREAM 6800 behavior)
    /// * BigSprite: Draw a 16x16 pixel sprite, ie. the same behavior as in hires (high-resolution
    /// 128x64 SUPER-CHIP/XO-CHIP) mode (Octo behavior)
    #[serde(rename = "loresDXY0Quirks")]
    pub lores_dxy0: LoResDxy0Behavior,
}

/// Returns a default where no quirks are enabled, except that the [`LoResDxy0Behavior`] assumed Octo behavior..
impl Default for OctoQuirks {
    fn default() -> Self {
        Self {
            shift: false,
            load_store: false,
            jump0: false,
            logic: false,
            clip: false,
            vblank: false,
            vf_order: false,
            lores_dxy0: LoResDxy0Behavior::BigSprite,
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

/// Representation of Octo options.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OctoOptions {
    /// The number of CHIP-8 instructions executed per 60Hz frame, ie. the "speed" of the virtual
    /// CPU. These are all approximations of hardware limitations, because on real hardware
    /// different instructions execute in different times, but it's a conventional middle ground.
    ///
    /// Common values:
    /// * 7–15 (approximate speed of the original interpreter for the COSMAC VIP)
    /// * 20–30 (approximate speed of the SUPER-CHIP interpreters for the HP 48 calculators)
    /// * 10000 (Octo's "Ludicrous speed" setting)
    pub tickrate: u16,
    /// The maximum amount of virtual memory, in bytes, that is available to the program. If the CHIP-8 program is
    /// larger than this, the interpreter should give an error.
    ///
    /// At least 512 bytes are always reserved for the CHIP-8 interpreter and unavailable to the
    /// CHIP-8 game; see the field [`start_address`].
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
    pub max_size: u16, // {3216, 3583, 3584, 65024}
    /// The orientation of the display.
    pub screen_rotation: ScreenRotation,
    /// The font style expected by the game.
    pub font_style: OctoFont, // OCTO_FONT_...
    /// The touch controls this game supports.
    pub touch_input_mode: OctoTouchMode, // OCTO_TOUCH_...
    /// The memory address in the virtual RAM that this game should be loaded from. On legacy
    /// hardware, the interpreter itself was loaded into the lower memory addresses, and then the
    /// game was loaded after it (usually at address `0x200`, ie. 512).
    ///
    /// Common values:
    /// * 512 (original interpreter for the COSMAC VIP, DREAM 6800, HP 48, etc)
    /// * 1536 (interpreter for the ETI-660)
    pub start_address: u16,

    /// Custom colors this game would like to use, if possible. It's not important for a CHIP-8
    /// interpreter to support custom colors although not doing so might impact the creator's
    /// artistic vision, especially for XO-CHIP games that use more than two colors.
    #[serde(flatten)]
    pub colors: OctoColors,

    /// Specific behaviors this game expects from the interpreter in order to run properly. See
    /// [`OctoQuirks`] for specifics.
    #[serde(flatten)]
    pub quirks: OctoQuirks,
}

/// Returns a default where no quirks are enabled, except that the [`LoResDxy0Behavior`] assumed Octo behavior..
impl Default for OctoOptions {
    fn default() -> Self {
        Self {
            tickrate: 500,
            max_size: 3584,
            screen_rotation: ScreenRotation::default(),
            font_style: OctoFont::default(),
            touch_input_mode: OctoTouchMode::default(),
            start_address: 512,
            colors: OctoColors::default(),
            quirks: OctoQuirks::default(),
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

/// Deserializes OctoOptions from a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl FromStr for OctoOptions {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Serializes OctoOptions into a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl fmt::Display for OctoOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(string) => write!(f, "{}", string),
            _ => Err(fmt::Error),
        }
    }
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn int_from_bool<S>(x: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(if *x { 1 } else { 0 })
}

/// Represents the different fonts a CHIP-8 interpreter can provide. The default is Octo's modern font,
/// which contains all the digits in both sizes and is used by most modern games.
///
/// It's not likely that many (or any) historical CHIP-8 games depend on a particular font, but it's
/// possible, and for that reason (and to make historical games look accurate) the font can be
/// overriden here _and_ you can get the sprite data for the fonts by calling [`get_font_data`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OctoFont {
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
    /// 0–F, but only big digits for 0–9.
    Schip,
    /// Custom font used by the Fish'n'Chips CHIP-8 emulator. Contains small digits
    /// for 0–F and big digits for 0–F.
    Fish,
}

/// The default font is Octo's font, as it's the modern standard and contains all hexadecimal digits
/// in both small and large variants.
impl Default for OctoFont {
    fn default() -> Self {
        Self::Octo
    }
}

/// Returns a tuple where the first element is an array of 16 sprites that are 5 bytes tall, where
/// each one represents the sprite data for a hexadecimal digit in a CHIP-8 font, and the other
/// optional element is a vector of sprites that are 10 bytes tall.
///
/// Not all fonts provide the larger sprites, as they became standard with SUPER-CHIP's high resolution mode.
/// Furthermore, the SUPER-CHIP font set itself only provides large sprites for the decimal digits
/// 0–9, not the hexadecimal A–F.
///
/// A modern CHIP-8 interpreter will put its font data (for one font) somewhere in the first 512 bytes of
/// memory, which are reserved for the interpreter, but the actual memory location doesn't matter.
/// It's common to put it at either address 0 or 80 (`0x50`).
pub fn get_font_data(font: OctoFont) -> ([u8; 5 * 16], Option<Vec<u8>>) {
    match font {
        OctoFont::Octo => (
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
        OctoFont::Vip => (
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
        OctoFont::Dream6800 => (
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
        OctoFont::Eti660 => (
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
        OctoFont::Schip => (
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
        OctoFont::Fish => (
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
    }
}

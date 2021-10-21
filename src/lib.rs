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

/// Represents the different fonts a CHIP-8 interpreter can provide.
/// TODO: Provide the actual fonts too.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OctoFont {
    /// The font used by [Octo](https://github.com/JohnEarnest).
    /// Contains small digits for 0–F, as well as big digits for 0–F.
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
    /// 0–F, and big digits for 0–9.
    Schip,
    /// Custom font used by the Fish'n'Chips CHIP-8 emulator. Contains small digits
    /// for 0–F and big digits for 0–F.
    Fish,
}

impl Default for OctoFont {
    fn default() -> Self {
        Self::Octo
    }
}

/// If the CHIP-8 interpreter supports custom colors for visual elements, it can use these values
/// for setting them.
#[derive(Serialize, Deserialize)]
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

/// Represents the different touch modes support by [Octo](https://github.com/JohnEarnest/Octo).
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

/// Represents the different possible behaviors of attempting to draw a sprite with 0 height with
/// the instruction DXY0 while in lores (low-resolution 64x32) mode.
#[derive(Serialize, Deserialize)]
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

impl Default for OctoTouchMode {
    fn default() -> Self {
        Self::None
    }
}

/// Representation of Octo options.
#[derive(Serialize, Deserialize)]
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

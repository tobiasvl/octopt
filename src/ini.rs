use crate::color::Color;
use crate::*;
//use crate::{some_bool_from_int, some_u16_from_int_or_str};
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::*;
use serde_with::skip_serializing_none;
use std::fmt;
use std::str::FromStr;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct OctoColorsIni {
    #[serde(rename = "colors.plane1", serialize_with = "without_hash")]
    fill_color: Option<Color>,
    #[serde(rename = "colors.plane2", serialize_with = "without_hash")]
    fill_color2: Option<Color>,
    #[serde(rename = "colors.plane3", serialize_with = "without_hash")]
    blend_color: Option<Color>,
    #[serde(rename = "colors.plane0", serialize_with = "without_hash")]
    background_color: Option<Color>,
    #[serde(rename = "colors.sound", serialize_with = "without_hash")]
    buzz_color: Option<Color>,
    #[serde(rename = "colors.background", serialize_with = "without_hash")]
    quiet_color: Option<Color>,
}

fn without_hash<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let color_str = color.as_ref().unwrap().to_string();
    serializer.serialize_str(if color_str.starts_with('#') {
        &color_str[1..]
    } else {
        &color_str
    })
}

impl Default for OctoColorsIni {
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

impl From<crate::OctoColors> for OctoColorsIni {
    fn from(colors: crate::OctoColors) -> Self {
        Self {
            fill_color: colors.fill_color,
            fill_color2: colors.fill_color2,
            blend_color: colors.blend_color,
            background_color: colors.background_color,
            buzz_color: colors.buzz_color,
            quiet_color: colors.quiet_color,
        }
    }
}

impl From<crate::OctoColorsIni> for OctoColors {
    fn from(colors: crate::OctoColorsIni) -> Self {
        Self {
            fill_color: colors.fill_color,
            fill_color2: colors.fill_color2,
            blend_color: colors.blend_color,
            background_color: colors.background_color,
            buzz_color: colors.buzz_color,
            quiet_color: colors.quiet_color,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OctoTouchModeIni {
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    Vip,
}

impl From<crate::OctoTouchMode> for OctoTouchModeIni {
    fn from(touch: crate::OctoTouchMode) -> Self {
        match touch {
            crate::OctoTouchMode::None => Self::None,
            crate::OctoTouchMode::Swipe => Self::Swipe,
            crate::OctoTouchMode::Seg16 => Self::Seg16,
            crate::OctoTouchMode::Seg16Fill => Self::Seg16Fill,
            crate::OctoTouchMode::Gamepad => Self::Gamepad,
            crate::OctoTouchMode::Vip => Self::Vip,
        }
    }
}

impl From<crate::OctoTouchModeIni> for OctoTouchMode {
    fn from(touch: crate::OctoTouchModeIni) -> Self {
        match touch {
            crate::OctoTouchModeIni::None => Self::None,
            crate::OctoTouchModeIni::Swipe => Self::Swipe,
            crate::OctoTouchModeIni::Seg16 => Self::Seg16,
            crate::OctoTouchModeIni::Seg16Fill => Self::Seg16Fill,
            crate::OctoTouchModeIni::Gamepad => Self::Gamepad,
            crate::OctoTouchModeIni::Vip => Self::Vip,
        }
    }
}

impl Default for OctoTouchModeIni {
    fn default() -> Self {
        Self::None
    }
}

impl Default for ScreenRotationIni {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for OctoFontIni {
    fn default() -> Self {
        Self::Octo
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct OctoQuirksIni {
    #[serde(
        rename = "quirks.shift",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    shift: Option<bool>,
    #[serde(
        rename = "quirks.loadstore",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    load_store: Option<bool>,
    #[serde(
        rename = "quirks.jump0",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    jump0: Option<bool>,
    #[serde(
        rename = "quirks.logic",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    logic: Option<bool>,
    #[serde(
        rename = "quirks.clip",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    clip: Option<bool>,
    #[serde(
        rename = "quirks.vblank",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    vblank: Option<bool>,
    #[serde(
        rename = "quirks.vforder",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    vf_order: Option<bool>,
    #[serde(rename = "quirks.lores_dxy0")]
    lores_dxy0: Option<crate::LoResDxy0Behavior>,
    #[serde(
        rename = "quirks.resclear",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    res_clear: Option<bool>,
    #[serde(
        rename = "quirks.delaywrap",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    delay_wrap: Option<bool>,
    #[serde(
        rename = "quirks.hirescollision",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    hires_collision: Option<bool>,
    #[serde(
        rename = "quirks.clipcollision",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    clip_collision: Option<bool>,
    #[serde(
        rename = "quirks.scroll",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    scroll: Option<bool>,
    #[serde(
        rename = "quirks.overflow_i",
        deserialize_with = "some_bool_from_int",
        serialize_with = "int_from_some_bool",
        default
    )]
    overflow_i: Option<bool>,
}

impl From<crate::OctoQuirks> for OctoQuirksIni {
    fn from(quirks: crate::OctoQuirks) -> Self {
        Self {
            shift: quirks.shift,
            load_store: quirks.load_store,
            jump0: quirks.jump0,
            logic: quirks.logic,
            clip: quirks.clip,
            vblank: quirks.vblank,
            lores_dxy0: quirks.lores_dxy0,
            res_clear: quirks.res_clear,
            delay_wrap: quirks.delay_wrap,
            hires_collision: quirks.hires_collision,
            scroll: quirks.scroll,
            overflow_i: quirks.overflow_i,
            clip_collision: quirks.clip_collision,
            vf_order: quirks.vf_order,
        }
    }
}

impl From<crate::OctoQuirksIni> for OctoQuirks {
    fn from(quirks: crate::OctoQuirksIni) -> Self {
        Self {
            shift: quirks.shift,
            load_store: quirks.load_store,
            jump0: quirks.jump0,
            logic: quirks.logic,
            clip: quirks.clip,
            vblank: quirks.vblank,
            lores_dxy0: quirks.lores_dxy0,
            res_clear: quirks.res_clear,
            delay_wrap: quirks.delay_wrap,
            hires_collision: quirks.hires_collision,
            scroll: quirks.scroll,
            overflow_i: quirks.overflow_i,
            clip_collision: quirks.clip_collision,
            vf_order: quirks.vf_order,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct OctoOptionsIni {
    #[serde(default, rename = "core.tickrate")]
    tickrate: Option<u16>,
    #[serde(default, rename = "core.max_rom")]
    max_size: Option<u16>,
    #[serde(default, rename = "core.rotation")]
    screen_rotation: ScreenRotationIni,
    #[serde(default, rename = "core.font")]
    font_style: OctoFontIni,
    #[serde(default, rename = "core.touch_mode")]
    touch_input_mode: OctoTouchModeIni,
    #[serde(default, rename = "core.start_address")]
    start_address: Option<u16>,

    #[serde(flatten)]
    colors: OctoColorsIni,

    #[serde(flatten)]
    quirks: OctoQuirksIni,
}

impl From<crate::OctoOptions> for OctoOptionsIni {
    fn from(options: crate::OctoOptions) -> Self {
        Self {
            tickrate: options.tickrate,
            max_size: options.max_size,
            screen_rotation: ScreenRotationIni::from(options.screen_rotation),
            font_style: OctoFontIni::from(options.font_style),
            touch_input_mode: OctoTouchModeIni::from(options.touch_input_mode),
            start_address: options.start_address,
            colors: OctoColorsIni::from(options.colors),
            quirks: OctoQuirksIni::from(options.quirks),
        }
    }
}

impl From<crate::OctoOptionsIni> for OctoOptions {
    fn from(options: crate::OctoOptionsIni) -> Self {
        Self {
            tickrate: options.tickrate,
            max_size: options.max_size,
            screen_rotation: ScreenRotation::from(options.screen_rotation),
            font_style: OctoFont::from(options.font_style),
            touch_input_mode: OctoTouchMode::from(options.touch_input_mode),
            start_address: options.start_address,
            colors: OctoColors::from(options.colors),
            quirks: OctoQuirks::from(options.quirks),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub(crate) enum ScreenRotationIni {
    Normal = 0,
    ClockWise = 90,
    UpsideDown = 180,
    CounterClockWise = 270,
}

impl From<crate::ScreenRotation> for ScreenRotationIni {
    fn from(rotation: crate::ScreenRotation) -> Self {
        match rotation {
            crate::ScreenRotation::Normal => Self::Normal,
            crate::ScreenRotation::ClockWise => Self::ClockWise,
            crate::ScreenRotation::CounterClockWise => Self::CounterClockWise,
            crate::ScreenRotation::UpsideDown => Self::UpsideDown,
        }
    }
}

impl From<crate::ScreenRotationIni> for ScreenRotation {
    fn from(rotation: crate::ScreenRotationIni) -> Self {
        match rotation {
            crate::ScreenRotationIni::Normal => Self::Normal,
            crate::ScreenRotationIni::ClockWise => Self::ClockWise,
            crate::ScreenRotationIni::CounterClockWise => Self::CounterClockWise,
            crate::ScreenRotationIni::UpsideDown => Self::UpsideDown,
        }
    }
}

/// Deserializes OctoOptions from a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl FromStr for OctoOptionsIni {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// Serializes OctoOptions into a JSON string.
///
/// This format is used by Octo in OctoCarts and HTML exports, as well as the Chip-8 Archive.
impl fmt::Display for OctoOptionsIni {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(string) => write!(f, "{}", string),
            _ => Err(fmt::Error),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OctoFontIni {
    Octo,
    Vip,
    Dream6800,
    Eti660,
    Schip,
    Fish,
    AKouZ1,
}

impl From<crate::OctoFont> for OctoFontIni {
    fn from(font: crate::OctoFont) -> Self {
        match font {
            crate::OctoFont::Octo => Self::Octo,
            crate::OctoFont::Vip => Self::Vip,
            crate::OctoFont::Dream6800 => Self::Dream6800,
            crate::OctoFont::Eti660 => Self::Eti660,
            crate::OctoFont::Schip => Self::Schip,
            crate::OctoFont::Fish => Self::Fish,
            crate::OctoFont::AKouZ1 => Self::AKouZ1,
        }
    }
}

impl From<crate::OctoFontIni> for OctoFont {
    fn from(font: crate::OctoFontIni) -> Self {
        match font {
            crate::OctoFontIni::Octo => Self::Octo,
            crate::OctoFontIni::Vip => Self::Vip,
            crate::OctoFontIni::Dream6800 => Self::Dream6800,
            crate::OctoFontIni::Eti660 => Self::Eti660,
            crate::OctoFontIni::Schip => Self::Schip,
            crate::OctoFontIni::Fish => Self::Fish,
            crate::OctoFontIni::AKouZ1 => Self::AKouZ1,
        }
    }
}

fn int_from_some_bool<S>(some_bool: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // This function will only be called during serialization, and we know that the argument isn't
    // None because we skip serializing these fields if they're None, so we simply unwrap here. If this
    // panics, some serde derive is missing, or we're using this function for something it's not
    // intended.
    serializer.serialize_u8(if some_bool.unwrap() { 1 } else { 0 })
}

fn some_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match (String::deserialize(deserializer)?).parse::<u8>().unwrap() {
        1 => Ok(Some(true)),
        0 => Ok(Some(false)),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

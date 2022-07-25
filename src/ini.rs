use crate::color::Color;
use crate::{u8, Colors, Font, LoResDxy0Behavior, Options, Quirks, ScreenRotation, TouchMode};
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use std::fmt;
use std::str::FromStr;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ColorsIni {
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
    serializer.serialize_str(if let Some(stripped) = color_str.strip_prefix('#') {
        stripped
    } else {
        &color_str
    })
}

impl Default for ColorsIni {
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

impl From<Colors> for ColorsIni {
    fn from(colors: Colors) -> Self {
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

impl From<ColorsIni> for Colors {
    fn from(colors: ColorsIni) -> Self {
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
pub(crate) enum TouchModeIni {
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    Vip,
}

impl From<TouchMode> for TouchModeIni {
    fn from(touch: TouchMode) -> Self {
        match touch {
            TouchMode::None => Self::None,
            TouchMode::Swipe => Self::Swipe,
            TouchMode::Seg16 => Self::Seg16,
            TouchMode::Seg16Fill => Self::Seg16Fill,
            TouchMode::Gamepad => Self::Gamepad,
            TouchMode::Vip => Self::Vip,
        }
    }
}

impl From<TouchModeIni> for TouchMode {
    fn from(touch: TouchModeIni) -> Self {
        match touch {
            TouchModeIni::None => Self::None,
            TouchModeIni::Swipe => Self::Swipe,
            TouchModeIni::Seg16 => Self::Seg16,
            TouchModeIni::Seg16Fill => Self::Seg16Fill,
            TouchModeIni::Gamepad => Self::Gamepad,
            TouchModeIni::Vip => Self::Vip,
        }
    }
}

impl Default for TouchModeIni {
    fn default() -> Self {
        Self::None
    }
}

impl Default for ScreenRotationIni {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for FontIni {
    fn default() -> Self {
        Self::Octo
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct QuirksIni {
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
    lores_dxy0: Option<LoResDxy0Behavior>,
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

impl From<Quirks> for QuirksIni {
    fn from(quirks: Quirks) -> Self {
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

impl From<QuirksIni> for Quirks {
    fn from(quirks: QuirksIni) -> Self {
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
pub(crate) struct OptionsIni {
    #[serde(default, rename = "core.tickrate")]
    tickrate: Option<u16>,
    #[serde(default, rename = "core.max_rom")]
    max_size: Option<u16>,
    #[serde(default, rename = "core.rotation")]
    screen_rotation: ScreenRotationIni,
    #[serde(default, rename = "core.font")]
    font_style: FontIni,
    #[serde(default, rename = "core.touch_mode")]
    touch_input_mode: TouchModeIni,
    #[serde(default, rename = "core.start_address")]
    start_address: Option<u16>,

    #[serde(flatten)]
    colors: ColorsIni,

    #[serde(flatten)]
    quirks: QuirksIni,
}

impl From<Options> for OptionsIni {
    fn from(options: Options) -> Self {
        Self {
            tickrate: options.tickrate,
            max_size: options.max_size,
            screen_rotation: ScreenRotationIni::from(options.screen_rotation),
            font_style: FontIni::from(options.font_style),
            touch_input_mode: TouchModeIni::from(options.touch_input_mode),
            start_address: options.start_address,
            colors: ColorsIni::from(options.colors),
            quirks: QuirksIni::from(options.quirks),
        }
    }
}

impl From<OptionsIni> for Options {
    fn from(options: OptionsIni) -> Self {
        Self {
            tickrate: options.tickrate,
            max_size: options.max_size,
            screen_rotation: ScreenRotation::from(options.screen_rotation),
            font_style: Font::from(options.font_style),
            touch_input_mode: TouchMode::from(options.touch_input_mode),
            start_address: options.start_address,
            colors: Colors::from(options.colors),
            quirks: Quirks::from(options.quirks),
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

impl From<ScreenRotation> for ScreenRotationIni {
    fn from(rotation: ScreenRotation) -> Self {
        match rotation {
            ScreenRotation::Normal => Self::Normal,
            ScreenRotation::ClockWise => Self::ClockWise,
            ScreenRotation::CounterClockWise => Self::CounterClockWise,
            ScreenRotation::UpsideDown => Self::UpsideDown,
        }
    }
}

impl From<ScreenRotationIni> for ScreenRotation {
    fn from(rotation: ScreenRotationIni) -> Self {
        match rotation {
            ScreenRotationIni::Normal => Self::Normal,
            ScreenRotationIni::ClockWise => Self::ClockWise,
            ScreenRotationIni::CounterClockWise => Self::CounterClockWise,
            ScreenRotationIni::UpsideDown => Self::UpsideDown,
        }
    }
}

/// Deserializes Options from a JSON string.
///
/// This format is used by Octo in Octocarts and HTML exports, as well as the Chip-8 Archive.
impl FromStr for OptionsIni {
    type Err = serde_ini::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_ini::from_str(s)
    }
}

/// Serializes Options into a JSON string.
///
/// This format is used by Octo in Octocarts and HTML exports, as well as the Chip-8 Archive.
impl fmt::Display for OptionsIni {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_ini::to_string(self) {
            Ok(string) => write!(f, "{}", string),
            _ => Err(fmt::Error),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FontIni {
    Octo,
    Vip,
    Dream6800,
    Eti660,
    Schip,
    Fish,
    AKouZ1,
}

impl From<Font> for FontIni {
    fn from(font: Font) -> Self {
        match font {
            Font::Octo => Self::Octo,
            Font::Vip => Self::Vip,
            Font::Dream6800 => Self::Dream6800,
            Font::Eti660 => Self::Eti660,
            Font::Schip => Self::Schip,
            Font::Fish => Self::Fish,
            Font::AKouZ1 => Self::AKouZ1,
        }
    }
}

impl From<FontIni> for Font {
    fn from(font: FontIni) -> Self {
        match font {
            FontIni::Octo => Self::Octo,
            FontIni::Vip => Self::Vip,
            FontIni::Dream6800 => Self::Dream6800,
            FontIni::Eti660 => Self::Eti660,
            FontIni::Schip => Self::Schip,
            FontIni::Fish => Self::Fish,
            FontIni::AKouZ1 => Self::AKouZ1,
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
            Unexpected::Unsigned(u64::from(other)),
            &"zero or one",
        )),
    }
}

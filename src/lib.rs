#![warn(missing_docs)]

mod color;
use color::Color;
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use std::u8;

/// Octo font variants
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum OctoFont {
    Octo,
    Vip,
    Dream6800,
    Eti660,
    Schip,
    Fish,
}

impl Default for OctoFont {
    fn default() -> Self {
        Self::Octo
    }
}

/// Octo touch input mode
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum OctoTouchMode {
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    Vip,
}

/// Representation of Octo options
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OctoOptions {
    // core settings
    tickrate: u32,                   // {7,15,20,30,100,200,500,1000,10000,...}
    max_size: u32,                   // {3216, 3583, 3584, 65024}
    screen_rotation: u32,            // {0, 90, 180, 270}
    font_style: OctoFont,            // OCTO_FONT_...
    touch_input_mode: OctoTouchMode, // OCTO_TOUCH_...

    fill_color: Color,
    fill_color2: Color,
    blend_color: Color,
    background_color: Color,
    buzz_color: Color,
    quiet_color: Color,

    // quirks flags
    #[serde(
        rename = "shiftQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_shift: bool,
    #[serde(
        rename = "loadStoreQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_loadstore: bool,
    #[serde(
        rename = "jumpQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_jump0: bool,
    #[serde(
        rename = "logicQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_logic: bool,
    #[serde(
        rename = "clipQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_clip: bool,
    #[serde(
        rename = "vBlankQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_vblank: bool,
    #[serde(
        rename = "vfOrderQuirks",
        serialize_with = "int_from_bool",
        deserialize_with = "bool_from_int"
    )]
    q_vforder: bool,
}

impl FromStr for OctoOptions {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

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

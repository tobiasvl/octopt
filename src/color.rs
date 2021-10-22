//! An RGB color triplet struct that can be used with [`serde`].
//!
//! Currently just an ugly, hacky wrapper around the crate [`css_color_parser2`] to make it support
//! hexadecimal strings with or without a leading # as well as CSS color names, but as an RGB
//! struct rather than an RGBA struct.

use css_color_parser2::{Color as CssColor, ColorParseError};
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// An RGB color which can be serialized into and deserialized from a hexadecimal color string.
///
/// Example:
/// ```
/// use octopt::color::Color;
/// use std::str::FromStr;
///
/// let red = Color { r: 255, g: 0, b: 0 };
/// assert_eq!(format!("{}", red), "#FF0000");
/// assert_eq!("#FF0000".parse::<Color>().unwrap(), red);
/// ```
#[derive(Default, Debug, PartialEq)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Blue
    pub g: u8,
    /// Green
    pub b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex RGB number between #000000 and #FFFFFF")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Color::from_str(value) {
            Ok(color) => Ok(color),
            _ => Err(E::custom(format!("Failed to parse hex color: {}", value))),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl FromStr for Color {
    type Err = ColorParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let css_color = match CssColor::from_str(s) {
            Ok(css_color) => css_color,
            Err(_) => CssColor::from_str(&format!("#{}", s))?,
        };

        Ok(Color {
            r: css_color.r,
            g: css_color.g,
            b: css_color.b,
        })
    }
}

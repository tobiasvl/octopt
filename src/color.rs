use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
// This is kind of silly, and probably far from perfect, but it was a nice exercise and I made it
// from scratch (just looking at serde docs).
// TODO: Compare with this similar (but probably better) implementation:
// https://docs.rs/twitchchat/0.6.3/src/twitchchat/twitch/color.rs.html

/// An RGB color triplet struct that can be used with [`serde_json`](serde_json).
#[derive(Default, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
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
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];
        let mut cur = if let Some(s) = s.strip_prefix('#') {
            s
        } else {
            &s
        };
        while !cur.is_empty() {
            let (chunk, rest) = cur.split_at(std::cmp::min(2, cur.len()));
            v.push(chunk);
            cur = rest;
        }

        let rgb = Color {
            r: u8::from_str_radix(v[0], 16)?,
            g: u8::from_str_radix(v[1], 16)?,
            b: u8::from_str_radix(v[2], 16)?,
        };

        Ok(rgb)
    }
}

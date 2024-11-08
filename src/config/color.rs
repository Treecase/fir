use serde::{de::Visitor, Deserialize};

/// Color struct used in the config file.
///
/// The color struct lets us abstract away from a specific color specification format and
/// generalize the concept for any possible color values. At present, it only has methods for
/// 8-bit RGB.
///
/// Since this struct is used primarily by the config file, it also implements the
/// [`serde::Deserialize`] trait. Currently, only 6-digit hexadecimal format (eg. #88ff00) is
/// allowed.
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Construct a new color from 8-bit red, green, and blue components.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Decompose the struct into an 8-bit RGB triple.
    pub fn as_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(ColorVisitor)
    }
}

struct ColorVisitor;
impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing an RGB value in format #rrggbb")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        const ERRMSG: &str = "colors must be in RGB hex code format \"#rrggbb\"";
        let r = u8::from_str_radix(&v[1..3], 16).map_err(|_| E::custom(ERRMSG))?;
        let g = u8::from_str_radix(&v[3..5], 16).map_err(|_| E::custom(ERRMSG))?;
        let b = u8::from_str_radix(&v[5..7], 16).map_err(|_| E::custom(ERRMSG))?;
        Ok(Color::from_rgb(r, g, b))
    }
}

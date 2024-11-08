use derive_more::From;
use sdl2::keyboard::Keycode;
use serde::{de::Visitor, Deserialize};

/// A struct for naming keys.
///
/// Presently, this is just a NewType wrapper around an [`sdl2::keyboard::Keycode`].
#[derive(Clone, Copy, Debug, From, PartialEq, Eq, Hash)]
pub struct KeyBind(pub Keycode);

impl std::str::FromStr for KeyBind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Keycode::from_name(s)
            .map(KeyBind::from)
            .ok_or("not a valid key name")
    }
}

impl<'de> Deserialize<'de> for KeyBind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(KeyBindVisitor)
    }
}

struct KeyBindVisitor;
impl<'de> Visitor<'de> for KeyBindVisitor {
    type Value = KeyBind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a key name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // TODO: Not all SDL key names are valid toml keys (eg. "Keypad +"). It would be better to
        // use XKB keysym names in the future.
        log::info!("v: '{v}' = '{}'", v.replace("_", " "));
        v.replace("_", " ").parse().map_err(E::custom)
    }
}

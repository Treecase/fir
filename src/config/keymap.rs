use sdl2::keyboard::Keycode;
use serde::de::Visitor;
use serde::Deserialize;

use super::Action;
use super::KeyBind;
use std::collections::HashMap;

/// Maps keys to actions.
///
/// The default keybinds are:
/// - Up Arrow => Shift Up
/// - Down Arrow => Shift Down
/// - Left Arrow => Shift Left
/// - Right Arrow => Shift Right
/// - Keypad 0 => Reset Transform
/// - Keypad Plus => Zoom In
/// - Keypad Minus => Zoom Out
/// - F => Toggle Fullscreen
/// - Q => Quit
#[derive(Clone, Debug)]
pub struct KeyMap {
    /// Binds a key to an action.
    binds: HashMap<KeyBind, Action>,
}

impl KeyMap {
    /// Get the [`Action`] bound to a given key.
    pub fn get(&self, bind: &KeyBind) -> Option<&Action> {
        self.binds.get(bind)
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            binds: HashMap::from([
                (KeyBind(Keycode::Up), Action::ShiftUp),
                (KeyBind(Keycode::Down), Action::ShiftDown),
                (KeyBind(Keycode::Left), Action::ShiftLeft),
                (KeyBind(Keycode::Right), Action::ShiftRight),
                (KeyBind(Keycode::Kp0), Action::ResetTransform),
                (KeyBind(Keycode::KpPlus), Action::ZoomIn),
                (KeyBind(Keycode::KpMinus), Action::ZoomOut),
                (KeyBind(Keycode::F), Action::ToggleFullscreen),
                (KeyBind(Keycode::Q), Action::Quit),
            ]),
        }
    }
}

impl<'de> Deserialize<'de> for KeyMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(KeyMapVisitor)
    }
}

struct KeyMapVisitor;
impl<'de> Visitor<'de> for KeyMapVisitor {
    type Value = KeyMap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a mapping of key names to actions")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut km = Self::Value::default();
        loop {
            match map.next_entry::<KeyBind, Action>() {
                Ok(Some((k, v))) => km.binds.insert(k, v),
                Ok(None) => break,
                Err(e) => return Err(e),
            };
        }
        Ok(km)
    }
}

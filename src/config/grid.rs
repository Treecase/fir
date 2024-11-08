use super::color::Color;
use serde::Deserialize;

/// Configuration options for the background transparency grid.
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Grid {
    /// Size of background grid squares in pixels. (default 16)
    pub size: u32,
    /// Color of the darker colored grid squares. (default #404040)
    pub color_dark: Color,
    /// Color of the lighter colored grid squares. (default #808080)
    pub color_light: Color,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            size: 16,
            color_dark: Color::from_rgb(0x40, 0x40, 0x40),
            color_light: Color::from_rgb(0x80, 0x80, 0x80),
        }
    }
}

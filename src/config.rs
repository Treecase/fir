//! Configuration file data types.

use serde::Deserialize;

mod action;
mod color;
mod grid;
mod keymap;
mod keys;

pub use action::Action;
pub use color::Color;
pub use grid::Grid;
pub use keymap::KeyMap;
pub use keys::KeyBind;

/// Top-level structure of a configuration file.
///
/// The config file is named `config.toml` and is located in the directory
/// `$XDG_CONFIG_HOME/fir`[^xdg].
///
/// [^xdg]: `$XDG_CONFIG_HOME` defaults to `$HOME/.config`. See the [`XDG basedir
/// specification`](https://specifications.freedesktop.org/basedir-spec/latest)
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Distance in pixels the shift actions will move the image. (default 16)
    pub shift_amount: u16,
    /// Background transparency grid settings.
    pub grid: Grid,
    /// Keybind definitions.
    pub binds: KeyMap,
}

impl Config {
    /// Attempts to load the config file.
    ///
    /// Fir will always set its default settings and keybinds first. Values set in the config file
    /// can override these defaults or--in the case of keybinds--add to them.
    pub fn from_config_toml() -> Config {
        xdg::BaseDirectories::with_prefix(crate::meta::NAME).map_or_else(
            |err| {
                log::error!(target: "config", "xdg::BaseDirectories failed: {err}");
                None
            },
            |xdg_dirs| {
                let path = xdg_dirs.get_config_file("config.toml");
                std::fs::read_to_string(&path).map_or_else(
                    |err| {
                        log::info!(target: "config", "failed to read \"{}\": {err}", path.display());
                        None
                    },
                    |s| {
                        toml::from_str(&s)
                            .map_err(|err| log::error!(target: "config", "{err}"))
                            .ok()
                    },
                )
            },
        ).unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shift_amount: 16,
            grid: Grid::default(),
            binds: KeyMap::default(),
        }
    }
}

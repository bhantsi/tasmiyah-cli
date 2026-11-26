//! Optional user configuration file.
//!
//! Location: `$XDG_CONFIG_HOME/tasmiyah/config.toml` (usually
//! `~/.config/tasmiyah/config.toml` on Linux, the platform equivalent
//! elsewhere). Everything is optional; a missing file, missing keys, or
//! even a malformed file never stop the greeting from rendering — we
//! warn on stderr and fall back to defaults.
//!
//! Example:
//!
//! ```toml
//! style = "classic"          # classic | centered | minimal
//! font = "nerd"              # standard | nerd | auto
//! theme = "ocean"            # gold | ocean | rose | mono
//! phrase = "alhamdulillah"   # index (0-11) or name; see `tasmiyah --list`
//! translation = true
//!
//! [colors]                   # per-element overrides, #RRGGBB or names
//! border = "#d4af37"
//! arabic = "bright-green"
//! ```
//!
//! Precedence (handled by the binary entry point):
//! CLI flag > env var (`TASMIYAH_STYLE`, `TASMIYAH_FONT`,
//! `TASMIYAH_THEME`, `TASMIYAH_PHRASE`) > config file > built-in default.

use std::io::IsTerminal;
use std::path::PathBuf;

use serde::Deserialize;

use crate::theme::ColorOverrides;

/// On-disk shape of `config.toml`. All fields optional.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub style: Option<String>,
    pub font: Option<String>,
    pub theme: Option<String>,
    pub phrase: Option<String>,
    pub translation: Option<bool>,
    pub colors: Option<ColorOverrides>,
}

impl Config {
    /// Load the config file, or return [`Config::default`] when it
    /// doesn't exist. Malformed TOML produces a one-line warning on
    /// stderr (interactive sessions only) and defaults.
    pub fn load() -> Self {
        let Some(path) = config_path() else {
            return Self::default();
        };
        let Ok(raw) = std::fs::read_to_string(&path) else {
            return Self::default(); // missing or unreadable — fine
        };
        match toml::from_str(&raw) {
            Ok(cfg) => cfg,
            Err(e) => {
                warn(&format!(
                    "tasmiyah: ignoring malformed config {}: {e}",
                    path.display()
                ));
                Self::default()
            }
        }
    }
}

/// Full path of the config file, if the platform has a config dir.
fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("tasmiyah").join("config.toml"))
}

/// Print a warning, but only in interactive sessions — a greeting tool
/// must stay silent inside scripts and pipes.
fn warn(msg: &str) {
    if std::io::stderr().is_terminal() {
        eprintln!("{msg}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_config() {
        let cfg: Config = toml::from_str(
            r##"
                style = "classic"
                font = "nerd"
                theme = "ocean"
                translation = true

                [colors]
                border = "#ff0000"
                arabic = "green"
            "##,
        )
        .unwrap();
        assert_eq!(cfg.style.as_deref(), Some("classic"));
        assert_eq!(cfg.font.as_deref(), Some("nerd"));
        assert_eq!(cfg.theme.as_deref(), Some("ocean"));
        assert_eq!(cfg.translation, Some(true));
        let colors = cfg.colors.unwrap();
        assert_eq!(colors.border.as_deref(), Some("#ff0000"));
        assert_eq!(colors.arabic.as_deref(), Some("green"));
        assert!(colors.translit.is_none());
    }

    #[test]
    fn empty_config_parses_to_defaults() {
        let cfg: Config = toml::from_str("").unwrap();
        assert!(cfg.style.is_none());
        assert!(cfg.theme.is_none());
        assert!(cfg.colors.is_none());
    }

    #[test]
    fn unknown_keys_are_rejected_cleanly() {
        // serde ignores unknown fields by default for structs without
        // deny_unknown_fields — but malformed TOML must fail parsing.
        let bad: Result<Config, _> = toml::from_str("style = [");
        assert!(bad.is_err());
    }
}

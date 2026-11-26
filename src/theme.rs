//! Color theme + color-mode detection.
//!
//! A `Theme` holds the four colors used by the banner. When the user
//! disables colors (via `--no-color`, the `NO_COLOR` env var, `TERM=dumb`,
//! or because stdout is not a TTY), we hand back a "plain" theme whose
//! every slot is `Color::Reset` — the renderer stays branch-free.
//!
//! ## Presets and custom colors
//!
//! When colors are enabled, the palette comes from one of the built-in
//! [`ThemePreset`]s, optionally overridden slot-by-slot by a
//! [`ColorOverrides`] struct parsed from the config file's `[colors]`
//! table. Selection precedence is handled by the caller (CLI flag >
//! `TASMIYAH_THEME` env var > config file > default `gold`).

use std::io::IsTerminal;

use clap::ValueEnum;
use crossterm::style::Color;
use serde::Deserialize;

use crate::cli::Args;

/// Colors applied to the four kinds of text in the banner.
///
/// When `enabled` is false, the renderer is expected to skip all
/// styling — even `Color::Reset` writes an ANSI escape, so the only
/// way to produce truly clean output is to not call `Stylize` at all.
pub struct Theme {
    pub enabled: bool,
    pub border: Color,
    pub arabic: Color,
    pub translit: Color,
    pub english: Color,
}

/// One of the built-in color palettes.
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreset {
    /// Warm gold border, green Arabic — the original look. *(Default.)*
    #[default]
    Gold,
    /// Teal/deep-blue palette reminiscent of night water.
    Ocean,
    /// Soft rose/pink palette.
    Rose,
    /// No per-element colors — everything in the terminal's default
    /// foreground (useful on terminals with carefully tuned palettes).
    Mono,
}

impl ThemePreset {
    /// The four palette colors for this preset.
    fn palette(self) -> (Color, Color, Color, Color) {
        match self {
            // Gold (matches a warm "Islamic manuscript" feel).
            ThemePreset::Gold => (
                Color::Rgb {
                    r: 218,
                    g: 165,
                    b: 32,
                },
                Color::Green,
                Color::Yellow,
                Color::White,
            ),
            ThemePreset::Ocean => (
                Color::Rgb {
                    r: 64,
                    g: 145,
                    b: 160,
                },
                Color::Cyan,
                Color::Blue,
                Color::White,
            ),
            ThemePreset::Rose => (
                Color::Rgb {
                    r: 219,
                    g: 112,
                    b: 147,
                },
                Color::Magenta,
                Color::Rgb {
                    r: 255,
                    g: 182,
                    b: 193,
                },
                Color::White,
            ),
            ThemePreset::Mono => (Color::Reset, Color::Reset, Color::Reset, Color::Reset),
        }
    }

    /// Parse a preset name (used for `TASMIYAH_THEME` and the config
    /// file, which bypass clap's value parsing). Case-insensitive.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_lowercase().as_str() {
            "gold" => Some(Self::Gold),
            "ocean" => Some(Self::Ocean),
            "rose" => Some(Self::Rose),
            "mono" => Some(Self::Mono),
            _ => None,
        }
    }
}

/// Per-element color overrides from the config file's `[colors]` table.
///
/// Every field is optional; a set field replaces the corresponding slot
/// of the active preset. Values are `#RRGGBB` hex strings or common
/// color names (`red`, `bright-green`, …).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ColorOverrides {
    pub border: Option<String>,
    pub arabic: Option<String>,
    pub translit: Option<String>,
    pub english: Option<String>,
}

impl Theme {
    /// Build a colored theme from a preset, with any custom overrides
    /// applied. Unparseable override values are ignored (the preset
    /// color stays) — config errors should never break the greeting.
    pub fn themed(preset: ThemePreset, overrides: Option<&ColorOverrides>) -> Self {
        let (mut border, mut arabic, mut translit, mut english) = preset.palette();
        if let Some(o) = overrides {
            if let Some(c) = o.border.as_deref().and_then(parse_color) {
                border = c;
            }
            if let Some(c) = o.arabic.as_deref().and_then(parse_color) {
                arabic = c;
            }
            if let Some(c) = o.translit.as_deref().and_then(parse_color) {
                translit = c;
            }
            if let Some(c) = o.english.as_deref().and_then(parse_color) {
                english = c;
            }
        }
        Self {
            enabled: true,
            border,
            arabic,
            translit,
            english,
        }
    }

    /// A no-op theme: colors are placeholders; the renderer must check
    /// `enabled` and write raw text when this theme is in effect.
    pub fn plain() -> Self {
        Self {
            enabled: false,
            border: Color::Reset,
            arabic: Color::Reset,
            translit: Color::Reset,
            english: Color::Reset,
        }
    }

    /// Pick the right theme based on user flags + environment, using
    /// the given preset and overrides (already resolved by the caller
    /// with the CLI > env > config > default precedence).
    pub fn for_args(args: &Args, preset: ThemePreset, overrides: Option<&ColorOverrides>) -> Self {
        if should_use_color(args) {
            Self::themed(preset, overrides)
        } else {
            Self::plain()
        }
    }
}

/// Decide whether to emit ANSI color escapes.
///
/// We say "no colors" when ANY of the following is true:
/// * `--no-color` was passed,
/// * the `NO_COLOR` env var is set to any value (see <https://no-color.org>),
/// * `TERM=dumb` (a terminal that cannot interpret escapes),
/// * stdout is not a TTY (e.g. piped into another program or redirected).
fn should_use_color(args: &Args) -> bool {
    if args.no_color {
        return false;
    }
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false) {
        return false;
    }
    if !std::io::stdout().is_terminal() {
        return false;
    }
    true
}

/// Parse a color string into a [`Color`].
///
/// Accepts `#RRGGBB` / `RRGGBB` hex triplets and a handful of named
/// colors (including `bright-*` and `grey`/`gray` spellings). Returns
/// `None` for anything unrecognized.
fn parse_color(raw: &str) -> Option<Color> {
    let s = raw.trim().to_lowercase();
    let named = match s.as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "grey" | "gray" => Some(Color::Grey),
        "dark-grey" | "dark-gray" | "darkgrey" | "darkgray" => Some(Color::DarkGrey),
        "bright-black" => Some(Color::DarkGrey),
        "bright-red" => Some(Color::Red),
        "bright-green" => Some(Color::Green),
        "bright-yellow" => Some(Color::Yellow),
        "bright-blue" => Some(Color::Blue),
        "bright-magenta" => Some(Color::Magenta),
        "bright-cyan" => Some(Color::Cyan),
        "bright-white" => Some(Color::White),
        _ => None,
    };
    if named.is_some() {
        return named;
    }
    let hex = s.strip_prefix('#').unwrap_or(&s);
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(Color::Rgb { r, g, b });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_theme_is_disabled() {
        let t = Theme::plain();
        assert!(!t.enabled);
    }

    #[test]
    fn gold_theme_is_enabled() {
        let t = Theme::themed(ThemePreset::Gold, None);
        assert!(t.enabled);
    }

    #[test]
    fn no_color_flag_yields_plain_theme() {
        let args = Args {
            no_color: true,
            translation: false,
            random: false,
            phrase: None,
            list: false,
            no_vcenter: false,
            style: Some(crate::cli::StyleKind::Centered),
            font: Some(crate::cli::FontArg::Auto),
            theme: None,
            command: None,
        };
        let t = Theme::for_args(&args, ThemePreset::Gold, None);
        assert!(!t.enabled, "--no-color must disable styling");
    }

    #[test]
    fn preset_names_parse_case_insensitively() {
        assert_eq!(ThemePreset::parse("gold"), Some(ThemePreset::Gold));
        assert_eq!(ThemePreset::parse(" OCEAN "), Some(ThemePreset::Ocean));
        assert_eq!(ThemePreset::parse("Rose"), Some(ThemePreset::Rose));
        assert_eq!(ThemePreset::parse("mono"), Some(ThemePreset::Mono));
        assert_eq!(ThemePreset::parse("neon"), None);
    }

    #[test]
    fn hex_colors_parse() {
        assert_eq!(
            parse_color("#ff0000"),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            parse_color("00ff80"),
            Some(Color::Rgb {
                r: 0,
                g: 255,
                b: 128
            })
        );
        assert_eq!(parse_color("#xyz"), None);
        assert_eq!(parse_color("#12345"), None);
    }

    #[test]
    fn named_colors_parse() {
        assert_eq!(parse_color("red"), Some(Color::Red));
        assert_eq!(parse_color("Dark-Grey"), Some(Color::DarkGrey));
        assert_eq!(parse_color("chartreuse"), None);
    }

    #[test]
    fn overrides_replace_preset_slots() {
        let overrides = ColorOverrides {
            border: Some("#ff0000".to_string()),
            arabic: None,
            translit: None,
            english: None,
        };
        let t = Theme::themed(ThemePreset::Gold, Some(&overrides));
        assert_eq!(t.border, Color::Rgb { r: 255, g: 0, b: 0 });
        // Untouched slots keep the preset color.
        assert_eq!(t.arabic, Color::Green);
    }

    #[test]
    fn invalid_override_falls_back_to_preset() {
        let overrides = ColorOverrides {
            border: Some("not-a-color".to_string()),
            arabic: None,
            translit: None,
            english: None,
        };
        let t = Theme::themed(ThemePreset::Gold, Some(&overrides));
        assert_eq!(
            t.border,
            Color::Rgb {
                r: 218,
                g: 165,
                b: 32
            },
            "bad override must keep the gold preset border"
        );
    }
}

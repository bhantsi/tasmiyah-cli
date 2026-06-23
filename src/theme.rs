//! Color theme + color-mode detection.
//!
//! A `Theme` holds the four colors used by the banner. When the user
//! disables colors (via `--no-color`, the `NO_COLOR` env var, or because
//! stdout is not a TTY), we hand back a "plain" theme whose every slot is
//! `Color::Reset` — the renderer stays branch-free.

use std::io::IsTerminal;

use crossterm::style::Color;

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

impl Theme {
    /// The gold/green theme described in the README.
    pub fn colored() -> Self {
        Self {
            enabled: true,
            // Gold (matches a warm "Islamic manuscript" feel).
            border: Color::Rgb {
                r: 218,
                g: 165,
                b: 32,
            },
            arabic: Color::Green,
            translit: Color::Yellow,
            english: Color::White,
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

    /// Pick the right theme based on user flags + environment.
    pub fn for_args(args: &Args) -> Self {
        if should_use_color(args) {
            Self::colored()
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
/// * stdout is not a TTY (e.g. piped into another program or redirected).
fn should_use_color(args: &Args) -> bool {
    if args.no_color {
        return false;
    }
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if !std::io::stdout().is_terminal() {
        return false;
    }
    true
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
    fn colored_theme_is_enabled() {
        let t = Theme::colored();
        assert!(t.enabled);
    }

    #[test]
    fn no_color_flag_yields_plain_theme() {
        let args = Args {
            no_color: true,
            translation: false,
            random: false,
        };
        let t = Theme::for_args(&args);
        assert!(!t.enabled, "--no-color must disable styling");
    }
}

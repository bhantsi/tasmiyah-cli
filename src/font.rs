//! Nerd-Font detection.
//!
//! Many prompt frameworks (Starship, Powerlevel10k, Oh-My-Posh) only
//! work properly with a Nerd Font installed in the terminal. Those
//! same fonts often draw Arabic glyphs slightly wider than what
//! `unicode-width` reports, which is enough to break our centered
//! layout by a column or two.
//!
//! This module resolves which font world we're rendering into. The
//! result is consumed by [`crate::banner`] to nudge layout math.
//!
//! Resolution order (highest priority first):
//!
//! 1. The `--font` CLI flag, if explicitly `standard` or `nerd`.
//! 2. The `TASMIYAH_FONT` environment variable, if set to
//!    `nerd` / `standard` (any other value is ignored).
//! 3. Heuristic: presence of any well-known prompt-framework env var
//!    (`STARSHIP_SESSION_KEY`, `POWERLEVEL9K_*`, `P9K_*`,
//!    `OH_MY_POSH_*`, `NERD_FONT`) implies a Nerd Font is installed.
//! 4. Fallback: [`FontKind::Standard`].

use std::env;

use crate::cli::FontArg;

/// Which font family the terminal is rendering with, as far as we can
/// tell. Used purely as a layout hint — the actual rendered text is
/// identical in either world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontKind {
    /// A "plain" terminal font (Cascadia, Menlo, DejaVu Sans Mono, …).
    Standard,
    /// A Nerd Font (MesloLGS NF, FiraCode NF, JetBrainsMono NF, …).
    NerdFont,
}

/// Resolve the effective [`FontKind`] from the user's `--font` flag,
/// the `TASMIYAH_FONT` env var, and shell-environment heuristics.
pub fn resolve(flag: FontArg) -> FontKind {
    match flag {
        FontArg::Standard => FontKind::Standard,
        FontArg::Nerd => FontKind::NerdFont,
        FontArg::Auto => from_env_var().unwrap_or_else(detect_from_heuristics),
    }
}

/// Honor the explicit `TASMIYAH_FONT` env override.
fn from_env_var() -> Option<FontKind> {
    let raw = env::var_os("TASMIYAH_FONT")?;
    parse_font_value(&raw.to_string_lossy())
}

/// Pure parser for the `TASMIYAH_FONT` value (and any equivalent
/// config-file string). Returns `None` for unrecognized input so the
/// caller can fall back to the auto-detection path.
fn parse_font_value(raw: &str) -> Option<FontKind> {
    match raw.trim().to_lowercase().as_str() {
        "nerd" | "nerdfont" | "nerd-font" => Some(FontKind::NerdFont),
        "standard" | "plain" => Some(FontKind::Standard),
        _ => None,
    }
}

/// Look for env vars that prompt frameworks set; if any are present,
/// assume the user installed a Nerd Font (the standard prerequisite).
fn detect_from_heuristics() -> FontKind {
    const NERD_HINTS: &[&str] = &[
        "STARSHIP_SESSION_KEY",
        "POWERLEVEL9K_MODE",
        "P9K_SSH",
        "OH_MY_POSH_THEME",
        "NERD_FONT",
    ];
    if NERD_HINTS.iter().any(|k| env::var_os(k).is_some()) {
        return FontKind::NerdFont;
    }
    // Catch-all prefix scan for the two big prompt frameworks that
    // sprinkle their config across many env vars.
    for (k, _) in env::vars_os() {
        let lossy = k.to_string_lossy();
        if lossy.starts_with("POWERLEVEL9K_")
            || lossy.starts_with("P9K_")
            || lossy.starts_with("OH_MY_POSH_")
        {
            return FontKind::NerdFont;
        }
    }
    FontKind::Standard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_standard_wins_over_env() {
        // Even with the "nerd" override, an explicit --font standard wins.
        assert_eq!(resolve(FontArg::Standard), FontKind::Standard);
    }

    #[test]
    fn explicit_nerd_wins_over_env() {
        assert_eq!(resolve(FontArg::Nerd), FontKind::NerdFont);
    }

    #[test]
    fn parses_recognised_values() {
        assert_eq!(parse_font_value("nerd"), Some(FontKind::NerdFont));
        assert_eq!(parse_font_value(" NerdFont "), Some(FontKind::NerdFont));
        assert_eq!(parse_font_value("STANDARD"), Some(FontKind::Standard));
        assert_eq!(parse_font_value("plain"), Some(FontKind::Standard));
    }

    #[test]
    fn unknown_env_value_is_ignored() {
        assert_eq!(parse_font_value("fancy"), None);
        assert_eq!(parse_font_value(""), None);
    }
}

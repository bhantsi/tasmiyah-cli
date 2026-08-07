//! tasmiyah-cli — a tiny terminal greeting that prints Bismillah.
//!
//! Entry point: load config, parse CLI args, resolve the effective
//! style / theme / font (CLI flag > env var > config > default),
//! pick a phrase, render the banner.

use clap::Parser;
use tasmiyah_cli::{banner, config, font, phrases, theme, update};

use tasmiyah_cli::cli::{Args, FontArg, StyleKind};
use tasmiyah_cli::theme::ThemePreset;

fn main() -> std::io::Result<()> {
    let cfg = config::Config::load();
    let mut args = Args::parse();

    // Resolve the effective style: CLI > TASMIYAH_STYLE > config > centered.
    let style = resolve_style(&args, &cfg);
    args.style = Some(style);

    // Resolve the effective font request: CLI > config > auto. The
    // TASMIYAH_FONT env var and heuristics are applied inside
    // `font::resolve`, which is what `auto` means.
    let font_arg = resolve_font(&args, &cfg);
    let font = font::resolve(font_arg);

    // Resolve the effective theme preset: CLI > TASMIYAH_THEME > config > gold.
    let preset = resolve_theme(&args, &cfg);

    // `translation` is a bool flag (clap can't tell "flag absent" from
    // "false"), so the config gets a vote only when the flag is absent.
    if !args.translation && cfg.translation == Some(true) {
        args.translation = true;
    }

    let theme = theme::Theme::for_args(&args, preset, cfg.colors.as_ref());
    let (phrase_index, phrase) = if args.random {
        phrases::random_with_index()
    } else {
        phrases::default_with_index()
    };
    let color = theme.enabled;
    banner::render(&args, &theme, font, phrase_index, phrase)?;
    // Non-fatal, daily-cached check for a newer release on crates.io.
    // All gating (TTY, opt-out env vars, error swallowing) lives in
    // `update::notify` so `main` stays a straight pipeline.
    update::notify(color);
    Ok(())
}

/// CLI flag > `TASMIYAH_STYLE` > config `style` > `centered`.
fn resolve_style(args: &Args, cfg: &config::Config) -> StyleKind {
    if let Some(s) = args.style {
        return s;
    }
    if let Some(s) = std::env::var("TASMIYAH_STYLE")
        .ok()
        .as_deref()
        .and_then(parse_style)
    {
        return s;
    }
    if let Some(s) = cfg.style.as_deref().and_then(parse_style) {
        return s;
    }
    StyleKind::Centered
}

/// CLI flag > config `font` > `auto`.
fn resolve_font(args: &Args, cfg: &config::Config) -> FontArg {
    if let Some(f) = args.font {
        return f;
    }
    if let Some(f) = cfg.font.as_deref().and_then(parse_font) {
        return f;
    }
    FontArg::Auto
}

/// CLI flag > `TASMIYAH_THEME` > config `theme` > `gold`.
fn resolve_theme(args: &Args, cfg: &config::Config) -> ThemePreset {
    if let Some(t) = args.theme {
        return t;
    }
    if let Some(t) = std::env::var("TASMIYAH_THEME")
        .ok()
        .as_deref()
        .and_then(ThemePreset::parse)
    {
        return t;
    }
    if let Some(t) = cfg.theme.as_deref().and_then(ThemePreset::parse) {
        return t;
    }
    ThemePreset::Gold
}

/// Lenient parser shared by `TASMIYAH_STYLE` and the config file.
fn parse_style(raw: &str) -> Option<StyleKind> {
    match raw.trim().to_lowercase().as_str() {
        "classic" => Some(StyleKind::Classic),
        "centered" => Some(StyleKind::Centered),
        "minimal" => Some(StyleKind::Minimal),
        _ => None,
    }
}

/// Lenient parser for the config file's `font` value.
fn parse_font(raw: &str) -> Option<FontArg> {
    match raw.trim().to_lowercase().as_str() {
        "standard" => Some(FontArg::Standard),
        "nerd" => Some(FontArg::Nerd),
        "auto" => Some(FontArg::Auto),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_style_accepts_known_values() {
        assert_eq!(parse_style("classic"), Some(StyleKind::Classic));
        assert_eq!(parse_style(" Centered "), Some(StyleKind::Centered));
        assert_eq!(parse_style("MINIMAL"), Some(StyleKind::Minimal));
        assert_eq!(parse_style("fancy"), None);
    }

    #[test]
    fn parse_font_accepts_known_values() {
        assert_eq!(parse_font("nerd"), Some(FontArg::Nerd));
        assert_eq!(parse_font("STANDARD"), Some(FontArg::Standard));
        assert_eq!(parse_font("auto"), Some(FontArg::Auto));
        assert_eq!(parse_font("comic-sans"), None);
    }
}

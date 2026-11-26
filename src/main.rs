//! tasmiyah-cli — a tiny terminal greeting that prints Bismillah.
//!
//! Entry point: load config, parse CLI args, resolve the effective
//! style / theme / font (CLI flag > env var > config > default),
//! pick a phrase, render the banner.

use std::io::{self, IsTerminal, Write};

use clap::Parser;
use tasmiyah_cli::{banner, config, font, phrases, shaping, terminal, theme, update};

use tasmiyah_cli::cli::{ArabicMode, Args, Command, FontArg, StyleKind};
use tasmiyah_cli::phrases::{LookupError, Phrase};
use tasmiyah_cli::theme::ThemePreset;

fn main() -> io::Result<()> {
    let cfg = config::Config::load();
    let mut args = Args::parse();

    // `--list` prints the phrase table and exits before any rendering
    // or update check.
    if args.list {
        return match print_list(&mut io::stdout().lock()) {
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
            r => r,
        };
    }

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

    // Same pattern: `vertical_center = false` in config acts as `--no-vcenter`.
    if !args.no_vcenter && cfg.vertical_center == Some(false) {
        args.no_vcenter = true;
    }

    let theme = theme::Theme::for_args(&args, preset, cfg.colors.as_ref());

    // Resolve the effective Arabic mode: CLI > TASMIYAH_ARABIC > config > auto.
    let arabic_mode = resolve_arabic_mode(&args, &cfg);

    // `doctor` prints diagnostics only: no banner, no update notice.
    if let Some(Command::Doctor { font_family }) = &args.command {
        return match tasmiyah_cli::doctor::run(
            font_family.as_deref(),
            arabic_mode,
            &theme,
            &mut io::stdout().lock(),
        ) {
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
            r => r,
        };
    }

    let (phrase_index, phrase) = match resolve_phrase(&args, &cfg) {
        Ok(found) => found,
        Err(e) => {
            eprintln!("tasmiyah: {e}");
            std::process::exit(2);
        }
    };
    let color = theme.enabled;
    let bidi = terminal::detect(&|k| std::env::var(k).ok());
    let shaped_phrase;
    let phrase = if shaping::resolve_arabic(arabic_mode, io::stdout().is_terminal(), bidi) {
        // Leaked once so the shaped line is `'static` like the table's
        // text; avoids changing every renderer's `&Phrase` signature.
        let arabic: &'static str =
            Box::leak(shaping::shape_for_ltr_terminal(phrase.arabic).into_boxed_str());
        shaped_phrase = Phrase { arabic, ..*phrase };
        &shaped_phrase
    } else {
        phrase
    };
    banner::render(&args, &theme, font, phrase_index, phrase)?;
    // Non-fatal, daily-cached check for a newer release on crates.io.
    // All gating (TTY, opt-out env vars, error swallowing) lives in
    // `update::notify` so `main` stays a straight pipeline.
    update::notify(color);
    Ok(())
}

/// `--phrase` > `--random` > `TASMIYAH_PHRASE` > config `phrase` > index 0.
///
/// Only an invalid `--phrase` is an error; bad env/config values are
/// ignored so a stale setting never breaks the greeting.
fn resolve_phrase(
    args: &Args,
    cfg: &config::Config,
) -> Result<(usize, &'static Phrase), LookupError> {
    if let Some(sel) = args.phrase.as_deref() {
        let i = phrases::find(sel)?;
        return Ok((i, &phrases::PHRASES[i]));
    }
    if args.random {
        return Ok(phrases::random_with_index());
    }
    if let Some(i) = std::env::var("TASMIYAH_PHRASE")
        .ok()
        .and_then(|s| phrases::find(&s).ok())
    {
        return Ok((i, &phrases::PHRASES[i]));
    }
    if let Some(i) = cfg.phrase.as_deref().and_then(|s| phrases::find(s).ok()) {
        return Ok((i, &phrases::PHRASES[i]));
    }
    Ok(phrases::default_with_index())
}

/// Write one line per phrase: index, name, transliteration, English.
fn print_list<W: Write>(out: &mut W) -> io::Result<()> {
    for (i, p) in phrases::PHRASES.iter().enumerate() {
        writeln!(
            out,
            "{i:>2}  {slug:<15} {translit} — {english}",
            slug = p.slug,
            translit = p.translit,
            english = p.english
        )?;
    }
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

/// CLI flag > `TASMIYAH_ARABIC` > config `arabic` > `auto`.
fn resolve_arabic_mode(args: &Args, cfg: &config::Config) -> ArabicMode {
    if let Some(m) = args.arabic {
        return m;
    }
    if let Some(m) = std::env::var("TASMIYAH_ARABIC")
        .ok()
        .as_deref()
        .and_then(parse_arabic)
    {
        return m;
    }
    if let Some(m) = cfg.arabic.as_deref().and_then(parse_arabic) {
        return m;
    }
    ArabicMode::Auto
}

/// Lenient parser shared by `TASMIYAH_ARABIC` and the config file.
fn parse_arabic(raw: &str) -> Option<ArabicMode> {
    match raw.trim().to_lowercase().as_str() {
        "auto" => Some(ArabicMode::Auto),
        "native" => Some(ArabicMode::Native),
        "shaped" => Some(ArabicMode::Shaped),
        _ => None,
    }
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
    fn parse_arabic_accepts_known_values() {
        assert_eq!(parse_arabic("shaped"), Some(ArabicMode::Shaped));
        assert_eq!(parse_arabic(" Native "), Some(ArabicMode::Native));
        assert_eq!(parse_arabic("AUTO"), Some(ArabicMode::Auto));
        assert_eq!(parse_arabic("sideways"), None);
    }

    #[test]
    fn parse_font_accepts_known_values() {
        assert_eq!(parse_font("nerd"), Some(FontArg::Nerd));
        assert_eq!(parse_font("STANDARD"), Some(FontArg::Standard));
        assert_eq!(parse_font("auto"), Some(FontArg::Auto));
        assert_eq!(parse_font("comic-sans"), None);
    }
}

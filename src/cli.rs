//! Command-line argument parsing.
//!
//! Uses `clap`'s derive API so `--help` and `--version` come for free,
//! pulled straight from `Cargo.toml` metadata.

use clap::{Parser, ValueEnum};

/// A beautiful, fast terminal greeting that prints Bismillah and Islamic phrases.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Disable ANSI colors (also honored: NO_COLOR env var, TERM=dumb,
    /// non-TTY stdout).
    #[arg(short = 'n', long)]
    pub no_color: bool,

    /// Show transliteration and English translation below the Arabic.
    #[arg(short = 't', long)]
    pub translation: bool,

    /// Print a random Islamic phrase (Alhamdulillah, SubhanAllah, ...).
    #[arg(short = 'r', long)]
    pub random: bool,

    /// Visual style. Default is a large BISMILLAH logo centered in the
    /// terminal; `classic` is the v0.1 unicode box; `minimal` strips
    /// all decoration for shell prompts and scripting. Falls back to
    /// `TASMIYAH_STYLE`, then the config file, then `centered`.
    #[arg(short = 's', long, value_enum)]
    pub style: Option<StyleKind>,

    /// Font world to assume when laying out Arabic glyphs. `auto`
    /// reads `TASMIYAH_FONT` then sniffs prompt-framework env vars
    /// (Starship, Powerlevel10k, Oh-My-Posh) to guess.
    #[arg(short = 'f', long, value_enum)]
    pub font: Option<FontArg>,

    /// Color palette. `gold` is the original look; `ocean`, `rose` and
    /// `mono` are built-in alternatives. Individual colors can be
    /// overridden in the config file's `[colors]` table. Falls back to
    /// `TASMIYAH_THEME`, then the config file, then `gold`.
    #[arg(long, value_enum)]
    pub theme: Option<crate::theme::ThemePreset>,
}

/// One of three banner layouts the renderer supports.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleKind {
    /// The original v0.1 unicode-box layout.
    Classic,
    /// Large `BISMILLAH` ASCII-art logo, centered vertically and
    /// horizontally in the terminal. *(Default.)*
    Centered,
    /// Plain text only — no border, no logo, no padding.
    Minimal,
}

/// User-facing `--font` selector. The actual layout decision lives in
/// [`crate::font::resolve`].
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontArg {
    /// Plain terminal font — no special width allowance.
    Standard,
    /// Nerd Font — apply small extra horizontal slack to Arabic widths.
    Nerd,
    /// Auto-detect via `TASMIYAH_FONT` and shell heuristics. *(Default.)*
    Auto,
}

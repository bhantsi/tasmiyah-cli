//! Command-line argument parsing.
//!
//! Uses `clap`'s derive API so `--help` and `--version` come for free,
//! pulled straight from `Cargo.toml` metadata.

use clap::{Parser, ValueEnum};

/// A beautiful, fast terminal greeting that prints Bismillah and Islamic phrases.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Disable ANSI colors (also honored: NO_COLOR env var, non-TTY stdout).
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
    /// all decoration for shell prompts and scripting.
    #[arg(short = 's', long, value_enum, default_value_t = StyleKind::Centered)]
    pub style: StyleKind,

    /// Font world to assume when laying out Arabic glyphs. `auto`
    /// (the default) reads `TASMIYAH_FONT` then sniffs prompt-framework
    /// env vars (Starship, Powerlevel10k, Oh-My-Posh) to guess.
    #[arg(short = 'f', long, value_enum, default_value_t = FontArg::Auto)]
    pub font: FontArg,
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

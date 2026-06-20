//! Command-line argument parsing.
//!
//! Uses `clap`'s derive API so `--help` and `--version` come for free,
//! pulled straight from `Cargo.toml` metadata.

use clap::Parser;

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

    /// Print a random Islamic phrase (coming in a future release).
    #[arg(short = 'r', long)]
    pub random: bool,
}

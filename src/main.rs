//! tasmiyah-cli — a tiny terminal greeting that prints Bismillah.
//!
//! Entry point: parse CLI args, pick a color theme, render the banner.

use clap::Parser;

mod banner;
mod cli;
mod theme;

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let theme = theme::Theme::for_args(&args);
    banner::render(&args, &theme)
}

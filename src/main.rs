//! tasmiyah-cli — a tiny terminal greeting that prints Bismillah.
//!
//! Entry point: parse CLI args, pick a phrase + theme + font hint,
//! render the banner.

use clap::Parser;

mod banner;
mod cli;
mod font;
mod phrases;
mod theme;

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let theme = theme::Theme::for_args(&args);
    let font = font::resolve(args.font);
    let phrase = if args.random {
        phrases::random()
    } else {
        phrases::default()
    };
    banner::render(&args, &theme, font, phrase)
}

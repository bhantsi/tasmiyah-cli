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
mod update;

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let theme = theme::Theme::for_args(&args);
    let font = font::resolve(args.font);
    let phrase = if args.random {
        phrases::random()
    } else {
        phrases::default()
    };
    let color = theme.enabled;
    banner::render(&args, &theme, font, phrase)?;
    // Non-fatal, daily-cached check for a newer release on crates.io.
    // All gating (TTY, opt-out env vars, error swallowing) lives in
    // `update::notify` so `main` stays a straight pipeline.
    update::notify(color);
    Ok(())
}

//! `--style minimal`: just the phrase lines, no border, no logo, no
//! padding. Designed for shell prompts, status bars, and other
//! contexts where a single line of text is the whole point.
//!
//! Output is left-aligned and only includes the requested lines (the
//! transliteration/English pair is gated on `--translation`).

use std::io::Write;

use super::common::paint;
use crate::cli::Args;
use crate::phrases::Phrase;
use crate::theme::Theme;

pub(super) fn render<W: Write>(
    args: &Args,
    theme: &Theme,
    phrase: &Phrase,
    out: &mut W,
) -> std::io::Result<()> {
    paint(out, phrase.arabic, theme.arabic, theme)?;
    writeln!(out)?;
    if args.translation {
        paint(out, phrase.translit, theme.translit, theme)?;
        writeln!(out)?;
        paint(out, phrase.english, theme.english, theme)?;
        writeln!(out)?;
    }
    Ok(())
}

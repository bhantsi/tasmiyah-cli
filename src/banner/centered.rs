//! `--style centered` (the default): a large BISMILLAH logo centered
//! both horizontally and vertically, with the Arabic / transliteration
//! / English lines beneath it.
//!
//! Layout (top-to-bottom):
//!
//! ```text
//!   <top padding to vertically center>
//!   <6 rows of BISMILLAH ANSI-Shadow art, each centered horizontally>
//!   <blank line>
//!   <Arabic line, centered>
//!   <transliteration, centered>   (only if --translation)
//!   <English, centered>           (only if --translation)
//! ```
//!
//! On terminals narrower than the logo (cols < [`art::BISMILLAH_WIDTH`])
//! we skip the logo entirely and just print the centered text lines —
//! the rest of the output stays readable on phones, popups, and split
//! panes.

use std::io::Write;

use crossterm::style::Color;
use unicode_width::UnicodeWidthStr;

use super::art::{BISMILLAH_ANSI_SHADOW, BISMILLAH_WIDTH};
use super::common::{paint, term_size};
use crate::cli::Args;
use crate::phrases::Phrase;
use crate::theme::Theme;

pub(super) fn render<W: Write>(
    args: &Args,
    theme: &Theme,
    phrase: &Phrase,
    out: &mut W,
) -> std::io::Result<()> {
    let (cols, rows) = term_size();
    let show_logo = cols >= BISMILLAH_WIDTH;

    // Build the list of text lines that go *under* the logo.
    let mut text_lines: Vec<(&str, Color)> = Vec::with_capacity(3);
    text_lines.push((phrase.arabic, theme.arabic));
    if args.translation {
        text_lines.push((phrase.translit, theme.translit));
        text_lines.push((phrase.english, theme.english));
    }

    // Total vertical height of everything we're about to draw.
    let logo_height = if show_logo {
        BISMILLAH_ANSI_SHADOW.len()
    } else {
        0
    };
    let gap = if show_logo { 1 } else { 0 };
    let banner_height = logo_height + gap + text_lines.len();
    let top_pad = rows.saturating_sub(banner_height) / 2;

    // Vertical centering — but only when we know the terminal is taller
    // than what we're about to draw. If `rows` is small or the terminal
    // size lookup failed, `top_pad` is 0 and we just start at the top.
    for _ in 0..top_pad {
        writeln!(out)?;
    }

    // Logo rows.
    if show_logo {
        let left = horizontal_pad(cols, BISMILLAH_WIDTH);
        for row in BISMILLAH_ANSI_SHADOW {
            write_centered_row(out, row, theme.border, left, theme)?;
        }
        writeln!(out)?;
    }

    // Text rows, each individually centered (they have different widths).
    for (text, color) in &text_lines {
        let w = UnicodeWidthStr::width(*text);
        let left = horizontal_pad(cols, w);
        write_centered_row(out, text, *color, left, theme)?;
    }

    Ok(())
}

/// Number of spaces of left-padding to horizontally center `content_w`
/// inside a `cols`-wide terminal.
fn horizontal_pad(cols: usize, content_w: usize) -> usize {
    cols.saturating_sub(content_w) / 2
}

fn write_centered_row<W: Write>(
    out: &mut W,
    text: &str,
    color: Color,
    left_pad: usize,
    theme: &Theme,
) -> std::io::Result<()> {
    write!(out, "{}", " ".repeat(left_pad))?;
    paint(out, text, color, theme)?;
    writeln!(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_pad_centers_within_terminal() {
        assert_eq!(horizontal_pad(80, 20), 30);
        // Content wider than the terminal => 0, never panic.
        assert_eq!(horizontal_pad(10, 65), 0);
        // Exact fit => 0.
        assert_eq!(horizontal_pad(65, 65), 0);
    }
}

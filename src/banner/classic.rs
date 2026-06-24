//! The original `--style classic` renderer: a unicode box around the phrase.
//!
//! Width is computed from the widest line (treating Arabic combining
//! marks as zero-width via `unicode-width`), padded, and capped to the
//! terminal width so the box always fits.

use std::io::Write;

use crossterm::style::Color;
use unicode_width::UnicodeWidthStr;

use super::common::{paint, term_size};
use crate::cli::Args;
use crate::font::FontKind;
use crate::phrases::Phrase;
use crate::theme::Theme;

// Horizontal padding (spaces) between text and the vertical box border.
const H_PADDING: usize = 4;

// Extra horizontal slack applied per content line when we believe the
// terminal is rendering with a Nerd Font, which tends to draw Arabic
// glyphs a little wider than `unicode-width` reports.
const NERD_FONT_SLACK: usize = 2;

/// One styled line of content for the banner.
struct Line<'a> {
    text: &'a str,
    color: Color,
}

pub(super) fn render<W: Write>(
    args: &Args,
    theme: &Theme,
    font: FontKind,
    phrase: &Phrase,
    out: &mut W,
) -> std::io::Result<()> {
    let mut lines: Vec<Line> = Vec::with_capacity(3);
    lines.push(Line {
        text: phrase.arabic,
        color: theme.arabic,
    });
    if args.translation {
        lines.push(Line {
            text: phrase.translit,
            color: theme.translit,
        });
        lines.push(Line {
            text: phrase.english,
            color: theme.english,
        });
    }

    let inner_width = inner_width(&lines, font);

    write_top(out, inner_width, theme)?;
    write_blank(out, inner_width, theme)?;
    for line in &lines {
        write_line(out, line, inner_width, theme)?;
    }
    write_blank(out, inner_width, theme)?;
    write_bottom(out, inner_width, theme)
}

/// Compute the inner width of the box (between the two `║` borders).
fn inner_width(lines: &[Line], font: FontKind) -> usize {
    let slack = match font {
        FontKind::NerdFont => NERD_FONT_SLACK,
        FontKind::Standard => 0,
    };
    let max_text = lines
        .iter()
        .map(|l| UnicodeWidthStr::width(l.text) + slack)
        .max()
        .unwrap_or(0);
    let desired = max_text + 2 * H_PADDING;

    let (cols, _) = term_size();
    let cap = cols.saturating_sub(2).max(max_text); // never crush the text itself
    desired.min(cap)
}

fn write_top<W: Write>(out: &mut W, inner: usize, theme: &Theme) -> std::io::Result<()> {
    let bar: String = "═".repeat(inner);
    paint(out, "╔", theme.border, theme)?;
    paint(out, &bar, theme.border, theme)?;
    paint(out, "╗", theme.border, theme)?;
    writeln!(out)
}

fn write_bottom<W: Write>(out: &mut W, inner: usize, theme: &Theme) -> std::io::Result<()> {
    let bar: String = "═".repeat(inner);
    paint(out, "╚", theme.border, theme)?;
    paint(out, &bar, theme.border, theme)?;
    paint(out, "╝", theme.border, theme)?;
    writeln!(out)
}

fn write_blank<W: Write>(out: &mut W, inner: usize, theme: &Theme) -> std::io::Result<()> {
    paint(out, "║", theme.border, theme)?;
    write!(out, "{}", " ".repeat(inner))?;
    paint(out, "║", theme.border, theme)?;
    writeln!(out)
}

fn write_line<W: Write>(
    out: &mut W,
    line: &Line,
    inner: usize,
    theme: &Theme,
) -> std::io::Result<()> {
    let text_width = UnicodeWidthStr::width(line.text);
    // Defensive: if a line is wider than the box, fall back to a minimum
    // single-space padding rather than panicking on subtract-overflow.
    let total_pad = inner.saturating_sub(text_width).max(2);
    let left = total_pad / 2;
    let right = total_pad - left;

    paint(out, "║", theme.border, theme)?;
    write!(out, "{}", " ".repeat(left))?;
    paint(out, line.text, line.color, theme)?;
    write!(out, "{}", " ".repeat(right))?;
    paint(out, "║", theme.border, theme)?;
    writeln!(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::FontKind;
    use crossterm::style::Color;

    #[test]
    fn inner_width_accommodates_widest_line_plus_padding() {
        let lines = vec![
            Line {
                text: "hi",
                color: Color::Reset,
            },
            Line {
                text: "hello",
                color: Color::Reset,
            },
        ];
        let w = inner_width(&lines, FontKind::Standard);
        assert!(w >= 5, "inner width {w} must fit 'hello'");
    }

    #[test]
    fn inner_width_handles_empty_input() {
        let lines: Vec<Line> = Vec::new();
        let _ = inner_width(&lines, FontKind::Standard);
    }

    #[test]
    fn nerd_font_adds_slack() {
        let lines = vec![Line {
            text: "hello",
            color: Color::Reset,
        }];
        let plain = inner_width(&lines, FontKind::Standard);
        let nerd = inner_width(&lines, FontKind::NerdFont);
        assert!(
            nerd >= plain,
            "nerd-font width {nerd} should be at least as wide as standard {plain}"
        );
    }
}

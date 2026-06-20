//! Renders the decorative Basmala banner.
//!
//! The renderer is deliberately small: it computes the visual width of
//! every line using `unicode-width` (which correctly treats Arabic
//! combining marks like fatha/damma/kasra as zero-width — matching what
//! a terminal actually draws), picks a sensible box width, and writes
//! the box to a locked, buffered stdout.

use std::io::{self, BufWriter, StdoutLock, Write};

use crossterm::style::{Color, Stylize};
use crossterm::terminal;
use unicode_width::UnicodeWidthStr;

use crate::cli::Args;
use crate::theme::Theme;

// Basmala — "In the name of Allah, the Most Gracious, the Most Merciful".
const BISMILLAH_AR: &str = "بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ";
const BISMILLAH_TRANSLIT: &str = "Bismillāh ir-Raḥmān ir-Raḥīm";
const BISMILLAH_EN: &str = "In the name of Allah, the Most Gracious, the Most Merciful";

// Horizontal padding (spaces) between text and the vertical box border.
const H_PADDING: usize = 4;

// Fallback width when terminal size detection fails (e.g. piped output).
const FALLBACK_COLS: u16 = 80;

/// One styled line of content for the banner.
struct Line<'a> {
    text: &'a str,
    color: Color,
}

/// Render the banner to stdout, honoring `args` and `theme`.
pub fn render(args: &Args, theme: &Theme) -> io::Result<()> {
    let stdout = io::stdout().lock();
    let mut out = BufWriter::new(stdout);

    let mut lines: Vec<Line> = Vec::with_capacity(3);
    lines.push(Line { text: BISMILLAH_AR, color: theme.arabic });
    if args.translation {
        lines.push(Line { text: BISMILLAH_TRANSLIT, color: theme.translit });
        lines.push(Line { text: BISMILLAH_EN, color: theme.english });
    }

    let inner_width = inner_width(&lines);

    write_top(&mut out, inner_width, theme)?;
    write_blank(&mut out, inner_width, theme)?;
    for line in &lines {
        write_line(&mut out, line, inner_width, theme)?;
    }
    write_blank(&mut out, inner_width, theme)?;
    write_bottom(&mut out, inner_width, theme)?;

    if args.random {
        writeln!(out)?;
        writeln!(
            out,
            "(note: --random will print a random Islamic phrase in a future release)"
        )?;
    }

    out.flush()
}

/// Compute the inner width of the box (the area between the two `║` borders).
///
/// We take the widest line, add horizontal padding on both sides, and
/// then cap at `terminal_cols - 2` so the box always fits even on a
/// narrow terminal.
fn inner_width(lines: &[Line]) -> usize {
    let max_text = lines
        .iter()
        .map(|l| UnicodeWidthStr::width(l.text))
        .max()
        .unwrap_or(0);

    let desired = max_text + 2 * H_PADDING;

    let cols = terminal::size().map(|(c, _)| c).unwrap_or(FALLBACK_COLS) as usize;
    let cap = cols.saturating_sub(2).max(max_text); // never crush the text itself
    desired.min(cap)
}

/// Write `text` either raw or styled with `color`, depending on the theme.
fn paint(
    out: &mut BufWriter<StdoutLock>,
    text: &str,
    color: Color,
    theme: &Theme,
) -> io::Result<()> {
    if theme.enabled {
        write!(out, "{}", text.with(color))
    } else {
        write!(out, "{}", text)
    }
}

fn write_top(out: &mut BufWriter<StdoutLock>, inner: usize, theme: &Theme) -> io::Result<()> {
    let bar: String = "═".repeat(inner);
    paint(out, "╔", theme.border, theme)?;
    paint(out, &bar, theme.border, theme)?;
    paint(out, "╗", theme.border, theme)?;
    writeln!(out)
}

fn write_bottom(out: &mut BufWriter<StdoutLock>, inner: usize, theme: &Theme) -> io::Result<()> {
    let bar: String = "═".repeat(inner);
    paint(out, "╚", theme.border, theme)?;
    paint(out, &bar, theme.border, theme)?;
    paint(out, "╝", theme.border, theme)?;
    writeln!(out)
}

fn write_blank(out: &mut BufWriter<StdoutLock>, inner: usize, theme: &Theme) -> io::Result<()> {
    paint(out, "║", theme.border, theme)?;
    write!(out, "{}", " ".repeat(inner))?;
    paint(out, "║", theme.border, theme)?;
    writeln!(out)
}

fn write_line(
    out: &mut BufWriter<StdoutLock>,
    line: &Line,
    inner: usize,
    theme: &Theme,
) -> io::Result<()> {
    let text_width = UnicodeWidthStr::width(line.text);
    // Defensive: if a line is somehow wider than the box, just print it
    // with single-space padding rather than panicking on subtract-overflow.
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

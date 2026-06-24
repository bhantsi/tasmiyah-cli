//! Banner renderers.
//!
//! The CLI ships three layout styles, selected at runtime via the
//! `--style` flag (default: [`StyleKind::Centered`]).
//!
//! * [`classic`]  — the original box-drawing layout (v0.1.0).
//! * [`centered`] — a large `BISMILLAH` ANSI-Shadow logo centered
//!   vertically and horizontally in the terminal, with the Arabic /
//!   transliteration / English lines beneath it.
//! * [`minimal`]  — three plain centered text lines, no decoration.
//!
//! Each style is implemented as a free `render()` function inside its
//! own module; the dispatcher below picks one based on `args.style`.

use std::io::{self, BufWriter, Write};

use crate::cli::{Args, StyleKind};
use crate::font::FontKind;
use crate::phrases::Phrase;
use crate::theme::Theme;

mod art;
mod centered;
mod classic;
mod minimal;

/// Common bits shared between styles.
mod common {
    use crossterm::style::{Color, Stylize};
    use std::io::Write;

    use crate::theme::Theme;

    /// Write `text` either raw or styled with `color`, depending on the
    /// theme. Every style must funnel its output through this helper so
    /// `--no-color` (and friends) is honored consistently.
    pub(super) fn paint<W: Write>(
        out: &mut W,
        text: &str,
        color: Color,
        theme: &Theme,
    ) -> std::io::Result<()> {
        if theme.enabled {
            write!(out, "{}", text.with(color))
        } else {
            write!(out, "{}", text)
        }
    }

    /// Fallback width when terminal size detection fails (e.g. piped output).
    pub(super) const FALLBACK_COLS: u16 = 80;
    pub(super) const FALLBACK_ROWS: u16 = 24;

    /// Best-effort terminal size, with sensible defaults for non-TTYs.
    pub(super) fn term_size() -> (usize, usize) {
        let (c, r) = crossterm::terminal::size().unwrap_or((FALLBACK_COLS, FALLBACK_ROWS));
        (c as usize, r as usize)
    }
}

/// Render `phrase` to stdout using the style requested in `args`.
pub fn render(args: &Args, theme: &Theme, font: FontKind, phrase: &Phrase) -> io::Result<()> {
    let stdout = io::stdout().lock();
    let mut out = BufWriter::new(stdout);
    match args.style {
        StyleKind::Classic => classic::render(args, theme, font, phrase, &mut out)?,
        StyleKind::Centered => centered::render(args, theme, phrase, &mut out)?,
        StyleKind::Minimal => minimal::render(args, theme, phrase, &mut out)?,
    }
    out.flush()
}

//! Pre-rendered ASCII art used by `--style centered`.
//!
//! `BISMILLAH_ANSI_SHADOW` is the word "BISMILLAH" rendered in the
//! `ANSI Shadow` figlet font (the upright block-shadow font used by
//! tools like Starship and Oh-My-Posh in their splash screens). It is
//! exactly 6 rows tall and 65 columns wide (visual width — each glyph
//! is single-cell under `unicode-width`).
//!
//! Keeping it as a baked-in constant means:
//!
//! * Zero startup cost (no figlet dependency, no parsing).
//! * Predictable rendering on every terminal that supports the basic
//!   Unicode block-drawing range.

/// Visual width of every row in [`BISMILLAH_ANSI_SHADOW`].
pub(super) const BISMILLAH_WIDTH: usize = 65;

/// "BISMILLAH" in ANSI Shadow, 6 rows × 65 cols.
pub(super) const BISMILLAH_ANSI_SHADOW: [&str; 6] = [
    "██████╗ ██╗███████╗███╗   ███╗██╗██╗     ██╗      █████╗ ██╗  ██╗",
    "██╔══██╗██║██╔════╝████╗ ████║██║██║     ██║     ██╔══██╗██║  ██║",
    "██████╔╝██║███████╗██╔████╔██║██║██║     ██║     ███████║███████║",
    "██╔══██╗██║╚════██║██║╚██╔╝██║██║██║     ██║     ██╔══██║██╔══██║",
    "██████╔╝██║███████║██║ ╚═╝ ██║██║███████╗███████╗██║  ██║██║  ██║",
    "╚═════╝ ╚═╝╚══════╝╚═╝     ╚═╝╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn every_row_has_the_declared_width() {
        for (i, row) in BISMILLAH_ANSI_SHADOW.iter().enumerate() {
            let w = UnicodeWidthStr::width(*row);
            assert_eq!(
                w, BISMILLAH_WIDTH,
                "row {i} is {w} cols wide, expected {BISMILLAH_WIDTH}\n{row}"
            );
        }
    }

    #[test]
    fn art_is_six_rows() {
        assert_eq!(BISMILLAH_ANSI_SHADOW.len(), 6);
    }
}

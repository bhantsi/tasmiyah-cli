//! Pre-rendered ASCII art used by `--style centered`.
//!
//! Each logo is rendered in the `ANSI Shadow` figlet font (the upright
//! block-shadow font used by tools like Starship and Oh-My-Posh in
//! their splash screens). Every logo is exactly 6 rows tall.
//!
//! Keeping them as baked-in constants means:
//!
//! * Zero startup cost (no figlet dependency, no parsing).
//! * Predictable rendering on every terminal that supports the basic
//!   Unicode block-drawing range.
//!
//! ## Per-phrase logos
//!
//! [`logo_for`] maps a phrase index (into [`crate::phrases::PHRASES`])
//! to its logo. Phrases without a dedicated logo return `None`, and the
//! centered renderer falls back to text-only output for them — the
//! `--random` path stays correct because `render()` already knows which
//! phrase was picked.

/// A rendered logo: its rows plus the visual width of each row.
#[derive(Clone, Copy)]
pub(super) struct Logo {
    pub rows: &'static [&'static str],
    pub width: usize,
}

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

/// Return the ANSI-Shadow logo for `phrase_index`, if one exists.
///
/// Index 0 is the Basmala and uses the BISMILLAH logo. The other
/// indices line up with [`crate::phrases::PHRASES`]; any phrase without
/// art returns `None` and the caller renders text-only.
pub(super) fn logo_for(phrase_index: usize) -> Option<Logo> {
    match phrase_index {
        0 => Some(Logo {
            rows: &BISMILLAH_ANSI_SHADOW,
            width: BISMILLAH_WIDTH,
        }),
        1 => Some(Logo {
            rows: &ALHAMDULILLAH,
            width: ALHAMDULILLAH_WIDTH,
        }),
        2 => Some(Logo {
            rows: &SUBHANALLAH,
            width: SUBHANALLAH_WIDTH,
        }),
        3 => Some(Logo {
            rows: &ALLAHU_AKBAR,
            width: ALLAHU_AKBAR_WIDTH,
        }),
        4 => Some(Logo {
            rows: &MASHAAALLAH,
            width: MASHAAALLAH_WIDTH,
        }),
        5 => Some(Logo {
            rows: &INSHAALLAH,
            width: INSHAALLAH_WIDTH,
        }),
        6 => Some(Logo {
            rows: &ASTAGHFIRULLAH,
            width: ASTAGHFIRULLAH_WIDTH,
        }),
        _ => None,
    }
}

pub(super) const ALHAMDULILLAH_WIDTH: usize = 103;
pub(super) const ALHAMDULILLAH: [&str; 6] = [
    " █████╗ ██╗     ██╗  ██╗ █████╗ ███╗   ███╗██████╗ ██╗   ██╗██╗     ██╗██╗     ██╗      █████╗ ██╗  ██╗",
    "██╔══██╗██║     ██║  ██║██╔══██╗████╗ ████║██╔══██╗██║   ██║██║     ██║██║     ██║     ██╔══██╗██║  ██║",
    "███████║██║     ███████║███████║██╔████╔██║██║  ██║██║   ██║██║     ██║██║     ██║     ███████║███████║",
    "██╔══██║██║     ██╔══██║██╔══██║██║╚██╔╝██║██║  ██║██║   ██║██║     ██║██║     ██║     ██╔══██║██╔══██║",
    "██║  ██║███████╗██║  ██║██║  ██║██║ ╚═╝ ██║██████╔╝╚██████╔╝███████╗██║███████╗███████╗██║  ██║██║  ██║",
    "╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚═════╝  ╚═════╝ ╚══════╝╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

pub(super) const ALLAHU_AKBAR_WIDTH: usize = 93;
pub(super) const ALLAHU_AKBAR: [&str; 6] = [
    " █████╗ ██╗     ██╗      █████╗ ██╗  ██╗██╗   ██╗     █████╗ ██╗  ██╗██████╗  █████╗ ██████╗ ",
    "██╔══██╗██║     ██║     ██╔══██╗██║  ██║██║   ██║    ██╔══██╗██║ ██╔╝██╔══██╗██╔══██╗██╔══██╗",
    "███████║██║     ██║     ███████║███████║██║   ██║    ███████║█████╔╝ ██████╔╝███████║██████╔╝",
    "██╔══██║██║     ██║     ██╔══██║██╔══██║██║   ██║    ██╔══██║██╔═██╗ ██╔══██╗██╔══██║██╔══██╗",
    "██║  ██║███████╗███████╗██║  ██║██║  ██║╚██████╔╝    ██║  ██║██║  ██╗██████╔╝██║  ██║██║  ██║",
    "╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝",
];

pub(super) const ASTAGHFIRULLAH_WIDTH: usize = 110;
pub(super) const ASTAGHFIRULLAH: [&str; 6] = [
    " █████╗ ███████╗████████╗ █████╗  ██████╗ ██╗  ██╗███████╗██╗██████╗ ██╗   ██╗██╗     ██╗      █████╗ ██╗  ██╗",
    "██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔════╝ ██║  ██║██╔════╝██║██╔══██╗██║   ██║██║     ██║     ██╔══██╗██║  ██║",
    "███████║███████╗   ██║   ███████║██║  ███╗███████║█████╗  ██║██████╔╝██║   ██║██║     ██║     ███████║███████║",
    "██╔══██║╚════██║   ██║   ██╔══██║██║   ██║██╔══██║██╔══╝  ██║██╔══██╗██║   ██║██║     ██║     ██╔══██║██╔══██║",
    "██║  ██║███████║   ██║   ██║  ██║╚██████╔╝██║  ██║██║     ██║██║  ██║╚██████╔╝███████╗███████╗██║  ██║██║  ██║",
    "╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

pub(super) const INSHAALLAH_WIDTH: usize = 77;
pub(super) const INSHAALLAH: [&str; 6] = [
    "██╗███╗   ██╗███████╗██╗  ██╗ █████╗  █████╗ ██╗     ██╗      █████╗ ██╗  ██╗",
    "██║████╗  ██║██╔════╝██║  ██║██╔══██╗██╔══██╗██║     ██║     ██╔══██╗██║  ██║",
    "██║██╔██╗ ██║███████╗███████║███████║███████║██║     ██║     ███████║███████║",
    "██║██║╚██╗██║╚════██║██╔══██║██╔══██║██╔══██║██║     ██║     ██╔══██║██╔══██║",
    "██║██║ ╚████║███████║██║  ██║██║  ██║██║  ██║███████╗███████╗██║  ██║██║  ██║",
    "╚═╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

pub(super) const MASHAAALLAH_WIDTH: usize = 91;
pub(super) const MASHAAALLAH: [&str; 6] = [
    "███╗   ███╗ █████╗ ███████╗██╗  ██╗ █████╗  █████╗  █████╗ ██╗     ██╗      █████╗ ██╗  ██╗",
    "████╗ ████║██╔══██╗██╔════╝██║  ██║██╔══██╗██╔══██╗██╔══██╗██║     ██║     ██╔══██╗██║  ██║",
    "██╔████╔██║███████║███████╗███████║███████║███████║███████║██║     ██║     ███████║███████║",
    "██║╚██╔╝██║██╔══██║╚════██║██╔══██║██╔══██║██╔══██║██╔══██║██║     ██║     ██╔══██║██╔══██║",
    "██║ ╚═╝ ██║██║  ██║███████║██║  ██║██║  ██║██║  ██║██║  ██║███████╗███████╗██║  ██║██║  ██║",
    "╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

pub(super) const SUBHANALLAH_WIDTH: usize = 91;
pub(super) const SUBHANALLAH: [&str; 6] = [
    "███████╗██╗   ██╗██████╗ ██╗  ██╗ █████╗ ███╗   ██╗ █████╗ ██╗     ██╗      █████╗ ██╗  ██╗",
    "██╔════╝██║   ██║██╔══██╗██║  ██║██╔══██╗████╗  ██║██╔══██╗██║     ██║     ██╔══██╗██║  ██║",
    "███████╗██║   ██║██████╔╝███████║███████║██╔██╗ ██║███████║██║     ██║     ███████║███████║",
    "╚════██║██║   ██║██╔══██╗██╔══██║██╔══██║██║╚██╗██║██╔══██║██║     ██║     ██╔══██║██╔══██║",
    "███████║╚██████╔╝██████╔╝██║  ██║██║  ██║██║ ╚████║██║  ██║███████╗███████╗██║  ██║██║  ██║",
    "╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝",
];

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_width::UnicodeWidthStr;

    /// Every logo: each row must match the declared width.
    fn check(logo: &Logo) {
        for (i, row) in logo.rows.iter().enumerate() {
            let w = UnicodeWidthStr::width(*row);
            assert_eq!(w, logo.width, "row {i} is {w} cols, expected {}", logo.width);
        }
        assert_eq!(logo.rows.len(), 6, "logos are 6 rows tall");
    }

    #[test]
    fn bismillah_rows_match_declared_width() {
        check(&logo_for(0).unwrap());
    }

    #[test]
    fn all_mapped_logos_have_consistent_widths() {
        for idx in 0..=6 {
            check(&logo_for(idx).unwrap());
        }
    }

    #[test]
    fn unmapped_phrase_returns_none() {
        assert!(logo_for(7).is_none(), "phrase 7 has no logo");
        assert!(logo_for(100).is_none());
    }
}

//! Pre-shaping of Arabic text for terminals without BiDi support
//! (Alacritty, kitty, ...): contextual presentation forms plus visual
//! (reversed) order.

use ar_reshaper::{ArabicReshaper, ReshaperConfig};

/// True for Arabic combining marks (harakat, Quranic annotation marks).
pub fn is_arabic_mark(c: char) -> bool {
    matches!(c as u32,
        0x064B..=0x065F | 0x0670 | 0x06D6..=0x06DC | 0x06DF..=0x06E4 | 0x06E7 | 0x06E8 | 0x06EA..=0x06ED)
}

/// Shape `text` into presentation forms and reverse it into visual order.
pub fn shape_for_ltr_terminal(text: &str) -> String {
    // The default config deletes all harakat and emits the Allah ligature
    // U+FDF2, which monospace terminal fonts lack; individual ligatures
    // cannot be toggled, so disable them all and keep the harakat.
    let cfg = ReshaperConfig {
        delete_harakat: false,
        support_ligatures: false,
        ..ReshaperConfig::default()
    };
    let reshaped = ArabicReshaper::new(cfg).reshape(text);

    // A cluster is one non-mark char plus the marks that follow it.
    let mut clusters: Vec<String> = Vec::new();
    for c in reshaped.chars() {
        match clusters.last_mut() {
            Some(last) if is_arabic_mark(c) => last.push(c),
            _ => clusters.push(c.to_string()),
        }
    }
    clusters.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_follow_their_base_after_reversal() {
        let s = shape_for_ltr_terminal("بِسْمِ");
        let cs: Vec<char> = s.chars().collect();
        assert!(
            !is_arabic_mark(cs[0]),
            "line must not start with a mark: {cs:?}"
        );
        assert!(
            ('\u{FEE1}'..='\u{FEE4}').contains(&cs[0]),
            "first glyph should be a meem form"
        );
        assert_eq!(cs[1], '\u{0650}');
    }

    #[test]
    fn spaces_and_word_order_reverse() {
        let s = shape_for_ltr_terminal("بِسْمِ ٱللَّٰهِ");
        assert!(s.ends_with(|c: char| ('\u{FE8F}'..='\u{FE92}').contains(&c) || is_arabic_mark(c)));
        assert_eq!(s.matches(' ').count(), 1);
    }

    #[test]
    fn quranic_letters_survive() {
        let s = shape_for_ltr_terminal(crate::phrases::PHRASES[0].arabic);
        assert!(s.contains('\u{0670}'));
        assert!(s
            .chars()
            .any(|c| c == '\u{0671}' || c == '\u{FB50}' || c == '\u{FB51}'));
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(shape_for_ltr_terminal(""), "");
    }

    #[test]
    fn every_phrase_shapes_without_base_arabic_letters_left() {
        for p in crate::phrases::PHRASES {
            let s = shape_for_ltr_terminal(p.arabic);
            assert!(
                !s.chars().any(|c| ('\u{0621}'..='\u{064A}').contains(&c)),
                "{}: {s:?}",
                p.slug
            );
        }
    }
}

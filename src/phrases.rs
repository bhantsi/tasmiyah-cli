//! Curated set of common Islamic phrases the CLI knows how to render.
//!
//! Each phrase has three pieces: the Arabic original (always shown), a
//! Latin-script transliteration, and a brief English meaning.

/// A single Islamic phrase with its three representations.
pub struct Phrase {
    pub arabic: &'static str,
    pub translit: &'static str,
    pub english: &'static str,
}

/// The phrase set. Index 0 is the Basmala — the default when no flags
/// are supplied. The order is also the order shown in `--help` discussions.
pub const PHRASES: &[Phrase] = &[
    Phrase {
        arabic: "بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ",
        translit: "Bismillāh ir-Raḥmān ir-Raḥīm",
        english: "In the name of Allah, the Most Gracious, the Most Merciful",
    },
    Phrase {
        arabic: "ٱلْحَمْدُ لِلَّٰهِ",
        translit: "Alḥamdulillāh",
        english: "All praise is due to Allah",
    },
    Phrase {
        arabic: "سُبْحَانَ ٱللَّٰهِ",
        translit: "Subḥān Allāh",
        english: "Glory be to Allah",
    },
    Phrase {
        arabic: "ٱللَّٰهُ أَكْبَرُ",
        translit: "Allāhu Akbar",
        english: "Allah is the Greatest",
    },
    Phrase {
        arabic: "مَا شَاءَ ٱللَّٰهُ",
        translit: "Mā shāʾ Allāh",
        english: "What Allah has willed",
    },
    Phrase {
        arabic: "إِنْ شَاءَ ٱللَّٰهُ",
        translit: "In shāʾ Allāh",
        english: "If Allah wills",
    },
    Phrase {
        arabic: "أَسْتَغْفِرُ ٱللَّٰهَ",
        translit: "Astaghfirullāh",
        english: "I seek forgiveness from Allah",
    },
    Phrase {
        arabic: "لَا إِلَٰهَ إِلَّا ٱللَّٰهُ",
        translit: "Lā ilāha illā Allāh",
        english: "There is no god but Allah",
    },
];

/// Pick a random index into `PHRASES`.
///
/// Uses a tiny xorshift64 step seeded from the current system time in
/// nanoseconds. This is good enough for "pick something different each
/// time the user opens a shell" — we deliberately avoid pulling in the
/// `rand` crate (and its ~30 transitive deps) for a one-shot index pick
/// in a tool that targets sub-5ms startup.
pub fn random_index() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);

    // Seed must be non-zero for xorshift to do anything.
    let mut x = nanos | 1;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    (x as usize) % PHRASES.len()
}

/// The default phrase (the Basmala).
pub fn default() -> &'static Phrase {
    &PHRASES[0]
}

/// Pick a random phrase.
pub fn random() -> &'static Phrase {
    &PHRASES[random_index()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phrase_set_is_non_empty() {
        assert!(
            !PHRASES.is_empty(),
            "PHRASES must contain at least one entry"
        );
    }

    #[test]
    fn every_phrase_has_all_three_fields() {
        for (i, p) in PHRASES.iter().enumerate() {
            assert!(!p.arabic.trim().is_empty(), "phrase #{i}: arabic is empty");
            assert!(
                !p.translit.trim().is_empty(),
                "phrase #{i}: translit is empty"
            );
            assert!(
                !p.english.trim().is_empty(),
                "phrase #{i}: english is empty"
            );
        }
    }

    #[test]
    fn default_phrase_is_the_basmala() {
        let p = default();
        assert!(p.arabic.contains('ٱ') || p.arabic.contains('ا'));
        assert!(p.translit.starts_with("Bismill"));
    }

    #[test]
    fn random_index_is_in_bounds() {
        // Many calls — every one must be a valid index.
        for _ in 0..256 {
            assert!(random_index() < PHRASES.len());
        }
    }
}

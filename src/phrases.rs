//! Curated set of common Islamic phrases the CLI knows how to render.
//!
//! Each phrase has three pieces: the Arabic original (always shown), a
//! Latin-script transliteration, and a brief English meaning.

/// A single Islamic phrase with its three representations.
pub struct Phrase {
    /// Stable ASCII identifier used by `--phrase`.
    pub slug: &'static str,
    pub arabic: &'static str,
    pub translit: &'static str,
    pub english: &'static str,
}

/// The phrase set. Index 0 is the Basmala — the default when no flags
/// are supplied. The order is also the order shown in `--help` discussions.
pub const PHRASES: &[Phrase] = &[
    Phrase {
        slug: "bismillah",
        arabic: "بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ",
        translit: "Bismillāh ir-Raḥmān ir-Raḥīm",
        english: "In the name of Allah, the Most Gracious, the Most Merciful",
    },
    Phrase {
        slug: "alhamdulillah",
        arabic: "ٱلْحَمْدُ لِلَّٰهِ",
        translit: "Alḥamdulillāh",
        english: "All praise is due to Allah",
    },
    Phrase {
        slug: "subhanallah",
        arabic: "سُبْحَانَ ٱللَّٰهِ",
        translit: "Subḥān Allāh",
        english: "Glory be to Allah",
    },
    Phrase {
        slug: "allahuakbar",
        arabic: "ٱللَّٰهُ أَكْبَرُ",
        translit: "Allāhu Akbar",
        english: "Allah is the Greatest",
    },
    Phrase {
        slug: "mashallah",
        arabic: "مَا شَاءَ ٱللَّٰهُ",
        translit: "Mā shāʾ Allāh",
        english: "What Allah has willed",
    },
    Phrase {
        slug: "inshallah",
        arabic: "إِنْ شَاءَ ٱللَّٰهُ",
        translit: "In shāʾ Allāh",
        english: "If Allah wills",
    },
    Phrase {
        slug: "astaghfirullah",
        arabic: "أَسْتَغْفِرُ ٱللَّٰهَ",
        translit: "Astaghfirullāh",
        english: "I seek forgiveness from Allah",
    },
    Phrase {
        slug: "tahlil",
        arabic: "لَا إِلَٰهَ إِلَّا ٱللَّٰهُ",
        translit: "Lā ilāha illā Allāh",
        english: "There is no god but Allah",
    },
    Phrase {
        slug: "hasbunallah",
        arabic: "حَسْبُنَا ٱللَّٰهُ وَنِعْمَ ٱلْوَكِيلُ",
        translit: "Ḥasbunā Allāhu wa niʿmal wakīl",
        english: "Allah is sufficient for us, and He is the best Disposer of affairs",
    },
    Phrase {
        slug: "hawqala",
        arabic: "لَا حَوْلَ وَلَا قُوَّةَ إِلَّا بِٱللَّٰهِ",
        translit: "Lā ḥawla wa lā quwwata illā billāh",
        english: "There is no might nor power except with Allah",
    },
    Phrase {
        slug: "rabbana",
        arabic: "رَبَّنَا آتِنَا فِي ٱلدُّنْيَا حَسَنَةً",
        translit: "Rabbanā ātinā fi d-dunyā ḥasanah",
        english: "Our Lord, give us in this world that which is good",
    },
    Phrase {
        slug: "salawat",
        arabic: "ٱللَّٰهُمَّ صَلِّ عَلَىٰ مُحَمَّدٍ",
        translit: "Allāhumma ṣalli ʿalā Muḥammad",
        english: "O Allah, send blessings upon Muhammad",
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

/// The default phrase (the Basmala) and its index (0).
pub fn default() -> &'static Phrase {
    &PHRASES[0]
}

/// Pick a random phrase.
pub fn random() -> &'static Phrase {
    &PHRASES[random_index()]
}

/// Pick a random phrase, returning both the phrase and its index into
/// [`PHRASES`] (the centered renderer needs the index to pick the
/// matching logo).
pub fn random_with_index() -> (usize, &'static Phrase) {
    let i = random_index();
    (i, &PHRASES[i])
}

/// The default phrase (the Basmala) with its index (0).
pub fn default_with_index() -> (usize, &'static Phrase) {
    (0, &PHRASES[0])
}

/// Error returned by [`find`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupError {
    /// A numeric selector beyond the last phrase.
    OutOfRange { index: usize, len: usize },
    /// A selector that matched no phrase (or matched ambiguously).
    Unknown(String),
}

impl std::fmt::Display for LookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookupError::OutOfRange { index, len } => write!(
                f,
                "phrase index {index} is out of range (0-{}); run `tasmiyah --list`",
                len.saturating_sub(1)
            ),
            LookupError::Unknown(sel) => {
                write!(f, "unknown phrase '{sel}'; run `tasmiyah --list`")
            }
        }
    }
}

impl std::error::Error for LookupError {}

/// Lowercase, fold common Arabic-transliteration diacritics to plain ASCII,
/// and drop everything that is not an ASCII letter or digit.
pub fn normalize(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .filter_map(|c| {
            let c = match c {
                'ā' | 'á' | 'â' | 'à' => 'a',
                'ī' | 'í' => 'i',
                'ū' | 'ú' => 'u',
                'ḥ' => 'h',
                'ṣ' => 's',
                'ḍ' => 'd',
                'ṭ' => 't',
                'ẓ' => 'z',
                other => other,
            };
            c.is_ascii_alphanumeric().then_some(c)
        })
        .collect()
}

/// Resolve a user-supplied selector (index, slug, transliteration, or
/// unique slug prefix of at least 3 characters) to an index into [`PHRASES`].
pub fn find(selector: &str) -> Result<usize, LookupError> {
    let trimmed = selector.trim();
    if !trimmed.is_empty() && trimmed.bytes().all(|b| b.is_ascii_digit()) {
        return match trimmed.parse::<usize>() {
            Ok(index) if index < PHRASES.len() => Ok(index),
            Ok(index) => Err(LookupError::OutOfRange {
                index,
                len: PHRASES.len(),
            }),
            Err(_) => Err(LookupError::Unknown(selector.to_string())),
        };
    }

    let n = normalize(selector);
    if let Some(i) = PHRASES
        .iter()
        .position(|p| p.slug == n || normalize(p.translit) == n)
    {
        return Ok(i);
    }

    if n.len() >= 3 {
        let mut hits = PHRASES
            .iter()
            .enumerate()
            .filter(|(_, p)| p.slug.starts_with(&n))
            .map(|(i, _)| i);
        if let (Some(i), None) = (hits.next(), hits.next()) {
            return Ok(i);
        }
    }

    Err(LookupError::Unknown(selector.to_string()))
}

/// Sorted, deduplicated Arabic-block characters (U+0600..=U+06FF) used
/// across all phrases. Used by `--doctor` to probe font coverage.
pub fn arabic_codepoints() -> Vec<char> {
    let mut v: Vec<char> = PHRASES
        .iter()
        .flat_map(|p| p.arabic.chars())
        .filter(|c| ('\u{0600}'..='\u{06FF}').contains(c))
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn arabic_codepoints_cover_basmala() {
        let cps = arabic_codepoints();
        assert!(cps.contains(&'ب'));
        assert!(cps.contains(&'ٱ'));
        assert!(cps.contains(&'\u{0670}'));
        assert!(cps.iter().all(|c| !c.is_ascii()));
        let mut sorted = cps.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(cps, sorted);
    }

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

    #[test]
    fn slugs_are_unique_ascii_and_in_order() {
        let want = [
            "bismillah",
            "alhamdulillah",
            "subhanallah",
            "allahuakbar",
            "mashallah",
            "inshallah",
            "astaghfirullah",
            "tahlil",
            "hasbunallah",
            "hawqala",
            "rabbana",
            "salawat",
        ];
        let got: Vec<_> = PHRASES.iter().map(|p| p.slug).collect();
        assert_eq!(got, want);
    }

    #[test]
    fn find_by_index() {
        assert_eq!(find("3").unwrap(), 3);
        assert_eq!(find(" 0 ").unwrap(), 0);
    }

    #[test]
    fn find_by_slug() {
        assert_eq!(find("inshallah").unwrap(), 5);
    }

    #[test]
    fn find_folds_case_diacritics_and_spaces() {
        assert_eq!(find("In shāʾ Allāh").unwrap(), 5);
        assert_eq!(find(" SubhanAllah ").unwrap(), 2);
        assert_eq!(find("subhan allah").unwrap(), 2);
        assert_eq!(find("Allāhu Akbar").unwrap(), 3);
    }

    #[test]
    fn find_unique_prefix() {
        assert_eq!(find("astagh").unwrap(), 6);
    }

    #[test]
    fn find_ambiguous_or_short_prefix_is_unknown() {
        assert!(matches!(find("al"), Err(LookupError::Unknown(_))));
        assert!(matches!(find("ha"), Err(LookupError::Unknown(_))));
    }

    #[test]
    fn find_out_of_range() {
        assert!(matches!(
            find("99"),
            Err(LookupError::OutOfRange { index: 99, len: 12 })
        ));
    }

    #[test]
    fn normalize_examples() {
        assert_eq!(normalize("Ḥasbunā Allāhu"), "hasbunaallahu");
        assert_eq!(normalize("Mā shāʾ Allāh"), "mashaallah");
    }
}

//! Terminal BiDi detection.
//!
//! "BiDi native" means the terminal emulator applies the Unicode
//! Bidirectional Algorithm itself, so Arabic text printed in logical
//! order is displayed right-to-left correctly. "BiDi missing" means the
//! terminal draws characters left-to-right cell by cell, so Arabic
//! appears reversed unless the text is pre-shaped/reordered.
//!
//! The list of terminals is heuristic: it is based on well-known
//! environment variables and may be wrong or outdated (for example a
//! terminal may gain BiDi support later, or the variables may be
//! inherited by a nested terminal). Treat the result as a hint.

/// Result of probing the terminal for BiDi support.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bidi {
    /// The terminal (named) renders bidirectional text itself.
    Native(&'static str),
    /// The terminal (named) is known not to implement BiDi.
    Missing(&'static str),
    /// The terminal could not be identified.
    Unknown,
}

/// Detect BiDi support from environment variables, via an injected lookup.
pub fn detect(env: &dyn Fn(&str) -> Option<String>) -> Bidi {
    let has = |k: &str| env(k).is_some();
    let is = |k: &str, v: &str| env(k).as_deref() == Some(v);

    if has("VTE_VERSION") {
        Bidi::Native("VTE")
    } else if has("KONSOLE_VERSION") {
        Bidi::Native("Konsole")
    } else if is("TERM_PROGRAM", "Apple_Terminal") {
        Bidi::Native("Terminal.app")
    } else if has("KITTY_WINDOW_ID") || is("TERM", "xterm-kitty") {
        Bidi::Missing("kitty")
    } else if has("ALACRITTY_WINDOW_ID") || has("ALACRITTY_SOCKET") || is("TERM", "alacritty") {
        Bidi::Missing("Alacritty")
    } else if is("TERM_PROGRAM", "WezTerm") {
        Bidi::Missing("WezTerm")
    } else if is("TERM_PROGRAM", "ghostty") {
        Bidi::Missing("Ghostty")
    } else if has("WT_SESSION") {
        Bidi::Missing("Windows Terminal")
    } else {
        Bidi::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let owned: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key: &str| owned.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }

    #[test]
    fn vte_is_native() {
        assert_eq!(
            detect(&env(&[("VTE_VERSION", "7800")])),
            Bidi::Native("VTE")
        );
    }

    #[test]
    fn vte_wins_over_term() {
        let e = env(&[("TERM", "xterm-kitty"), ("VTE_VERSION", "7800")]);
        assert_eq!(detect(&e), Bidi::Native("VTE"));
    }

    #[test]
    fn kitty_is_missing() {
        assert_eq!(
            detect(&env(&[("KITTY_WINDOW_ID", "1")])),
            Bidi::Missing("kitty")
        );
    }

    #[test]
    fn alacritty_by_term() {
        assert_eq!(
            detect(&env(&[("TERM", "alacritty")])),
            Bidi::Missing("Alacritty")
        );
    }

    #[test]
    fn wezterm_is_missing() {
        assert_eq!(
            detect(&env(&[("TERM_PROGRAM", "WezTerm")])),
            Bidi::Missing("WezTerm")
        );
    }

    #[test]
    fn windows_terminal_is_missing() {
        assert_eq!(
            detect(&env(&[("WT_SESSION", "abc")])),
            Bidi::Missing("Windows Terminal")
        );
    }

    #[test]
    fn empty_env_is_unknown() {
        assert_eq!(detect(&env(&[])), Bidi::Unknown);
    }

    #[test]
    fn konsole_is_native() {
        assert_eq!(
            detect(&env(&[("KONSOLE_VERSION", "230800")])),
            Bidi::Native("Konsole")
        );
    }

    #[test]
    fn apple_terminal_is_native() {
        assert_eq!(
            detect(&env(&[("TERM_PROGRAM", "Apple_Terminal")])),
            Bidi::Native("Terminal.app")
        );
    }

    #[test]
    fn ghostty_is_missing() {
        assert_eq!(
            detect(&env(&[("TERM_PROGRAM", "ghostty")])),
            Bidi::Missing("Ghostty")
        );
    }

    #[test]
    fn alacritty_by_socket() {
        assert_eq!(
            detect(&env(&[("ALACRITTY_SOCKET", "/run/user/1000/a.sock")])),
            Bidi::Missing("Alacritty")
        );
    }

    #[test]
    fn kitty_by_term() {
        assert_eq!(
            detect(&env(&[("TERM", "xterm-kitty")])),
            Bidi::Missing("kitty")
        );
    }

    #[test]
    fn empty_vte_version_counts_as_set() {
        let e = env(&[("VTE_VERSION", ""), ("KITTY_WINDOW_ID", "1")]);
        assert_eq!(detect(&e), Bidi::Native("VTE"));
    }
}

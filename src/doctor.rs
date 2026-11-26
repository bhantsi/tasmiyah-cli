//! Pure diagnostic checks for `--doctor`.
//!
//! Every function here is side-effect free: environment, fontconfig
//! results and paths are injected by the caller.

use crate::terminal::{self, Bidi};
use crate::theme::Theme;
use crossterm::style::Stylize;
use std::io::{self, Write};
use std::path::Path;

const FONT_RECIPE: &str = "see docs/fontconfig/60-tasmiyah-arabic.conf and https://github.com/bhantsi/tasmiyah-cli/blob/main/docs/USER_GUIDE.md#arabic-letters-look-disconnected-or-have-gaps";
const FIXED_WIDTH_ARABIC: &[&str] = &["Kawkab Mono", "Thabit"];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Ok,
    Warn,
    Info,
}

#[derive(Debug)]
pub struct Check {
    pub status: Status,
    pub title: &'static str,
    pub detail: String,
}

fn check(status: Status, title: &'static str, detail: impl Into<String>) -> Check {
    Check {
        status,
        title,
        detail: detail.into(),
    }
}

pub fn check_locale(env: &dyn Fn(&str) -> Option<String>) -> Check {
    if cfg!(windows) {
        return check(Status::Info, "Locale", "not applicable on Windows");
    }
    let found = ["LC_ALL", "LC_CTYPE", "LANG"]
        .iter()
        .find_map(|k| env(k).filter(|v| !v.is_empty()));
    match found {
        Some(v) => {
            let l = v.to_lowercase();
            if l.contains("utf-8") || l.contains("utf8") {
                check(Status::Ok, "Locale", format!("{v} is UTF-8"))
            } else {
                check(
                    Status::Warn,
                    "Locale",
                    format!("{v} is not UTF-8; Arabic may not display"),
                )
            }
        }
        None => check(
            Status::Warn,
            "Locale",
            "no LC_ALL/LC_CTYPE/LANG set; Arabic may not display",
        ),
    }
}

pub fn check_terminal(bidi: Bidi) -> Check {
    match bidi {
        Bidi::Native(n) => check(
            Status::Ok,
            "Terminal",
            format!("{n} renders Arabic right-to-left"),
        ),
        Bidi::Missing(n) => check(
            Status::Warn,
            "Terminal",
            format!("{n} has no BiDi; Arabic may appear reversed"),
        ),
        Bidi::Unknown => check(Status::Info, "Terminal", "could not identify the terminal"),
    }
}

pub fn check_fonts(family: &str, served: Option<&[(char, String)]>) -> Check {
    let Some(served) = served else {
        return check(
            Status::Info,
            "Arabic font",
            "fontconfig not found; font check skipped",
        );
    };
    let mut head = format!("checked \"{family}\"");
    if family == "monospace" {
        head.push_str(" (pass --font-family \"<your terminal font>\" for an exact check)");
    }
    let mut fams: Vec<&str> = served.iter().map(|(_, f)| f.as_str()).collect();
    fams.sort_unstable();
    fams.dedup();
    if fams.len() > 1 {
        return check(
            Status::Warn,
            "Arabic font",
            format!(
                "{head}: split across {} fonts: {} — letters won't join; {FONT_RECIPE}",
                fams.len(),
                fams.join(", ")
            ),
        );
    }
    match fams.first() {
        Some(f) if FIXED_WIDTH_ARABIC.contains(f) => {
            check(Status::Ok, "Arabic font", format!("{head}: served by {f}"))
        }
        Some(f) => check(
            Status::Warn,
            "Arabic font",
            format!("{head}: {f} is proportional; letters may show gaps; {FONT_RECIPE}"),
        ),
        None => check(
            Status::Info,
            "Arabic font",
            format!("{head}: no font information returned"),
        ),
    }
}

pub fn check_install_path(exe: &Path, home: Option<&Path>) -> Check {
    let in_cargo = home.is_some_and(|h| exe.starts_with(h.join(".cargo").join("bin")));
    if in_cargo {
        check(
            Status::Info,
            "Shell startup",
            "installed in ~/.cargo/bin, which some shells add to PATH only after rc files run; \
             in rc files call \"$HOME/.cargo/bin/tasmiyah\"",
        )
    } else {
        check(
            Status::Ok,
            "Shell startup",
            format!("installed at {}", exe.display()),
        )
    }
}

/// Ask fontconfig which family serves each character of `family`.
///
/// Returns `None` when `fc-match` cannot be spawned (not installed, or a
/// non-unix platform) or when no character produced a family name.
#[cfg(unix)]
pub fn query_fontconfig(family: &str, chars: &[char]) -> Option<Vec<(char, String)>> {
    use std::process::{Command, Stdio};
    let mut served = Vec::new();
    for &c in chars {
        let pattern = format!("{family}:charset={:x}", c as u32);
        let out = Command::new("fc-match")
            .args(["-f", "%{family[0]}", &pattern])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output();
        let out = match out {
            Ok(o) => o,
            Err(_) => return None,
        };
        if !out.status.success() {
            continue;
        }
        let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !name.is_empty() {
            served.push((c, name));
        }
    }
    if served.is_empty() {
        None
    } else {
        Some(served)
    }
}

#[cfg(not(unix))]
pub fn query_fontconfig(_family: &str, _chars: &[char]) -> Option<Vec<(char, String)>> {
    None
}

/// Run every check against the real environment and print the results.
pub fn run<W: Write>(font_family: &str, theme: &Theme, out: &mut W) -> io::Result<()> {
    let env = |k: &str| std::env::var(k).ok();
    let served = query_fontconfig(font_family, &crate::phrases::arabic_codepoints());
    let home = dirs::home_dir();
    let checks = [
        check_locale(&env),
        check_terminal(terminal::detect(&env)),
        check_fonts(font_family, served.as_deref()),
        match std::env::current_exe() {
            Ok(exe) => check_install_path(&exe, home.as_deref()),
            Err(_) => check(
                Status::Info,
                "Shell startup",
                "could not determine the executable path",
            ),
        },
    ];
    for c in &checks {
        let (mark, color) = match c.status {
            Status::Ok => ("✓", Some(crossterm::style::Color::Green)),
            Status::Warn => ("!", Some(crossterm::style::Color::Yellow)),
            Status::Info => ("i", None),
        };
        match color {
            Some(col) if theme.enabled => write!(out, "{}", mark.with(col))?,
            _ => write!(out, "{mark}")?,
        }
        writeln!(out, " {}: {}", c.title, c.detail)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::Bidi;
    use std::path::Path;

    fn env_of(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |k| {
            pairs
                .iter()
                .find(|(n, _)| *n == k)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn locale_utf8_ok() {
        let c = check_locale(&env_of(&[("LANG", "en_US.UTF-8")]));
        assert_eq!(c.status, Status::Ok);
        assert_eq!(c.title, "Locale");
    }

    #[test]
    #[cfg(not(windows))]
    fn locale_missing_is_warn() {
        assert_eq!(check_locale(&env_of(&[])).status, Status::Warn);
    }

    #[test]
    #[cfg(not(windows))]
    fn lc_all_overrides_lang() {
        let c = check_locale(&env_of(&[("LC_ALL", "C"), ("LANG", "en_US.UTF-8")]));
        assert_eq!(c.status, Status::Warn);
    }

    #[test]
    #[cfg(windows)]
    fn locale_windows_is_info() {
        assert_eq!(check_locale(&env_of(&[])).status, Status::Info);
    }

    #[test]
    fn terminal_native_ok_and_missing_warns() {
        let ok = check_terminal(Bidi::Native("kitty"));
        assert_eq!(ok.status, Status::Ok);
        assert_eq!(ok.detail, "kitty renders Arabic right-to-left");
        let w = check_terminal(Bidi::Missing("xterm"));
        assert_eq!(w.status, Status::Warn);
        assert_eq!(w.detail, "xterm has no BiDi; Arabic may appear reversed");
        assert_eq!(check_terminal(Bidi::Unknown).status, Status::Info);
    }

    #[test]
    fn fonts_none_is_info() {
        let c = check_fonts("monospace", None);
        assert_eq!(c.status, Status::Info);
        assert!(c
            .detail
            .contains("fontconfig not found; font check skipped"));
    }

    #[test]
    fn split_fonts_warn() {
        let served = [
            ('ب', "DejaVu Sans".to_string()),
            ('ٱ', "Noto Sans Arabic".to_string()),
        ];
        let c = check_fonts("Foo", Some(&served));
        assert_eq!(c.status, Status::Warn);
        assert!(c.detail.contains("split across 2 fonts"));
        assert!(c.detail.contains("60-tasmiyah-arabic.conf"));
    }

    #[test]
    fn single_kawkab_ok() {
        let served = [
            ('ب', "Kawkab Mono".to_string()),
            ('ٱ', "Kawkab Mono".to_string()),
        ];
        let c = check_fonts("Kawkab Mono", Some(&served));
        assert_eq!(c.status, Status::Ok);
        assert!(c.detail.starts_with("checked \"Kawkab Mono\""));
    }

    #[test]
    fn single_proportional_font_warns() {
        let served = [
            ('ب', "Noto Sans Arabic".to_string()),
            ('ٱ', "Noto Sans Arabic".to_string()),
        ];
        let c = check_fonts("Foo", Some(&served));
        assert_eq!(c.status, Status::Warn);
        assert!(c.detail.contains("Noto Sans Arabic is proportional"));
        assert!(c.detail.contains("60-tasmiyah-arabic.conf"));
    }

    #[test]
    fn monospace_family_adds_hint() {
        let served = [('ب', "Thabit".to_string())];
        let c = check_fonts("monospace", Some(&served));
        assert!(c.detail.contains("--font-family \"<your terminal font>\""));
        let c2 = check_fonts("Thabit", Some(&served));
        assert!(!c2.detail.contains("<your terminal font>"));
    }

    #[test]
    fn cargo_bin_gets_tip() {
        let home = Path::new("/home/u");
        let c = check_install_path(Path::new("/home/u/.cargo/bin/tasmiyah"), Some(home));
        assert_eq!(c.status, Status::Info);
        assert!(c.detail.contains("\"$HOME/.cargo/bin/tasmiyah\""));
        let o = check_install_path(Path::new("/usr/local/bin/tasmiyah"), Some(home));
        assert_eq!(o.status, Status::Ok);
        assert!(o.detail.contains("/usr/local/bin/tasmiyah"));
    }
}

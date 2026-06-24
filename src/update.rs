//! Crates.io update notifier.
//!
//! Runs a daily-cached check (via the `update-informer` crate) after the
//! banner has already been rendered, and prints a one-line footer when a
//! newer version of `tasmiyah-cli` is available on crates.io.
//!
//! The check is gated to keep it polite and predictable:
//!
//! * **No network in scripts / pipes** — skipped when `stdout` is not a TTY.
//! * **Opt-out** — set `NO_UPDATE_NOTIFIER=1` (the de-facto env var used
//!   by npm, deno, etc.) to silence the check entirely. We also honor
//!   `CI` so it stays quiet in continuous-integration environments.
//! * **Best-effort** — any error (offline, DNS failure, rate limit,
//!   unwritable cache dir) is swallowed silently. The notifier must
//!   never interfere with the actual greeting.
//! * **Cached** — `update-informer` caches the latest version it saw
//!   for 24h in `~/.cache/update-informer/`, so most invocations do no
//!   network I/O at all.

use std::io::{self, IsTerminal, Write};

use crossterm::style::{Color, Stylize};
use update_informer::{registry, Check};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check crates.io for a newer release and, if one exists, print a short
/// footer to `stderr`. Honors the gates documented at the module level.
///
/// `color` mirrors the banner's color setting so the notice matches the
/// rest of the output (no ANSI escapes when colors are off).
pub fn notify(color: bool) {
    if !should_check() {
        return;
    }

    let informer = update_informer::new(registry::Crates, PKG_NAME, PKG_VERSION);
    let Ok(Some(new_version)) = informer.check_version() else {
        return;
    };

    // Print to stderr so piping the banner into another tool stays clean.
    // (We've already returned early when stdout isn't a TTY, but a user
    // could still be redirecting just stderr.)
    let mut err = io::stderr().lock();
    let _ = print_notice(&mut err, &new_version.to_string(), color);
}

/// Should we even try to fetch from crates.io?
fn should_check() -> bool {
    // Explicit opt-out (npm/deno convention).
    if std::env::var_os("NO_UPDATE_NOTIFIER").is_some() {
        return false;
    }
    // CI: noisy and pointless, and the cache dir is usually ephemeral.
    if std::env::var_os("CI").is_some() {
        return false;
    }
    // Scripts and pipes: stdout is the user-facing channel; if it's
    // not a TTY, treat the run as non-interactive and stay silent.
    if !io::stdout().is_terminal() {
        return false;
    }
    true
}

/// Render the two-line "new version available" footer. Split out for
/// unit testing — the function does no I/O of its own beyond the writer
/// it's handed.
fn print_notice<W: Write>(out: &mut W, new_version: &str, color: bool) -> io::Result<()> {
    // Blank line so the notice doesn't collide with the banner's
    // bottom edge (each style ends without a trailing newline of its
    // own to keep prompts tight).
    writeln!(out)?;
    if color {
        writeln!(
            out,
            "{}  A new release of {} is available: {} → {}",
            "✨".with(Color::Yellow),
            PKG_NAME.with(Color::Cyan),
            PKG_VERSION.with(Color::DarkGrey),
            new_version.with(Color::Green),
        )?;
        writeln!(
            out,
            "   Run {} to upgrade (or {} for Homebrew).",
            "`cargo install tasmiyah-cli`".with(Color::Cyan),
            "`brew upgrade bhantsi/tap/tasmiyah-cli`".with(Color::Cyan),
        )?;
        writeln!(
            out,
            "   Silence with {}.",
            "NO_UPDATE_NOTIFIER=1".with(Color::DarkGrey),
        )?;
    } else {
        writeln!(
            out,
            "A new release of {PKG_NAME} is available: {PKG_VERSION} -> {new_version}"
        )?;
        writeln!(
            out,
            "Run `cargo install tasmiyah-cli` to upgrade \
             (or `brew upgrade bhantsi/tap/tasmiyah-cli` for Homebrew)."
        )?;
        writeln!(out, "Silence with NO_UPDATE_NOTIFIER=1.")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_notice_contains_versions_and_no_ansi() {
        let mut buf = Vec::new();
        print_notice(&mut buf, "9.9.9", false).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains(PKG_VERSION), "current version missing: {s}");
        assert!(s.contains("9.9.9"), "new version missing: {s}");
        assert!(
            s.contains("cargo install tasmiyah-cli"),
            "cargo hint missing: {s}"
        );
        assert!(
            !s.contains('\u{1b}'),
            "plain notice must not contain ANSI escapes: {s:?}"
        );
    }

    #[test]
    fn colored_notice_uses_ansi() {
        let mut buf = Vec::new();
        print_notice(&mut buf, "9.9.9", true).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(
            s.contains('\u{1b}'),
            "colored notice should contain ANSI escapes"
        );
    }

    #[test]
    fn opt_out_env_disables_check() {
        // SAFETY: tests are single-threaded for env mutation here; we
        // restore the previous value at end.
        let prev = std::env::var_os("NO_UPDATE_NOTIFIER");
        std::env::set_var("NO_UPDATE_NOTIFIER", "1");
        assert!(!should_check());
        match prev {
            Some(v) => std::env::set_var("NO_UPDATE_NOTIFIER", v),
            None => std::env::remove_var("NO_UPDATE_NOTIFIER"),
        }
    }
}

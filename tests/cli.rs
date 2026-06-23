//! End-to-end tests for the `tasmiyah` binary.
//!
//! These run on every supported platform (Linux/macOS/Windows) under CI.
//! We deliberately avoid `assert_cmd` to keep the dev-dependency tree
//! tiny — `env!("CARGO_BIN_EXE_tasmiyah")` is the official Cargo-supplied
//! path to the compiled binary inside an integration test.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tasmiyah"))
}

fn run(args: &[&str]) -> (String, String, i32) {
    let out = bin()
        .args(args)
        // Force the "no colors" code path so the output is deterministic
        // and identical across platforms regardless of TTY detection.
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to execute tasmiyah binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let code = out.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

#[test]
fn prints_default_basmala() {
    let (stdout, _stderr, code) = run(&[]);
    assert_eq!(code, 0, "default invocation must exit 0");
    assert!(
        stdout.contains("بِسْمِ"),
        "stdout missing Arabic Basmala:\n{stdout}"
    );
    // Box-drawing border must be rendered.
    assert!(stdout.contains("╔") && stdout.contains("╝"));
}

#[test]
fn translation_flag_adds_english() {
    let (stdout, _stderr, code) = run(&["--translation"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("Bismill"),
        "transliteration missing:\n{stdout}"
    );
    assert!(
        stdout.contains("Merciful"),
        "english translation missing:\n{stdout}"
    );
}

#[test]
fn no_color_output_is_pure_text() {
    let (stdout, _stderr, code) = run(&["--no-color"]);
    assert_eq!(code, 0);
    // No ANSI escape (ESC = 0x1B) should ever be emitted in this mode.
    assert!(
        !stdout.contains('\u{1b}'),
        "--no-color output contained an ANSI escape:\n{:?}",
        stdout
    );
}

#[test]
fn random_flag_prints_a_known_phrase() {
    let (stdout, _stderr, code) = run(&["--random", "--translation"]);
    assert_eq!(code, 0);
    // Whatever phrase was picked, it must come from our curated set —
    // check that *some* known transliteration appears. Listing them all
    // here also guards against accidental phrase-table edits.
    let known = [
        "Bismill",
        "Alḥamdulillāh",
        "Subḥān",
        "Allāhu Akbar",
        "Mā shāʾ",
        "In shāʾ",
        "Astaghfirullāh",
        "Lā ilāha",
    ];
    assert!(
        known.iter().any(|k| stdout.contains(k)),
        "random output didn't contain any known phrase:\n{stdout}"
    );
}

#[test]
fn help_flag_works() {
    let (stdout, _stderr, code) = run(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.to_lowercase().contains("usage"));
}

#[test]
fn version_flag_reports_cargo_version() {
    let (stdout, _stderr, code) = run(&["--version"]);
    assert_eq!(code, 0);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_flag_is_rejected() {
    let (_stdout, _stderr, code) = run(&["--definitely-not-a-real-flag"]);
    assert_ne!(code, 0, "unknown flag must produce a non-zero exit code");
}

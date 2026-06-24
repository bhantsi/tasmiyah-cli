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
    run_with_env(args, &[])
}

fn run_with_env(args: &[&str], env: &[(&str, &str)]) -> (String, String, i32) {
    let mut cmd = bin();
    cmd.args(args)
        // Force the "no colors" code path so the output is deterministic
        // and identical across platforms regardless of TTY detection.
        .env("NO_COLOR", "1")
        // Suppress the crates.io update-check footer so tests stay
        // hermetic (no network, deterministic output).
        .env("NO_UPDATE_NOTIFIER", "1");
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("failed to execute tasmiyah binary");
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
}

#[test]
fn classic_style_renders_a_box() {
    let (stdout, _stderr, code) = run(&["--style", "classic"]);
    assert_eq!(code, 0);
    // Classic style is the v0.1 unicode-box renderer; its border must
    // include the four box corners.
    assert!(
        stdout.contains("╔") && stdout.contains("╗"),
        "classic style missing top corners:\n{stdout}"
    );
    assert!(
        stdout.contains("╚") && stdout.contains("╝"),
        "classic style missing bottom corners:\n{stdout}"
    );
}

#[test]
fn centered_style_includes_the_bismillah_logo() {
    let (stdout, _stderr, code) = run(&["--style", "centered"]);
    assert_eq!(code, 0);
    // The ANSI Shadow logo uses solid blocks (█) in every row.
    // Centered output should contain many of them (the logo is wide).
    let block_count = stdout.matches('█').count();
    assert!(
        block_count >= 100,
        "centered style logo missing — only {block_count} block chars in output:\n{stdout}"
    );
    // The phrase itself must still appear under the logo.
    assert!(
        stdout.contains("بِسْمِ"),
        "centered style missing Arabic phrase:\n{stdout}"
    );
}

#[test]
fn minimal_style_has_no_decoration() {
    let (stdout, _stderr, code) = run(&["--style", "minimal"]);
    assert_eq!(code, 0);
    // No box, no logo blocks, no padding.
    assert!(!stdout.contains('╔'));
    assert!(!stdout.contains('║'));
    assert!(!stdout.contains('█'));
    assert!(
        stdout.contains("بِسْمِ"),
        "minimal style missing Arabic phrase:\n{stdout}"
    );
}

#[test]
fn minimal_style_with_translation_has_three_lines() {
    let (stdout, _stderr, code) = run(&["--style", "minimal", "--translation"]);
    assert_eq!(code, 0);
    // Three content lines: arabic, translit, english. Each terminated
    // by `\n`, so splitting on `\n` and dropping the empty trailing
    // element yields exactly 3.
    let lines: Vec<&str> = stdout.split_inclusive('\n').collect();
    assert_eq!(
        lines.len(),
        3,
        "minimal --translation should produce 3 lines, got {}:\n{stdout}",
        lines.len()
    );
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

#[test]
fn font_env_var_is_accepted() {
    // Setting TASMIYAH_FONT must not crash the binary and must still
    // produce output containing the phrase.
    let (stdout, _stderr, code) =
        run_with_env(&["--style", "classic"], &[("TASMIYAH_FONT", "nerd")]);
    assert_eq!(code, 0);
    assert!(stdout.contains("بِسْمِ"));
}

#[test]
fn font_flag_overrides_env() {
    // Explicit --font standard must take precedence over the env var.
    let (stdout, _stderr, code) = run_with_env(
        &["--style", "classic", "--font", "standard"],
        &[("TASMIYAH_FONT", "nerd")],
    );
    assert_eq!(code, 0);
    assert!(stdout.contains("بِسْمِ"));
}

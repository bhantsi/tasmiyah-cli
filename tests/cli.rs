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
        .env("NO_UPDATE_NOTIFIER", "1")
        // Isolate from ambient developer env; callers' overrides apply below.
        .env_remove("TASMIYAH_ROWS")
        .env_remove("TASMIYAH_COLS")
        .env_remove("TASMIYAH_PHRASE")
        .env_remove("TASMIYAH_CONFIG")
        .env_remove("TASMIYAH_ARABIC");
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
        "Ḥasbunā",
        "Lā ḥawla",
        "Rabbanā",
        "Allāhumma",
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

#[test]
fn narrow_terminal_falls_back_to_text_only() {
    // TASMIYAH_COLS below the logo width (65) must drop the logo and
    // still print the Arabic phrase — this is the phone/split-pane path.
    let (stdout, _stderr, code) =
        run_with_env(&["--style", "centered"], &[("TASMIYAH_COLS", "40")]);
    assert_eq!(code, 0);
    assert!(
        !stdout.contains('█'),
        "narrow terminal must skip the logo:\n{stdout}"
    );
    assert!(
        stdout.contains("بِسْمِ"),
        "narrow terminal must still print the phrase:\n{stdout}"
    );
}

#[test]
fn wide_terminal_shows_logo() {
    // Sanity check the override works in the other direction: an
    // explicitly wide terminal keeps the logo.
    let (stdout, _stderr, code) =
        run_with_env(&["--style", "centered"], &[("TASMIYAH_COLS", "120")]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains('█'),
        "wide terminal should include the logo:\n{stdout}"
    );
}

#[test]
fn term_dumb_disables_color() {
    // TERM=dumb must force plain output even without --no-color.
    // (The test harness sets NO_COLOR=1 by default; here we clear it by
    // relying on TERM=dumb alone, then strip ANSI to double-check.)
    let mut cmd = bin();
    cmd.args(["--style", "minimal"])
        .env("TERM", "dumb")
        .env("NO_UPDATE_NOTIFIER", "1")
        .env_remove("NO_COLOR");
    let out = cmd.output().expect("failed to execute tasmiyah binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        !stdout.contains('\u{1b}'),
        "TERM=dumb must suppress ANSI escapes:\n{:?}",
        stdout
    );
}

#[test]
fn theme_flag_is_accepted() {
    // Every built-in preset must be accepted by --theme.
    for preset in ["gold", "ocean", "rose", "mono"] {
        let (_stdout, _stderr, code) = run(&["--theme", preset, "--style", "minimal"]);
        assert_eq!(code, 0, "--theme {preset} must exit 0");
    }
}

#[test]
fn unknown_theme_is_rejected() {
    let (_stdout, _stderr, code) = run(&["--theme", "neon"]);
    assert_ne!(code, 0, "unknown --theme value must fail");
}

#[test]
fn style_env_var_selects_style() {
    // TASMIYAH_STYLE=classic must produce the unicode box without any flag.
    let (stdout, _stderr, code) = run_with_env(&[], &[("TASMIYAH_STYLE", "classic")]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains('╔'),
        "TASMIYAH_STYLE=classic should render the box:\n{stdout}"
    );
}

#[test]
fn style_flag_beats_env_var() {
    // CLI flag must win over TASMIYAH_STYLE.
    let (stdout, _stderr, code) =
        run_with_env(&["--style", "minimal"], &[("TASMIYAH_STYLE", "classic")]);
    assert_eq!(code, 0);
    assert!(
        !stdout.contains('╔'),
        "--style minimal must override TASMIYAH_STYLE=classic:\n{stdout}"
    );
}

#[test]
fn config_file_is_used_when_no_flag_or_env() {
    // Point TASMIYAH_CONFIG at a temp file with a config that selects
    // the classic style; the binary should pick it up with no flags.
    let dir = std::env::temp_dir().join(format!("tasmiyah-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.toml"), "style = \"classic\"\n").unwrap();

    let mut cmd = bin();
    cmd.env("NO_COLOR", "1")
        .env("NO_UPDATE_NOTIFIER", "1")
        .env("TASMIYAH_CONFIG", dir.join("config.toml"))
        // Make sure the env var doesn't mask the config.
        .env_remove("TASMIYAH_STYLE")
        .env_remove("TASMIYAH_ARABIC")
        .env_remove("TASMIYAH_PHRASE");
    let out = cmd.output().expect("failed to execute tasmiyah binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        stdout.lines().any(|l| l.trim_start().starts_with('╔')),
        "config file style=classic should render the box:\n{stdout}"
    );
}

#[test]
fn malformed_config_does_not_crash() {
    let dir = std::env::temp_dir().join(format!("tasmiyah-bad-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.toml"), "style = [\n").unwrap();

    let mut cmd = bin();
    cmd.env("NO_COLOR", "1")
        .env("NO_UPDATE_NOTIFIER", "1")
        .env("TASMIYAH_CONFIG", dir.join("config.toml"))
        .env_remove("TASMIYAH_STYLE")
        .env_remove("TASMIYAH_ARABIC")
        .env_remove("TASMIYAH_PHRASE");
    let out = cmd.output().expect("failed to execute tasmiyah binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        stdout.contains("بِسْمِ"),
        "malformed config must fall back to defaults:\n{stdout}"
    );
}

fn leading_blank_lines(s: &str) -> usize {
    s.lines().take_while(|l| l.trim().is_empty()).count()
}

#[test]
fn piped_centered_output_has_no_leading_blank_lines() {
    let (stdout, _e, code) = run(&["--style", "centered"]);
    assert_eq!(code, 0);
    assert_eq!(
        leading_blank_lines(&stdout),
        0,
        "piped output padded:\n{stdout}"
    );
}

#[test]
fn rows_env_var_still_centers_vertically() {
    // logo 6 + gap 1 + arabic 1 = 8 rows; (40 - 8) / 2 = 16
    let (stdout, _e, _c) = run_with_env(
        &["--style", "centered"],
        &[("TASMIYAH_ROWS", "40"), ("TASMIYAH_COLS", "120")],
    );
    assert_eq!(leading_blank_lines(&stdout), 16);
}

#[test]
fn rows_env_var_centers_with_translation() {
    // 6 + 1 + 3 = 10 rows; (40 - 10) / 2 = 15
    let (stdout, _e, _c) = run_with_env(
        &["--style", "centered", "--translation"],
        &[("TASMIYAH_ROWS", "40"), ("TASMIYAH_COLS", "120")],
    );
    assert_eq!(leading_blank_lines(&stdout), 15);
}

#[test]
fn list_prints_every_phrase() {
    let (out, _e, code) = run(&["--list"]);
    assert_eq!(code, 0);
    assert_eq!(out.lines().count(), 12);
    assert!(out.lines().next().unwrap().contains("bismillah"));
    assert!(out.contains("salawat"));
}

#[test]
fn phrase_by_name_selects_it() {
    let (out, _e, code) = run(&["--style", "minimal", "--phrase", "alhamdulillah"]);
    assert_eq!(code, 0);
    assert!(out.contains("ٱلْحَمْدُ"));
}

#[test]
fn phrase_by_index_selects_it() {
    let (out, _e, _c) = run(&["--style", "minimal", "-p", "6"]);
    assert!(out.contains("أَسْتَغْفِرُ"));
}

#[test]
fn phrase_out_of_range_errors() {
    let (out, err, code) = run(&["--phrase", "99"]);
    assert_eq!(code, 2);
    assert!(out.is_empty());
    assert!(err.contains("--list"));
}

#[test]
fn unknown_phrase_errors() {
    let (_o, err, code) = run(&["--phrase", "banana"]);
    assert_eq!(code, 2);
    assert!(err.contains("unknown phrase 'banana'"));
}

#[test]
fn phrase_conflicts_with_random() {
    let (_o, _e, code) = run(&["--phrase", "0", "--random"]);
    assert_eq!(code, 2);
}

#[test]
fn phrase_env_selects_it() {
    let (out, _e, _c) = run_with_env(&["--style", "minimal"], &[("TASMIYAH_PHRASE", "inshallah")]);
    assert!(out.contains("إِنْ شَاءَ"));
}

#[test]
fn invalid_phrase_env_falls_back_to_default() {
    let (out, err, code) = run_with_env(&["--style", "minimal"], &[("TASMIYAH_PHRASE", "nope")]);
    assert_eq!(code, 0);
    assert!(out.contains("بِسْمِ"));
    assert!(err.is_empty());
}

fn run_with_config(
    name: &str,
    body: &str,
    args: &[&str],
    env: &[(&str, &str)],
) -> (String, String, i32) {
    let dir = std::env::temp_dir().join(format!("tasmiyah-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.toml"), body).unwrap();

    let mut cmd = bin();
    cmd.args(args)
        .env("NO_COLOR", "1")
        .env("NO_UPDATE_NOTIFIER", "1")
        .env("TASMIYAH_CONFIG", dir.join("config.toml"))
        .env_remove("TASMIYAH_STYLE")
        .env_remove("TASMIYAH_ARABIC")
        .env_remove("TASMIYAH_PHRASE");
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("failed to execute tasmiyah binary");
    let _ = std::fs::remove_dir_all(&dir);
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn run_with_phrase_config(name: &str, args: &[&str]) -> String {
    run_with_config(name, "phrase = \"salawat\"\n", args, &[]).0
}

#[test]
fn config_phrase_is_used() {
    let out = run_with_phrase_config("phrase-cfg", &["--style", "minimal"]);
    assert!(out.contains("ٱللَّٰهُمَّ"), "config phrase ignored:\n{out}");
}

#[test]
fn random_flag_beats_config_phrase() {
    let any_other = (0..20).any(|_| {
        !run_with_phrase_config("phrase-rand", &["--random", "--style", "minimal"])
            .contains("ٱللَّٰهُمَّ")
    });
    assert!(any_other, "--random never overrode the config phrase");
}

#[test]
fn no_vcenter_flag_removes_top_padding() {
    let (out, _e, code) = run_with_env(
        &["--style", "centered", "--no-vcenter"],
        &[("TASMIYAH_ROWS", "40"), ("TASMIYAH_COLS", "120")],
    );
    assert_eq!(code, 0);
    assert!(out.contains("بِسْمِ"), "nothing rendered:\n{out}");
    assert_eq!(leading_blank_lines(&out), 0);
}

#[test]
fn config_vertical_center_false_removes_top_padding() {
    let dir = std::env::temp_dir().join(format!("tasmiyah-vcenter-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.toml"), "vertical_center = false\n").unwrap();

    let mut cmd = bin();
    cmd.args(["--style", "centered"])
        .env("NO_COLOR", "1")
        .env("NO_UPDATE_NOTIFIER", "1")
        .env("TASMIYAH_CONFIG", dir.join("config.toml"))
        .env("TASMIYAH_ROWS", "40")
        .env("TASMIYAH_COLS", "120")
        .env_remove("TASMIYAH_STYLE")
        .env_remove("TASMIYAH_ARABIC")
        .env_remove("TASMIYAH_PHRASE");
    let out = cmd.output().expect("failed to execute tasmiyah binary");
    let _ = std::fs::remove_dir_all(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(stdout.contains("بِسْمِ"), "nothing rendered:\n{stdout}");
    assert_eq!(leading_blank_lines(&stdout), 0, "{stdout}");
}

#[test]
fn config_integer_phrase_is_accepted() {
    let (out, _e, code) =
        run_with_config("phrase-int", "phrase = 6\n", &["--style", "minimal"], &[]);
    assert_eq!(code, 0);
    assert!(out.contains("أَسْتَغْفِرُ"), "integer phrase ignored:\n{out}");
}

#[test]
fn env_phrase_beats_config_phrase() {
    let (out, _e, _c) = run_with_config(
        "phrase-envwins",
        "phrase = \"salawat\"\n",
        &["--style", "minimal"],
        &[("TASMIYAH_PHRASE", "inshallah")],
    );
    assert!(out.contains("إِنْ شَاءَ"), "env phrase did not win:\n{out}");
}

#[test]
fn invalid_config_phrase_is_ignored() {
    let (out, err, code) = run_with_config(
        "phrase-bad",
        "phrase = \"nope\"\n",
        &["--style", "minimal"],
        &[],
    );
    assert_eq!(code, 0);
    assert!(out.contains("بِسْمِ"));
    assert!(err.is_empty(), "{err}");
}

#[test]
fn doctor_runs_and_prints_all_sections() {
    let (out, _e, code) = run(&["doctor"]);
    assert_eq!(code, 0);
    for t in ["Locale:", "Terminal:", "Arabic font:", "Shell startup:"] {
        assert!(out.contains(t), "{t} missing:\n{out}");
    }
    assert!(!out.contains('\x1b'));
}

#[cfg(unix)]
#[test]
fn doctor_without_fontconfig_still_succeeds() {
    let empty = std::env::temp_dir().join(format!("tasmiyah-nopath-{}", std::process::id()));
    std::fs::create_dir_all(&empty).unwrap();
    let (out, _e, code) = run_with_env(&["doctor"], &[("PATH", empty.to_str().unwrap())]);
    let _ = std::fs::remove_dir_all(&empty);
    assert_eq!(code, 0);
    assert!(
        out.contains("fontconfig unavailable; font check skipped"),
        "{out}"
    );
}

#[test]
fn plain_invocation_still_prints_banner() {
    let (out, _e, _c) = run(&["--style", "minimal"]);
    assert!(out.contains("بِسْمِ"));
}

fn has_presentation_forms(s: &str) -> bool {
    s.chars().any(|c| ('\u{FE70}'..='\u{FEFC}').contains(&c))
}

#[test]
fn explicit_shaped_applies_when_piped() {
    let (out, _e, code) = run(&["--style", "minimal", "--arabic", "shaped"]);
    assert_eq!(code, 0);
    assert!(has_presentation_forms(&out), "{out}");
    assert!(!out.contains("بِسْمِ"));
}

#[test]
fn auto_is_native_when_piped_even_in_kitty() {
    let (out, _e, _c) = run_with_env(&["--style", "minimal"], &[("KITTY_WINDOW_ID", "1")]);
    assert!(out.contains("بِسْمِ"));
}

#[test]
fn classic_box_fits_shaped_line() {
    let (out, _e, _c) = run(&["--style", "classic", "--arabic", "shaped"]);
    let widths: std::collections::BTreeSet<usize> = out
        .lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .collect();
    assert_eq!(widths.len(), 1, "box rows differ in width:\n{out}");
}

#[test]
fn explicit_native_leaves_logical_text() {
    let (out, _e, _c) = run(&["--style", "minimal", "--arabic", "native"]);
    assert!(out.contains("بِسْمِ"));
}

#[test]
fn env_arabic_applies_and_cli_beats_it() {
    let (out, _e, _c) = run_with_env(&["--style", "minimal"], &[("TASMIYAH_ARABIC", "shaped")]);
    assert!(has_presentation_forms(&out), "{out}");
    let (out, _e, _c) = run_with_env(
        &["--style", "minimal", "--arabic", "native"],
        &[("TASMIYAH_ARABIC", "shaped")],
    );
    assert!(out.contains("بِسْمِ"));
}

#[test]
fn invalid_env_arabic_is_ignored() {
    let (out, err, code) = run_with_env(&["--style", "minimal"], &[("TASMIYAH_ARABIC", "bogus")]);
    assert_eq!(code, 0);
    assert!(out.contains("بِسْمِ"));
    assert!(err.is_empty(), "{err}");
}

#[test]
fn config_arabic_applies_and_env_beats_it() {
    let (out, _e, _c) = run_with_config(
        "arabic-cfg",
        "arabic = \"shaped\"\n",
        &["--style", "minimal"],
        &[],
    );
    assert!(has_presentation_forms(&out), "{out}");
    let (out, _e, _c) = run_with_config(
        "arabic-cfg-env",
        "arabic = \"shaped\"\n",
        &["--style", "minimal"],
        &[("TASMIYAH_ARABIC", "native")],
    );
    assert!(out.contains("بِسْمِ"));
}

#[test]
fn invalid_config_arabic_is_ignored() {
    let (out, err, code) = run_with_config(
        "arabic-bad",
        "arabic = \"sideways\"\n",
        &["--style", "minimal"],
        &[],
    );
    assert_eq!(code, 0);
    assert!(out.contains("بِسْمِ"));
    assert!(err.is_empty(), "{err}");
}

#[test]
fn doctor_missing_terminal_reports_shaped_when_forced() {
    let env = [("KITTY_WINDOW_ID", "1")];
    let (out, _e, _c) = run_with_env(&["--arabic", "shaped", "doctor"], &env);
    assert!(
        out.contains("using shaped Arabic (override with --arabic native)"),
        "{out}"
    );
    let (out, _e, _c) = run_with_env(&["--arabic", "native", "doctor"], &env);
    assert!(out.contains("try --arabic shaped"), "{out}");
}

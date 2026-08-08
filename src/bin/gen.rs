//! Build-time artifact generator: shell completions + man page.
//!
//! Usage: `cargo run --bin gen`
//!
//! Writes to `target/gen/`:
//!   completions/tasmiyah.{bash,zsh,fish,ps1} (+ elvish)
//!   man/tasmiyah.1
//!
//! These are packaged into the release archives and referenced by the
//! Homebrew formula.

use std::fs;
use std::path::Path;

use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use clap_mangen::Man;

use tasmiyah_cli::cli::Args;

fn main() -> std::io::Result<()> {
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/gen");
    let completions = out_dir.join("completions");
    let man = out_dir.join("man");
    fs::create_dir_all(&completions)?;
    fs::create_dir_all(&man)?;

    let mut cmd = Args::command();
    let bin = "tasmiyah";

    for shell in [
        shells::Shell::Bash,
        shells::Shell::Zsh,
        shells::Shell::Fish,
        shells::Shell::PowerShell,
        shells::Shell::Elvish,
    ] {
        let path = generate_to(shell, &mut cmd, bin, &completions)?;
        eprintln!("completion: {}", path.display());
    }

    let man_page = Man::new(cmd);
    let mut buf = Vec::new();
    man_page.render(&mut buf)?;
    let man_path = man.join(format!("{bin}.1"));
    fs::write(&man_path, buf)?;
    eprintln!("man page:   {}", man_path.display());

    Ok(())
}

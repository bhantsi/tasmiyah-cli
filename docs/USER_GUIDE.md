# 📖 Tasmiyah-CLI — User Guide

The complete reference for `tasmiyah` users. If you only have 30 seconds,
jump to [Quick Start](#-quick-start). For an overview of the project, see
the [README](../README.md).

---

## Table of Contents

1. [Quick Start](#-quick-start)
2. [Install](#-install)
3. [Command Reference](#-command-reference)
4. [Examples & Recipes](#-examples--recipes)
5. [Shell Integration](#-shell-integration)
6. [Environment Variables](#-environment-variables)
7. [Phrase Library](#-phrase-library)
8. [Troubleshooting](#-troubleshooting)
9. [Upgrade](#-upgrade)
10. [Uninstall](#-uninstall)
11. [Getting Help](#-getting-help)

---

## 🚀 Quick Start

```bash
# Install (any platform with Rust)
cargo install tasmiyah-cli

# Run it
tasmiyah
```

You'll see a large `BISMILLAH` ASCII logo with the Arabic Basmala
beneath it in gold/green. Add `--translation` to also see the
transliteration and English meaning:

```bash
tasmiyah --translation
```

Prefer the original v0.1 unicode-box layout? Pass `--style classic`:

```bash
tasmiyah --style classic --translation
```

That's it — the rest of this guide just shows you the knobs.

---

## 📦 Install

Pick whichever channel fits your workflow.

### Cargo

```bash
cargo install tasmiyah-cli
```

Places `tasmiyah` in `~/.cargo/bin/`, which is already on your `PATH` if
you installed Rust with `rustup`.

### Homebrew (macOS / Linux)

```bash
brew install bhantsi/tap/tasmiyah-cli
```

### Prebuilt binary

Static binaries for every supported OS/architecture are attached to each
[GitHub Release](https://github.com/bhantsi/tasmiyah-cli/releases) along
with `.sha256` checksums.

```bash
# Linux x86_64 — static musl build, works on any distro
curl -L https://github.com/bhantsi/tasmiyah-cli/releases/latest/download/tasmiyah-x86_64-unknown-linux-musl.tar.gz \
  | tar -xz
sudo install -m 755 tasmiyah /usr/local/bin/tasmiyah
```

Available platforms:

| Platform | Targets |
|---|---|
| Linux | `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl` |
| macOS | `aarch64-apple-darwin`, `x86_64-apple-darwin` *(both native)* |
| Windows | `x86_64-pc-windows-msvc` |

### From source

```bash
git clone https://github.com/bhantsi/tasmiyah-cli.git
cd tasmiyah-cli
cargo build --release

# Linux / macOS — system-wide
sudo install -m 755 target/release/tasmiyah /usr/local/bin/tasmiyah

# Or without sudo — make sure ~/.local/bin is on your PATH
mkdir -p ~/.local/bin
cp target/release/tasmiyah ~/.local/bin/
```

---

## 🎛️ Command Reference

`tasmiyah` is a single command with a handful of flags. Every flag has
both a short (`-x`) and a long (`--xxx`) form.

| Flag | Short | Description |
|------|-------|-------------|
| `--no-color` | `-n` | Disable ANSI colors. Useful when piping or for plain-text logs. |
| `--translation` | `-t` | Show the Latin transliteration **and** the English meaning under the Arabic. |
| `--random` | `-r` | Pick a random phrase from the [library](#-phrase-library) instead of the Basmala. |
| `--style <STYLE>` | `-s` | Layout: `centered` *(default)*, `classic`, or `minimal`. See [Styles](#styles). |
| `--font <FONT>` | `-f` | Arabic-width layout hint: `auto` *(default)*, `standard`, or `nerd`. See [Environment Variables](#-environment-variables). |
| `--help` | `-h` | Print the auto-generated help and exit. |
| `--version` | `-V` | Print the version and exit. |

Flags compose freely. The two most common combinations:

- `tasmiyah -t` → default Basmala **with** translation
- `tasmiyah -rt` → random phrase **with** translation

### Styles

| Style | Description |
|---|---|
| `centered` *(default)* | Large `BISMILLAH` ASCII logo, vertically centered, with the phrase line(s) underneath. Falls back to centered text when the terminal is narrower than 65 columns. |
| `classic` | The v0.1 unicode-box layout: phrases inside a `╔═══…═══╗` frame. |
| `minimal` | Plain text only — no logo, no frame, no padding. Ideal for shell prompts and scripts. |

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Banner printed successfully. |
| non-zero | Invalid flag, or stdout write failed (e.g. broken pipe). |

---

## 🧩 Examples & Recipes

### Default Basmala

```bash
tasmiyah
```

### With transliteration + English

```bash
tasmiyah --translation
```

### A random phrase every time

```bash
tasmiyah --random
```

### A random phrase with full translation

```bash
tasmiyah -rt
# equivalent to: tasmiyah --random --translation
```

### Plain text (no colors, no escapes)

```bash
tasmiyah --no-color
```

> 💡 You usually don't even need the flag — `tasmiyah` auto-detects
> when stdout isn't a terminal (pipe, redirect, etc.) and disables
> colors automatically.

### Pick a visual style

```bash
tasmiyah --style centered    # default: large logo, centered
tasmiyah --style classic     # the v0.1 unicode-box layout
tasmiyah --style minimal     # plain text only
```

### One-line greeting in a shell prompt

```bash
tasmiyah --style minimal --translation
```

### Save the banner to a file

```bash
tasmiyah --no-color > bismillah.txt
```

### Use it as a `motd` (message of the day)

```bash
tasmiyah --translation | sudo tee /etc/motd
```

### Show a different phrase on every new tmux pane

```tmux
# in ~/.tmux.conf
set-hook -g session-created 'run-shell "tasmiyah --random --translation"'
```

### Use it in a Git hook

```bash
# .git/hooks/post-commit
#!/usr/bin/env bash
tasmiyah --random
```

---

## 🐚 Shell Integration

Run `tasmiyah` automatically every time you open a new shell session.

The `command -v` / `Get-Command` / `type -q` / `which` guard means:
*if `tasmiyah` isn't installed, fail silently* instead of printing a
confusing "command not found" on every login.

### Bash

Add to `~/.bashrc`:

```bash
command -v tasmiyah >/dev/null 2>&1 && tasmiyah
```

### Zsh

Add to `~/.zshrc`:

```bash
command -v tasmiyah >/dev/null 2>&1 && tasmiyah
```

> ⚡ **Using Powerlevel10k?** Put the line **above** the p10k
> *instant-prompt* block at the top of `~/.zshrc`, or set
> `POWERLEVEL9K_INSTANT_PROMPT=quiet` in `~/.p10k.zsh`. See
> [Powerlevel10k warns about "console output during zsh initialization"](#powerlevel10k-warns-about-console-output-during-zsh-initialization).

### Fish

Add to `~/.config/fish/config.fish`:

```fish
if type -q tasmiyah
    tasmiyah
end
```

### PowerShell

Add to your `$PROFILE` (run `notepad $PROFILE` to edit):

```powershell
if (Get-Command tasmiyah -ErrorAction SilentlyContinue) {
    tasmiyah
}
```

### Nushell

Add to `~/.config/nushell/config.nu`:

```nu
if (which tasmiyah | is-not-empty) { tasmiyah }
```

### Want a random phrase on every shell?

Append flags in any of the snippets above:

```bash
tasmiyah --random --translation
```

---

## 🌐 Environment Variables

`tasmiyah` follows widely-used conventions and introduces nothing custom.

| Variable | Effect |
|----------|--------|
| `NO_COLOR` | If set to **any** value (even empty), disables ANSI colors. Standardized at <https://no-color.org>. |
| `TASMIYAH_FONT` | Override font detection. Set to `nerd` (or `nerd-font`) to force Nerd-Font layout, or `standard` to force the plain layout. Lower precedence than `--font`. |
| `NO_UPDATE_NOTIFIER` | If set to **any** value, suppresses the "a new release is available" footer. See [Upgrade](#-upgrade). |
| `CI` | If set (most CI systems set this automatically), also suppresses the update-check footer. |

stdout TTY detection also disables colors automatically when output is
redirected or piped, so you almost never need to set `NO_COLOR`
manually.

```bash
NO_COLOR=1 tasmiyah      # plain output for this one invocation
export NO_COLOR=1        # plain output for the whole session
```

---

## 📚 Phrase Library

When you pass `--random`, `tasmiyah` picks uniformly from this curated
set:

| # | Arabic | Transliteration | English |
|---|--------|-----------------|---------|
| 1 | بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ | Bismillāh ir-Raḥmān ir-Raḥīm | In the name of Allah, the Most Gracious, the Most Merciful |
| 2 | ٱلْحَمْدُ لِلَّٰهِ | Alḥamdulillāh | All praise is due to Allah |
| 3 | سُبْحَانَ ٱللَّٰهِ | Subḥān Allāh | Glory be to Allah |
| 4 | ٱللَّٰهُ أَكْبَرُ | Allāhu Akbar | Allah is the Greatest |
| 5 | مَا شَاءَ ٱللَّٰهُ | Mā shāʾ Allāh | What Allah has willed |
| 6 | إِنْ شَاءَ ٱللَّٰهُ | In shāʾ Allāh | If Allah wills |
| 7 | أَسْتَغْفِرُ ٱللَّٰهَ | Astaghfirullāh | I seek forgiveness from Allah |
| 8 | لَا إِلَٰهَ إِلَّا ٱللَّٰهُ | Lā ilāha illā Allāh | There is no god but Allah |

Index 1 (the Basmala) is the default when no flags are passed.

---

## 🛠️ Troubleshooting

### The Arabic looks like boxes or question marks

Your terminal font doesn't include Arabic glyphs. Switch to a font that
does:

- **Noto Sans Arabic** (free, ships with most Linux distros)
- **Amiri** (traditional Naskh)
- **Scheherazade New**
- macOS: the system default usually works.
- Windows Terminal: install *Noto Sans Arabic* and set it as the
  profile's `fontFace`.

### The Arabic looks shifted by a column or two

Nerd Fonts (MesloLGS NF, FiraCode NF, JetBrainsMono NF, …) sometimes
draw Arabic slightly wider than `unicode-width` reports. `tasmiyah`
tries to detect this; force it if it doesn't:

```bash
tasmiyah --font nerd          # one-off
export TASMIYAH_FONT=nerd     # for the whole session
```

### The logo doesn't appear in a narrow pane

The `centered` style needs **at least 65 columns** to draw the
`BISMILLAH` logo. Narrower terminals fall back to centered text without
the logo. Either widen the pane or pass `--style minimal` to remove all
decoration.

### Classic-style box borders are misaligned

The terminal is rendering Arabic glyphs with an unexpected visual
width. Try, in order:

1. A different monospace font with good Arabic coverage (Noto Sans
   Mono Arabic).
2. Running `tasmiyah --no-color` — sometimes makes width issues
   visually easier to diagnose.
3. Confirming the terminal's encoding is set to **UTF-8**.

### I see raw ANSI escape codes like `[38;2;...m`

Your terminal isn't an ANSI-aware TTY (old Windows console, an editor
"terminal" that wraps things oddly, etc.). Pass `--no-color` or set
`NO_COLOR=1`.

### "tasmiyah: command not found"

The binary isn't on your `PATH`. Either run it by full path
(`./target/release/tasmiyah`) or copy it somewhere on `PATH` — see
[Install](#-install).

### Starting `tasmiyah` feels slow

It almost certainly isn't — startup is in the 1–5 ms range. If it
feels slow, your **shell** is taking time to load. Time it explicitly:

```bash
time tasmiyah --no-color
```

### Powerlevel10k warns about "console output during zsh initialization"

If you call `tasmiyah` from `~/.zshrc` while using
[Powerlevel10k](https://github.com/romkatv/powerlevel10k), you may see:

```
[WARNING]: Console output during zsh initialization detected.
```

This isn't a `tasmiyah` bug. Powerlevel10k's *instant prompt* feature
sources a cached prompt at the very top of `~/.zshrc` and then buffers
**any** subsequent stdout/stderr until after the prompt is ready, warning
you so the cached prompt doesn't get visually corrupted. Any greeting
tool (`fortune`, `cowsay`, `neofetch`, …) triggers the same warning.

Three ways to fix it — pick one:

1. **Move the `tasmiyah` line above the p10k instant-prompt block** in
   `~/.zshrc`. Cleanest fix — the banner still prints, the warning goes
   away, the instant prompt keeps working:

   ```bash
   # ~/.zshrc
   command -v tasmiyah >/dev/null 2>&1 && tasmiyah   # ← BEFORE the p10k block

   if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
     source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
   fi
   # … rest of your zshrc …
   ```

2. **Silence the warning** without moving anything by adding the
   following to `~/.p10k.zsh` (p10k's official escape hatch):

   ```zsh
   typeset -g POWERLEVEL9K_INSTANT_PROMPT=quiet
   ```

3. **Move the call to `~/.zprofile`** so it only runs for login shells.
   No warning ever, but on Linux GUI terminals (which open *non-login*
   shells by default) you won't see the greeting on every new tab.

For the full upstream explanation, see
<https://github.com/romkatv/powerlevel10k#instant-prompt>.

---

## ⬆️ Upgrade

Upgrade through the same channel you installed from.

> 📡 `tasmiyah` checks crates.io at most once every 24 hours and prints
> a short footer when a newer version exists. The check is skipped when
> stdout isn't a TTY (so scripts stay clean) and can be disabled
> entirely with `NO_UPDATE_NOTIFIER=1`.

### Cargo

```bash
cargo install tasmiyah-cli
```

`cargo install` auto-detects a newer version and replaces the binary
in `~/.cargo/bin/`. You only need `--force` if the installed version
already matches the latest (i.e. you want to reinstall the same
version) or you're downgrading:

```bash
cargo install tasmiyah-cli --version 0.2.0 --force
```

If you use [`cargo-update`](https://crates.io/crates/cargo-update), one
command updates every crate-installed binary on your system:

```bash
cargo install-update -a              # update everything
cargo install-update tasmiyah-cli    # or just this one
```

### Homebrew

```bash
brew update                                   # refresh the tap
brew upgrade bhantsi/tap/tasmiyah-cli         # install the new bottle
```

### Prebuilt binary

Re-download the latest archive for your platform and overwrite the
file on your `PATH`:

```bash
# Linux x86_64
curl -L https://github.com/bhantsi/tasmiyah-cli/releases/latest/download/tasmiyah-x86_64-unknown-linux-musl.tar.gz \
  | tar -xz
sudo install -m 755 tasmiyah /usr/local/bin/tasmiyah
```

### From source

```bash
cd path/to/tasmiyah-cli
git pull
cargo build --release
sudo install -m 755 target/release/tasmiyah /usr/local/bin/tasmiyah
```

### Verifying the upgrade

```bash
tasmiyah --version
```

If it still prints the old version, check that the binary you just
installed is the first `tasmiyah` on your `PATH`:

```bash
which -a tasmiyah
```

### Silencing the update-check footer

```bash
export NO_UPDATE_NOTIFIER=1   # add to ~/.bashrc / ~/.zshrc to persist
```

The footer is also automatically suppressed when:

- Stdout isn't a TTY (scripts, pipes, redirects).
- The `CI` env var is set.

---

## 🧹 Uninstall

```bash
# If you copied the binary manually
sudo rm /usr/local/bin/tasmiyah
# or
rm ~/.local/bin/tasmiyah

# If you installed via cargo
cargo uninstall tasmiyah-cli

# If you installed via Homebrew
brew uninstall tasmiyah-cli
brew untap bhantsi/tap          # optional: remove the tap too
```

Don't forget to also remove the `tasmiyah` line from your shell rc file
(`~/.bashrc`, `~/.zshrc`, `~/.config/fish/config.fish`, or `$PROFILE`).

---

## 🆘 Getting Help

- Run `tasmiyah --help` for the auto-generated CLI help.
- File an issue: <https://github.com/bhantsi/tasmiyah-cli/issues>
- Have an idea for a new phrase or feature? PRs welcome.

May this small tool be a reminder to start every task with His name.
**Bismillāh.** 🌙

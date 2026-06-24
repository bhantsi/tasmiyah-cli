# 📖 Tasmiyah-CLI — User Guide

Welcome! This guide walks you through everything `tasmiyah` can do, from a
first run to advanced shell integration. If you only have 30 seconds, jump to
[Quick Start](#-quick-start). For the project overview, see the
[README](../README.md).

---

## Table of Contents

1. [Quick Start](#-quick-start)
2. [Installing](#-installing)
3. [Command Reference](#-command-reference)
4. [Examples & Recipes](#-examples--recipes)
5. [Shell Integration](#-shell-integration)
6. [Environment Variables](#-environment-variables)
7. [Phrase Library](#-phrase-library)
8. [Tips & Troubleshooting](#-tips--troubleshooting)
9. [Uninstalling](#-uninstalling)
10. [Getting Help](#-getting-help)

---

## 🚀 Quick Start

```bash
# 1. Build it
cargo build --release

# 2. Run it
./target/release/tasmiyah
```

You'll see a large `BISMILLAH` logo centered in your terminal, with the
Arabic Basmala underneath it in gold/green:

```
              ██████╗ ██╗███████╗███╗   ███╗██╗██╗     ██╗      █████╗ ██╗  ██╗
              ██╔══██╗██║██╔════╝████╗ ████║██║██║     ██║     ██╔══██╗██║  ██║
              ██████╔╝██║███████╗██╔████╔██║██║██║     ██║     ███████║███████║
              ██╔══██╗██║╚════██║██║╚██╔╝██║██║██║     ██║     ██╔══██║██╔══██║
              ██████╔╝██║███████║██║ ╚═╝ ██║██║███████╗███████╗██║  ██║██║  ██║
              ╚═════╝ ╚═╝╚══════╝╚═╝     ╚═╝╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝

                  بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ
```

Add `--translation` to also see the transliteration and English meaning:

```bash
tasmiyah --translation
```

Prefer the original v0.1 unicode-box layout? Pass `--style classic`:

```bash
tasmiyah --style classic --translation
```

That's it — the rest of this guide just shows you the knobs.

---

## 📦 Installing

Pick whichever channel fits your workflow.

### Via Cargo

```bash
cargo install tasmiyah-cli
```

This places `tasmiyah` in `~/.cargo/bin/`, which should already be on your
`PATH` if you installed Rust with `rustup`.

### Via Homebrew (macOS / Linux)

```bash
brew install bhantsi/tap/tasmiyah-cli
```

### Prebuilt binaries

Static binaries for every supported OS/architecture are attached to each
[GitHub Release](https://github.com/bhantsi/tasmiyah-cli/releases) along
with `.sha256` checksums.

```bash
# Linux x86_64 (static musl build — works on any distro)
curl -L https://github.com/bhantsi/tasmiyah-cli/releases/latest/download/tasmiyah-x86_64-unknown-linux-musl.tar.gz \
  | tar -xz
sudo mv tasmiyah /usr/local/bin/
```

### From source

```bash
git clone https://github.com/bhantsi/tasmiyah-cli.git
cd tasmiyah-cli
cargo build --release

# Put it on your PATH (Linux / macOS)
sudo cp target/release/tasmiyah /usr/local/bin/

# Or, without sudo, into ~/.local/bin (make sure that's on your PATH)
mkdir -p ~/.local/bin
cp target/release/tasmiyah ~/.local/bin/
```

---

## 🎛️ Command Reference

`tasmiyah` is a single command with a handful of flags. All flags have both a
short (`-x`) and a long (`--xxx`) form.

| Flag | Short | Description |
|------|-------|-------------|
| `--no-color` | `-n` | Disable ANSI colors. Useful when piping output or for plain-text logs. |
| `--translation` | `-t` | Show the Latin-script transliteration **and** English meaning under the Arabic. |
| `--random` | `-r` | Pick a random phrase from the library (instead of the Basmala). |
| `--style <STYLE>` | `-s` | Pick a layout: `centered` (default, large logo), `classic` (v0.1 unicode box), or `minimal` (text only). |
| `--font <FONT>` | `-f` | Layout hint for Arabic widths: `auto` (default), `standard`, or `nerd`. See [Environment Variables](#-environment-variables). |
| `--help` | `-h` | Print the auto-generated help and exit. |
| `--version` | `-V` | Print the version (from `Cargo.toml`) and exit. |

Flags can be combined freely. The two most common combinations are:

- `tasmiyah -t` → default Basmala **with** translation
- `tasmiyah -rt` → random phrase **with** translation

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Banner printed successfully. |
| non-zero | Invalid flag, or stdout write failed (e.g. broken pipe). |

---

## 🧩 Examples & Recipes

### Print the Basmala (default)

```bash
tasmiyah
```

### Print with transliteration + English

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

This is the safe variant for:
- Piping into other tools (`tasmiyah --no-color | tee greeting.txt`)
- Including in scripts that may run in environments without ANSI support
- Capturing into log files

> 💡 You don't even need the flag if you redirect — `tasmiyah` auto-detects
> that stdout isn't a terminal and disables colors automatically.

### Pick a visual style

```bash
tasmiyah --style centered        # default: large logo, centered
tasmiyah --style classic          # the v0.1 unicode-box layout
tasmiyah --style minimal          # plain text only (great for prompts)
```

On very narrow terminals (`< 65` columns), the `centered` style automatically
skips the logo and just prints the centered text lines so nothing wraps.

### One-line greeting in a shell prompt

The `minimal` style emits exactly the phrase line(s), no decoration —
perfect for prompt frameworks:

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

```bash
# in ~/.tmux.conf
set-hook -g session-created 'run-shell "tasmiyah --random --translation"'
```

### Use it in a Git hook

`.git/hooks/post-commit`:

```bash
#!/usr/bin/env bash
tasmiyah --random
```

---

## 🐚 Shell Integration

Run `tasmiyah` automatically every time you open a new shell session.

### Bash

Add to `~/.bashrc`:

```bash
# Greet me when a new shell opens
command -v tasmiyah >/dev/null 2>&1 && tasmiyah
```

The `command -v` guard means: if `tasmiyah` isn't installed, fail silently
instead of printing a confusing "command not found" every login.

### Zsh

Add to `~/.zshrc`:

```bash
command -v tasmiyah >/dev/null 2>&1 && tasmiyah
```

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

Just append the flag in any of the snippets above:

```bash
tasmiyah --random --translation
```

---

## 🌐 Environment Variables

`tasmiyah` follows widely-used conventions and pulls in nothing custom.

| Variable | Effect |
|----------|--------|
| `NO_COLOR` | If set to **any** value (even empty), disables ANSI colors. Standardized at <https://no-color.org>. |
| `TASMIYAH_FONT` | Override font detection. Set to `nerd` (or `nerd-font`) to force Nerd-Font layout, or `standard` to force the plain layout. Lower precedence than the `--font` flag. |

stdout TTY detection also disables colors automatically when output is
redirected or piped — so you almost never need to set `NO_COLOR` manually.

```bash
NO_COLOR=1 tasmiyah          # plain output for this one invocation
export NO_COLOR=1             # plain output for the whole session
```

---

## 📚 Phrase Library

When you pass `--random`, `tasmiyah` picks from this curated set:

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

Index 1 is the default phrase used when no flags are passed.

---

## 🛠️ Tips & Troubleshooting

### The Arabic looks like boxes / question marks

Your terminal font doesn't include Arabic glyphs. Switch to a font that does,
such as:

- **Noto Sans Arabic** (free, ships with most Linux distros)
- **Amiri** (traditional Naskh)
- **Scheherazade New**
- On macOS: the system default usually works out of the box.
- On Windows Terminal: install *Noto Sans Arabic* and add it to your profile's
  `fontFace`.

### The Arabic line is shifted by a column or two

Nerd Fonts (MesloLGS NF, FiraCode NF, JetBrainsMono NF, etc.) sometimes draw
Arabic glyphs slightly wider than `unicode-width` reports. `tasmiyah` tries to
detect this automatically; if it doesn't, force it:

```bash
tasmiyah --font nerd            # one-off
export TASMIYAH_FONT=nerd        # for the whole session
```

### The logo doesn't appear in a narrow pane

The `centered` style needs at least 65 columns to draw the BISMILLAH logo.
Narrower terminals fall back to centered text without the logo. Either widen
the pane or pass `--style minimal` to remove all decoration.

### The box borders are misaligned

This usually means the terminal is rendering Arabic glyphs with an unexpected
visual width. Try:

1. A different monospace font that has Arabic coverage (Noto Sans Mono Arabic).
2. Running `tasmiyah --no-color` — it sometimes makes width issues easier to
   diagnose visually.
3. Making sure your terminal's encoding is set to **UTF-8**.

### The output is "too colorful" for my theme

Use any of these (any one is sufficient):

```bash
tasmiyah --no-color
NO_COLOR=1 tasmiyah
tasmiyah > some-file   # redirection also disables colors
```

### I see ANSI escape codes (like `[38;2;...m`) instead of colors

Your terminal isn't an ANSI-aware TTY (e.g. it's an old Windows console, or
you're inside an editor's terminal that's wrapping things oddly). Pass
`--no-color` or set `NO_COLOR=1`.

### `tasmiyah: command not found`

The binary isn't on your `PATH`. Either:
- Run it by full path: `./target/release/tasmiyah`
- Or copy it somewhere on `PATH` (see [Installing](#-installing)).

### It feels slow to start in my shell

It really shouldn't — startup is in the 1–5 ms range. If it feels slow, you're
almost certainly seeing your *shell* take time to load, not `tasmiyah`. Time
it explicitly:

```bash
time tasmiyah --no-color
```

---

## 🧹 Uninstalling

```bash
# If you copied the binary
sudo rm /usr/local/bin/tasmiyah
# or
rm ~/.local/bin/tasmiyah

# If you installed via cargo
cargo uninstall tasmiyah-cli
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

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
7. [Configuration File](#️-configuration-file)
8. [Phrase Library](#-phrase-library)
9. [Diagnostics](#-diagnostics)
10. [Troubleshooting](#-troubleshooting)
11. [Upgrade](#-upgrade)
12. [Uninstall](#-uninstall)
13. [Getting Help](#-getting-help)

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

### Shell completions & man page

Each release archive bundles generated shell completions (bash, zsh,
fish, PowerShell, Elvish) under `completions/` and a man page under
`man/`. Homebrew installs them automatically; from a manual binary
install you can copy them yourself:

```bash
# bash
sudo install -m 644 completions/tasmiyah.bash /usr/share/bash-completion/completions/tasmiyah
# zsh
sudo install -m 644 completions/_tasmiyah /usr/share/zsh/site-functions/_tasmiyah
# fish
sudo install -m 644 completions/tasmiyah.fish /usr/share/fish/vendor_completions.d/tasmiyah.fish
# man page
sudo install -m 644 man/tasmiyah.1 /usr/local/share/man/man1/tasmiyah.1
```

To regenerate them from source (they're built with `clap_complete` /
`clap_mangen`, gated behind a cargo feature so the shipped binary stays
lean):

```bash
cargo run --features gen --bin gen   # writes to target/gen/
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
| `--phrase <INDEX\|NAME>` | `-p` | Show a specific phrase by index (`0`–`11`) or name (e.g. `alhamdulillah`). Conflicts with `--random`. See [Phrase Library](#-phrase-library). |
| `--list` | `-l` | List every phrase with its index and name, then exit. |
| `--no-vcenter` | — | Don't pad the centered banner vertically; start right below existing output. Config: `vertical_center = false`. |
| `--style <STYLE>` | `-s` | Layout: `centered` *(default)*, `classic`, or `minimal`. See [Styles](#styles). |
| `--font <FONT>` | `-f` | Arabic-width layout hint: `auto` *(default)*, `standard`, or `nerd`. See [Environment Variables](#-environment-variables). |
| `--theme <THEME>` | — | Color palette: `gold` *(default)*, `ocean`, `rose`, or `mono`. See [Themes](#themes). |
| `--help` | `-h` | Print the auto-generated help and exit. |
| `--version` | `-V` | Print the version and exit. |

Flags compose freely. The two most common combinations:

- `tasmiyah -t` → default Basmala **with** translation
- `tasmiyah -rt` → random phrase **with** translation
- `tasmiyah -p subhanallah -t` → a specific phrase **with** translation

An invalid `--phrase` value prints an error on stderr and exits with
status 2; run `tasmiyah --list` to see valid indices and names.

### Styles

| Style | Description |
|---|---|
| `centered` *(default)* | Large ASCII logo, vertically centered, with the phrase line(s) underneath. The logo matches the phrase (BISMILLAH for the default; ALHAMDULILLAH, SUBHANALLAH, etc. when `--random` or `--phrase` selects one with art). Falls back to centered text when the terminal is narrower than the logo. |
| `classic` | The v0.1 unicode-box layout: phrases inside a `╔═══…═══╗` frame. |
| `minimal` | Plain text only — no logo, no frame, no padding. Ideal for shell prompts and scripts. |

### Themes

The active palette is chosen with `--theme`, falling back to
`TASMIYAH_THEME`, then the [config file](#%EF%B8%8F-configuration-file),
then `gold`.

| Theme | Feel |
|---|---|
| `gold` *(default)* | Warm gold border, green Arabic — the original "manuscript" look. |
| `ocean` | Teal/deep-blue border, cyan Arabic. |
| `rose` | Soft rose/pink border, magenta Arabic. |
| `mono` | No per-element colors — everything in the terminal's default foreground. |

Individual colors can be overridden slot-by-slot in the config file's
`[colors]` table (see [Configuration File](#%EF%B8%8F-configuration-file)).

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

> 💡 When stdout is not a terminal (pipes, files, `/etc/motd`), the
> `centered` style skips the vertical top/bottom padding unless
> `TASMIYAH_ROWS` is set. For just the text with no decoration at all,
> add `--style minimal`.

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
confusing "command not found" on every login. (The bash/zsh snippets
also check `~/.cargo/bin/tasmiyah` directly first; see the note in those
sections.)

### Bash

Add to `~/.bashrc`:

```bash
if [ -x "$HOME/.cargo/bin/tasmiyah" ]; then "$HOME/.cargo/bin/tasmiyah"
elif command -v tasmiyah >/dev/null 2>&1; then tasmiyah; fi
```

> 💡 Why not just `command -v tasmiyah`? `~/.cargo/env` (which adds
> `~/.cargo/bin` to `PATH`) is often sourced *later* in the rc file, so
> `command -v` fails when the line sits near the top (e.g. above the p10k
> block). Checking `~/.cargo/bin/tasmiyah` directly avoids that.

### Zsh

Add to `~/.zshrc`:

```bash
if [ -x "$HOME/.cargo/bin/tasmiyah" ]; then "$HOME/.cargo/bin/tasmiyah"
elif command -v tasmiyah >/dev/null 2>&1; then tasmiyah; fi
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

### Running another greeter first?

If you run something like `fastfetch` before `tasmiyah`, use
`--no-vcenter` so the banner starts right below its output instead of
being padded down the terminal.

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
| `TASMIYAH_STYLE` | Default style (`classic` / `centered` / `minimal`). Lower precedence than `--style`, higher than the config file. |
| `TASMIYAH_PHRASE` | Default phrase (index `0`–`11` or name; see `tasmiyah --list`). Lower precedence than `--phrase` and `--random`, higher than the config file. Invalid values are ignored. |
| `TASMIYAH_THEME` | Default theme (`gold` / `ocean` / `rose` / `mono`). Lower precedence than `--theme`, higher than the config file. |
| `TASMIYAH_FONT` | Override font detection. Set to `nerd` (or `nerd-font`) to force Nerd-Font layout, or `standard` to force the plain layout. Lower precedence than `--font`. |
| `TASMIYAH_COLS` / `TASMIYAH_ROWS` | Override the detected terminal size (columns/rows). Mainly useful for tests, screenshots, and demos. |
| `TASMIYAH_CONFIG` | Full path to a config TOML file; overrides the default config location (e.g. for testing or dotfiles). |
| `NO_UPDATE_NOTIFIER` | If set to **any** value, suppresses the "a new release is available" footer. See [Upgrade](#-upgrade). |
| `CI` | If set (most CI systems set this automatically), also suppresses the update-check footer. |

stdout TTY detection also disables colors automatically when output is
redirected or piped, so you almost never need to set `NO_COLOR`
manually. `TERM=dumb` disables colors too.

```bash
NO_COLOR=1 tasmiyah      # plain output for this one invocation
export NO_COLOR=1        # plain output for the whole session
```

---

## ⚙️ Configuration File

For preferences you want every session, `tasmiyah` reads an optional
TOML config file instead of forcing flags into your shell rc:

- **Linux:** `~/.config/tasmiyah/config.toml`
- **macOS:** `~/Library/Application Support/tasmiyah/config.toml`
- **Windows:** `%APPDATA%\tasmiyah\config.toml`

Set `TASMIYAH_CONFIG=/path/to/file.toml` to override the config location
(e.g. for testing or dotfiles).

Everything is optional; a missing or malformed file falls back to
defaults (a malformed file also prints a one-line warning on stderr in
interactive sessions).

```toml
style = "classic"          # classic | centered | minimal
font = "nerd"              # standard | nerd | auto
theme = "ocean"            # gold | ocean | rose | mono
phrase = "alhamdulillah"   # index (0-11) or name; see `tasmiyah --list`
translation = true         # always show transliteration + English
vertical_center = false    # no top padding in the centered style (like --no-vcenter)

[colors]                   # per-element overrides, #RRGGBB or a name
border = "#d4af37"
arabic = "green"
translit = "yellow"
english = "white"
```

**Precedence** (highest first): CLI flag → env var → config file →
built-in default.

---

## 📚 Phrase Library

When you pass `--random`, `tasmiyah` picks uniformly from this curated
set. Select one explicitly with `--phrase <INDEX|NAME>` (or
`TASMIYAH_PHRASE` / the config file's `phrase`); `tasmiyah --list` prints
this table's index and name columns. Names are case-insensitive and
ignore spaces and punctuation, and a unique prefix of 3+ characters is enough (for
example `allahu` or `hasb`).

| # | Name | Arabic | Transliteration | English |
|---|------|--------|-----------------|---------|
| 0 | `bismillah` | بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ | Bismillāh ir-Raḥmān ir-Raḥīm | In the name of Allah, the Most Gracious, the Most Merciful |
| 1 | `alhamdulillah` | ٱلْحَمْدُ لِلَّٰهِ | Alḥamdulillāh | All praise is due to Allah |
| 2 | `subhanallah` | سُبْحَانَ ٱللَّٰهِ | Subḥān Allāh | Glory be to Allah |
| 3 | `allahuakbar` | ٱللَّٰهُ أَكْبَرُ | Allāhu Akbar | Allah is the Greatest |
| 4 | `mashallah` | مَا شَاءَ ٱللَّٰهُ | Mā shāʾ Allāh | What Allah has willed |
| 5 | `inshallah` | إِنْ شَاءَ ٱللَّٰهُ | In shāʾ Allāh | If Allah wills |
| 6 | `astaghfirullah` | أَسْتَغْفِرُ ٱللَّٰهَ | Astaghfirullāh | I seek forgiveness from Allah |
| 7 | `tahlil` | لَا إِلَٰهَ إِلَّا ٱللَّٰهُ | Lā ilāha illā Allāh | There is no god but Allah |
| 8 | `hasbunallah` | حَسْبُنَا ٱللَّٰهُ وَنِعْمَ ٱلْوَكِيلُ | Ḥasbunā Allāhu wa niʿmal wakīl | Allah is sufficient for us, and He is the best Disposer of affairs |
| 9 | `hawqala` | لَا حَوْلَ وَلَا قُوَّةَ إِلَّا بِٱللَّٰهِ | Lā ḥawla wa lā quwwata illā billāh | There is no might nor power except with Allah |
| 10 | `rabbana` | رَبَّنَا آتِنَا فِي ٱلدُّنْيَا حَسَنَةً | Rabbanā ātinā fi d-dunyā ḥasanah | Our Lord, give us in this world that which is good |
| 11 | `salawat` | ٱللَّٰهُمَّ صَلِّ عَلَىٰ مُحَمَّدٍ | Allāhumma ṣalli ʿalā Muḥammad | O Allah, send blessings upon Muhammad |

Index 0 (the Basmala) is the default when no flags are passed.

In the `centered` style, phrases 0–7 have a dedicated ASCII logo; the
newer phrases (8–11) render as centered text only.

---

## 🩺 Diagnostics

`tasmiyah doctor` checks the usual causes of broken Arabic and shell-startup
problems. It prints four lines, never changes anything, and always exits 0.

```console
$ tasmiyah doctor --font-family "JetBrainsMono Nerd Font"
✓ Locale: en_US.UTF-8 is UTF-8
✓ Terminal: VTE renders Arabic right-to-left
✓ Arabic font: checked "JetBrainsMono Nerd Font": served by Kawkab Mono
i Shell startup: installed in ~/.cargo/bin, which some shells add to PATH only after rc files run; in rc files call "$HOME/.cargo/bin/tasmiyah"
```

Marks: `✓` fine, `!` a likely problem, `i` information only.

- **Locale** — your `LC_ALL` / `LC_CTYPE` / `LANG` must name a UTF-8 locale.
- **Terminal** — whether your terminal is known to render Arabic right-to-left
  (BiDi). `i` means it could not be identified.
- **Arabic font** — asks fontconfig (`fc-match`) which font serves the Arabic
  letters for `--font-family` (default `monospace`; pass your terminal's font
  for an exact check). A `!` means the letters are split across several fonts
  or served by a proportional font, so they won't join; see
  [the fontconfig recipe](#arabic-letters-look-disconnected-or-have-gaps).
  Without fontconfig (macOS, Windows) this check is skipped.
- **Shell startup** — where the binary is installed, with a tip when it lives
  in `~/.cargo/bin` and your shell rc files may not find it.

---

## 🛠️ Troubleshooting

> Not sure what is wrong? Run `tasmiyah doctor` first; see [Diagnostics](#-diagnostics).

### The Arabic looks like boxes or question marks

Your terminal font doesn't include Arabic glyphs. Switch to a font that
does:

- **Noto Sans Arabic** (free, ships with most Linux distros)
- **Amiri** (traditional Naskh)
- **Scheherazade New**
- macOS: the system default usually works.
- Windows Terminal: install *Noto Sans Arabic* and set it as the
  profile's `fontFace`.

Noto Sans Arabic fixes *coverage* (no more boxes). If the letters show
up but look disconnected, see
[Arabic letters look disconnected or have gaps](#arabic-letters-look-disconnected-or-have-gaps).

### Arabic letters look disconnected or have gaps

Two things commonly cause this on Linux terminals:

1. **Per-glyph font fallback splits one word across several fonts.**
   Fontconfig may pick one font for most letters and another for a
   few (for example the alef wasla `ٱ` in the Basmala), which breaks the
   joins. Check which font each character resolves to:

   ```bash
   fc-match "<Your Font>:charset=0628" family   # ب
   fc-match "<Your Font>:charset=0671" family   # ٱ
   ```

   If the two answers differ, the word is being split.
2. **Terminals draw every glyph in a fixed-width cell.** Proportional
   Arabic fonts (Noto Sans Arabic, Amiri, DejaVu Sans) don't reach the
   cell edges, leaving visible gaps between letters.

**Fix:** use fixed-width Arabic fonts as the fallback chain.

1. Install **Kawkab Mono** (OFL licensed, no sudo needed):

   ```bash
   mkdir -p ~/.local/share/fonts/kawkab-mono
   curl -L -o /tmp/kawkab-mono.zip https://github.com/aiaf/kawkab-mono/releases/download/v0.501/kawkab-mono-0.501.zip
   unzip -o /tmp/kawkab-mono.zip -d ~/.local/share/fonts/kawkab-mono/
   rm /tmp/kawkab-mono.zip
   fc-cache -f
   ```

2. Install **Thabit** (covers the isolated forms Kawkab Mono lacks):

   ```bash
   sudo apt install fonts-hosny-thabit     # Debian / Ubuntu
   ```

3. Copy [`docs/fontconfig/60-tasmiyah-arabic.conf`](fontconfig/60-tasmiyah-arabic.conf)
   to `~/.config/fontconfig/conf.d/` and replace `JetBrainsMono Nerd Font`
   in it with your terminal's exact font family. Find the exact name with
   `fc-list : family | grep -i <name>`; if your terminal uses the system
   default *Monospace*, use the family printed by `fc-match monospace family`.
   Then rebuild the cache:

   ```bash
   fc-cache -f
   ```

4. **Quit the terminal completely** (close all its windows — a new tab
   is not enough) and reopen it.

Your primary font stays the Nerd Font, so Latin text, icons and your
prompt are unaffected; only the Arabic fallback changes.

Verify — every line should print `Kawkab Mono`:

```bash
for c in 0628 0644 FEDF 0651 0671; do fc-match "<Your Font>:charset=$c" family; done
```

(Kawkab Mono covers U+0671 and all base letters and harakat, but lacks
the isolated presentation forms, which fall through to Thabit
automatically. Noto Sans Arabic is only needed for the alef wasla
presentation forms U+FB50/U+FB51, which Thabit also lacks.)

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

1. A different monospace font with good Arabic coverage — Kawkab Mono
   (see [Arabic letters look disconnected or have gaps](#arabic-letters-look-disconnected-or-have-gaps)).
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

### The greeting doesn't appear in new terminals

Check, in order:

1. **`PATH` ordering.** `~/.cargo/env` is often sourced *after* the
   line that calls `tasmiyah`, so `command -v tasmiyah` fails at that
   point. Use the snippet from [Shell Integration](#-shell-integration),
   which checks `~/.cargo/bin/tasmiyah` directly.
2. **The terminal runs a different shell than you edited.** Some
   terminals are configured to run a custom command — e.g. Tilix *Run a
   custom command* = `bash --login` reads `~/.profile` → `~/.bashrc`,
   not `~/.zshrc`. Check what is actually running:

   ```bash
   ps -o comm= -p $$
   ```

   Then add the snippet to the matching rc file.
3. **Retest** without opening a new window:

   ```bash
   exec $SHELL
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
   # tasmiyah — keep this BEFORE the p10k block
   if [ -x "$HOME/.cargo/bin/tasmiyah" ]; then "$HOME/.cargo/bin/tasmiyah"
   elif command -v tasmiyah >/dev/null 2>&1; then tasmiyah; fi

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

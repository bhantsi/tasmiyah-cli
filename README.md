# 🌙 Tasmiyah-CLI

> A beautiful, fast terminal greeting that prints **Bismillah** and other Islamic phrases — written in Rust.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)]()
[![CI](https://github.com/bhantsi/tasmiyah-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/bhantsi/tasmiyah-cli/actions/workflows/ci.yml)
[![Release](https://github.com/bhantsi/tasmiyah-cli/actions/workflows/release.yml/badge.svg)](https://github.com/bhantsi/tasmiyah-cli/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/tasmiyah-cli.svg)](https://crates.io/crates/tasmiyah-cli)

---

## ✨ What is this?

`tasmiyah-cli` is a tiny terminal utility — in the spirit of [`neofetch`](https://github.com/dylanaraps/neofetch), [`cowsay`](https://github.com/piuccio/cowsay), and [`fortune`](https://en.wikipedia.org/wiki/Fortune_(Unix)) — that prints **بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ** (*"In the name of Allah, the Most Gracious, the Most Merciful"*) every time you open your terminal.

It's fast (single static binary, ~1-5ms startup), looks beautiful, and works on **Linux, macOS, and Windows**.

## 🖼️ Preview

```
╔════════════════════════════════════════════╗
║                                            ║
║      بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ      ║
║                                            ║
║    Bismillāh ir-Raḥmān ir-Raḥīm           ║
║    In the name of Allah, the Most          ║
║    Gracious, the Most Merciful             ║
║                                            ║
╚════════════════════════════════════════════╝
```

## 📦 Installation

### Via Cargo

```bash
cargo install tasmiyah-cli
```

### Via Homebrew (macOS / Linux)

```bash
brew install bhantsi/tap/tasmiyah-cli
```

### Prebuilt binaries

Grab the archive for your platform from the
[Releases page](https://github.com/bhantsi/tasmiyah-cli/releases), extract it,
and drop `tasmiyah` somewhere on your `PATH`:

```bash
# Linux x86_64 (static musl build — works on any distro)
curl -L https://github.com/bhantsi/tasmiyah-cli/releases/latest/download/tasmiyah-x86_64-unknown-linux-musl.tar.gz \
  | tar -xz
sudo mv tasmiyah /usr/local/bin/
```

Archives are published for: Linux (x86_64 glibc/musl, aarch64 musl),
macOS (Intel & Apple Silicon), and Windows (x86_64). Each is accompanied by
a `.sha256` checksum file.

## 🚀 Usage

```bash
tasmiyah                 # print the default Bismillah banner
tasmiyah --no-color      # plain text, no ANSI colors
tasmiyah --translation   # show transliteration + English meaning
tasmiyah --random        # random Islamic phrase (Alhamdulillah, SubhanAllah, ...)
tasmiyah --help          # show all options
```

📖 **Want more detail?** See the full [User Guide](docs/USER_GUIDE.md) for
flag-by-flag reference, recipes, shell integration for every major shell,
environment variables, and troubleshooting.

## 🐚 Shell Integration

Add to your shell config to print Bismillah every time you open a terminal:

**Bash** (`~/.bashrc`):
```bash
tasmiyah
```

**Zsh** (`~/.zshrc`):
```bash
tasmiyah
```

**Fish** (`~/.config/fish/config.fish`):
```fish
tasmiyah
```

**PowerShell** (`$PROFILE`):
```powershell
tasmiyah
```

## 🛣️ Roadmap

- [x] Step 1: Create repository
- [x] Step 2: Scaffold Rust project
- [x] Step 3: Arabic banner with decorative box
- [x] Step 4: ANSI colors (gold/green theme)
- [x] Step 5: CLI flags (`clap`)
- [x] Step 6: Transliteration + English translation
- [x] Step 7: Random Islamic phrases mode
- [x] Step 8: Cross-platform testing (unit + integration tests, CI on Linux/macOS/Windows)
- [x] Step 9: GitHub Actions release pipeline (prebuilt binaries for 6 targets, automated on tag push)
- [x] Step 10: Publish to crates.io + Homebrew tap (automated by the release workflow)

🎉 **All roadmap items complete!** Future work tracked in [GitHub Issues](https://github.com/bhantsi/tasmiyah-cli/issues).

## 🛠️ Build from source

```bash
git clone https://github.com/bhantsi/tasmiyah-cli.git
cd tasmiyah-cli
cargo build --release
./target/release/tasmiyah
```

## 🧪 Running the tests

```bash
cargo test            # unit tests + cross-platform integration tests
cargo fmt --check     # formatting
cargo clippy -- -D warnings  # lints
```

## 🛬 Cutting a release (maintainers)

See [docs/RELEASING.md](docs/RELEASING.md) for the full release process.
Short version: bump `Cargo.toml`, tag `v*.*.*`, push — the GitHub Actions
release pipeline builds and publishes binaries for all 6 targets automatically.

## 📜 License

MIT © [bhantsi](https://github.com/bhantsi)

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
macOS (Apple Silicon), and Windows (x86_64). Each is accompanied by a
`.sha256` checksum file. Intel-Mac users should install with
`cargo install tasmiyah-cli`.

### Upgrading

`tasmiyah` prints a one-line notice the next time you run it after a new
release lands on crates.io. To actually upgrade, use the same channel you
originally installed from:

```bash
# Cargo — auto-detects the new version and replaces the binary.
cargo install tasmiyah-cli

# Homebrew
brew update && brew upgrade bhantsi/tap/tasmiyah-cli

# Prebuilt binary — re-download the latest archive and replace the file
# on your PATH (see the install command above).
```

Confirm with `tasmiyah --version`. To silence the update-check footer,
set `NO_UPDATE_NOTIFIER=1` in your shell rc.

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

## ️ Build from source

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

Maintainers: see [docs/RELEASING.md](docs/RELEASING.md) for the release process.

## 📜 License

MIT © [bhantsi](https://github.com/bhantsi)

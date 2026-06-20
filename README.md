# 🌙 Tasmiyah-CLI

> A beautiful, fast terminal greeting that prints **Bismillah** and other Islamic phrases — written in Rust.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)]()

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

> 🚧 _Coming soon — being built step by step._

```bash
# Via cargo (once published)
cargo install tasmiyah-cli

# Via Homebrew (macOS/Linux)
brew install bhantsi/tap/tasmiyah-cli

# Via prebuilt binary
# Download from the Releases page
```

## 🚀 Usage

```bash
tasmiyah                 # print the default Bismillah banner
tasmiyah --no-color      # plain text, no ANSI colors
tasmiyah --translation   # show transliteration + English meaning
tasmiyah --random        # random Islamic phrase (Alhamdulillah, SubhanAllah, ...)
tasmiyah --help          # show all options
```

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
- [ ] Step 2: Scaffold Rust project ← **in progress**
- [ ] Step 3: Arabic banner with decorative box
- [ ] Step 4: ANSI colors (gold/green theme)
- [ ] Step 5: CLI flags (`clap`)
- [ ] Step 6: Transliteration + English translation
- [ ] Step 7: Random Islamic phrases mode
- [ ] Step 8: Cross-platform testing
- [ ] Step 9: GitHub Actions release pipeline
- [ ] Step 10: Publish to crates.io + Homebrew

## 🛠️ Build from source

```bash
git clone https://github.com/bhantsi/tasmiyah-cli-.git
cd tasmiyah-cli-
cargo build --release
./target/release/tasmiyah
```

## 📜 License

MIT © [bhantsi](https://github.com/bhantsi)

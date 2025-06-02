# 🧰 Runefix CLI - Unicode-Aware Display Toolkit

**Runefix CLI** is a powerful, minimal, and precise command-line toolkit for working with Unicode text — especially in CJK and terminal environments. It acts as the unified frontend for the [`runefix-core`](https://crates.io/crates/runefix-core) engine.

[![Crates.io](https://img.shields.io/crates/v/runefix-cli)](https://crates.io/crates/runefix-cli)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/runefix-labs/runefix-cli/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/runefix-labs/runefix-cli/actions/workflows/ci.yml)

---

## ✨ Features

- 🔍 Display-aware **width calculation** (CJK, emoji, symbols)
- ✂️ Accurate **grapheme** and **atom** segmentation
- 🛡️ Safe **slicing** and **truncation** based on visible width
- 🧠 Unicode grapheme and width policy support via [`runefix-core`](https://crates.io/crates/runefix-core)
- ⚡ Zero-bloat CLI: All dependencies are carefully selected for performance, size, and clarity
- 🧭 Built-in `init` command with **XDG-compliant** config path (`~/.config/runefix/`)
- 🐍 Powerful `slice` command inspired by **Python-style slicing**, designed to replace `cut` in Unicode-aware contexts


## 🚀 Installation

### Via Cargo

```bash
cargo install runefix-cli
```

This will install the CLI binary as `runefix`.


## 🛠️ Initialization

After installing `Runefix`, you can optionally run `runefix init` to configure your preferred language. This step is **not required** — the default fallback is English. The config file follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/), and currently supports three languages:

```bash
# When you run `runefix init`, you'll be prompted:

? 🎛️  Select your preferred language:
> 🇺🇸 English (en-US)
  🇨🇳 简体中文 (zh-CN)
  🇯🇵 日本語 (ja-JP)

# After pressing Enter, you'll see:
🎛️  Select your preferred language: 🇺🇸 English (en-US)

✅  Language saved to "~/.config/runefix/config"
```


## 🧪 Example Usage

Here are a few quick examples to demonstrate **Runefix CLI** commands:

```bash
runefix atoms "Love 👩‍❤️‍💋‍👨"                    # Segment Unicode atoms
runefix graphemes "Hello 👋 世界"           # Identify grapheme clusters
runefix width --verbose "Hello 👋 世界"     # Measure display width
runefix slice -g "[0:3]" "你好👨‍👩‍👧‍👦Hello世界"  # Slice by grapheme index
```

📄 Full CLI output: [examples/output.txt](examples/output.txt)


## 🔧 Commands

| Command     | Description                                   |
| ----------- | --------------------------------------------- |
| `width`     | Total display width of the input              |
| `widths`    | Width of each segment                         |
| `graphemes` | Split input into Unicode grapheme clusters    |
| `atoms`     | Split input into width-sensitive visual atoms |
| `split`     | Chunk text into lines with max width          |
| `truncate`  | Truncate string to a max visible width        |
| `slice`     | Slice string using layout-aware range         |
| `version`   | Show version and runtime info                 |
| `init`      | Create config scaffold (if needed)            |

Use `--help` with any command for more details.


## 📦 Module Overview

- `commands/cmds/` – All CLI subcommands (slice, width, truncate, etc.)
- `config/` – Global config loader & context handler
- `i18n/` – Multilingual support (en, zh, jp)
- `style/` – Terminal print utils (color, alignment)


## 📌 CHANGELOG

See [CHANGELOG.md](./CHANGELOG.md) for version history.


## 📜 License

Licensed under either of:

- MIT License 
- Apache License (Version 2.0)

See [`LICENSE-MIT`](./LICENSE-MIT) or [`LICENSE-APACHE`](./LICENSE-APACHE) for full terms.


## 💬 Contributing

We welcome all forms of contribution — from ideas and bug reports to pull requests and feedback.

**Ways to contribute:**

- 🐞 [Report a bug](https://github.com/runefix-labs/runefix-cli/discussions/categories/bug-reports)
- 🙋 [Ask a question or get help](https://github.com/runefix-labs/runefix-cli/discussions/categories/help)
- 💡 [Suggest a new feature](https://github.com/runefix-labs/runefix-cli/discussions/categories/ideas)
- ✨ [Show off your usage or project](https://github.com/runefix-labs/runefix-cli/discussions/categories/show-tell)
- 🔧 [Submit a pull request](https://github.com/runefix-labs/runefix-cli/pulls)

If you're unsure where to start, feel free to [start a discussion](https://github.com/runefix-labs/runefix-cli/discussions) — we're friendly!

⭐ Star the repo if you find it useful — it helps the project grow.

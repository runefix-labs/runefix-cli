# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/).


## [0.1.0] - 2025-05-31

### Added
- Initial release of `runefix-cli`.
- Supports subcommands: `atoms`, `graphemes`, `width`, `widths`, `split`, `truncate`, `slice`, `version`, and `init`.
- Built-in multilingual i18n system with `--init` config.
- Integration with [`runefix-core`](https://crates.io/crates/runefix-core) for accurate Unicode width policies.
- Python-style slice command supporting grapheme, char, width modes.

//! Loads and parses user-level configuration for `runefix`.
//!
//! This module reads from a platform-specific config file:
//! - Linux/macOS: `$XDG_CONFIG_HOME/runefix/config` or `~/.config/runefix/config`
//! - Windows: `{FOLDERID_RoamingAppData}\runefix\config`
//!
//! # Supported Keys
//! - `lang = zh-CN | en-US | ja-JP`
//!
//! # Fallback
//! - If config file is missing or invalid, defaults to `en-US`.

use crate::i18n::lang::Lang;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub lang: Lang,
}

impl Config {
    /// Load configuration from platform-specific path.
    pub fn load() -> Self {
        let path = Self::config_path();

        let content = fs::read_to_string(&path).unwrap_or_default();
        let lang = Self::parse_lang(&content).unwrap_or(Lang::EnUS);

        Config { lang }
    }

    /// Return the platform-specific config file path following XDG spec (Linux/macOS)
    /// and %APPDATA% convention (Windows).
    ///
    /// Examples:
    /// - Linux/macOS: ~/.config/runefix/config or $XDG_CONFIG_HOME/runefix/config
    /// - Windows:     %APPDATA%\runefix\config
    #[cfg(not(target_os = "windows"))]
    pub fn config_path() -> PathBuf {
        if let Some(dir) = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
            dir.join("runefix").join("config")
        } else if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
            home.join(".config").join("runefix").join("config")
        } else {
            eprintln!("❌ Could not determine config path (missing $HOME or $XDG_CONFIG_HOME)");
            std::process::exit(1);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn config_path() -> PathBuf {
        if let Some(appdata) = std::env::var_os("APPDATA").map(PathBuf::from) {
            appdata.join("runefix").join("config")
        } else {
            eprintln!("❌ Could not determine config path (missing %APPDATA%)");
            std::process::exit(1);
        }
    }

    fn parse_lang(content: &str) -> Option<Lang> {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "lang" {
                    let lang_str = value.trim().trim_matches('"');
                    return match lang_str {
                        "zh-CN" => Some(Lang::ZhCN),
                        "en-US" => Some(Lang::EnUS),
                        "ja-JP" => Some(Lang::JaJP),
                        _ => None,
                    };
                }
            }
        }
        None
    }
}

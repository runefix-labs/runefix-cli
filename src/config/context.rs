//! Global runtime context for `runefix` CLI.
//!
//! This module defines a `Context` struct that holds global runtime state,
//! such as the user's selected language. It is initialized once at startup
//! by reading the user's config file (e.g. `~/.config/runefix/config` on Linux).
//!
//! The context is passed into all command handlers to enable consistent behavior.

use crate::config::settings::Config;
use crate::i18n::{keys::MessageKey, lang::Lang};

/// Global runtime context
///
/// Holds resolved runtime configuration such as language,
/// and provides helpers like internationalized message lookup.
#[derive(Debug, Clone)]
pub struct Context {
    pub lang: Lang,
}

impl Context {
    /// Initialize context by loading user configuration from disk.
    ///
    /// Falls back to defaults (e.g. `en-US`) if config file is missing or invalid.
    pub fn init() -> Self {
        let config = Config::load();
        Context { lang: config.lang }
    }

    /// Universal i18n resolver: supports any key type (e.g. `TitleKey`, `ErrorKey`)
    ///
    /// Looks up a localized message for the current language. Falls back to debug text if missing.
    pub fn t<K: MessageKey>(&self, key: K) -> String {
        key.lookup(&self.lang)
            .unwrap_or_else(|| format!("{{:?}} {:?}", key))
    }
}

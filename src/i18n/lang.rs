//! Language selector for i18n tables.
//!
//! Provides methods to load localized strings by category.

use crate::i18n::keys::{ErrorKey, FooterKey, ReportKey, TitleKey};
use crate::i18n::locales::{en_us, ja_jp, zh_cn};
use std::collections::HashMap;

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    EnUS,
    ZhCN,
    JaJP,
}

impl Lang {
    /// Returns the language code string.
    pub fn code(&self) -> &'static str {
        match self {
            Lang::EnUS => "en-US",
            Lang::ZhCN => "zh-CN",
            Lang::JaJP => "ja-JP",
        }
    }

    /// Returns localized error messages for the selected language.
    pub fn error(self) -> HashMap<ErrorKey, &'static str> {
        match self {
            Lang::EnUS => en_us::error_en_us(),
            Lang::ZhCN => zh_cn::error_zh_cn(),
            Lang::JaJP => ja_jp::error_ja_jp(),
        }
    }

    /// Returns localized section titles.
    pub fn title(self) -> HashMap<TitleKey, &'static str> {
        match self {
            Lang::EnUS => en_us::title_en_us(),
            Lang::ZhCN => zh_cn::title_zh_cn(),
            Lang::JaJP => ja_jp::title_ja_jp(),
        }
    }

    /// Returns localized footer messages or legends.
    pub fn footer(self) -> HashMap<FooterKey, &'static str> {
        match self {
            Lang::EnUS => en_us::footer_en_us(),
            Lang::ZhCN => zh_cn::footer_zh_cn(),
            Lang::JaJP => ja_jp::footer_ja_jp(),
        }
    }

    /// Returns localized report content strings.
    pub fn report(self) -> HashMap<ReportKey, &'static str> {
        match self {
            Lang::EnUS => en_us::report_en_us(),
            Lang::ZhCN => zh_cn::report_zh_cn(),
            Lang::JaJP => ja_jp::report_ja_jp(),
        }
    }
}

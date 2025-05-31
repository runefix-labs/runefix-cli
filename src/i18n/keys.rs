//! Defines all i18n message keys used in `runefix`.
//!
//! Each key enum corresponds to a category of localized messages (errors, titles, footers).
//! The `MessageKey` trait provides unified lookup behavior.

use crate::i18n::lang::Lang;
use std::fmt::Debug;
use std::hash::Hash;

/// Keys for localized error messages.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrorKey {
    InputTextNoProvided,
    SliceTrim,
    SliceBrackets,
    SliceSpaces,
    SliceFormat,
    SliceParseSingle,
    SliceParseStart,
    SliceParseEnd,
    SliceExprFallback,
    SliceOutOfBounds,
    SliceWidthUnaligned,
}

/// Keys for command titles or section headers.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TitleKey {
    DisplayWidth,
    GraphemeClusters,
    SlicePreview,
    UnicodeInfo,
    TruncatedOutput,
    WidthPerChar,
    RunefixVersion,
    SplitLines,
}

/// Keys for footers, legends, or result annotations.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FooterKey {
    AtomsDetail,
    LegendGlossary,
    SliceSummary,
}

/// Keys used for reporting diagnostic messages in CLI output.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ReportKey {
    WidthText,
    WidthPolicy,
    WidthDetail,
    SplitLine,
}

/// Trait for all localizable keys.
pub trait MessageKey: Debug {
    fn lookup(&self, lang: &Lang) -> Option<String>;
}

impl MessageKey for ErrorKey {
    fn lookup(&self, lang: &Lang) -> Option<String> {
        lang.error().get(self).map(|s| s.to_string())
    }
}

impl MessageKey for TitleKey {
    fn lookup(&self, lang: &Lang) -> Option<String> {
        lang.title().get(self).map(|s| s.to_string())
    }
}

impl MessageKey for FooterKey {
    fn lookup(&self, lang: &Lang) -> Option<String> {
        lang.footer().get(self).map(|s| s.to_string())
    }
}

impl MessageKey for ReportKey {
    fn lookup(&self, lang: &Lang) -> Option<String> {
        lang.report().get(self).map(|s| s.to_string())
    }
}

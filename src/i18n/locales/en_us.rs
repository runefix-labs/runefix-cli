//! English (en-US) locale messages for `runefix`.

use crate::i18n::keys::{
    ErrorKey, ErrorKey::*, FooterKey, FooterKey::*, ReportKey, ReportKey::*, TitleKey, TitleKey::*,
};
use std::collections::HashMap;

/// English error messages mapped by `ErrorKey`.
#[rustfmt::skip]
pub fn error_en_us() -> HashMap<ErrorKey, &'static str> {
    HashMap::from([
        (InputTextNoProvided, "❗ Error: no input text provided. Use --help to see usage."),
        (SliceTrim, "⚠️ slice expression must not have leading/trailing spaces"),
        (SliceBrackets, "❌ slice must be in [start:end] format (with brackets)"),
        (SliceSpaces, "⚠️ slice range must not contain spaces"),
        (SliceFormat, "❌ slice format must be [start:end]"),
        (SliceParseSingle, "❌ slice index must be a non-negative integer (e.g. [0], [3])"),
        (SliceParseStart, "❌ start index must be a non-negative integer"),
        (SliceParseEnd, "❌ end index must be a non-negative integer"),
        (SliceExprFallback, "❌ failed to parse slice expression"),
        (SliceOutOfBounds, "❌ slice range out of bounds (len = {len})"),
        (SliceWidthUnaligned, "❌ width slice must align with visual cell boundaries\nvalid boundaries: {boundaries}"),
    ])
}

/// English title messages mapped by `TitleKey`.
#[rustfmt::skip]
pub fn title_en_us() -> HashMap<TitleKey, &'static str> {
    HashMap::from([
        (DisplayWidth, "📏 Display Width"),
        (GraphemeClusters, "🧩 Grapheme Clusters"),
        (SlicePreview, "✂️ Slice Preview"),
        (UnicodeInfo, "🧬 Unicode Atom Info"),
        (TruncatedOutput, "📉 Truncated Output"),
        (WidthPerChar, "📐 Width Per Character"),
        (RunefixVersion, "🪪 Runefix Version Info"),
        (SplitLines, "🪓 Split Lines"),
    ])
}

/// English footer messages mapped by `FooterKey`.
#[rustfmt::skip]
pub fn footer_en_us() -> HashMap<FooterKey, &'static str> {
    HashMap::from([
        (AtomsDetail, "Total Display Width"),
        (LegendGlossary, "📘 Legend"),
        (SliceSummary, "Total units: {total}, Range: [{start}..{end}]"),
    ])
}

/// English report messages mapped by `ReportKey`.
#[rustfmt::skip]
pub fn report_en_us() -> HashMap<ReportKey, &'static str> {
    HashMap::from([
        (WidthText, "Text"),
        (WidthPolicy, "Policy"),
        (WidthDetail, "Display Width = {total} columns"),
        (SplitLine, "Line"),
    ])
}

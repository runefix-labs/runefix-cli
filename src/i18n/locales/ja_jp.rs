//! Japanese (ja-JP) locale messages for `runefix`.

use crate::i18n::keys::{
    ErrorKey, ErrorKey::*, FooterKey, FooterKey::*, ReportKey, ReportKey::*, TitleKey, TitleKey::*,
};
use std::collections::HashMap;

/// Japanese error messages mapped by `ErrorKey`.
#[rustfmt::skip]
pub fn error_ja_jp() -> HashMap<ErrorKey, &'static str> {
    HashMap::from([
        (InputTextNoProvided, "❗ 入力テキストが指定されていません。--help で使い方を確認してください。"),
        (SliceTrim, "⚠️ スライス式の前後に空白を含めないでください"),
        (SliceBrackets, "❌ スライスは [start:end] 形式である必要があります"),
        (SliceSpaces, "⚠️ 範囲には空白を含めないでください"),
        (SliceFormat, "❌ スライスの形式は [start:end] である必要があります"),
        (SliceParseSingle, "❌ 単一のインデックスは非負整数でなければなりません（例: [0], [3]）"),
        (SliceParseStart, "❌ 開始インデックスは非負整数でなければなりません"),
        (SliceParseEnd, "❌ 終了インデックスは非負整数でなければなりません"),
        (SliceExprFallback, "❌ スライス式の解析に失敗しました"),
        (SliceOutOfBounds, "❌ スライス範囲が無効です（長さ = {len}）"),
        (SliceWidthUnaligned, "❌ 幅モードではスライスは表示セルの境界に揃える必要があります\n有効な境界: {boundaries}"),
    ])
}

/// Japanese title messages mapped by `TitleKey`.
#[rustfmt::skip]
pub fn title_ja_jp() -> HashMap<TitleKey, &'static str> {
    HashMap::from([
        (DisplayWidth, "📏 表示幅"),
        (GraphemeClusters, "🧩 グラフェム分割"),
        (SlicePreview, "✂️ スライス結果"),
        (UnicodeInfo, "🧬 Unicode 原子情報"),
        (TruncatedOutput, "📉 切り捨て出力"),
        (WidthPerChar, "📐 文字ごとの幅"),
        (RunefixVersion, "🪪 Runefix バージョン情報"),
        (SplitLines, "🪓 行の分割"),
    ])
}

/// Japanese footer messages mapped by `FooterKey`.
#[rustfmt::skip]
pub fn footer_ja_jp() -> HashMap<FooterKey, &'static str> {
    HashMap::from([
        (AtomsDetail, "表示幅合計"),
        (LegendGlossary, "📘 凡例"),
        (SliceSummary, "合計ユニット数: {total}、範囲: [{start}..{end}]"),
    ])
}

/// Japanese report messages mapped by `ReportKey`.
#[rustfmt::skip]
pub fn report_ja_jp() -> HashMap<ReportKey, &'static str> {
    HashMap::from([
        (WidthText, "テキスト"),
        (WidthPolicy, "ポリシー"),
        (WidthDetail, "表示幅 = {total} カラム"),
        (SplitLine, "行"),
    ])
}

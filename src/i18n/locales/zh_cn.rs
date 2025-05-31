//! Simplified Chinese (zh-CN) locale messages for `runefix`.

use crate::i18n::keys::{
    ErrorKey, ErrorKey::*, FooterKey, FooterKey::*, ReportKey, ReportKey::*, TitleKey, TitleKey::*,
};
use std::collections::HashMap;

/// Chinese error messages mapped by `ErrorKey`.
#[rustfmt::skip]
pub fn error_zh_cn() -> HashMap<ErrorKey, &'static str> {
    HashMap::from([
        (InputTextNoProvided, "❗ 未提供输入文本。请使用 --help 查看用法说明。"),
        (SliceTrim, "⚠️ 切片表达式不能有首尾空格"),
        (SliceBrackets, "❌ 切片必须采用 [start:end] 格式"),
        (SliceSpaces, "⚠️ 切片范围中不能含有空格"),
        (SliceFormat, "❌ 切片格式必须是 [start:end]"),
        (SliceParseSingle, "❌ 切片索引必须是非负整数（如 [0] 或 [3]）"),
        (SliceParseStart, "❌ 起始索引必须是非负整数"),
        (SliceParseEnd, "❌ 结束索引必须是非负整数"),
        (SliceExprFallback, "❌ 解析切片表达式失败"),
        (SliceOutOfBounds, "❌ 切片索引超出范围（总长度 = {len}）"),
        (SliceWidthUnaligned, "❌ width 模式下切片必须对齐显示单元边界\n合法边界值包括：{boundaries}"),
    ])
}

/// Chinese title messages mapped by `TitleKey`.
#[rustfmt::skip]
pub fn title_zh_cn() -> HashMap<TitleKey, &'static str> {
    HashMap::from([
        (DisplayWidth, "📏 显示宽度"),
        (GraphemeClusters, "🧩 字符分段"),
        (SlicePreview, "✂️ 切片结果"),
        (UnicodeInfo, "🧬 Unicode 原子信息"),
        (TruncatedOutput, "📉 截断结果"),
        (WidthPerChar, "📐 单字符宽度"),
        (RunefixVersion, "🪪 Runefix 版本信息"),
        (SplitLines, "🪓 拆分行"),
    ])
}

/// Chinese footer messages mapped by `FooterKey`.
#[rustfmt::skip]
pub fn footer_zh_cn() -> HashMap<FooterKey, &'static str> {
    HashMap::from([
        (AtomsDetail, "总显示宽度"),
        (LegendGlossary, "📘 图例"),
        (SliceSummary, "总单元数：{total}，切片区间：[{start}..{end}]"),
    ])
}

/// Chinese report messages mapped by `ReportKey`.
#[rustfmt::skip]
pub fn report_zh_cn() -> HashMap<ReportKey, &'static str> {
    HashMap::from([
        (WidthText, "文本宽度"),
        (WidthPolicy, "策略"),
        (WidthDetail, "显示宽度 = {total} 列"),
        (SplitLine, "行"),
    ])
}

//! Execute the `atoms` command: decompose text into layout-affecting atomic segments,
//! and inspect Unicode codepoints and display widths.

use crate::config::Context;
use crate::i18n::keys::{FooterKey, TitleKey};
use crate::i18n::lang::Lang;
use crate::style::{consts::WIDTH_LINE, print::*, term::*};
use runefix_core::{RuneDisplayWidth, atoms};

/// Convert a string into a Unicode codepoint sequence (e.g., U+1F600).
fn str_to_unicode_sequence(s: &str) -> String {
    s.chars()
        .map(|c| format!("U+{:04X}", c as u32))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Return true if character is a Zero Width Joiner (ZWJ)
fn is_zwj(s: &str) -> bool {
    s == "\u{200D}"
}

/// Return true if character is an emoji variant selector (VS16)
fn is_emoji_vs(s: &str) -> bool {
    s == "\u{FE0F}"
}

/// Return true if character is a combining enclosing keycap
fn is_combining_mark(s: &str) -> bool {
    s == "\u{20E3}"
}

/// Return true if character is a Fitzpatrick skin tone modifier
fn is_skin_tone_modifier(s: &str) -> bool {
    matches!(
        s,
        "\u{1F3FB}" | "\u{1F3FC}" | "\u{1F3FD}" | "\u{1F3FE}" | "\u{1F3FF}"
    )
}

/// Return true if character is a hair/appearance component emoji
fn is_hair_component(s: &str) -> bool {
    matches!(s, "\u{1F9B0}" | "\u{1F9B1}" | "\u{1F9B2}" | "\u{1F9B3}")
}

/// Return visual fallback for non-printable characters
fn display_char(s: &str) -> &str {
    if is_zwj(s) || is_emoji_vs(s) || is_combining_mark(s) {
        "''"
    } else {
        s
    }
}

/// Return display width for the segment (taking emoji modifiers into account)
fn measured_width(s: &str) -> usize {
    if is_zwj(s) || is_emoji_vs(s) || is_combining_mark(s) {
        0
    } else if is_skin_tone_modifier(s) || is_hair_component(s) {
        2
    } else {
        s.width()
    }
}

/// Return Unicode codepoint string and semantic label
fn display_unicode_hint(s: &str) -> String {
    let base = str_to_unicode_sequence(s);
    if is_zwj(s) {
        format!("{:7} (ZWJ)", base)
    } else if is_emoji_vs(s) {
        format!("{:7} (Emoji Variant)", base)
    } else if is_combining_mark(s) {
        format!("{:7} (Combining Mark)", base)
    } else if is_skin_tone_modifier(s) {
        format!("{:7} (Skin Tone)", base)
    } else if is_hair_component(s) {
        format!("{:7} (Hair Colors)", base)
    } else {
        base
    }
}

/// Decompose a string into atomic segments and inspect each for Unicode info.
///
/// This command is especially useful for inspecting emoji sequences, ZWJ chains,
/// and layout-impacting Unicode modifiers. It prints a detailed table of all atoms
/// along with semantic annotations for special cases (ZWJ, skin tone, hair, etc.).
///
/// # Arguments
/// * `ctx` - Global context (i18n, language settings, etc.)
/// * `input` - Input text to be segmented
pub fn run_atoms(ctx: &Context, input: &str) {
    // Show localized title header
    print_title(&ctx.t(TitleKey::UnicodeInfo));

    // Segment input into width-sensitive atoms
    let atoms = atoms(input);

    // Print table header (bold if in terminal)
    if is_stdout_terminal() {
        println!(
            " \x1b[1m{:<4}   {:<4}    {:<26}      {:>5}\x1b[0m",
            "No.", "Rune", "Unicode / Hint", "Width"
        );
    } else {
        println!(
            " {:<4}   {:<4}    {:<26}      {:>5}",
            "No.", "Rune", "Unicode / Hint", "Width"
        );
    }

    // Draw separator line
    println!("{}", "─".repeat(WIDTH_LINE));

    // Print each atom's index, visual rune, Unicode info, and width
    for (i, s) in atoms.iter().enumerate() {
        let ch = display_char(s);
        let w = measured_width(s);
        let u = display_unicode_hint(s);
        let spacing = match w {
            2 => "   ",
            _ => "    ",
        };
        println!(" {:02}     {:<4}{}{: <26}      {:>5}", i, ch, spacing, u, w);
    }

    // Draw footer line
    println!("{}", "─".repeat(WIDTH_LINE));

    // Show total width summary (localized label, right-aligned)
    let label = ctx.t(FooterKey::AtomsDetail);
    let total: usize = atoms.iter().map(|s| measured_width(s)).sum();
    let padding = if ctx.lang == Lang::EnUS { 26 } else { 35 };
    println!(
        "{:>padding$}「{}」: {:2}",
        "",
        label,
        total,
        padding = padding
    );

    // Print legend title (bold if terminal)
    println!();
    let legend = ctx.t(FooterKey::LegendGlossary);
    let styled = if is_stdout_terminal() {
        bold(&legend)
    } else {
        legend
    };
    println!("{styled}:");

    // Print glossary of special Unicode atoms
    println!("ZWJ = Zero Width Joiner (U+200D)");
    println!("Emoji Variant = U+FE0F (Forces Emoji Presentation)");
    println!("Combining Mark = U+20E3 (Combining Enclosing Keycap)");
    println!("Skin Tone = U+1F3FB–U+1F3FF (Fitzpatrick Modifiers)");
    println!("Hair Colors = U+1F9B0–U+1F9B3 (Hair/Beard modifiers)");
}

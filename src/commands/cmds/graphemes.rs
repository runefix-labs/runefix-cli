//! Execute the `graphemes` command: list all grapheme clusters and their widths.

use crate::config::Context;
use crate::i18n::keys::TitleKey;
use crate::style::{consts::WIDTH_LINE, print::*};
use runefix_core::RuneDisplayWidth;
use unicode_segmentation::UnicodeSegmentation;

/// Segment input into grapheme clusters and display their visual widths.
///
/// This function uses `UnicodeSegmentation` to divide the input string into
/// grapheme clusters (user-perceived characters). Each cluster is printed with
/// its index and display width, aligned in a tabular format for clarity.
///
/// # Arguments
/// * `ctx` - Global application context (used for i18n and theming).
/// * `input` - Input string to be analyzed.
pub fn run_graphemes(ctx: &Context, input: &str) {
    // Display localized section title
    print_title(&ctx.t(TitleKey::GraphemeClusters));

    // Segment into grapheme clusters
    let graphemes: Vec<&str> = input.graphemes(true).collect();

    // Compute maximum display width for alignment
    let max_width = graphemes.iter().map(|g| g.width()).max().unwrap_or(1);

    // Print header
    println!(
        " {:<4}   {:<width$}    Width",
        "No.",
        "Grapheme",
        width = max_width
    );
    println!("{}", "─".repeat(WIDTH_LINE));

    // Print each grapheme cluster with its width
    for (i, g) in graphemes.iter().enumerate() {
        let visual_width = g.width();
        let pad = max_width.saturating_sub(visual_width);
        let padded = format!("{g}{}", " ".repeat(pad));
        println!(" {:02}     {}          {}", i, padded, visual_width);
    }

    // Add bottom border if many entries
    if graphemes.len() >= 5 {
        println!("{}", "─".repeat(WIDTH_LINE));
    }
}

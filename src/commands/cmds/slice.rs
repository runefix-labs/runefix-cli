//! Execute the `slice` command: extract part of a string by char, grapheme, or display width.
//!
//! Supports Python-style `[start:end]` expressions, with optional strict mode validation.
//! Also provides display-aware slicing (width mode) using visual boundaries for terminal layout.

use crate::config::Context;
use crate::i18n::format_i18n;
use crate::i18n::keys::{ErrorKey, FooterKey, TitleKey};
use crate::style::print::*;
use anyhow::{Context as _, Result, anyhow, bail};
use runefix_core::RuneDisplayWidth;
use unicode_segmentation::UnicodeSegmentation;

/// Slice mode enum used to determine unit of slicing.
#[derive(Clone, Copy)]
pub enum SliceMode {
    // raw UTF-8 characters
    Char,

    // Unicode grapheme clusters (default)
    Grapheme,

    // display width units (terminal-visible columns)
    Width,
}

/// Entry point for `slice` command.
/// Handles multi-line input, slicing each line individually.
pub fn run_slice(
    ctx: &Context,
    input: &str,
    slice_expr: &str,
    mode: SliceMode,
    verbose: bool,
    strict: bool,
) -> Result<()> {
    let lines: Vec<&str> = input.lines().collect();
    let is_single_line = lines.len() == 1;

    for (i, line) in input.lines().enumerate() {
        // Skip or pass through empty lines
        if line.trim().is_empty() {
            println!();
            continue;
        }

        // Process each line with fallback on error
        match run_slice_single(ctx, line, slice_expr, mode, verbose, strict) {
            Ok(()) => (),
            Err(e) => {
                if is_single_line {
                    // In single-line mode, propagate the error directly
                    return Err(e);
                } else {
                    // In multi-line mode, print error with line number
                    eprintln!("line {}: {}", i + 1, e);
                }
            }
        }
    }

    Ok(())
}

/// Core slice handler: supports Python-style slicing with display-width logic.
pub fn run_slice_single(
    ctx: &Context,
    input: &str,
    slice_expr: &str,
    mode: SliceMode,
    verbose: bool,
    strict: bool,
) -> Result<()> {
    // Optional header (only in verbose mode)
    if verbose {
        // Note: stdout/stderr output order may vary due to OS-level buffering.
        print_title(&ctx.t(TitleKey::SlicePreview));
    }

    // Split string into units based on slice mode
    let (units, visual_boundaries) = split_str_units(input, mode);

    // Parse user input: slice expression like [2:5]
    let range = parse_slice_range(ctx, slice_expr).context(ctx.t(ErrorKey::SliceExprFallback))?;

    let start = range.start;
    let end = range.end.unwrap_or(units.len());

    // Case 1: char/grapheme slicing
    if !matches!(mode, SliceMode::Width) {
        // Strict mode: check for out-of-bounds or reversed range
        if strict && (start > units.len() || end > units.len() || start > end) {
            let msg = format_i18n(
                &ctx.t(ErrorKey::SliceOutOfBounds),
                &[("len", units.len().to_string())],
            );
            bail!(msg);
        }

        // Non-strict: clamp range to safe bounds
        let safe_start = start.min(units.len());
        let safe_end = end.min(units.len());

        // Output result
        let sliced = &units[safe_start..safe_end];
        println!("{}", sliced.concat());

        // Optionally print summary footer
        maybe_print_footer(ctx, verbose, units.len(), safe_start, safe_end);

        return Ok(());
    }

    // Case 2: display-width slicing
    if matches!(mode, SliceMode::Width) {
        // Strict: start/end must be aligned to display boundaries
        if strict && (!visual_boundaries.contains(&start) || !visual_boundaries.contains(&end)) {
            let msg = format_i18n(
                &ctx.t(ErrorKey::SliceWidthUnaligned),
                &[("boundaries", format!("{:?}", visual_boundaries))],
            );
            bail!(msg);
        }

        // Non-strict: fallback to nearest boundary
        let slice_start = visual_boundaries
            .iter()
            .position(|&v| v >= start)
            .unwrap_or(visual_boundaries.len() - 1);
        let slice_end = visual_boundaries
            .iter()
            .position(|&v| v >= end)
            .unwrap_or(visual_boundaries.len());

        let sliced = &units[slice_start..slice_end];
        println!("{}", sliced.concat());

        // Optional summary
        maybe_print_footer(
            ctx,
            verbose,
            visual_boundaries.last().copied().unwrap_or(0),
            start,
            end,
        );

        return Ok(());
    }

    Ok(())
}

/// Result of parsed slice expression, e.g. [start:end]
pub struct ParsedRange {
    pub start: usize,
    pub end: Option<usize>,
}

/// Parses a slice expression string in `[start:end]` or `[N]` form.
///
/// # Supported Formats
/// - `[start:end]` — classic Python-like slicing (e.g. `[0:3]`, `[1:]`, `[:2]`)
/// - `[N]`         — shorthand form, interpreted as `[N:N+1]`
///
/// # Optional Wrappers
/// Slice expressions may optionally be wrapped in quotes:
/// - `'[0:3]'`
/// - `"[1]"`
/// - `[2]`
///
/// # Design Rules
/// - **Brackets required**: Must start with `[` and end with `]`
/// - **No leading/trailing spaces**: Expression must be trimmed even inside quotes
/// - **No inner spaces**: `[ 1 : 3 ]` is invalid
/// - **Default start**: Omitted start (`[:3]`) implies `start = 0`
/// - **Open-ended slice**: Omitted end (`[2:]`) implies `end = None`
/// - **Shorthand**: Single number like `[4]` is interpreted as `[4:5]`
///
/// # Error Strategy
/// - Reports detailed ❌ errors on malformed formats
/// - Validates `usize` bounds for indices
///
/// # Feature Note
/// - Currently, all indices are parsed as `usize` for simplicity and safety.
/// - In the future, this may be **generalized to `isize`** to support negative indices (e.g. `[-1]`)
///   with semantics similar to Python-style indexing (e.g. last element).
/// - If adopted, error handling and bounds resolution will account for signed offsets.
///
/// # Examples
/// ```
/// assert_eq!(parse_slice_range("[1:4]")?, ParsedRange { start: 1, end: Some(4) });
/// assert_eq!(parse_slice_range("[3]")?, ParsedRange { start: 3, end: Some(4) });
/// assert_eq!(parse_slice_range("'[:2]'")?, ParsedRange { start: 0, end: Some(2) });
/// ```
///
/// # Errors
/// Returns descriptive errors if:
/// - Expression is not bracketed
/// - Contains leading/trailing or internal spaces
/// - Indices are not valid non-negative integers
fn parse_slice_range(ctx: &Context, expr: &str) -> Result<ParsedRange> {
    // Disallow trimming — expression must be validated in its raw form
    let raw = expr;

    // Allow three wrapping forms: raw, single-quoted, or double-quoted
    let expr = raw
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .or_else(|| raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
        .unwrap_or(raw);

    // Leading/trailing spaces are not allowed
    if expr != expr.trim() {
        bail!(ctx.t(ErrorKey::SliceTrim));
    }

    // Must start with `[` and end with `]`
    if !expr.starts_with('[') || !expr.ends_with(']') {
        bail!(ctx.t(ErrorKey::SliceBrackets));
    }

    // Extract the inner content, e.g. "[1:3]" → "1:3"
    let content = &expr[1..expr.len() - 1];

    // Inner spaces are not allowed (e.g. `[ 1 : 3 ]` is invalid)
    if content.contains(char::is_whitespace) {
        bail!(ctx.t(ErrorKey::SliceSpaces));
    }

    // Handle shorthand `[N]` → interpreted as `[N:N+1]`
    if !content.contains(':') {
        let idx = content
            .parse::<usize>()
            .map_err(|_| anyhow!(ctx.t(ErrorKey::SliceParseSingle)))?;

        return Ok(ParsedRange {
            start: idx,
            end: Some(idx + 1),
        });
    }

    // Handle full `[start:end]` form
    let parts: Vec<_> = content.split(':').collect();
    if parts.len() != 2 {
        bail!(ctx.t(ErrorKey::SliceFormat));
    }

    // Parse the `start` part
    let start = if parts[0].is_empty() {
        0
    } else {
        parts[0]
            .parse::<usize>()
            .map_err(|_| anyhow!(ctx.t(ErrorKey::SliceParseStart)))?
    };

    // Parse the `end` part
    let end = if parts[1].is_empty() {
        None
    } else {
        Some(
            parts[1]
                .parse::<usize>()
                .map_err(|_| anyhow!(ctx.t(ErrorKey::SliceParseEnd)))?,
        )
    };

    // Return parsed result
    Ok(ParsedRange { start, end })
}

/// Splits input string into units by mode (char / grapheme / width).
///
/// Returns both the unit slices and their boundary indices.
pub fn split_str_units(input: &str, mode: SliceMode) -> (Vec<&str>, Vec<usize>) {
    match mode {
        // Char mode: split by individual Unicode scalar values
        SliceMode::Char => {
            let vec: Vec<&str> = input
                .chars()
                .map(|c| Box::leak(c.to_string().into_boxed_str()) as &str)
                .collect();
            let boundaries = (0..=vec.len()).collect(); // Index boundaries
            (vec, boundaries)
        }

        // Grapheme mode: split by extended grapheme clusters (Unicode segmentation)
        SliceMode::Grapheme => {
            let vec: Vec<&str> = input.graphemes(true).collect();
            let boundaries = (0..=vec.len()).collect(); // Grapheme boundaries
            (vec, boundaries)
        }

        // Width mode: split by visual display width (terminal columns)
        SliceMode::Width => {
            let mut result: Vec<&str> = Vec::new();
            let mut boundaries: Vec<usize> = vec![0]; // Start with 0-width

            let mut acc = String::new(); // Buffer for accumulating graphemes
            let mut acc_width = 0; // Accumulated width of current segment
            let mut width_total = 0; // Running total of column width

            for g in input.graphemes(true) {
                let w = g.display_width(); // Use runefix-core for precise width

                acc.push_str(g);
                acc_width += w;

                // Only flush if width > 0 (ignore zero-width like ZWJ)
                if acc_width > 0 {
                    let leaked = Box::leak(acc.clone().into_boxed_str());
                    result.push(leaked);
                    width_total += acc_width;
                    boundaries.push(width_total);

                    acc.clear();
                    acc_width = 0;
                }
            }

            // Flush any remaining buffered segment
            if !acc.is_empty() {
                let leaked = Box::leak(acc.into_boxed_str());
                result.push(leaked);
                boundaries.push(width_total);
            }

            (result, boundaries)
        }
    }
}

/// Optionally prints a footer with slice summary (verbose mode only).
fn maybe_print_footer(ctx: &Context, verbose: bool, total: usize, start: usize, end: usize) {
    if verbose {
        let msg = format_i18n(
            &ctx.t(FooterKey::SliceSummary),
            &[
                ("total", total.to_string()),
                ("start", start.to_string()),
                ("end", end.to_string()),
            ],
        );
        println!("\n{msg}");
    }
}

//! Execute the `width` command: compute total display width of input text.

use crate::config::Context;
use crate::i18n::format_i18n;
use crate::i18n::keys::{ReportKey, TitleKey};
use crate::style::print::*;
use runefix_core::WidthPolicy;

/// Compute and print the display width of the entire input string.
///
/// Uses the specified `WidthPolicy` to determine how wide the string renders
/// in terminal or other contexts.
///
/// # Arguments
/// * `ctx` - Global application context.
/// * `input` - The text to analyze.
/// * `policy` - Display width strategy.
/// * `verbose` - Enable verbose output.
pub fn run_width(ctx: &Context, input: &str, policy: WidthPolicy, verbose: bool) {
    // Optional header (only in verbose mode)
    if verbose {
        // Display localized section title
        print_title(&ctx.t(TitleKey::DisplayWidth));
    }

    // Calculate display width using selected policy
    let width = runefix_core::display_width_with_policy(input, Some(&policy));

    // Print input and display width information
    println!("{}: \"{}\"", ctx.t(ReportKey::WidthText), input);
    println!(
        "{}: {}",
        ctx.t(ReportKey::WidthPolicy),
        describe_policy(&policy)
    );
    println!();
    let msg = format_i18n(
        &ctx.t(ReportKey::WidthDetail),
        &[("total", width.to_string())],
    );
    println!("{}", msg);
}

/// Returns the name of a given width policy (for CLI display or logging).
///
/// Matches the policy against built-in presets and returns:
/// - `"Terminal"`
/// - `"Markdown"`
/// - `"Compact"`
/// - `"Custom"` (for unknown or user-defined configurations)
fn describe_policy(policy: &WidthPolicy) -> &'static str {
    // Match by comparing fields using known constructor output
    if policy_eq(policy, &WidthPolicy::terminal()) {
        "Terminal"
    } else if policy_eq(policy, &WidthPolicy::markdown()) {
        "Markdown"
    } else if policy_eq(policy, &WidthPolicy::compact()) {
        "Compact"
    } else {
        "Custom"
    }
}

/// Compares two policies by their underlying field values.
///
/// This is used instead of `PartialEq` to avoid trait bounds.
/// Policies are compared as tuples of internal fields.
fn policy_eq(a: &WidthPolicy, b: &WidthPolicy) -> bool {
    a.as_tuple() == b.as_tuple()
}

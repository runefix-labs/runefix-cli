//! CLI command dispatcher for `runefix`.
//!
//! Maps parsed CLI arguments to the corresponding command execution logic.
//! This module connects `clap`-derived argument structures to actual `run_*` functions.

use crate::commands::cli::{
    Cli, Commands, PolicyInput, SliceArgs, TextInput, VersionArgs, WidthInput,
};
use crate::commands::cmds::slice::SliceMode;
use crate::commands::cmds::*;
use crate::commands::input::resolve_input;
use crate::config::Context;
use anyhow::Result;
use runefix_core::WidthPolicy;

/// Parses a policy name string into a `WidthPolicy` object.
/// Falls back to `terminal` policy if the name is unknown.
fn parse_policy(name: &str) -> WidthPolicy {
    match name {
        "terminal" => WidthPolicy::terminal(),
        "markdown" => WidthPolicy::markdown(),
        "compact" => WidthPolicy::compact(),
        _ => {
            eprintln!("⚠ Unknown policy '{name}', falling back to terminal.");
            WidthPolicy::terminal()
        }
    }
}

/// Entrypoint for dispatching CLI commands.
/// This function matches each command variant and invokes its corresponding execution function.
///
/// # Arguments
/// * `ctx` - Global context, including language, style, and configuration.
/// * `cli` - Parsed CLI arguments and subcommand variants.
///
/// # Returns
/// * `Result<()>` - Returns `Ok` on success, or propagates errors from execution.
pub fn dispatch(ctx: Context, cli: Cli) -> Result<()> {
    match cli.command {
        // Run the `atoms` command: segments text into Unicode "atoms".
        Commands::Atoms(TextInput { text }) => {
            let input = resolve_input(&ctx, text);
            run_atoms(&ctx, &input);
        }

        // Run the `graphemes` command: segments text by grapheme clusters.
        Commands::Graphemes(TextInput { text }) => {
            let input = resolve_input(&ctx, text);
            run_graphemes(&ctx, &input);
        }

        // Run the `width` command: calculate display width of entire input.
        Commands::Width(PolicyInput {
            text,
            policy,
            verbose,
        }) => {
            let input = resolve_input(&ctx, text);
            run_width(&ctx, &input, parse_policy(&policy), verbose);
        }

        // Run the `widths` command: compute width of each grapheme segment.
        Commands::Widths(PolicyInput {
            text,
            policy,
            verbose,
        }) => {
            let input = resolve_input(&ctx, text);
            run_widths(&ctx, &input, parse_policy(&policy), verbose);
        }

        // Run the `split` command: split input based on cumulative display width.
        Commands::Split(WidthInput {
            text,
            width,
            policy,
            verbose,
        }) => {
            let input = resolve_input(&ctx, text);
            run_split(&ctx, &input, width, parse_policy(&policy), verbose);
        }

        // Run the `truncate` command: truncate input to a fixed display width.
        Commands::Truncate(WidthInput {
            text,
            width,
            policy,
            verbose,
        }) => {
            let input = resolve_input(&ctx, text);
            run_truncate(&ctx, &input, width, parse_policy(&policy), verbose);
        }

        // Run the `slice` command: slice text by char, grapheme, or width.
        Commands::Slice(SliceArgs {
            text,
            range,
            char,
            width,
            grapheme,
            verbose,
            strict,
        }) => {
            let mode = if char {
                SliceMode::Char
            } else if width {
                SliceMode::Width
            } else if grapheme {
                SliceMode::Grapheme
            } else {
                // Default fallback mode
                SliceMode::Grapheme
            };
            let input = resolve_input(&ctx, text);
            run_slice(&ctx, &input, &range, mode, verbose, strict)?;
        }

        // Run the `version` command: display version info in plain or JSON format.
        Commands::Version(VersionArgs { json }) => {
            run_version(&ctx, json);
        }

        // Run the `init` command: launch interactive setup for user preferences.
        Commands::Init => run_init()?,
    }

    Ok(())
}

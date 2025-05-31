//! Entry point for the `runefix` CLI application.
//!
//! This module initializes global context, parses command-line arguments,
//! and dispatches execution to the appropriate subcommand handler.

mod commands;
mod config;
mod i18n;
mod style;

use anyhow::Result;
use clap::Parser;
use commands::cli::Cli;
use commands::dispatch::dispatch;
use config::Context;

/// Main entry point for the CLI.
///
/// Initializes runtime context, parses arguments, and routes commands.
fn main() -> Result<()> {
    // Initialize global config context (e.g. language, theme)
    let ctx = Context::init();

    // Parse CLI arguments using Clap
    let cli = Cli::parse();

    // Dispatch to the appropriate command handler
    dispatch(ctx, cli)
}

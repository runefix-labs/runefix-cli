//! CLI argument definitions for `runefix`.
//!
//! Defines top-level commands, options, and subcommand-specific arguments
//! using `clap` derive macros.

use clap::{Args, Parser, Subcommand};

// Top-level CLI entrypoint.
//
// This struct is parsed from command-line arguments
// and delegates to one of the subcommands.
#[derive(Parser)]
#[command(
    name = "Runefix",
    author,
    version,
    about = "Unicode-aware display width CLI toolkit"
)]
// Defines the root CLI parser structure, holding all subcommands.
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

// Supported `runefix` subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Segment input into display-sensitive width atoms
    Atoms(TextInput),

    /// Split input into grapheme clusters
    Graphemes(TextInput),

    /// Measure total display width of input
    Width(PolicyInput),

    /// Show width of each segment
    Widths(PolicyInput),

    /// Split input text into segments fitting width
    Split(WidthInput),

    /// Truncate input to a max width (preserving layout)
    Truncate(WidthInput),

    /// Slice input using [start:end] style expression
    Slice(SliceArgs),

    /// Show version information
    Version(VersionArgs),

    /// Initialize user configuration
    Init,
}

// Common input text wrapper (used in `atoms`, `graphemes`)
#[derive(Args)]
pub struct TextInput {
    /// Input text to analyze (can be piped via stdin)
    pub text: Option<String>,
}

// Input + policy for display width-related commands
#[derive(Args)]
pub struct PolicyInput {
    /// Input text to measure (can be piped via stdin)
    pub text: Option<String>,

    /// Layout policy: terminal, markdown, or compact
    #[arg(short, long, default_value = "terminal")]
    pub policy: String,

    /// Enable verbose output (e.g. titles)
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

// Input + policy + width for truncation/splitting
#[derive(Args)]
pub struct WidthInput {
    /// Input text to measure (can be piped via stdin)
    pub text: Option<String>,

    /// Max display width in columns
    #[arg(short, long)]
    pub width: usize,

    /// Layout policy: terminal, markdown, or compact
    #[arg(short, long, default_value = "terminal")]
    pub policy: String,

    /// Enable verbose output (e.g. titles)
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

// Arguments for `runefix version`
#[derive(Args)]
pub struct VersionArgs {
    /// Show version info as machine-readable JSON
    #[arg(long)]
    pub json: bool,
}

// Arguments for `runefix slice`
#[derive(Args)]
pub struct SliceArgs {
    /// Slice range in [start:end] format
    #[arg(value_name = "RANGE")]
    pub range: String,

    /// Input text (can be piped via stdin)
    #[arg(value_name = "TEXT")]
    pub text: Option<String>,

    /// Slice by UTF-8 characters
    #[arg(short = 'c', long)]
    pub char: bool,

    /// Slice by display width units
    #[arg(short = 'w', long)]
    pub width: bool,

    /// Slice by grapheme clusters (default)
    #[arg(short = 'g', long)]
    pub grapheme: bool,

    /// Enable verbose output (e.g. titles, legends)
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Enable strict range checks (errors on overflow)
    #[clap(short = 's', long)]
    pub strict: bool,
}

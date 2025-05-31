//! Execute the `version` command: show CLI, Core, and Unicode version info.

use crate::config::Context;
use crate::i18n::keys::TitleKey;
use crate::style::print::*;
use runefix_core::UNICODE_VERSION;

/// Runefix CLI version (from Cargo.toml)
const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Extract `runefix-core` version from embedded Cargo.lock.
///
/// This parses the lockfile at compile time to retrieve the version
/// of the core library dependency. Used for display purposes only.
fn extract_core_version_from_lock() -> Option<String> {
    let cargo_lock = include_str!("../../../Cargo.lock");

    let mut lines = cargo_lock.lines().peekable();

    while let Some(line) = lines.next() {
        if line.trim() == "name = \"runefix-core\"" {
            let nested_iter = lines.by_ref();

            for next_line in nested_iter {
                if next_line.trim_start().starts_with("version = ") {
                    let version = next_line
                        .trim()
                        .strip_prefix("version = \"")?
                        .trim_end_matches('"');
                    return Some(version.to_string());
                }
                if next_line.trim().is_empty() {
                    break;
                }
            }
        }
    }

    None
}

/// Display CLI, Core, and Unicode version info.
///
/// Supports plain and JSON output formats.
///
/// # Arguments
/// * `ctx` - Global application context.
/// * `json` - Whether to print output as pretty JSON.
pub fn run_version(ctx: &Context, json: bool) {
    let core_version = extract_core_version_from_lock().unwrap_or_else(|| "<unknown>".to_string());
    let (major, minor, patch) = UNICODE_VERSION;
    let unicode_version = format!("{}.{}.{}", major, minor, patch);

    if json {
        println!(
            "{{\n  \"cli-version\": \"{}\",\n  \"core-version\": \"{}\",\n  \"unicode-version\": \"{}\"\n}}",
            CLI_VERSION, core_version, unicode_version
        );
    } else {
        print_title(&ctx.t(TitleKey::RunefixVersion));

        println!("{:<16}{}", "Runefix CLI", CLI_VERSION);
        println!("{:<16}{}", "Runefix Core", core_version);
        println!("{:<16}{}", "Unicode Data", unicode_version);
    }
}

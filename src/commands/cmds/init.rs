//! Interactive initializer for runefix CLI.

use crate::config::Config;
use crate::i18n::lang::Lang;
use anyhow::Result;
use inquire::Select;
use std::fs;

/// Run the `init` command: generate a config file at XDG-compliant location,
/// by interactively selecting a preferred language.
pub fn run_init() -> Result<()> {
    // Define language options
    let options = [
        ("🇺🇸 English (en-US)", Lang::EnUS),
        ("🇨🇳 简体中文 (zh-CN)", Lang::ZhCN),
        ("🇯🇵 日本語 (ja-JP)", Lang::JaJP),
    ];

    // Display prompt
    let selected = Select::new(
        "🎛️  Select your preferred language:",
        options.iter().map(|x| x.0).collect(),
    )
    .with_help_message("↑↓ to move, enter to select")
    .prompt()?;

    let selected_lang = options.iter().find(|x| x.0 == selected).unwrap().1;

    // Build config content
    let content = format!("lang = \"{}\"\n", selected_lang.code());

    // Get XDG-compatible config path
    let path = Config::config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; // ensure parent dirs exist
    }

    // Save file
    fs::write(&path, content)?;

    // Localized confirmation
    let msg = match selected_lang {
        Lang::EnUS => "Language saved to",
        Lang::ZhCN => "语言已保存到",
        Lang::JaJP => "言語を保存しました",
    };

    println!("\n✅  {} \"{}\"\n", msg, path.display());
    Ok(())
}

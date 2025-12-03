// WezTerm Settings TUI - Main entry point

mod app;
mod ui;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};
use wezterm_settings_gui_lib::{config, models};

#[derive(Parser, Debug)]
#[command(name = "wezterm-settings-tui")]
#[command(author, version, about = "A TUI for managing WezTerm configuration")]
struct Args {
    /// Jump directly to a settings panel (colors, fonts, window, cursor, gpu, keys)
    #[arg(value_name = "PANEL")]
    panel: Option<String>,

    /// Path to WezTerm config directory
    #[arg(short, long, value_name = "DIR")]
    config_dir: Option<String>,

    /// Export current config as JSON to stdout
    #[arg(long)]
    export: bool,

    /// Import config from JSON file
    #[arg(long, value_name = "FILE")]
    import: Option<String>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check for updates
    CheckUpdate,
    
    /// Update to the latest version
    Update,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle subcommands first
    if let Some(cmd) = args.command {
        return match cmd {
            Commands::CheckUpdate => update::print_update_status(),
            Commands::Update => update::run_update(),
        };
    }

    // Handle non-interactive modes
    if args.export {
        return export_config(&args.config_dir);
    }

    if let Some(import_path) = &args.import {
        return import_config(import_path, &args.config_dir);
    }

    // Run the TUI
    let mut app = app::App::new(args.config_dir, args.panel)?;
    app.run()
}

fn export_config(config_dir: &Option<String>) -> Result<()> {
    let config = config::load_config(config_dir.as_deref())?;
    let json = serde_json::to_string_pretty(&config)?;
    println!("{}", json);
    Ok(())
}

fn import_config(import_path: &str, config_dir: &Option<String>) -> Result<()> {
    let contents = std::fs::read_to_string(import_path)?;
    let config: models::AppearanceConfig = serde_json::from_str(&contents)?;
    config::save_config(&config, config_dir.as_deref())?;
    println!("Config imported successfully");
    Ok(())
}

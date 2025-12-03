// Self-update functionality for wezterm-settings-tui

use anyhow::{anyhow, Result};
use semver::Version;
use serde::Deserialize;
use std::process::Command;

const GITHUB_API_URL: &str = "https://api.github.com/repos/jms830/wezterm-settings-ui/releases/latest";
const REPO_URL: &str = "https://github.com/jms830/wezterm-settings-ui";
const CRATE_NAME: &str = "wezterm-settings-tui";

/// Information about the latest release
#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
}

/// Check result
#[derive(Debug)]
pub struct UpdateCheck {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub release_notes: Option<String>,
}

/// Get the current version from Cargo.toml
pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Check GitHub for the latest release
pub fn check_for_updates() -> Result<UpdateCheck> {
    let current = current_version();
    
    // Make request to GitHub API
    let response: GithubRelease = ureq::get(GITHUB_API_URL)
        .set("User-Agent", &format!("{}/{}", CRATE_NAME, current))
        .set("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| anyhow!("Failed to check for updates: {}", e))?
        .into_json()
        .map_err(|e| anyhow!("Failed to parse release info: {}", e))?;
    
    // Parse versions (strip 'v' prefix if present)
    let latest_str = response.tag_name.trim_start_matches('v');
    let current_ver = Version::parse(current).unwrap_or_else(|_| Version::new(0, 0, 0));
    let latest_ver = Version::parse(latest_str).unwrap_or_else(|_| Version::new(0, 0, 0));
    
    Ok(UpdateCheck {
        current_version: current.to_string(),
        latest_version: latest_str.to_string(),
        update_available: latest_ver > current_ver,
        release_url: response.html_url,
        release_notes: response.body,
    })
}

/// Print update status to stdout
pub fn print_update_status() -> Result<()> {
    println!("Checking for updates...\n");
    
    match check_for_updates() {
        Ok(check) => {
            println!("Current version: {}", check.current_version);
            println!("Latest version:  {}", check.latest_version);
            println!();
            
            if check.update_available {
                println!("A new version is available!");
                println!();
                println!("To update, run:");
                println!("  cargo install --git {} {} --force", REPO_URL, CRATE_NAME);
                println!();
                println!("Or view release: {}", check.release_url);
                
                if let Some(notes) = &check.release_notes {
                    println!();
                    println!("Release notes:");
                    // Truncate long release notes
                    let truncated: String = notes.chars().take(500).collect();
                    println!("{}", truncated);
                    if notes.len() > 500 {
                        println!("... (see full notes at {})", check.release_url);
                    }
                }
            } else {
                println!("You're running the latest version!");
            }
            Ok(())
        }
        Err(e) => {
            println!("Could not check for updates: {}", e);
            println!();
            println!("You can manually check at: {}/releases", REPO_URL);
            Ok(())
        }
    }
}

/// Run the update command
pub fn run_update() -> Result<()> {
    println!("Updating wezterm-settings-tui...\n");
    
    // First check if update is needed
    let check = check_for_updates();
    
    if let Ok(ref c) = check {
        if !c.update_available {
            println!("Already at latest version ({})", c.current_version);
            return Ok(());
        }
        println!("Updating from {} to {}...\n", c.current_version, c.latest_version);
    }
    
    // Run cargo install
    let status = Command::new("cargo")
        .args([
            "install",
            "--git",
            REPO_URL,
            CRATE_NAME,
            "--force",
        ])
        .status()
        .map_err(|e| anyhow!("Failed to run cargo: {}", e))?;
    
    if status.success() {
        println!();
        println!("Update complete!");
        println!();
        println!("To update the WezTerm plugin, open WezTerm's debug overlay");
        println!("(Ctrl+Shift+L) and run: wezterm.plugin.update_all()");
        Ok(())
    } else {
        Err(anyhow!("Update failed. Please try manually:\n  cargo install --git {} {} --force", REPO_URL, CRATE_NAME))
    }
}

/// Quick check that can be shown in TUI status bar
/// Returns Some(latest_version) if update available, None otherwise
pub fn quick_update_check() -> Option<String> {
    // Use a short timeout for background check
    let response = ureq::get(GITHUB_API_URL)
        .set("User-Agent", &format!("{}/{}", CRATE_NAME, current_version()))
        .timeout(std::time::Duration::from_secs(3))
        .call()
        .ok()?;
    
    let release: GithubRelease = response.into_json().ok()?;
    let latest_str = release.tag_name.trim_start_matches('v');
    
    let current_ver = Version::parse(current_version()).ok()?;
    let latest_ver = Version::parse(latest_str).ok()?;
    
    if latest_ver > current_ver {
        Some(latest_str.to_string())
    } else {
        None
    }
}

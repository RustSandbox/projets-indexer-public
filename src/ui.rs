//! Terminal UI components
//!
//! This module provides user-friendly terminal UI components for displaying
//! progress and status information during project indexing.

use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::time::Duration;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "");
static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");
static BOOKS: Emoji<'_, '_> = Emoji("📚 ", "");
static CONSTRUCTION: Emoji<'_, '_> = Emoji("🏗️  ", "");
static FOLDER: Emoji<'_, '_> = Emoji("📁 ", "");
static TAG: Emoji<'_, '_> = Emoji("🏷️  ", "");
static CHART: Emoji<'_, '_> = Emoji("📊 ", "");
static GEAR: Emoji<'_, '_> = Emoji("⚙️  ", "");
static CLOCK: Emoji<'_, '_> = Emoji("🕒 ", "");

/// ASCII art banner for the project indexer
pub fn print_banner() {
    println!(
        "{}",
        style(
            r#"
 ____            _           _     ___           _
|  _ \ _ __ ___ (_) ___  ___| |_  |_ _|_ __   __| | _____  _____ _ __
| |_) | '__/ _ \| |/ _ \/ __| __|  | || '_ \ / _` |/ _ \ \/ / _ \ '__|
|  __/| | | (_) | |  __/ (__| |_   | || | | | (_| |  __/>  <  __/ |
|_|   |_|  \___// |\___|\___|\__| |___|_| |_|\__,_|\___/_/\_\___|_|
              |__/
"#
        )
        .cyan()
        .bold()
    );
    println!(
        "{} {}",
        style("Version:").dim(),
        style(env!("CARGO_PKG_VERSION")).cyan()
    );
    println!();
    println!(
        "{}",
        style("A powerful tool for indexing and organizing your projects")
            .italic()
            .dim()
    );
    println!();
}

/// Create a progress bar for directory scanning
pub fn create_scan_progress() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{prefix:.bold.dim} {spinner} {wide_msg}\n{bar:40.cyan/blue}")
            .unwrap(),
    );
    pb.set_prefix(format!("{} Scanning", LOOKING_GLASS));
    pb.set_message("Starting directory scan...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a progress bar for project processing
pub fn create_process_progress(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{bar:40.cyan/blue}] {pos}/{len} ({eta})\n{wide_msg}")
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_prefix(format!("{} Processing", PACKAGE));
    pb
}

/// Print a section header
pub fn print_section(emoji: &str, text: &str) {
    println!(
        "\n{} {}\n{}",
        style(emoji).bold(),
        style(text).bold(),
        style("═".repeat(text.len() + 3)).dim()
    );
}

/// Print a success message
pub fn print_success(msg: &str) {
    println!(
        "{} {}",
        style(format!("{} Success:", SPARKLES)).green().bold(),
        style(msg).green()
    );
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!(
        "{} {}",
        style(format!("{} Info:", CLOCK)).blue().bold(),
        style(msg).dim()
    );
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!(
        "{} {}",
        style("⚠ Warning:").yellow().bold(),
        style(msg).yellow()
    );
}

/// Print an error message
pub fn print_error(msg: &str) {
    println!("{} {}", style("✖ Error:").red().bold(), style(msg).red());
}

/// Print detailed project information
pub fn print_project_details(
    name: &str,
    category: &str,
    status: &str,
    tags: &[String],
    path: &str,
) {
    println!("\n{} {}", FOLDER, style(name).bold().underlined());
    println!("   {} Category: {}", CHART, style(category).cyan());
    println!(
        "   {} Status: {}",
        GEAR,
        match status {
            "active" => style(status).green(),
            "archived" => style(status).yellow(),
            _ => style(status).dim(),
        }
    );
    println!(
        "   {} Tags: {}",
        TAG,
        if tags.is_empty() {
            style("none").dim().to_string()
        } else {
            style(tags.join(", ")).cyan().to_string()
        }
    );
    println!("   {} Path: {}", LOOKING_GLASS, style(path).dim());
}

/// Print project statistics with categories
pub fn print_detailed_stats(
    total_projects: usize,
    active_projects: usize,
    archived_projects: usize,
    projects_by_category: &HashMap<String, usize>,
    total_tags: usize,
) {
    println!("\n{}", style("📊 Project Statistics").bold());
    println!("{}", style("═".repeat(50)).dim());

    // Overall statistics
    println!(
        "{} Total Projects: {}",
        BOOKS,
        style(total_projects).cyan().bold()
    );
    println!(
        "{} Active Projects: {}",
        ROCKET,
        style(active_projects).green().bold()
    );
    println!(
        "{} Archived Projects: {}",
        CONSTRUCTION,
        style(archived_projects).yellow().bold()
    );
    println!("{} Total Tags: {}", TAG, style(total_tags).cyan().bold());

    // Category breakdown
    println!("\n{}", style("Projects by Category").bold());
    println!("{}", style("─".repeat(30)).dim());
    for (category, count) in projects_by_category.iter() {
        println!(
            "{} {}: {}",
            FOLDER,
            style(category).cyan(),
            style(count).bold()
        );
    }
}

/// Print configuration details
pub fn print_config_details(projects_dir: &str, index_file: &str, enable_ollama: bool) {
    println!("\n{}", style("Configuration Details").bold());
    println!("{}", style("═".repeat(30)).dim());
    println!(
        "{} Projects Directory: {}",
        FOLDER,
        style(projects_dir).cyan()
    );
    println!("{} Index File: {}", PACKAGE, style(index_file).cyan());
    println!(
        "{} Ollama Enabled: {}",
        GEAR,
        if enable_ollama {
            style("yes").green()
        } else {
            style("no").yellow()
        }
    );
}

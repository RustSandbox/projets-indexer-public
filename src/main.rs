use std::collections::HashMap;
use std::path::PathBuf;

mod config;
mod error;
mod indexer;
mod models;
mod ollama;
mod ui;

use projets_indexer::cli::{parse_args, Commands};
use projets_indexer::config::indexer_config::IndexerConfig;
use projets_indexer::error::Result;
use projets_indexer::indexer::project_indexer::ProjectIndexer;
use projets_indexer::models::project::ProjectStatus;
use projets_indexer::ui::{
    create_process_progress, create_scan_progress, print_banner, print_config_details,
    print_detailed_stats, print_error, print_info, print_project_details, print_section,
    print_success, print_warning,
};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = parse_args();

    // Initialize logging with appropriate level
    let log_level = if cli.verbose { "debug" } else { "info" };
    fmt()
        .with_env_filter(EnvFilter::new(format!("projets_indexer={}", log_level)))
        .init();

    // Print welcome banner
    print_banner();

    match cli.command {
        Commands::Index {
            projects_dir,
            output,
            ollama,
            max_depth,
            min_depth,
            exclude,
        } => {
            // Create indexer configuration
            print_section("⚙️", "Configuration");
            let config = IndexerConfig::new(projects_dir, output, ollama);

            // Print detailed configuration
            print_config_details(
                config.projects_dir.to_str().unwrap_or_default(),
                config.index_file.to_str().unwrap_or_default(),
                config.enable_ollama,
            );

            // Create and initialize the indexer
            print_section("🔧", "Initialization");
            let indexer = match ProjectIndexer::new(config) {
                Ok(indexer) => {
                    print_success("Indexer initialized successfully");
                    indexer
                }
                Err(e) => {
                    print_error(&format!("Failed to initialize indexer: {}", e));
                    return Err(e);
                }
            };

            // Check Ollama availability if enabled
            if indexer.config.enable_ollama {
                if let Some(client) = &indexer.config.ollama_client {
                    match client.check_availability().await {
                        Ok(true) => print_success("Ollama service is available"),
                        Ok(false) => print_warning(
                            "Ollama service is not available - tags will not be generated",
                        ),
                        Err(e) => print_warning(&format!("Failed to check Ollama service: {}", e)),
                    }
                }
            }

            // Start indexing
            print_section("🔍", "Indexing Projects");
            let scan_progress = create_scan_progress();
            scan_progress.set_message("Scanning directory...");

            match indexer
                .index_projects(|project_name| {
                    scan_progress.set_message(format!("Scanning project: {}", project_name));
                })
                .await
            {
                Ok(projects) => {
                    scan_progress.finish_with_message("Directory scan complete");

                    // Process projects with progress bar
                    let total_projects = projects.len() as u64;
                    let process_progress = create_process_progress(total_projects);

                    // Print detailed information for each project
                    for project in &projects {
                        process_progress.inc(1);
                        process_progress.set_message(format!("Processing {}", project.name));

                        print_project_details(
                            &project.name,
                            &project.category,
                            match project.status {
                                ProjectStatus::Active => "active",
                                ProjectStatus::Archived => "archived",
                                ProjectStatus::Unknown => "unknown",
                            },
                            &project.tags,
                            &project.path,
                        );
                    }

                    // Calculate statistics
                    let active_count = projects
                        .iter()
                        .filter(|p| matches!(p.status, ProjectStatus::Active))
                        .count();
                    let archived_count = projects
                        .iter()
                        .filter(|p| matches!(p.status, ProjectStatus::Archived))
                        .count();

                    // Calculate projects by category
                    let mut projects_by_category: HashMap<String, usize> = HashMap::new();
                    let mut total_tags = 0;
                    for project in &projects {
                        *projects_by_category
                            .entry(project.category.clone())
                            .or_insert(0) += 1;
                        total_tags += project.tags.len();
                    }

                    process_progress.finish_and_clear();
                    print_success(&format!("Successfully indexed {} projects", total_projects));
                    print_detailed_stats(
                        projects.len(),
                        active_count,
                        archived_count,
                        &projects_by_category,
                        total_tags,
                    );
                }
                Err(e) => {
                    scan_progress.abandon_with_message("Indexing failed");
                    print_error(&format!("Failed to index projects: {}", e));
                    return Err(e);
                }
            }
        }
        Commands::Search {
            query,
            index_file,
            tags_only,
            category_only,
        } => {
            // TODO: Implement search functionality
            println!("Search functionality coming soon!");
            println!("Query: {}", query);
            println!("Index file: {}", index_file.display());
            println!("Tags only: {}", tags_only);
            println!("Category only: {}", category_only);
        }
        Commands::Stats {
            index_file,
            detailed,
        } => {
            // TODO: Implement stats functionality
            println!("Stats functionality coming soon!");
            println!("Index file: {}", index_file.display());
            println!("Detailed: {}", detailed);
        }
        Commands::GenerateTags {
            project_dir,
            output,
        } => {
            // TODO: Implement tag generation functionality
            println!("Tag generation functionality coming soon!");
            println!("Project directory: {}", project_dir.display());
            if let Some(output) = output {
                println!("Output file: {}", output.display());
            }
        }
    }

    Ok(())
}

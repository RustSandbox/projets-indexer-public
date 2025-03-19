use std::path::PathBuf;

mod config;
mod error;
mod indexer;
mod models;
mod ollama;
mod ui;

use clap::Parser;
use error::AppError;
use indexer::ProjectIndexer;
use ollama::{check_ollama_installation, ensure_model_available, ClientConfig, OllamaClient};
use ui::{print_banner, print_error, print_info, print_success};

// Import CLI module
use crate::cli::Cli;

mod cli;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Print banner
    print_banner();

    // Check for Ollama and model if needed
    if cli.ollama {
        if let Err(e) = ensure_model_available().await {
            print_error(&format!("Ollama setup failed: {}", e));
            return Err(e.into());
        }
        print_success("Ollama and required model are ready");
    }

    // Initialize Ollama client if needed
    let ollama_client = if cli.ollama {
        let config = ClientConfig {
            base_url: cli.ollama_url.clone(),
            timeout: std::time::Duration::from_secs(30),
        };

        match OllamaClient::new(config) {
            Ok(client) => Some(client),
            Err(e) => {
                print_error(&format!("Failed to initialize Ollama client: {}", e));
                return Err(e);
            }
        }
    } else {
        None
    };

    // Execute command
    match cli.command {
        cli::Commands::Index {
            projects_dir,
            output,
            max_depth,
            min_depth,
            exclude,
        } => {
            // Create indexer config
            let config = indexer::project_indexer::IndexerConfig::new(
                projects_dir,
                output,
                max_depth,
                min_depth,
                exclude,
            );

            // Create project indexer
            let indexer = ProjectIndexer::new(config, ollama_client);

            print_info("Starting project indexing...");
            let projects = indexer.index_projects(|msg| print_info(msg)).await?;
            print_success(&format!("Successfully indexed {} projects", projects.len()));
        }
        cli::Commands::Search {
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
        cli::Commands::Stats {
            index_file,
            detailed,
        } => {
            // TODO: Implement stats functionality
            println!("Stats functionality coming soon!");
            println!("Index file: {}", index_file.display());
            println!("Detailed: {}", detailed);
        }
        cli::Commands::GenerateTags {
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

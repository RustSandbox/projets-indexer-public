//! Command-line interface for the project indexer
//!
//! This module provides the CLI interface using clap, including argument parsing
//! and command-line options.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A powerful tool for indexing and organizing your projects
#[derive(Parser, Debug)]
#[command(
    author = "Hamze Ghalebi <ghalebi@gmail.com>",
    version = "0.1.0",
    about = "A tool for indexing and organizing your projects. It can scan directories, \
                  detect project types, generate tags using Local llm models, and provide detailed statistics \
                  about your project collection.",
    long_about = "A tool for indexing and organizing your projects. It can scan directories, \
                  detect project types, generate tags using AI, and provide detailed statistics \
                  about your project collection."
)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output (debug level logging)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable color output in terminal
    #[arg(short, long, global = true)]
    pub no_color: bool,
}

/// Available commands for the project indexer
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Index projects in the specified directory
    #[command(
        about = "Index projects in the specified directory",
        long_about = "Scan and index projects in the specified directory. This command will:\n\
        - Recursively scan the directory for projects\n\
        - Detect project status (active/archived)\n\
        - Generate tags using Ollama (if enabled)\n\
        - Save project metadata to a JSON file"
    )]
    Index {
        /// Directory containing projects to index
        #[arg(
            short = 'd',
            long,
            default_value = "~/projects",
            help = "Directory containing projects to index"
        )]
        projects_dir: PathBuf,

        /// Output file for the index
        #[arg(
            short,
            long,
            default_value = "projects_index.json",
            help = "JSON file to store the project index"
        )]
        output: PathBuf,

        /// Enable Ollama for tag generation
        #[arg(
            short = 'a',
            long,
            default_value_t = true,
            help = "Enable Ollama AI for generating project tags"
        )]
        ollama: bool,

        /// Maximum depth to traverse directories
        #[arg(
            short = 'x',
            long,
            default_value_t = 3,
            help = "Maximum directory depth to traverse"
        )]
        max_depth: usize,

        /// Minimum depth to traverse directories
        #[arg(
            short = 'm',
            long,
            default_value_t = 3,
            help = "Minimum directory depth to traverse"
        )]
        min_depth: usize,

        /// Exclude specific directories (comma-separated)
        #[arg(
            short = 'e',
            long,
            default_value = ".git,node_modules,__pycache__,target,.idea,.vscode",
            help = "Directories to exclude (comma-separated)"
        )]
        exclude: String,
    },

    /// Search through indexed projects
    #[command(
        about = "Search through indexed projects",
        long_about = "Search for projects in the index based on name, tags, or category."
    )]
    Search {
        /// Search query
        #[arg(help = "Text to search for in project names, tags, or categories")]
        query: String,

        /// Index file to search in
        #[arg(
            short,
            long,
            default_value = "projects_index.json",
            help = "JSON file containing the project index"
        )]
        index_file: PathBuf,

        /// Search by tags only
        #[arg(short, long, help = "Only search in project tags")]
        tags_only: bool,

        /// Search by category only
        #[arg(short, long, help = "Only search in project categories")]
        category_only: bool,
    },

    /// Show project statistics
    #[command(
        about = "Show statistics about indexed projects",
        long_about = "Display detailed statistics about your project collection, including:\n\
        - Total number of projects\n\
        - Active vs archived projects\n\
        - Projects by category\n\
        - Tag statistics"
    )]
    Stats {
        /// Index file to analyze
        #[arg(
            short,
            long,
            default_value = "projects_index.json",
            help = "JSON file containing the project index"
        )]
        index_file: PathBuf,

        /// Show detailed category breakdown
        #[arg(short, long, help = "Show detailed statistics for each category")]
        detailed: bool,
    },

    /// Generate tags for a specific project
    #[command(
        about = "Generate tags for a specific project",
        long_about = "Use Ollama to generate descriptive tags for a specific project directory."
    )]
    GenerateTags {
        /// Project directory
        #[arg(
            short,
            long,
            help = "Directory containing the project to generate tags for"
        )]
        project_dir: PathBuf,

        /// Output file for the tags
        #[arg(short, long, help = "Optional file to save the generated tags")]
        output: Option<PathBuf>,
    },
}

/// Parse command-line arguments
pub fn parse_args() -> Cli {
    Cli::parse()
}

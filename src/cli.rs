//! Command-line interface for the project indexer
//!
//! This module provides the CLI interface using clap, including argument parsing
//! and command-line options.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A powerful tool for indexing and organizing your projects
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output (debug level logging)
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable color output in terminal
    #[arg(short, long)]
    pub no_color: bool,

    /// Enable Ollama for tag generation
    #[arg(short, long)]
    pub ollama: bool,

    /// Ollama API URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub ollama_url: String,

    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the project indexer
#[derive(Subcommand)]
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

        /// Maximum directory depth to traverse
        #[arg(
            short = 'x',
            long,
            default_value_t = 3,
            help = "Maximum directory depth to traverse"
        )]
        max_depth: u32,

        /// Minimum directory depth to traverse
        #[arg(
            short = 'm',
            long,
            default_value_t = 3,
            help = "Minimum directory depth to traverse"
        )]
        min_depth: u32,

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

        /// Search only in tags
        #[arg(short, long, help = "Only search in project tags")]
        tags_only: bool,

        /// Search only in categories
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

        /// Show detailed statistics
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

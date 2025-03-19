//! # Projects Indexer
//!
//! `projets-indexer` is a powerful command-line tool for indexing and organizing your projects
//! with AI-powered tag generation. It helps developers maintain a clear overview of their project
//! collection by scanning directories, detecting project types, and providing detailed statistics.
//!
//! ## Features
//!
//! - 🔍 Recursive directory scanning with configurable depth
//! - 📊 Project status detection (active/archived) based on git history
//! - 🏷️ AI-powered tag generation using Ollama
//! - 📁 Smart project categorization based on directory structure
//! - 🔎 Search functionality across projects, tags, and categories
//! - 📈 Detailed project statistics and insights
//!
//! ## Usage
//!
//! ```bash
//! # Index projects with default settings
//! projets-indexer index
//!
//! # Index projects with custom directory and output
//! projets-indexer index -d ~/my-projects -o my-index.json
//!
//! # Search projects
//! projets-indexer search "machine learning"
//!
//! # Show project statistics
//! projets-indexer stats
//!
//! # Generate tags for a specific project
//! projets-indexer generate-tags -p ~/projects/my-project
//! ```
//!
//! ## Modules
//!
//! - [`cli`](cli/index.html): Command-line interface implementation using clap
//! - [`config`](config/index.html): Configuration structures and handling
//! - [`indexer`](indexer/index.html): Core project indexing functionality
//! - [`models`](models/index.html): Data structures for projects and related entities
//! - [`ollama`](ollama/index.html): Integration with Ollama for AI-powered tag generation
//! - [`ui`](ui/index.html): Terminal UI components and progress indicators
//! - [`error`](error/index.html): Error types and handling
//!
//! ## Examples
//!
//! Basic usage of the library:
//!
//! ```rust,no_run
//! use projets_indexer::{
//!     config::IndexerConfig,
//!     indexer::ProjectIndexer,
//! };
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create indexer configuration
//!     let config = IndexerConfig::new(
//!         PathBuf::from("~/projects"),
//!         PathBuf::from("index.json"),
//!         true, // enable Ollama
//!     );
//!
//!     // Initialize project indexer
//!     let indexer = ProjectIndexer::new(config)?;
//!
//!     // Index projects with a progress callback
//!     let projects = indexer.index_projects(|project_name| {
//!         println!("Indexing: {}", project_name);
//!     }).await?;
//!
//!     println!("Found {} projects", projects.len());
//!     Ok(())
//! }
//! ```
//!
//! ## Re-exports
//!
//! The following types are re-exported for convenience:
//!
//! - [`IndexerConfig`]: Configuration for the project indexer
//! - [`ProjectIndexer`]: Main indexer implementation
//! - [`Project`]: Project metadata structure
//! - [`ProjectStatus`]: Project status enumeration
//! - [`OllamaClient`]: Client for interacting with Ollama API
//!
//! ## Error Handling
//!
//! The library uses a custom error type [`OllamaError`] and a type alias [`Result`]
//! for consistent error handling across all modules.
//!
//! ## Dependencies
//!
//! - `tokio`: Async runtime
//! - `serde`: Serialization/deserialization
//! - `walkdir`: Directory traversal
//! - `tracing`: Logging and diagnostics
//! - `reqwest`: HTTP client for Ollama API

pub mod cli;
pub mod config;
pub mod error;
pub mod indexer;
pub mod models;
pub mod ollama;
pub mod ui;

pub use cli::{Cli, Commands};
pub use config::indexer_config::IndexerConfig;
pub use error::{OllamaError, Result};
pub use indexer::project_indexer::ProjectIndexer;
pub use models::project::{Project, ProjectStatus};
pub use ollama::{ClientConfig, GenerateRequest, GenerateResponse, OllamaClient};

/// Common types and traits for the projects indexer.
///
/// This module re-exports commonly used types and traits to simplify imports.
/// It's recommended to use this module when you need multiple types from different
/// submodules.
pub mod prelude {
    pub use crate::error::{OllamaError, Result};
    pub use crate::ollama::{
        ClientConfig, GenerateOptions, GenerateRequest, GenerateResponse, OllamaClient,
    };
}

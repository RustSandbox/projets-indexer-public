//! A powerful command-line tool for indexing and organizing your projects with AI-powered tag generation.
//!
//! This crate provides functionality to:
//! - Scan and index projects in a directory
//! - Detect project status (active/archived)
//! - Generate tags using Ollama's AI capabilities
//! - Search through indexed projects
//! - Generate project statistics
//!
//! # Features
//!
//! - 🔍 Recursive directory scanning
//! - 🚀 Project status detection
//! - 📦 AI-powered tag generation
//! - 📚 Smart project categorization
//! - 🔎 Search functionality
//! - 📊 Detailed project statistics
//!
//! # Usage
//!
//! ```bash
//! # Index projects with Ollama tag generation
//! projets-indexer index --ollama
//!
//! # Search through indexed projects
//! projets-indexer search "your query"
//!
//! # Show project statistics
//! projets-indexer stats
//!
//! # Generate tags for a specific project
//! projets-indexer generate-tags /path/to/project
//! ```
//!
//! # Examples
//!
//! ```
//! use projets_indexer::{
//!     ollama::{OllamaClient, ClientConfig},
//!     error::Result,
//! };
//! use std::time::Duration;
//! use mockito::Server;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut server = Server::new_async().await;
//!     let mock = server.mock("POST", "/api/generate")
//!         .with_status(200)
//!         .with_header("content-type", "application/json")
//!         .with_body(r#"{"response": "rust, cli, project management"}"#)
//!         .create_async()
//!         .await;
//!
//!     let config = ClientConfig {
//!         base_url: server.url(),
//!         timeout: Duration::from_secs(30),
//!     };
//!     let client = OllamaClient::new(config)?;
//!     let response = client.generate_tags("my-project").await?;
//!     assert_eq!(response, vec!["rust", "cli", "project management"]);
//!     Ok(())
//! }
//! ```
//!
//! # Modules
//!
//! - `cli`: Command-line interface and argument parsing
//! - `config`: Configuration types and settings
//! - `indexer`: Project scanning and indexing functionality
//! - `models`: Data models and types
//! - `ollama`: Ollama API client and integration
//! - `ui`: User interface components and formatting
//! - `error`: Error types and handling

pub mod cli;
pub mod config;
pub mod error;
pub mod indexer;
pub mod models;
pub mod ollama;
pub mod ui;

pub use error::{AppError, Result};
pub use indexer::ProjectIndexer;
pub use models::{Project, ProjectStatus};
pub use ollama::{ClientConfig, OllamaClient};

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

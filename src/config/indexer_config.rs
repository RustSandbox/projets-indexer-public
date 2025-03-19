//! Indexer configuration
//!
//! This module contains the configuration types and settings used by the project indexer.
//! It defines how the indexer should behave, including directory paths and feature flags.

use crate::ollama::OllamaClient;
use std::path::PathBuf;

/// Configuration for the project indexer
///
/// This struct holds all the configuration options needed to run the project indexer.
/// It includes paths for input and output, as well as feature flags for optional
/// functionality like AI-powered tag generation.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::config::indexer_config::IndexerConfig;
/// use std::path::PathBuf;
///
/// let config = IndexerConfig::new(
///     PathBuf::from("/path/to/projects"),
///     PathBuf::from("projects_index.json"),
///     true,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IndexerConfig {
    /// Directory containing projects to index
    ///
    /// This is the root directory that will be scanned for projects.
    /// The indexer will look for git repositories and project directories
    /// within this path.
    pub projects_dir: PathBuf,

    /// Output file for the index
    ///
    /// The path where the generated project index will be saved.
    /// This should be a JSON file that can be used by other tools
    /// to access project metadata.
    pub index_file: PathBuf,

    /// Whether to enable Ollama for tag generation
    ///
    /// When enabled, the indexer will use Ollama's AI capabilities to
    /// generate technical tags for each project. This requires:
    /// - Ollama to be installed and running
    /// - Internet connectivity for API calls
    /// - Sufficient system resources for AI processing
    pub enable_ollama: bool,

    /// Optional Ollama client
    ///
    /// The client instance used to communicate with the Ollama API.
    /// This is initialized when `enable_ollama` is true and can be
    /// used to generate project tags.
    pub ollama_client: Option<OllamaClient>,
}

impl IndexerConfig {
    /// Create a new default configuration
    ///
    /// This function creates a new `IndexerConfig` with the specified
    /// project directory and output file paths. The Ollama client is
    /// initialized later when needed.
    ///
    /// # Arguments
    ///
    /// * `projects_dir` - The root directory to scan for projects
    /// * `index_file` - Where to save the generated index
    /// * `enable_ollama` - Whether to enable AI-powered tag generation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use projets_indexer::config::indexer_config::IndexerConfig;
    /// use std::path::PathBuf;
    ///
    /// let config = IndexerConfig::new(
    ///     PathBuf::from("/path/to/projects"),
    ///     PathBuf::from("projects_index.json"),
    ///     true,
    /// );
    /// ```
    pub fn new(projects_dir: PathBuf, index_file: PathBuf, enable_ollama: bool) -> Self {
        Self {
            projects_dir,
            index_file,
            enable_ollama,
            ollama_client: None,
        }
    }
}

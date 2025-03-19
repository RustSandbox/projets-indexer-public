//! Project indexer implementation
//!
//! This module contains the core functionality for indexing projects.
//! It handles directory traversal, git repository analysis, and project
//! metadata generation.

use crate::config::indexer_config::IndexerConfig;
use crate::error::{OllamaError, Result};
use crate::models::project::{Project, ProjectStatus};
use crate::ollama::{ClientConfig, GenerateRequest, OllamaClient};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing::{error, info};
use walkdir::WalkDir;

/// Main project indexer functionality
///
/// This struct implements the core indexing logic for scanning and analyzing
/// project directories. It handles:
/// - Directory traversal and filtering
/// - Git repository status detection
/// - Project categorization
/// - Tag generation using Ollama
///
/// # Examples
///
/// ```rust,no_run
/// use projets_indexer::{IndexerConfig, ProjectIndexer};
/// use std::path::PathBuf;
///
/// let config = IndexerConfig::new(
///     PathBuf::from("/path/to/projects"),
///     PathBuf::from("projects_index.json"),
///     true,
/// );
///
/// let indexer = ProjectIndexer::new(config)?;
/// indexer.index_projects().await?;
/// ```
pub struct ProjectIndexer {
    /// Configuration for the indexer
    ///
    /// Contains all the settings and options needed to run the indexer,
    /// including paths and feature flags.
    pub config: IndexerConfig,
}

impl ProjectIndexer {
    /// Create a new project indexer with the given configuration
    ///
    /// This function initializes a new `ProjectIndexer` with the provided
    /// configuration. If Ollama is enabled, it will also initialize the
    /// Ollama client for tag generation.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use for the indexer
    ///
    /// # Returns
    ///
    /// A `Result<Self>` containing the initialized `ProjectIndexer` or an error
    /// if initialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use projets_indexer::{IndexerConfig, ProjectIndexer};
    /// use std::path::PathBuf;
    ///
    /// let config = IndexerConfig::new(
    ///     PathBuf::from("/path/to/projects"),
    ///     PathBuf::from("projects_index.json"),
    ///     true,
    /// );
    ///
    /// let indexer = ProjectIndexer::new(config)?;
    /// ```
    pub fn new(config: IndexerConfig) -> Result<Self> {
        let ollama_client = if config.enable_ollama {
            Some(OllamaClient::new(ClientConfig::default())?)
        } else {
            None
        };

        Ok(Self {
            config: IndexerConfig {
                projects_dir: config.projects_dir,
                index_file: config.index_file,
                enable_ollama: config.enable_ollama,
                ollama_client,
            },
        })
    }

    /// Index all projects in the configured directory
    ///
    /// This is the main function that performs the project indexing process.
    /// It:
    /// 1. Traverses the configured directory
    /// 2. Identifies and processes project directories
    /// 3. Generates metadata for each project
    /// 4. Saves the results to the configured output file
    ///
    /// # Returns
    ///
    /// A `Result<Vec<Project>>` containing the list of indexed projects or an error if
    /// the indexing process fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use projets_indexer::{IndexerConfig, ProjectIndexer};
    /// use std::path::PathBuf;
    ///
    /// let config = IndexerConfig::new(
    ///     PathBuf::from("/path/to/projects"),
    ///     PathBuf::from("projects_index.json"),
    ///     true,
    /// );
    ///
    /// let indexer = ProjectIndexer::new(config)?;
    /// indexer.index_projects().await?;
    /// ```
    pub async fn index_projects<F>(&self, mut progress_callback: F) -> Result<Vec<Project>>
    where
        F: FnMut(&str),
    {
        info!(
            "Starting project indexing from: {}",
            self.config.projects_dir.display()
        );

        let mut projects = Vec::new();

        // Common system directories and build artifacts to exclude
        let excluded_dirs = [
            ".git",
            "node_modules",
            "__pycache__",
            "target",
            ".idea",
            ".vscode",
            ".env",
            ".mypy_cache",
            "venv",
            ".gradio",
            "__MACOSX",
            "build",
            "dist",
            ".next",
            ".cache",
            ".pytest_cache",
            ".tox",
            ".eggs",
            "*.egg-info",
            "coverage",
            "htmlcov",
            ".coverage",
            ".DS_Store",
        ];

        for entry in WalkDir::new(&self.config.projects_dir)
            .min_depth(3) // Skip top-level directories
            .max_depth(3) // Don't go too deep
            .into_iter()
            .filter_entry(|e| {
                let file_name = e.file_name().to_string_lossy();
                !excluded_dirs.iter().any(|excluded| file_name == *excluded)
                    && !file_name.starts_with('.')
            })
        {
            let entry = entry?;
            if entry.file_type().is_dir() {
                let project_name = entry.file_name().to_string_lossy().to_string();
                progress_callback(&project_name);
                if let Some(project) = self.process_project_directory(entry.path()).await {
                    projects.push(project);
                }
            }
        }

        // Sort projects by category and name
        projects.sort_by(|a, b| a.category.cmp(&b.category).then(a.name.cmp(&b.name)));

        // Write the index file
        info!("Writing index file: {}", self.config.index_file.display());
        let json = serde_json::to_string_pretty(&projects).expect("Failed to serialize projects");
        let mut file = File::create(&self.config.index_file)?;
        file.write_all(json.as_bytes())?;

        info!("Successfully indexed {} projects", projects.len());
        Ok(projects)
    }

    /// Process a project directory and return a Project if it's valid
    ///
    /// This function analyzes a directory to determine if it's a valid project
    /// and generates the appropriate metadata. It:
    /// 1. Checks if the directory is a valid project
    /// 2. Determines the project category
    /// 3. Gets the git status
    /// 4. Generates tags if Ollama is enabled
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the project directory
    ///
    /// # Returns
    ///
    /// An `Option<Project>` containing the project metadata if the directory
    /// is a valid project, or `None` otherwise.
    async fn process_project_directory(&self, path: &Path) -> Option<Project> {
        // Skip if the directory is a .git directory or inside one
        if path.to_string_lossy().contains("/.git/") {
            return None;
        }

        let name = path.file_name()?.to_string_lossy().to_string();
        let category = self.determine_project_category(path);
        let status = self.get_git_status(path).await;
        let tags = if self.config.enable_ollama {
            self.generate_project_tags(path).await.unwrap_or_default()
        } else {
            Vec::new()
        };

        Some(Project {
            name,
            category,
            status,
            tags,
            path: path.to_string_lossy().to_string(),
        })
    }

    /// Get the git status of a project
    ///
    /// This function checks if a directory is a git repository and determines
    /// its status (active, archived, or unknown) based on git commands.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the project directory
    ///
    /// # Returns
    ///
    /// A `ProjectStatus` indicating the current state of the project.
    async fn get_git_status(&self, path: &Path) -> ProjectStatus {
        // Check if this is a git repository
        let git_dir = path.join(".git");
        if !git_dir.exists() {
            return ProjectStatus::Unknown;
        }

        // Skip if we're inside a .git directory
        if path.to_string_lossy().contains("/.git/") {
            return ProjectStatus::Unknown;
        }

        // Try to get git status
        let output = tokio::process::Command::new("git")
            .arg("status")
            .current_dir(path)
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                let status = String::from_utf8_lossy(&output.stdout);
                if status.contains("nothing to commit") {
                    ProjectStatus::Active
                } else {
                    ProjectStatus::Active // Consider all git repos as active for now
                }
            }
            _ => ProjectStatus::Unknown,
        }
    }

    /// Generate tags for a project
    ///
    /// This function generates technical tags for a project using the Ollama
    /// AI model if enabled, or returns default tags if not.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the project directory
    ///
    /// # Returns
    ///
    /// A `Result<Vec<String>>` containing the generated tags or an error if
    /// tag generation fails.
    async fn generate_project_tags(&self, path: &Path) -> Result<Vec<String>> {
        // Generate tags if Ollama client is available
        if let Some(_) = &self.config.ollama_client {
            let project_name = path
                .file_name()
                .ok_or_else(|| OllamaError::ValidationError("Invalid project name".to_string()))?
                .to_string_lossy()
                .to_string();

            match self
                .generate_tags_with_ollama(
                    project_name.as_str(),
                    path.to_string_lossy().to_string().as_str(),
                )
                .await
            {
                Ok(tags) => Ok(tags),
                Err(e) => {
                    error!("Failed to generate tags: {}", e);
                    Ok(Vec::new())
                }
            }
        } else {
            // Default tags when Ollama is not enabled
            Ok(vec!["rust".to_string(), "cli".to_string()])
        }
    }

    /// Determine the category of a project based on its path
    ///
    /// This function determines a project's category based on its location
    /// in the directory structure. The category is typically the name of
    /// the parent directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the project directory
    ///
    /// # Returns
    ///
    /// A `String` containing the project's category, or "uncategorized"
    /// if the category cannot be determined.
    fn determine_project_category(&self, path: &Path) -> String {
        path.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("uncategorized")
            .to_string()
    }

    /// Generate tags using Ollama
    ///
    /// This function uses the Ollama API to generate technical tags for a
    /// project based on its name and description.
    ///
    /// # Arguments
    ///
    /// * `project_name` - The name of the project
    /// * `description` - A description or path of the project
    ///
    /// # Returns
    ///
    /// A `Result<Vec<String>>` containing the generated tags or an error if
    /// the API call fails.
    async fn generate_tags_with_ollama(
        &self,
        project_name: &str,
        description: &str,
    ) -> Result<Vec<String>> {
        let client = OllamaClient::new(ClientConfig::default())?;
        let prompt = format!(
            "Generate 3-5 technical tags for this project named '{}'. Description: {}. \
            Output ONLY comma-separated tags, no explanations or additional text.",
            project_name, description
        );

        let request = GenerateRequest {
            model: "gemma3:1b".to_string(),
            prompt,
            system: Some("You are a technical project tagger. Output ONLY comma-separated tags, no explanations or additional text.".to_string()),
            template: None,
            context: None,
            options: None,
            stream: false,
            format: None,
        };

        let response = client.generate(request).await?;

        // Clean up the response and extract tags
        let tags: Vec<String> = response
            .response
            .trim() // Remove leading/trailing whitespace
            .lines() // Split by newlines
            .flat_map(|line| line.split(',')) // Split each line by comma
            .map(|tag| tag.trim().to_lowercase()) // Clean up each tag
            .filter(|tag| !tag.is_empty()) // Remove empty tags
            .map(|tag| tag.replace(&['*', ':', '.', '(', ')', '[', ']', '{', '}'][..], "")) // Remove special characters
            .collect();

        if tags.is_empty() {
            // Provide default tags if no valid tags were found
            Ok(vec!["rust".to_string(), "cli".to_string()])
        } else {
            Ok(tags)
        }
    }
}

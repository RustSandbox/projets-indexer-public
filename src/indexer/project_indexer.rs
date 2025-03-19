//! Project indexer implementation
//!
//! This module contains the core functionality for indexing projects.
//! It handles directory traversal, git repository analysis, and project
//! metadata generation.

use crate::{
    error::{OllamaError, Result},
    models::{Project, ProjectStatus},
    ollama::{ClientConfig, OllamaClient},
};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
use walkdir::WalkDir;

/// Configuration for the project indexer
#[derive(Debug, Clone)]
pub struct IndexerConfig {
    /// Directory containing projects to index
    pub projects_dir: PathBuf,

    /// JSON file to store the index
    pub index_file: PathBuf,

    /// Maximum directory depth to traverse
    pub max_depth: u32,

    /// Minimum directory depth to traverse
    pub min_depth: u32,

    /// Directories to exclude
    pub exclude: String,

    /// Ollama client for tag generation
    pub ollama_client: Option<OllamaClient>,
}

impl IndexerConfig {
    /// Create a new indexer configuration
    pub fn new(
        projects_dir: PathBuf,
        index_file: PathBuf,
        max_depth: u32,
        min_depth: u32,
        exclude: String,
    ) -> Self {
        Self {
            projects_dir,
            index_file,
            max_depth,
            min_depth,
            exclude,
            ollama_client: None,
        }
    }
}

/// Main project indexer implementation
pub struct ProjectIndexer {
    config: IndexerConfig,
}

impl ProjectIndexer {
    /// Create a new project indexer
    pub fn new(config: IndexerConfig, ollama_client: Option<OllamaClient>) -> Self {
        let mut config = config;
        config.ollama_client = ollama_client;
        Self { config }
    }

    /// Index projects in the configured directory
    pub async fn index_projects<F>(&self, mut progress_callback: F) -> Result<Vec<Project>>
    where
        F: FnMut(&str),
    {
        let mut projects = Vec::new();
        let exclude_dirs: Vec<&str> = self.config.exclude.split(',').collect();

        for entry in WalkDir::new(&self.config.projects_dir)
            .max_depth(self.config.max_depth as usize)
            .min_depth(self.config.min_depth as usize)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir()
                && !exclude_dirs
                    .iter()
                    .any(|&dir| path.to_string_lossy().contains(dir))
            {
                progress_callback(
                    path.file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default(),
                );
                if let Ok(project) = self.process_project(path).await {
                    projects.push(project);
                }
            }
        }

        // Sort projects by category and name
        projects.sort_by(|a, b| a.category.cmp(&b.category).then(a.name.cmp(&b.name)));

        // Save index to file
        self.save_index(&projects)?;

        Ok(projects)
    }

    /// Process a single project directory
    async fn process_project(&self, path: &Path) -> Result<Project> {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let category = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("uncategorized")
            .to_string();

        let mut project = Project::new(name, path.to_path_buf());
        project.category = category;

        // Detect project status
        if path.join(".git").exists() {
            project.status = self.detect_git_status(path).await;
        }

        // Generate tags if Ollama is enabled
        if let Some(client) = &self.config.ollama_client {
            if let Ok(tags) = client
                .generate_tags(path.to_str().unwrap_or_default())
                .await
            {
                project.tags = tags;
            }
        }

        Ok(project)
    }

    /// Detect project status based on git repository
    async fn detect_git_status(&self, path: &Path) -> ProjectStatus {
        // TODO: Implement git status detection
        ProjectStatus::Unknown
    }

    /// Save project index to file
    fn save_index(&self, projects: &[Project]) -> Result<()> {
        let json = serde_json::to_string_pretty(projects)
            .map_err(|e| OllamaError::JsonError(e.to_string()))?;
        fs::write(&self.config.index_file, json).map_err(|e| OllamaError::IoError(e))?;
        Ok(())
    }

    /// Search through indexed projects
    pub async fn search_projects(&self, query: &str) -> Result<Vec<Project>> {
        // TODO: Implement project search
        Ok(Vec::new())
    }

    /// Get statistics about indexed projects
    pub async fn get_statistics(&self) -> Result<ProjectStatistics> {
        // TODO: Implement statistics calculation
        Ok(ProjectStatistics {
            total_projects: 0,
            active_projects: 0,
            archived_projects: 0,
            projects_by_category: HashMap::new(),
        })
    }

    /// Generate tags for a specific project
    pub async fn generate_tags(&self, project_name: &str) -> Result<Vec<String>> {
        // TODO: Implement tag generation
        Ok(Vec::new())
    }
}

/// Statistics about indexed projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub total_projects: usize,
    pub active_projects: usize,
    pub archived_projects: usize,
    pub projects_by_category: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_index_projects() {
        let temp_dir = tempdir().unwrap();
        let config = IndexerConfig::new(
            temp_dir.path().to_path_buf(),
            temp_dir.path().join("index.json"),
            3,
            3,
            ".git,node_modules".to_string(),
        );

        let indexer = ProjectIndexer::new(config, None);
        let result = indexer.index_projects(|_| {}).await;
        assert!(result.is_ok());
    }
}

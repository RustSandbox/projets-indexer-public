//! Project data models
//!
//! This module contains the core data structures used to represent projects
//! and their metadata in the indexer.

use serde::{Deserialize, Serialize};

/// Project status based on git repository state
///
/// This enum represents the current state of a project based on its git repository.
/// It helps identify whether a project is actively maintained or archived.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::models::project::ProjectStatus;
///
/// let status = ProjectStatus::Active;
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    /// Project is actively maintained
    ///
    /// This status indicates that the project has recent commits and is being
    /// actively developed or maintained.
    Active,

    /// Project is archived or no longer maintained
    ///
    /// This status indicates that the project has been archived or is no longer
    /// being actively maintained. This could be determined by:
    /// - No recent commits
    /// - Presence of an ARCHIVED.md file
    /// - Repository marked as archived on the hosting platform
    Archived,

    /// Project status is unknown
    ///
    /// This status is used when the project's state cannot be determined,
    /// such as when:
    /// - The directory is not a git repository
    /// - Git commands fail to execute
    /// - The repository is inaccessible
    Unknown,
}

/// Project metadata
///
/// This struct contains all the metadata associated with a project,
/// including its name, category, status, and technical tags.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::models::project::{Project, ProjectStatus};
///
/// let project = Project {
///     name: "my-project".to_string(),
///     category: "tools".to_string(),
///     status: ProjectStatus::Active,
///     tags: vec!["rust".to_string(), "cli".to_string()],
///     path: "/path/to/project".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// Name of the project
    ///
    /// This is typically the name of the project directory or repository.
    /// It should be a human-readable identifier for the project.
    pub name: String,

    /// Category of the project
    ///
    /// The category is determined by the parent directory name in the
    /// projects directory structure. This helps organize projects into
    /// logical groups.
    pub category: String,

    /// Status of the project
    ///
    /// The current status of the project as determined by git repository
    /// analysis and other heuristics.
    pub status: ProjectStatus,

    /// Technical tags for the project
    ///
    /// A list of technical tags that describe the project's technologies,
    /// frameworks, and characteristics. These tags are either:
    /// - Generated using Ollama AI
    /// - Manually specified
    /// - Default tags when AI generation is disabled
    pub tags: Vec<String>,

    /// Path to the project directory
    ///
    /// The absolute or relative path to the project's root directory.
    /// This is used for navigation and file system operations.
    pub path: String,
}

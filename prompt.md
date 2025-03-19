
# Rust LLM Agent Coder Prompt: Automated Project Indexer

## Overview

We need a Rust program that scans a given project directory and builds a master JSON index (`projects_index.json`). The index should list all projects with metadata, including:
- **Project Name**
- **Category:** Derived from the directory structure (e.g., Learning, Research, Production, Personal)
- **Status:** Determined by the date of the last git commit:
  - **Active:** Last commit is less than 7 days old.
  - **On-Hold:** Last commit is between 7 and 28 days old.
  - **Archived:** Last commit is more than 28 days old.
- **Tags:** A list of tags (generated optionally using a local model via Ollama)
- **Path:** Full path to the project folder

## Directory Structure Assumption

The program assumes the projects directory follows this structure:

```
projects/
├── <Category>/
│   ├── <Lifecycle placeholder (ignored)>/   <-- (Not used; status is computed dynamically)
│   │   └── <Project Folder>/
│   │       ├── .git/ (if a git repository)
│   │       ├── code/
│   │       ├── data/
│   │       └── README.md
...
```

*Note:* The program will ignore any pre-existing lifecycle folder since status is computed by reading the latest git commit timestamp.

## Functional Requirements

1. **Directory Traversal:**
   - Use the [walkdir](https://crates.io/crates/walkdir) crate to recursively scan the projects directory.
   - Limit scanning to the project folders (e.g., minimum depth 3, maximum depth 3) to extract project names and categories.

2. **Git Commit Analysis:**
   - For each project folder that contains a `.git` directory:
     - Execute the command `git log -1 --format=%ct` in the project folder.
     - Parse the Unix timestamp returned.
     - Compute the difference between the current time and the commit timestamp.
   - Based on the difference:
     - **Active:** Less than 7 days old.
     - **On-Hold:** Between 7 and 28 days old.
     - **Archived:** More than 28 days old.
   - If the folder is not a git repository or if the command fails, default the status to **Archived**.

3. **Tag Generation with Ollama (Optional):**
   - Implement a function that simulates calling a local LLM model (using Ollama) to generate tags.
   - The function should build a prompt (e.g., "Generate tags for project: `<project_name>` at path: `<project_path>`") and print/log the call.
   - For now, you may return a dummy tag (e.g., `["example-tag"]`). In a production setup, replace this with an actual call to the Ollama CLI or API.

4. **Index Creation:**
   - Create a `Project` struct (using Serde) with the fields: name, category, status, tags, and path.
   - Collect all projects into a vector.
   - Serialize the vector into a pretty-formatted JSON string.
   - Write this JSON string to a file named `projects_index.json` at the root of your projects directory.

5. **Error Handling & Logging:**
   - Handle errors gracefully (e.g., missing .git folder, failure to parse commit timestamp).
   - Log key actions such as directory scanning, git status computation, and tag generation calls.

## Dependencies

Your `Cargo.toml` should include at least these dependencies:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
walkdir = "2.3"
```

*Additional dependencies can be added as needed for enhanced logging or command execution.*

## Detailed Code Example

Below is an example code snippet to serve as a starting point:

```rust
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    category: String,
    status: String,
    tags: Vec<String>,
    path: String,
}

fn main() -> std::io::Result<()> {
    // Define the base directory for projects.
    let projects_dir = "projects";
    let mut projects: Vec<Project> = Vec::new();

    // Walk through the directory tree.
    // Expected structure: projects/<Category>/<Lifecycle placeholder>/<Project Folder>
    for entry in WalkDir::new(projects_dir)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let path = entry.path();
            if let Some(project_name) = path.file_name().and_then(|s| s.to_str()) {
                // Get category from the grandparent folder.
                if let Some(category) = path.parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                {
                    // Compute status based on git commit history.
                    let status = get_git_status(path).unwrap_or_else(|| "Archived".to_string());

                    let mut project = Project {
                        name: project_name.to_string(),
                        category: category.to_string(),
                        status,
                        tags: Vec::new(),
                        path: path.to_string_lossy().to_string(),
                    };

                    // Optionally generate tags using a local model via Ollama.
                    if project.tags.is_empty() {
                        project.tags = generate_tags_with_ollama(&project.name, &project.path);
                    }
                    projects.push(project);
                }
            }
        }
    }

    // Serialize projects to JSON.
    let json = serde_json::to_string_pretty(&projects)
        .expect("Failed to serialize projects");
    let mut file = File::create("projects_index.json")?;
    file.write_all(json.as_bytes())?;
    println!("projects_index.json created with {} projects.", projects.len());
    Ok(())
}

/// Determines the git status based on the last commit timestamp.
/// - Active: Last commit is less than 7 days old.
/// - On-Hold: Last commit is between 7 and 28 days old.
/// - Archived: Last commit is more than 28 days old.
fn get_git_status(project_path: &Path) -> Option<String> {
    // Check if the project is a git repository.
    if !project_path.join(".git").exists() {
        return None;
    }

    // Execute git command to fetch the last commit timestamp.
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%ct")
        .current_dir(project_path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let commit_timestamp = stdout.trim().parse::<u64>().ok()?;

    // Calculate the time difference between now and the commit.
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let diff_seconds = now.saturating_sub(commit_timestamp);
    let diff_days = diff_seconds / 86400; // Convert seconds to days

    let status = if diff_days < 7 {
        "Active"
    } else if diff_days < 28 {
        "On-Hold"
    } else {
        "Archived"
    };
    Some(status.to_string())
}

/// Simulates generating tags using a local model via Ollama.
/// In a real scenario, replace this with a call to the actual Ollama CLI or API.
fn generate_tags_with_ollama(project_name: &str, project_path: &str) -> Vec<String> {
    let prompt = format!("Generate tags for project: {} at path: {}", project_name, project_path);
    println!("Calling Ollama with prompt: {}", prompt);

    // Example: Call an external command if you have an Ollama CLI.
    // Uncomment and adjust the following block if applicable:
    //
    // let output = Command::new("ollama")
    //     .arg("query")
    //     .arg(&prompt)
    //     .output()
    //     .expect("Failed to execute ollama command");
    // let result = String::from_utf8_lossy(&output.stdout);
    // Parse the result to extract tags.
    //
    // For now, return a dummy tag.
    vec!["example-tag".to_string()]
}
```

## Additional Instructions

- **Code Quality:**  
  - Ensure proper error handling and logging.
  - Write modular code with clear function responsibilities.

- **Testing:**  
  - Test the program with various project setups (with and without a `.git` folder).
  - Validate that the status classification works correctly based on the commit dates.

- **Documentation:**  
  - Add comments and documentation to explain key parts of the code.
  - Update the `README.md` if you add new features or change the directory structure.

- **Local Model Integration:**  
  - Investigate the actual usage of Ollama CLI or API.
  - Update the `generate_tags_with_ollama` function accordingly to replace the dummy implementation with a real call.

## Conclusion

This prompt should provide you, as a Rust LLM agent coder, with a comprehensive guide to implement an automated project indexer. The program will help maintain an up-to-date `projects_index.json` file by analyzing git commit history and optionally leveraging a local LLM model for enhanced metadata tagging.

Please follow the instructions carefully, adapt the code to your project’s specific needs, and ensure the integration with Ollama (or another local model) is properly implemented.


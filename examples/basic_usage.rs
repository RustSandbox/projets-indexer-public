// examples/basic_usage.rs
//
// This example demonstrates basic usage of the Ollama client,
// including text generation and embedding creation.

use projets_indexer::{
    error::Result,
    ollama::{ClientConfig, OllamaClient},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create Ollama client with default configuration
    let config = ClientConfig {
        base_url: "http://localhost:11434".to_string(),
        timeout: std::time::Duration::from_secs(30),
    };
    let client = OllamaClient::new(config)?;

    // Generate tags for a project
    let project_path = "/path/to/your/project";
    let tags = client.generate_tags(project_path).await?;
    println!("Generated tags for {}: {:?}", project_path, tags);

    Ok(())
}

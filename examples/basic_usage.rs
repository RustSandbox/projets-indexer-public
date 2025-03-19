// examples/basic_usage.rs
//
// This example demonstrates basic usage of the Ollama client,
// including text generation and embedding creation.

use projets_indexer::{EmbeddingRequest, GenerateOptions, GenerateRequest, OllamaClient, Result};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with default configuration
    let client = OllamaClient::new_default()?;

    // List available models
    println!("Available models:");
    let models = client.list_models().await?;
    for model in &models {
        println!("- {}", model.name);
    }

    // Generate text
    let generate_request = GenerateRequest {
        model: "llama2".to_string(),
        prompt: "What is Rust programming language?".to_string(),
        system: Some("You are a helpful programming assistant.".to_string()),
        template: None,
        context: None,
        options: Some(GenerateOptions {
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(40),
            num_predict: Some(100),
            stop: Some(vec!["###".to_string()]),
        }),
    };

    println!("\nGenerating text...");
    let response = client.generate(generate_request).await?;
    println!("Response: {}", response.response);
    println!("Generation stats:");
    println!(
        "- Total duration: {:?}ms",
        response.total_duration.unwrap_or(0)
    );
    println!(
        "- Eval count: {:?} tokens",
        response.eval_count.unwrap_or(0)
    );

    // Generate embeddings
    let embedding_request = EmbeddingRequest {
        model: "llama2".to_string(),
        prompt: "Rust programming language".to_string(),
    };

    println!("\nGenerating embeddings...");
    let embedding = client.create_embedding(embedding_request).await?;
    println!("Embedding vector length: {}", embedding.embedding.len());

    Ok(())
}

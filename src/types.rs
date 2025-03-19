// types.rs
//
// This module defines the data structures used for requests and responses
// when interacting with the Ollama API. All structures implement
// serialization/deserialization via serde.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a model in the Ollama API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Name of the model
    pub name: String,
    /// Model configuration parameters
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Request structure for generating completions
#[derive(Debug, Clone, Serialize)]
pub struct GenerateRequest {
    /// The model to use for generation
    pub model: String,
    /// The prompt to generate from
    pub prompt: String,
    /// Optional system prompt to condition the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Optional template to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Optional context window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i32>>,
    /// Optional parameters for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GenerateOptions>,
}

/// Options for generation requests
#[derive(Debug, Clone, Serialize)]
pub struct GenerateOptions {
    /// Temperature for sampling (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    /// Number of tokens to predict
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

/// Response structure for generation requests
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponse {
    /// The generated text
    pub response: String,
    /// Whether this is the final response
    pub done: bool,
    /// Total duration of the request
    #[serde(default)]
    pub total_duration: Option<u64>,
    /// Duration of the load phase
    #[serde(default)]
    pub load_duration: Option<u64>,
    /// Number of tokens in the response
    #[serde(default)]
    pub eval_count: Option<u32>,
    /// Tokens generated per second
    #[serde(default)]
    pub eval_duration: Option<u64>,
}

/// Request structure for embedding generation
#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingRequest {
    /// The model to use for embeddings
    pub model: String,
    /// The prompt to generate embeddings for
    pub prompt: String,
}

/// Response structure for embedding requests
#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingResponse {
    /// The generated embedding vector
    pub embedding: Vec<f32>,
}

// ollama.rs
//
// This module provides a client for interacting with the Ollama API,
// specifically focused on text generation capabilities needed for
// project tag generation.

//! Ollama API client
//!
//! This module provides functionality for interacting with the Ollama API.
//! It includes types for API requests and responses, as well as a client
//! implementation for making API calls.

use crate::error::{OllamaError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the Ollama client
///
/// This struct contains settings for the HTTP client used to communicate
/// with the Ollama API, including timeouts and other connection parameters.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::ollama::ClientConfig;
/// use std::time::Duration;
///
/// let config = ClientConfig {
///     timeout: Duration::from_secs(30),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Timeout for HTTP requests
    ///
    /// The maximum amount of time to wait for a response from the Ollama API.
    /// If no response is received within this time, the request will fail.
    pub timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }
}

/// Options for generating text with Ollama
///
/// This struct contains parameters that control how the Ollama model
/// generates text, including temperature, top-p sampling, and other
/// generation parameters.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::ollama::GenerateOptions;
///
/// let options = GenerateOptions {
///     temperature: Some(0.7),
///     top_p: Some(0.9),
///     top_k: Some(40),
///     num_predict: Some(100),
///     stop: Some(vec!["\n".to_string()]),
///     seed: Some(42),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateOptions {
    /// Temperature for text generation
    ///
    /// Controls the randomness of the output. Higher values make the output
    /// more random, while lower values make it more deterministic.
    pub temperature: Option<f64>,

    /// Top-p sampling parameter
    ///
    /// Controls diversity via nucleus sampling. Higher values allow more
    /// diverse outputs, while lower values make the output more focused.
    pub top_p: Option<f64>,

    /// Top-k sampling parameter
    ///
    /// Controls diversity by limiting the number of tokens considered for
    /// each step of text generation.
    pub top_k: Option<u32>,

    /// Maximum number of tokens to generate
    ///
    /// The maximum length of the generated text in tokens.
    pub num_predict: Option<u32>,

    /// Stop sequences
    ///
    /// A list of strings that, when encountered, will stop the generation.
    pub stop: Option<Vec<String>>,

    /// Random seed for generation
    ///
    /// A seed value for the random number generator used in text generation.
    /// This allows for reproducible outputs.
    pub seed: Option<u64>,
}

/// Request for generating text with Ollama
///
/// This struct represents a request to the Ollama API for text generation.
/// It includes the model to use, the input prompt, and various generation options.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::ollama::GenerateRequest;
///
/// let request = GenerateRequest {
///     model: "gemma3:1b".to_string(),
///     prompt: "Generate a tag for this project".to_string(),
///     system: Some("You are a technical project tagger.".to_string()),
///     template: None,
///     context: None,
///     options: None,
///     stream: false,
///     format: None,
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    /// Name of the model to use
    ///
    /// The identifier of the Ollama model to use for text generation.
    pub model: String,

    /// Input prompt for generation
    ///
    /// The text prompt that will be used to generate the response.
    pub prompt: String,

    /// System prompt for the model
    ///
    /// Optional system-level instructions that guide the model's behavior.
    pub system: Option<String>,

    /// Template for formatting the prompt
    ///
    /// Optional template string for formatting the prompt with variables.
    pub template: Option<String>,

    /// Context from previous interactions
    ///
    /// Optional context from previous interactions to maintain conversation
    /// history or state.
    pub context: Option<Vec<u32>>,

    /// Generation options
    ///
    /// Optional parameters that control how the model generates text.
    pub options: Option<GenerateOptions>,

    /// Whether to stream the response
    ///
    /// If true, the response will be streamed token by token.
    pub stream: bool,

    /// Response format
    ///
    /// Optional format specification for the response.
    pub format: Option<String>,
}

/// Response from the Ollama API
///
/// This struct represents the response received from the Ollama API after
/// a text generation request. It includes the generated text and metadata
/// about the generation process.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::ollama::GenerateResponse;
///
/// let response = GenerateResponse {
///     model: "gemma3:1b".to_string(),
///     created_at: "2024-03-19T12:00:00Z".to_string(),
///     response: "rust, cli, tool".to_string(),
///     done: true,
///     done_reason: None,
///     context: None,
/// };
/// ```
#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    /// Name of the model used
    ///
    /// The identifier of the Ollama model that generated the response.
    pub model: String,

    /// Timestamp of when the response was created
    ///
    /// The time at which the response was generated, in ISO 8601 format.
    pub created_at: String,

    /// Generated text response
    ///
    /// The actual text generated by the model in response to the prompt.
    pub response: String,

    /// Whether generation is complete
    ///
    /// Indicates whether the text generation process has finished.
    pub done: bool,

    /// Reason for completion
    ///
    /// Optional explanation of why the generation process completed.
    pub done_reason: Option<String>,

    /// Context for future interactions
    ///
    /// Optional context that can be used in subsequent requests to maintain
    /// conversation state.
    pub context: Option<Vec<u32>>,
}

/// Client for interacting with the Ollama API
///
/// This struct provides methods for making requests to the Ollama API,
/// including text generation and model management.
///
/// # Examples
///
/// ```rust,no_run
/// use projets_indexer::ollama::{OllamaClient, ClientConfig};
///
/// let client = OllamaClient::new(ClientConfig::default())?;
/// let response = client.generate(request).await?;
/// ```
#[derive(Debug, Clone)]
pub struct OllamaClient {
    /// HTTP client for making requests
    ///
    /// The underlying HTTP client used to communicate with the Ollama API.
    client: Client,
}

impl OllamaClient {
    /// Create a new Ollama client
    ///
    /// This function initializes a new `OllamaClient` with the provided
    /// configuration. It sets up the HTTP client with the specified timeout
    /// and other settings.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the HTTP client
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `OllamaClient` or an error
    /// if initialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use projets_indexer::ollama::{OllamaClient, ClientConfig};
    /// use std::time::Duration;
    ///
    /// let config = ClientConfig {
    ///     timeout: Duration::from_secs(30),
    /// };
    ///
    /// let client = OllamaClient::new(config)?;
    /// ```
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| OllamaError::RequestError(e.to_string()))?;

        Ok(Self { client })
    }

    /// Check if the Ollama service is available
    ///
    /// This function sends a simple request to the Ollama API to verify
    /// that the service is running and accessible.
    ///
    /// # Returns
    ///
    /// A `Result<bool>` indicating whether the service is available.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use projets_indexer::ollama::OllamaClient;
    ///
    /// let client = OllamaClient::new(ClientConfig::default())?;
    /// if client.check_availability().await? {
    ///     println!("Ollama service is available");
    /// }
    /// ```
    pub async fn check_availability(&self) -> Result<bool> {
        match self
            .client
            .get("http://localhost:11434/api/version")
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Generate text using the Ollama API
    ///
    /// This function sends a text generation request to the Ollama API
    /// and returns the generated response.
    ///
    /// # Arguments
    ///
    /// * `request` - The generation request parameters
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated response or an error if the
    /// request fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use projets_indexer::ollama::{OllamaClient, GenerateRequest};
    ///
    /// let client = OllamaClient::new(ClientConfig::default())?;
    /// let request = GenerateRequest {
    ///     model: "gemma3:1b".to_string(),
    ///     prompt: "Generate a tag for this project".to_string(),
    ///     system: None,
    ///     template: None,
    ///     context: None,
    ///     options: None,
    ///     stream: false,
    ///     format: None,
    /// };
    ///
    /// let response = client.generate(request).await?;
    /// ```
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let response = self
            .client
            .post("http://localhost:11434/api/generate")
            .json(&request)
            .send()
            .await
            .map_err(|e| OllamaError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OllamaError::ApiError {
                message: format!("API request failed with status: {}", response.status()),
                status_code: Some(response.status().as_u16()),
            });
        }

        let response = response
            .json::<GenerateResponse>()
            .await
            .map_err(|e| OllamaError::JsonError(e.to_string()))?;

        Ok(response)
    }
}

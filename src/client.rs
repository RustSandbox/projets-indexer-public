// client.rs
//
// This module implements the main Ollama API client.
// It provides a high-level interface for interacting with
// the Ollama REST API endpoints.

use crate::error::{OllamaError, Result};
use crate::types::*;
use reqwest::{Client as ReqwestClient, Url};
use std::time::Duration;

/// Configuration for the Ollama client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the Ollama API
    pub base_url: Url,
    /// Optional timeout for requests
    pub timeout: Option<Duration>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse("http://localhost:11434").unwrap(),
            timeout: Some(Duration::from_secs(30)),
        }
    }
}

/// The main Ollama API client
#[derive(Debug, Clone)]
pub struct OllamaClient {
    config: ClientConfig,
    client: ReqwestClient,
}

impl OllamaClient {
    /// Create a new Ollama client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let mut builder = ReqwestClient::builder();

        if let Some(timeout) = config.timeout {
            builder = builder.timeout(timeout);
        }

        let client = builder.build().map_err(OllamaError::RequestError)?;

        Ok(Self { config, client })
    }

    /// Create a new client with default configuration
    pub fn new_default() -> Result<Self> {
        Self::new(ClientConfig::default())
    }

    /// Generate text from a prompt
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let url = self.config.base_url.join("/api/generate")?;

        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(OllamaError::RequestError)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OllamaError::ApiError {
                message: error_message,
                status_code: Some(status.as_u16()),
            });
        }

        response.json().await.map_err(OllamaError::RequestError)
    }

    /// Generate embeddings for a prompt
    pub async fn create_embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        let url = self.config.base_url.join("/api/embeddings")?;

        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(OllamaError::RequestError)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OllamaError::ApiError {
                message: error_message,
                status_code: Some(status.as_u16()),
            });
        }

        response.json().await.map_err(OllamaError::RequestError)
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let url = self.config.base_url.join("/api/tags")?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(OllamaError::RequestError)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OllamaError::ApiError {
                message: error_message,
                status_code: Some(status.as_u16()),
            });
        }

        #[derive(Deserialize)]
        struct ModelsResponse {
            models: Vec<Model>,
        }

        let models_response: ModelsResponse =
            response.json().await.map_err(OllamaError::RequestError)?;
        Ok(models_response.models)
    }
}

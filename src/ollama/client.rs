use crate::error::{OllamaError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the Ollama client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Base URL for the Ollama API
    pub base_url: String,
    /// Request timeout
    pub timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// Options for generating text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOptions {
    /// Temperature for text generation (0.0 to 1.0)
    pub temperature: f64,
    /// Maximum number of tokens to generate
    pub max_tokens: usize,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 100,
        }
    }
}

/// Request for generating text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// Model to use for generation
    pub model: String,
    /// Prompt to generate text from
    pub prompt: String,
    /// Generation options
    pub options: GenerateOptions,
}

/// Response from the Ollama API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    /// Generated text
    pub response: String,
}

/// Client for interacting with the Ollama API
#[derive(Debug, Clone)]
pub struct OllamaClient {
    config: ClientConfig,
    client: Client,
}

impl OllamaClient {
    /// Create a new Ollama client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| OllamaError::ConnectionError(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Generate tags for a project
    pub async fn generate_tags(&self, project_path: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Generate 3-5 technical tags for this project: {}. \
            Output ONLY comma-separated tags, no explanations or additional text.",
            project_path
        );

        let request = GenerateRequest {
            model: "mistral".to_string(),
            prompt,
            options: GenerateOptions::default(),
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.config.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| OllamaError::ConnectionError(e.to_string()))?
            .json::<GenerateResponse>()
            .await
            .map_err(|e| OllamaError::ParseError(e.to_string()))?;

        // Clean up the response and extract tags
        let tags: Vec<String> = response
            .response
            .trim()
            .lines()
            .flat_map(|line| line.split(','))
            .map(|tag| tag.trim().to_lowercase())
            .filter(|tag| !tag.is_empty())
            .map(|tag| tag.replace(&['*', ':', '.', '(', ')', '[', ']', '{', '}'][..], ""))
            .collect();

        if tags.is_empty() {
            Ok(vec!["rust".to_string(), "cli".to_string()])
        } else {
            Ok(tags)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_generate_tags() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mock_response = r#"{
            "model": "mistral",
            "response": "project management, task tracking, productivity"
        }"#;

        // Create a new server
        let mut server = mockito::Server::new_async().await;

        // Set up mock endpoint
        let mock = server
            .mock("POST", "/api/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;

        // Create client with mock server URL
        let config = ClientConfig {
            base_url: server.url(),
            timeout: Duration::from_secs(30),
        };
        let client = OllamaClient::new(config)?;

        // Test generate_tags
        let tags = client.generate_tags("/path/to/project").await?;

        // Verify the response
        assert!(!tags.is_empty());
        assert_eq!(
            tags,
            vec!["project management", "task tracking", "productivity"]
        );

        // Verify that the mock was called
        mock.assert_async().await;

        Ok(())
    }
}

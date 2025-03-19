use crate::error::OllamaError;
use std::process::Command;

const REQUIRED_MODEL: &str = "mistral";

/// Checks if Ollama is installed and accessible
pub fn check_ollama_installation() -> Result<bool, OllamaError> {
    let output = Command::new("ollama")
        .arg("--version")
        .output()
        .map_err(|e| OllamaError::Setup(format!("Failed to check Ollama installation: {}", e)))?;

    Ok(output.status.success())
}

/// Checks if the required model is pulled
pub fn check_model_availability() -> Result<bool, OllamaError> {
    let output = Command::new("ollama")
        .arg("list")
        .output()
        .map_err(|e| OllamaError::Setup(format!("Failed to list Ollama models: {}", e)))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.contains(REQUIRED_MODEL))
}

/// Pulls the required model if it's not already available
pub async fn ensure_model_available() -> Result<(), OllamaError> {
    if !check_ollama_installation()? {
        return Err(OllamaError::Setup(
            "Ollama is not installed. Please install it first.".to_string(),
        ));
    }

    if !check_model_availability()? {
        println!("Pulling required model '{}'...", REQUIRED_MODEL);
        let status = Command::new("ollama")
            .arg("pull")
            .arg(REQUIRED_MODEL)
            .status()
            .map_err(|e| OllamaError::Setup(format!("Failed to pull model: {}", e)))?;

        if !status.success() {
            return Err(OllamaError::Setup(
                "Failed to pull the required model".to_string(),
            ));
        }
        println!("Model '{}' pulled successfully!", REQUIRED_MODEL);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_installation_check() {
        let result = check_ollama_installation();
        assert!(result.is_ok());
    }

    #[test]
    fn test_model_availability_check() {
        let result = check_model_availability();
        assert!(result.is_ok());
    }
}

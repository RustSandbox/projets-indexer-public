mod client;
mod setup;

pub use client::{ClientConfig, GenerateOptions, GenerateRequest, GenerateResponse, OllamaClient};
pub use setup::{check_model_availability, check_ollama_installation, ensure_model_available};

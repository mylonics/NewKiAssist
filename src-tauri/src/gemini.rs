use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Candidate {
    content: Content,
}

pub async fn send_message_to_gemini(
    api_key: &str,
    model: &str,
    message: &str,
) -> Result<String> {
    let client = Client::new();
    
    // Map model name to full Gemini model ID
    let model_id = match model {
        "2.5-flash" => "gemini-2.0-flash-exp",
        "2.5-pro" => "gemini-exp-1206",
        "3-flash" => "gemini-2.0-flash-exp",
        "3-pro" => "gemini-exp-1206",
        _ => "gemini-2.0-flash-exp", // Default to flash
    };
    
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_id, api_key
    );
    
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: message.to_string(),
            }],
        }],
    };
    
    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
    }
    
    let gemini_response: GeminiResponse = response.json().await?;
    
    if let Some(candidate) = gemini_response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            return Ok(part.text.clone());
        }
    }
    
    Err(anyhow::anyhow!("No response from Gemini API"))
}

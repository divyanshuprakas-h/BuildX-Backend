use std::io;

use reqwest::Client;
use serde_json::{Value, json};

use crate::config::{ai_api_key, ai_model, ai_provider};
use crate::models::generated_code::CodePreviewResponse;

pub async fn generate_code_with_ai(ai_prompt: &str) -> io::Result<Option<CodePreviewResponse>> {
    let provider = ai_provider();

    if provider != "gemini" {
        println!("AI provider is not gemini. Using fallback generator.");
        return Ok(None);
    }

    let Some(api_key) = ai_api_key() else {
        println!("AI_API_KEY is missing. Using fallback generator.");
        return Ok(None);
    };

    if api_key.trim().is_empty() || api_key == "your_gemini_api_key_here" {
        println!("AI_API_KEY is empty or placeholder. Using fallback generator.");
        return Ok(None);
    }

    let model = ai_model();

    println!("Calling Gemini model: {}", model);

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let request_body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": ai_prompt
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.2,
            "maxOutputTokens": 8192,
            "responseMimeType": "application/json"
        }
    });

    let client = Client::new();

    let response = client
        .post(api_url)
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(to_io_error)?;

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_default();

        println!("Gemini API failed with status: {}", status);
        println!("Gemini API error body: {}", error_body);

        return Ok(None);
    }

    let response_json: Value = response.json().await.map_err(to_io_error)?;

    let Some(output_text) = extract_gemini_output_text(&response_json) else {
        println!("Could not extract Gemini output text.");
        println!("Full Gemini response: {}", response_json);
        return Ok(None);
    };

    println!("Gemini output received.");
    println!(
        "Gemini output preview: {}",
        output_text.chars().take(500).collect::<String>()
    );

    let cleaned_json = clean_ai_json_output(&output_text);

    let parsed_response: CodePreviewResponse = match serde_json::from_str(&cleaned_json) {
        Ok(parsed) => parsed,
        Err(error) => {
            println!("Failed to parse Gemini JSON: {}", error);
            println!(
                "Cleaned Gemini JSON preview: {}",
                cleaned_json.chars().take(1000).collect::<String>()
            );

            return Ok(None);
        }
    };

    Ok(Some(parsed_response))
}

fn extract_gemini_output_text(response_json: &Value) -> Option<String> {
    let candidates = response_json.get("candidates")?.as_array()?;
    let first_candidate = candidates.first()?;

    let parts = first_candidate.get("content")?.get("parts")?.as_array()?;

    let first_part = parts.first()?;

    first_part
        .get("text")?
        .as_str()
        .map(|text| text.to_string())
}

fn clean_ai_json_output(output_text: &str) -> String {
    output_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
        .to_string()
}

fn to_io_error(error: reqwest::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error.to_string())
}

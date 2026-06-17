use std::io;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::config::{ai_api_key, ai_model, ai_provider};
use crate::models::generated_code::{CodePreviewResponse, GeneratedFile, GeneratedFileManifest};

pub async fn generate_manifest_with_ai(
    ai_prompt: &str,
) -> io::Result<Option<GeneratedFileManifest>> {
    generate_json_with_gemini::<GeneratedFileManifest>(ai_prompt, "manifest").await
}

pub async fn generate_file_with_ai(ai_prompt: &str) -> io::Result<Option<GeneratedFile>> {
    generate_json_with_gemini::<GeneratedFile>(ai_prompt, "file").await
}

pub async fn generate_code_bundle_with_ai(
    ai_prompt: &str,
) -> io::Result<Option<CodePreviewResponse>> {
    generate_json_with_gemini::<CodePreviewResponse>(ai_prompt, "code_bundle").await
}

async fn generate_json_with_gemini<T>(ai_prompt: &str, response_name: &str) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
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

    println!("Calling Gemini model: {} for {}", model, response_name);

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let max_output_tokens = if response_name == "manifest" {
        2048
    } else {
        8192
    };

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
            "maxOutputTokens": max_output_tokens,
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

    println!("Gemini {} output received.", response_name);

    let cleaned_json = clean_ai_json_output(&output_text);

    match serde_json::from_str::<T>(&cleaned_json) {
        Ok(parsed) => Ok(Some(parsed)),
        Err(error) => {
            println!("Failed to parse Gemini {} JSON: {}", response_name, error);
            println!(
                "Cleaned Gemini JSON preview: {}",
                cleaned_json.chars().take(1000).collect::<String>()
            );

            Ok(None)
        }
    }
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

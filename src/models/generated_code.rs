use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct GeneratedFile {
    pub path: String,
    pub language: String,

    #[serde(deserialize_with = "deserialize_content_as_string")]
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CodePreviewResponse {
    pub files: Vec<GeneratedFile>,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct GeneratedFilePlan {
    pub path: String,
    pub language: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct GeneratedFileManifest {
    pub project_name: String,
    pub file_plans: Vec<GeneratedFilePlan>,
    pub summary: String,
}

fn deserialize_content_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(content) => Ok(content),

        other_value => serde_json::to_string_pretty(&other_value).map_err(serde::de::Error::custom),
    }
}

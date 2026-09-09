use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRepository {
    pub id: String,
    pub name: String,
    pub description: String,
    pub website: Option<String>,
    pub extensions: Vec<ExtensionManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub media_type: ExtensionMediaType,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionMediaType {
    Anime,
    Manga,
    Both,
}

impl ExtensionRepository {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Anime,
    Manga,
}

#[derive(Debug, Clone)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,

    // Information supplied by the source extension.
    pub source_id: Option<String>,
    pub source_url: Option<String>,

    // Remote cover supplied by the source.
    pub cover_url: Option<String>,

    // Local cached cover on the user's device.
    pub local_cover_path: Option<String>,
}

impl Media {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        media_type: MediaType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            media_type,
            source_id: None,
            source_url: None,
            cover_url: None,
            local_cover_path: None,
        }
    }
}
use crate::media::{Media, MediaType};
use crate::sources::Source;

pub struct TestSource;

impl Source for TestSource {
    fn id(&self) -> &str {
        "test_source"
    }

    fn name(&self) -> &str {
        "Test Source"
    }

    fn media_type(&self) -> MediaType {
        MediaType::Anime
    }

    fn search(&self, query: &str) -> Result<Vec<Media>, String> {
        let query = query.to_lowercase();

        let titles = [
            ("1", "One Piece"),
            ("2", "Frieren"),
            ("3", "Sakamoto Days"),
        ];

        let mut results = Vec::new();

        for (id, title) in titles {
            if title.to_lowercase().contains(&query) {
                let mut media = Media::new(id, title, MediaType::Anime);

                media.source_id = Some(self.id().to_string());

                results.push(media);
            }
        }

        Ok(results)
    }
}
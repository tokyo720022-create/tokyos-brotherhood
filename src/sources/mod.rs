use crate::media::{Media, MediaType};

pub mod repository;
pub mod test_source;

pub trait Source {
    /// Unique identifier for this source.
    fn id(&self) -> &str;

    /// Human-readable source name.
    fn name(&self) -> &str;

    /// Whether this source provides anime or manga.
    fn media_type(&self) -> MediaType;

    /// Search this source.
    fn search(&self, query: &str) -> Result<Vec<Media>, String>;
}

pub struct SourceManager {
    sources: Vec<Box<dyn Source>>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Register a source with Tokyo's Brotherhood.
    pub fn add_source(&mut self, source: Box<dyn Source>) {
        self.sources.push(source);
    }

    /// Return all registered sources.
    pub fn sources(&self) -> &[Box<dyn Source>] {
        &self.sources
    }

    /// Search every registered source.
    pub fn search_all(&self, query: &str) -> Vec<Media> {
        let mut results = Vec::new();

        for source in &self.sources {
            match source.search(query) {
                Ok(mut source_results) => {
                    results.append(&mut source_results);
                }
                Err(error) => {
                    eprintln!(
                        "Source '{}' failed to search: {}",
                        source.name(),
                        error
                    );
                }
            }
        }

        results
    }
}
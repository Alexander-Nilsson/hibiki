use crate::infrastructure::font_provider::{FontError, FontProvider};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct TypographyService;

impl TypographyService {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Retrieve the list of available system fonts.
    ///
    /// # Errors
    ///
    /// Returns `FontError` if the font provider fails to load fonts.
    pub async fn get_system_fonts(&self) -> Result<Arc<Vec<String>>, FontError> {
        FontProvider::load_system_fonts_asynchronous().await
    }
}

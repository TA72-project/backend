use serde::Deserialize;
use utoipa::IntoParams;

/// Represents the parameters that could be given to search for something.
#[derive(Default, Clone, Deserialize, IntoParams)]
#[serde(default)]
pub struct SearchParam {
    /// The string to search for in the database.
    ///
    /// Searches are case **insensitive** and look for **substring**.
    search: String,
}

impl SearchParam {
    /// Returns the value wrapped with `%` to be used in database query.
    #[must_use]
    pub fn value(&self) -> String {
        format!("%{}%", self.search)
    }

    /// Returns the value as it was received.
    #[must_use]
    #[allow(dead_code)]
    pub fn raw_value(&self) -> &String {
        &self.search
    }
}

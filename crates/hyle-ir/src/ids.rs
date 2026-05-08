use serde::{Deserialize, Serialize};

/// A validated symbolic identifier used across the IR.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new identifier from a string-like value.
    ///
    /// This placeholder accepts any non-empty trimmed string. Stricter lexical
    /// validation can be added once the language surface is stable.
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            None
        } else {
            Some(Self(trimmed.to_owned()))
        }
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self("unnamed".to_owned())
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0
    }
}

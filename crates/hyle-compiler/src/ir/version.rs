use serde::{Deserialize, Serialize};

/// Schema versions understood by the current IR scaffold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaVersion {
    /// First scaffolded schema for the reset repository.
    #[default]
    #[serde(rename = "v1alpha1")]
    V1Alpha1,
}

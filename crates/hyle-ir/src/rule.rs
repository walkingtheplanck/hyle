use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A rule lowered from the Hyle DSL.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleIr {
    /// Stable rule name.
    pub name: Identifier,
    /// Placeholder textual expression until a real rule IR lands.
    pub expression: String,
}

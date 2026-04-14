//! Declarative cell-state constraints used by blueprint contracts.

/// One named state declared by a portable cell schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateDef {
    /// Human-readable state name.
    pub name: &'static str,
}

impl StateDef {
    /// Construct a named state definition.
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

/// Portable schema metadata for a blueprint cell model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellSchema {
    name: &'static str,
    states: &'static [StateDef],
}

impl CellSchema {
    /// Construct an opaque schema with no enumerated state list.
    pub const fn opaque(name: &'static str) -> Self {
        Self { name, states: &[] }
    }

    /// Construct an enumerated schema with a fixed set of named states.
    pub const fn enumeration(name: &'static str, states: &'static [StateDef]) -> Self {
        Self { name, states }
    }

    /// Human-readable model name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Declared named states, if this model is enumerated.
    pub const fn states(&self) -> &'static [StateDef] {
        self.states
    }

    /// Number of declared states, if the schema is enumerated.
    pub const fn state_count(&self) -> Option<usize> {
        if self.states.is_empty() {
            None
        } else {
            Some(self.states.len())
        }
    }
}

/// A portable cell value that can appear in a blueprint contract.
///
/// This trait captures the data-level requirements shared by
/// blueprint specs, descriptors, and analysis. Solver-facing behavior
/// belongs to [`crate::Cell`], which extends this trait.
pub trait CellState: Copy + Default + Eq + Send + Sync + 'static {}

impl<T> CellState for T where T: Copy + Default + Eq + Send + Sync + 'static {}

/// A portable Rust cell type with shared schema metadata.
pub trait CellModel: CellState {
    /// Portable schema metadata for this cell type.
    fn schema() -> CellSchema;
}

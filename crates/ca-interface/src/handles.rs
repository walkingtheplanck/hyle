//! Shared opaque handle and identifier types.

/// Opaque handle to one logical cell in the active runtime grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellId(u32);

impl CellId {
    /// Construct a cell handle from its raw runtime value.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw cell handle value.
    pub const fn raw(self) -> u32 {
        self.0
    }

    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Stable numeric material identifier used by solvers and runtimes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MaterialId(u16);

impl MaterialId {
    /// Construct an identifier from its raw numeric value.
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Return the identifier as a dense zero-based index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Stable numeric attribute identifier used by solvers and runtimes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttributeId(u16);

impl AttributeId {
    /// Construct an identifier from its raw numeric value.
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Return the identifier as a dense zero-based index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Stable numeric neighborhood identifier used by solvers and runtimes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NeighborhoodId(u16);

impl NeighborhoodId {
    /// Construct an identifier from its raw numeric value.
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Return the identifier as a dense zero-based index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

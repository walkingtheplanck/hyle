//! Enum-backed symbol traits used by schema authoring.

use std::any::TypeId;

use crate::{AttributeType, AttributeValue};

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

/// Type-erased reference to one material symbol from a specific material set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef {
    owner: TypeId,
    id: MaterialId,
    label: &'static str,
}

impl MaterialRef {
    /// Construct a new material reference.
    pub fn new<M: MaterialSet>(material: M) -> Self {
        Self {
            owner: TypeId::of::<M>(),
            id: material.id(),
            label: material.label(),
        }
    }

    /// Return the owning material-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved material identifier.
    pub const fn id(self) -> MaterialId {
        self.id
    }

    /// Return the human-readable material label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}

/// Type-erased reference to one attribute symbol from a specific attribute set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AttributeRef {
    owner: TypeId,
    id: AttributeId,
    label: &'static str,
    value_type: AttributeType,
}

impl AttributeRef {
    /// Construct a new attribute reference.
    pub fn new<A: AttributeSet>(attribute: A) -> Self {
        Self {
            owner: TypeId::of::<A>(),
            id: attribute.id(),
            label: attribute.label(),
            value_type: attribute.value_type(),
        }
    }

    /// Return the owning attribute-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved attribute identifier.
    pub const fn id(self) -> AttributeId {
        self.id
    }

    /// Return the human-readable attribute label.
    pub const fn label(self) -> &'static str {
        self.label
    }

    /// Return the declared scalar type.
    pub const fn value_type(self) -> AttributeType {
        self.value_type
    }
}

/// Type-erased reference to one neighborhood symbol from a specific neighborhood set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeighborhoodRef {
    owner: TypeId,
    id: NeighborhoodId,
    label: &'static str,
}

impl NeighborhoodRef {
    /// Construct a new neighborhood reference.
    pub fn new<N: NeighborhoodSet>(neighborhood: N) -> Self {
        Self {
            owner: TypeId::of::<N>(),
            id: neighborhood.id(),
            label: neighborhood.label(),
        }
    }

    /// Return the owning neighborhood-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved neighborhood identifier.
    pub const fn id(self) -> NeighborhoodId {
        self.id
    }

    /// Return the human-readable neighborhood label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}

/// Enum-backed material universe used by a schema.
pub trait MaterialSet: Copy + Default + Eq + Send + Sync + 'static {
    /// Return the full ordered material set.
    fn variants() -> &'static [Self];

    /// Return the human-readable material label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this material.
    fn id(self) -> MaterialId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("material must appear in its declared variant list");
        MaterialId::new(index as u16)
    }

    /// Return a type-erased reference to this material.
    fn material(self) -> MaterialRef {
        MaterialRef::new(self)
    }

    /// Return the default material identifier used to initialize new grids.
    fn default_material() -> MaterialId {
        Self::default().id()
    }
}

/// Enum-backed attribute universe used by a schema.
pub trait AttributeSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered attribute set.
    fn variants() -> &'static [Self];

    /// Return the human-readable attribute label.
    fn label(self) -> &'static str;

    /// Return the scalar type of this attribute.
    fn value_type(self) -> AttributeType;

    /// Return the stable numeric identifier for this attribute.
    fn id(self) -> AttributeId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("attribute must appear in its declared variant list");
        AttributeId::new(index as u16)
    }

    /// Return a type-erased reference to this attribute.
    fn attribute(self) -> AttributeRef {
        AttributeRef::new(self)
    }

    /// Return the zero value for this attribute.
    fn zero(self) -> AttributeValue {
        AttributeValue::zero(self.value_type())
    }
}

/// Enum-backed neighborhood universe used by a schema.
pub trait NeighborhoodSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered neighborhood set.
    fn variants() -> &'static [Self];

    /// Return the human-readable neighborhood label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this neighborhood.
    fn id(self) -> NeighborhoodId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("neighborhood must appear in its declared variant list");
        NeighborhoodId::new(index as u16)
    }

    /// Return a type-erased reference to this neighborhood.
    fn neighborhood(self) -> NeighborhoodRef {
        NeighborhoodRef::new(self)
    }

    /// Return the default neighborhood identifier, using the first variant.
    fn default_neighborhood() -> NeighborhoodId {
        Self::variants()
            .first()
            .copied()
            .expect("neighborhood set must not be empty")
            .id()
    }
}

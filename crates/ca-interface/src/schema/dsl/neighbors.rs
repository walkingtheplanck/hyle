use std::ops::RangeInclusive;

use crate::schema::{MaterialRef, MaterialSet};

use super::{Condition, CountComparison, Weight, WeightComparison};

/// Select neighbors equal to a specific material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborSelector {
    material: MaterialRef,
}

impl NeighborSelector {
    /// Start a count comparison for the selected material.
    ///
    /// This counts matching neighbor cells, not weighted samples.
    pub fn count(self) -> NeighborCount {
        NeighborCount {
            material: self.material,
        }
    }

    /// Start a weighted-sum comparison for the selected material.
    ///
    /// This uses the neighborhood's interpreted weights rather than plain
    /// neighbor cardinality.
    pub fn weighted_sum(self) -> NeighborWeightedSum {
        NeighborWeightedSum {
            material: self.material,
        }
    }

    /// Require at least one matching neighbor.
    ///
    /// This is shorthand for `neighbors(material).count().at_least(1)`.
    pub fn any(self) -> Condition {
        self.count().at_least(1)
    }

    /// Require no matching neighbors.
    ///
    /// This is shorthand for `neighbors(material).count().eq(0)`.
    pub fn none(self) -> Condition {
        self.count().eq(0)
    }
}

/// A pending numeric comparison on the count of matching neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborCount {
    material: MaterialRef,
}

impl NeighborCount {
    /// Require an exact neighbor count.
    pub fn eq(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::Eq(count),
        }
    }

    /// Require the neighbor count to fall inside an inclusive range.
    pub fn in_range(self, range: RangeInclusive<u32>) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::InRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to fall outside an inclusive range.
    pub fn not_in(self, range: RangeInclusive<u32>) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::NotInRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to be at least the given value.
    pub fn at_least(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::AtLeast(count),
        }
    }

    /// Require the neighbor count to be at most the given value.
    pub fn at_most(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::AtMost(count),
        }
    }
}

/// A pending weighted comparison on matching neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborWeightedSum {
    material: MaterialRef,
}

impl NeighborWeightedSum {
    /// Require an exact weighted sum.
    pub fn eq(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::Eq(weight),
        }
    }

    /// Require the weighted sum to fall inside an inclusive range.
    pub fn in_range(self, range: RangeInclusive<Weight>) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::InRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the weighted sum to fall outside an inclusive range.
    pub fn not_in(self, range: RangeInclusive<Weight>) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::NotInRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the weighted sum to be at least the given value.
    pub fn at_least(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::AtLeast(weight),
        }
    }

    /// Require the weighted sum to be at most the given value.
    pub fn at_most(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::AtMost(weight),
        }
    }
}

/// Select neighbors equal to a specific material.
///
/// The returned selector is still declarative; no neighborhood is chosen until
/// the surrounding rule either supplies one or falls back to the schema default.
pub fn neighbors<M: MaterialSet>(material: M) -> NeighborSelector {
    NeighborSelector {
        material: material.material(),
    }
}

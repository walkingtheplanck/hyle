use std::ops::RangeInclusive;

use crate::schema::{AttributeRef, AttributeSet};
use crate::{AttributeType, AttributeValue};

use super::{AttributeComparison, Condition};

/// Center-cell attribute selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeSelector {
    attribute: AttributeRef,
}

impl AttributeSelector {
    /// Require the attribute to equal an exact value.
    ///
    /// Type compatibility is checked later against the declared attribute type.
    pub fn eq(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::Eq(value.into()),
        }
    }

    /// Require the attribute to lie inside an inclusive range.
    pub fn in_range<T>(self, range: RangeInclusive<T>) -> Condition
    where
        T: Into<AttributeValue> + Copy,
    {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::InRange {
                min: (*range.start()).into(),
                max: (*range.end()).into(),
            },
        }
    }

    /// Require the attribute to lie outside an inclusive range.
    pub fn not_in<T>(self, range: RangeInclusive<T>) -> Condition
    where
        T: Into<AttributeValue> + Copy,
    {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::NotInRange {
                min: (*range.start()).into(),
                max: (*range.end()).into(),
            },
        }
    }

    /// Require the attribute to be at least the given value.
    pub fn at_least(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::AtLeast(value.into()),
        }
    }

    /// Require the attribute to be at most the given value.
    pub fn at_most(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::AtMost(value.into()),
        }
    }

    /// Return the declared scalar type for this attribute.
    ///
    /// This is mostly useful for helper code that wants to branch while still
    /// staying in the typed DSL world.
    pub const fn value_type(self) -> AttributeType {
        self.attribute.value_type()
    }
}

/// Select the center cell's attached attribute.
///
/// The returned selector targets the rule's center cell, not neighboring cells.
pub fn attr<A: AttributeSet>(attribute: A) -> AttributeSelector {
    AttributeSelector {
        attribute: attribute.attribute(),
    }
}

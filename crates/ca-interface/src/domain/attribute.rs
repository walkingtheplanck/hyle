//! Shared scalar attribute value types.

/// Scalar type used by an attached per-cell attribute channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttributeType {
    /// Boolean attribute.
    Bool,
    /// Unsigned 8-bit integer attribute.
    U8,
    /// Unsigned 16-bit integer attribute.
    U16,
    /// Unsigned 32-bit integer attribute.
    U32,
    /// Signed 8-bit integer attribute.
    I8,
    /// Signed 16-bit integer attribute.
    I16,
    /// Signed 32-bit integer attribute.
    I32,
}

impl AttributeType {
    /// Return whether this scalar type is boolean.
    pub const fn is_boolean(self) -> bool {
        matches!(self, AttributeType::Bool)
    }
}

/// Default scalar value declared for an attached per-cell attribute channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeValue {
    /// Boolean default.
    Bool(bool),
    /// Unsigned 8-bit default.
    U8(u8),
    /// Unsigned 16-bit default.
    U16(u16),
    /// Unsigned 32-bit default.
    U32(u32),
    /// Signed 8-bit default.
    I8(i8),
    /// Signed 16-bit default.
    I16(i16),
    /// Signed 32-bit default.
    I32(i32),
}

impl AttributeValue {
    /// Return the declared scalar type for this default value.
    pub const fn value_type(self) -> AttributeType {
        match self {
            AttributeValue::Bool(_) => AttributeType::Bool,
            AttributeValue::U8(_) => AttributeType::U8,
            AttributeValue::U16(_) => AttributeType::U16,
            AttributeValue::U32(_) => AttributeType::U32,
            AttributeValue::I8(_) => AttributeType::I8,
            AttributeValue::I16(_) => AttributeType::I16,
            AttributeValue::I32(_) => AttributeType::I32,
        }
    }

    /// Construct the zero/default value for a declared scalar type.
    pub const fn zero(value_type: AttributeType) -> Self {
        match value_type {
            AttributeType::Bool => AttributeValue::Bool(false),
            AttributeType::U8 => AttributeValue::U8(0),
            AttributeType::U16 => AttributeValue::U16(0),
            AttributeType::U32 => AttributeValue::U32(0),
            AttributeType::I8 => AttributeValue::I8(0),
            AttributeType::I16 => AttributeValue::I16(0),
            AttributeType::I32 => AttributeValue::I32(0),
        }
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue::Bool(value)
    }
}

impl From<u8> for AttributeValue {
    fn from(value: u8) -> Self {
        AttributeValue::U8(value)
    }
}

impl From<u16> for AttributeValue {
    fn from(value: u16) -> Self {
        AttributeValue::U16(value)
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        AttributeValue::U32(value)
    }
}

impl From<i8> for AttributeValue {
    fn from(value: i8) -> Self {
        AttributeValue::I8(value)
    }
}

impl From<i16> for AttributeValue {
    fn from(value: i16) -> Self {
        AttributeValue::I16(value)
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        AttributeValue::I32(value)
    }
}

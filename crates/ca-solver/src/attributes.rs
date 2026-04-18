//! Double-buffered attached attribute storage.

use hyle_ca_interface::{AttributeDef, AttributeId, AttributeType, AttributeValue};

pub(crate) struct AttributeStore {
    buffers: Vec<AttributeBuffer>,
}

impl AttributeStore {
    pub(crate) fn new(total_cells: usize, defs: &[AttributeDef]) -> Self {
        let mut buffers = Vec::with_capacity(defs.len());
        for def in defs {
            buffers.push(AttributeBuffer::new(
                total_cells,
                AttributeValue::zero(def.value_type),
            ));
        }
        Self { buffers }
    }

    pub(crate) fn contains(&self, attribute: AttributeId) -> bool {
        attribute.index() < self.buffers.len()
    }

    pub(crate) fn prepare_step(&mut self) {
        for buffer in &mut self.buffers {
            buffer.prepare_step();
        }
    }

    pub(crate) fn swap(&mut self) {
        for buffer in &mut self.buffers {
            buffer.swap();
        }
    }

    pub(crate) fn get(&self, attribute: AttributeId, cell_index: usize) -> AttributeValue {
        self.buffers[attribute.index()].get(cell_index)
    }

    pub(crate) fn set_next(
        &mut self,
        attribute: AttributeId,
        cell_index: usize,
        value: AttributeValue,
    ) {
        self.buffers[attribute.index()].set_next(cell_index, value);
    }

    pub(crate) fn set_current(
        &mut self,
        attribute: AttributeId,
        cell_index: usize,
        value: AttributeValue,
    ) -> Result<(), (AttributeType, AttributeType)> {
        self.buffers[attribute.index()].set_current(cell_index, value)
    }

    pub(crate) fn reset_next_to_defaults(
        &mut self,
        cell_index: usize,
        defaults: &[AttributeValue],
    ) {
        for (buffer, value) in self.buffers.iter_mut().zip(defaults.iter().copied()) {
            buffer.set_next(cell_index, value);
        }
    }
}

enum AttributeBuffer {
    Bool { current: Vec<bool>, next: Vec<bool> },
    U8 { current: Vec<u8>, next: Vec<u8> },
    U16 { current: Vec<u16>, next: Vec<u16> },
    U32 { current: Vec<u32>, next: Vec<u32> },
    I8 { current: Vec<i8>, next: Vec<i8> },
    I16 { current: Vec<i16>, next: Vec<i16> },
    I32 { current: Vec<i32>, next: Vec<i32> },
}

impl AttributeBuffer {
    fn value_type(&self) -> AttributeType {
        match self {
            Self::Bool { .. } => AttributeType::Bool,
            Self::U8 { .. } => AttributeType::U8,
            Self::U16 { .. } => AttributeType::U16,
            Self::U32 { .. } => AttributeType::U32,
            Self::I8 { .. } => AttributeType::I8,
            Self::I16 { .. } => AttributeType::I16,
            Self::I32 { .. } => AttributeType::I32,
        }
    }

    fn new(total_cells: usize, default: AttributeValue) -> Self {
        match default {
            AttributeValue::Bool(value) => Self::Bool {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U8(value) => Self::U8 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U16(value) => Self::U16 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U32(value) => Self::U32 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I8(value) => Self::I8 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I16(value) => Self::I16 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I32(value) => Self::I32 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
        }
    }

    fn prepare_step(&mut self) {
        match self {
            Self::Bool { current, next } => next.copy_from_slice(current),
            Self::U8 { current, next } => next.copy_from_slice(current),
            Self::U16 { current, next } => next.copy_from_slice(current),
            Self::U32 { current, next } => next.copy_from_slice(current),
            Self::I8 { current, next } => next.copy_from_slice(current),
            Self::I16 { current, next } => next.copy_from_slice(current),
            Self::I32 { current, next } => next.copy_from_slice(current),
        }
    }

    fn swap(&mut self) {
        match self {
            Self::Bool { current, next } => std::mem::swap(current, next),
            Self::U8 { current, next } => std::mem::swap(current, next),
            Self::U16 { current, next } => std::mem::swap(current, next),
            Self::U32 { current, next } => std::mem::swap(current, next),
            Self::I8 { current, next } => std::mem::swap(current, next),
            Self::I16 { current, next } => std::mem::swap(current, next),
            Self::I32 { current, next } => std::mem::swap(current, next),
        }
    }

    fn get(&self, cell_index: usize) -> AttributeValue {
        match self {
            Self::Bool { current, .. } => AttributeValue::Bool(current[cell_index]),
            Self::U8 { current, .. } => AttributeValue::U8(current[cell_index]),
            Self::U16 { current, .. } => AttributeValue::U16(current[cell_index]),
            Self::U32 { current, .. } => AttributeValue::U32(current[cell_index]),
            Self::I8 { current, .. } => AttributeValue::I8(current[cell_index]),
            Self::I16 { current, .. } => AttributeValue::I16(current[cell_index]),
            Self::I32 { current, .. } => AttributeValue::I32(current[cell_index]),
        }
    }

    fn set_next(&mut self, cell_index: usize, value: AttributeValue) {
        let expected = self.value_type();
        let actual = value.value_type();
        debug_assert_eq!(
            expected, actual,
            "attribute value type must match its storage buffer"
        );

        match (self, value) {
            (Self::Bool { next, .. }, AttributeValue::Bool(value)) => next[cell_index] = value,
            (Self::U8 { next, .. }, AttributeValue::U8(value)) => next[cell_index] = value,
            (Self::U16 { next, .. }, AttributeValue::U16(value)) => next[cell_index] = value,
            (Self::U32 { next, .. }, AttributeValue::U32(value)) => next[cell_index] = value,
            (Self::I8 { next, .. }, AttributeValue::I8(value)) => next[cell_index] = value,
            (Self::I16 { next, .. }, AttributeValue::I16(value)) => next[cell_index] = value,
            (Self::I32 { next, .. }, AttributeValue::I32(value)) => next[cell_index] = value,
            _ => {}
        }
    }

    fn set_current(
        &mut self,
        cell_index: usize,
        value: AttributeValue,
    ) -> Result<(), (AttributeType, AttributeType)> {
        let actual = value.value_type();
        let expected = self.value_type();
        match (self, value) {
            (Self::Bool { current, .. }, AttributeValue::Bool(value)) => {
                current[cell_index] = value
            }
            (Self::U8 { current, .. }, AttributeValue::U8(value)) => current[cell_index] = value,
            (Self::U16 { current, .. }, AttributeValue::U16(value)) => current[cell_index] = value,
            (Self::U32 { current, .. }, AttributeValue::U32(value)) => current[cell_index] = value,
            (Self::I8 { current, .. }, AttributeValue::I8(value)) => current[cell_index] = value,
            (Self::I16 { current, .. }, AttributeValue::I16(value)) => current[cell_index] = value,
            (Self::I32 { current, .. }, AttributeValue::I32(value)) => current[cell_index] = value,
            _ => return Err((expected, actual)),
        }
        Ok(())
    }
}

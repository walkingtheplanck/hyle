//! Double-buffered attached attribute storage.

use std::collections::HashMap;

use hyle_ca_interface::{AttributeDef, AttributeValue};

pub(crate) struct AttributeStore {
    index_by_name: HashMap<String, usize>,
    buffers: Vec<AttributeBuffer>,
}

impl AttributeStore {
    pub(crate) fn new(total_cells: usize, defs: &[AttributeDef]) -> Self {
        let mut index_by_name = HashMap::with_capacity(defs.len());
        let mut buffers = Vec::with_capacity(defs.len());

        for (index, def) in defs.iter().enumerate() {
            index_by_name.insert(def.name.clone(), index);
            buffers.push(AttributeBuffer::new(total_cells, def.default));
        }

        Self {
            index_by_name,
            buffers,
        }
    }

    pub(crate) fn index_of(&self, name: &str) -> Option<usize> {
        self.index_by_name.get(name).copied()
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

    pub(crate) fn get(&self, attribute: usize, cell_index: usize) -> AttributeValue {
        self.buffers[attribute].get(cell_index)
    }

    pub(crate) fn set_next(&mut self, attribute: usize, cell_index: usize, value: AttributeValue) {
        self.buffers[attribute].set_next(cell_index, value);
    }

    pub(crate) fn set_current(
        &mut self,
        attribute: usize,
        cell_index: usize,
        value: AttributeValue,
    ) {
        self.buffers[attribute].set_current(cell_index, value);
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
    fn new(total_cells: usize, default: AttributeValue) -> Self {
        match default {
            AttributeValue::Bool(value) => AttributeBuffer::Bool {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U8(value) => AttributeBuffer::U8 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U16(value) => AttributeBuffer::U16 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::U32(value) => AttributeBuffer::U32 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I8(value) => AttributeBuffer::I8 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I16(value) => AttributeBuffer::I16 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
            AttributeValue::I32(value) => AttributeBuffer::I32 {
                current: vec![value; total_cells],
                next: vec![value; total_cells],
            },
        }
    }

    fn prepare_step(&mut self) {
        match self {
            AttributeBuffer::Bool { current, next } => next.copy_from_slice(current),
            AttributeBuffer::U8 { current, next } => next.copy_from_slice(current),
            AttributeBuffer::U16 { current, next } => next.copy_from_slice(current),
            AttributeBuffer::U32 { current, next } => next.copy_from_slice(current),
            AttributeBuffer::I8 { current, next } => next.copy_from_slice(current),
            AttributeBuffer::I16 { current, next } => next.copy_from_slice(current),
            AttributeBuffer::I32 { current, next } => next.copy_from_slice(current),
        }
    }

    fn swap(&mut self) {
        match self {
            AttributeBuffer::Bool { current, next } => std::mem::swap(current, next),
            AttributeBuffer::U8 { current, next } => std::mem::swap(current, next),
            AttributeBuffer::U16 { current, next } => std::mem::swap(current, next),
            AttributeBuffer::U32 { current, next } => std::mem::swap(current, next),
            AttributeBuffer::I8 { current, next } => std::mem::swap(current, next),
            AttributeBuffer::I16 { current, next } => std::mem::swap(current, next),
            AttributeBuffer::I32 { current, next } => std::mem::swap(current, next),
        }
    }

    fn get(&self, cell_index: usize) -> AttributeValue {
        match self {
            AttributeBuffer::Bool { current, .. } => AttributeValue::Bool(current[cell_index]),
            AttributeBuffer::U8 { current, .. } => AttributeValue::U8(current[cell_index]),
            AttributeBuffer::U16 { current, .. } => AttributeValue::U16(current[cell_index]),
            AttributeBuffer::U32 { current, .. } => AttributeValue::U32(current[cell_index]),
            AttributeBuffer::I8 { current, .. } => AttributeValue::I8(current[cell_index]),
            AttributeBuffer::I16 { current, .. } => AttributeValue::I16(current[cell_index]),
            AttributeBuffer::I32 { current, .. } => AttributeValue::I32(current[cell_index]),
        }
    }

    fn set_next(&mut self, cell_index: usize, value: AttributeValue) {
        match (self, value) {
            (AttributeBuffer::Bool { next, .. }, AttributeValue::Bool(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::U8 { next, .. }, AttributeValue::U8(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::U16 { next, .. }, AttributeValue::U16(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::U32 { next, .. }, AttributeValue::U32(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::I8 { next, .. }, AttributeValue::I8(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::I16 { next, .. }, AttributeValue::I16(value)) => {
                next[cell_index] = value
            }
            (AttributeBuffer::I32 { next, .. }, AttributeValue::I32(value)) => {
                next[cell_index] = value
            }
            _ => panic!("attribute value type must match its storage buffer"),
        }
    }

    fn set_current(&mut self, cell_index: usize, value: AttributeValue) {
        match (self, value) {
            (AttributeBuffer::Bool { current, .. }, AttributeValue::Bool(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::U8 { current, .. }, AttributeValue::U8(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::U16 { current, .. }, AttributeValue::U16(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::U32 { current, .. }, AttributeValue::U32(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::I8 { current, .. }, AttributeValue::I8(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::I16 { current, .. }, AttributeValue::I16(value)) => {
                current[cell_index] = value
            }
            (AttributeBuffer::I32 { current, .. }, AttributeValue::I32(value)) => {
                current[cell_index] = value
            }
            _ => panic!("attribute value type must match its storage buffer"),
        }
    }
}

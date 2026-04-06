//! Action — what a rule wants to happen to the center cell.

use super::{Cell, Direction};

/// What a rule wants to happen to the center cell.
/// Generic over the cell type `C`.
#[derive(Clone, Copy, Debug)]
pub enum Action<C: Cell> {
    /// Leave the center cell unchanged.
    Keep,
    /// Replace the center cell with a new value.
    Become(C),
    /// Swap the center cell with one face-adjacent neighbor.
    /// Last-write-wins when two cells try to swap simultaneously.
    Swap(Direction),
    /// Overwrite a specific face-adjacent neighbor.
    /// Useful for "fire spreads to wood" style rules.
    Set(Direction, C),
}

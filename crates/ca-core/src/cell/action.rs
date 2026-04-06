//! Action — what a rule wants to happen to the center cell.

use super::Cell;

/// What a rule wants to happen to the center cell.
///
/// Rules can only affect the cell they're evaluating — never neighbors.
/// This prevents data races in the double-buffered step and ensures
/// results are independent of iteration order.
#[derive(Clone, Copy, Debug)]
pub enum Action<C: Cell> {
    /// Leave the center cell unchanged.
    Keep,
    /// Replace the center cell with a new value.
    Become(C),
}

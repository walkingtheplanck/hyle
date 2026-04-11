//! The Cell trait — the core abstraction for cell state.

/// A cell in the automaton. Implement this trait for your own cell type.
///
/// The built-in `u32` implementation uses the low byte as `rule_id`
/// and treats any non-zero value as alive — suitable for classic
/// binary CA like Game of Life.
///
/// For richer automata (fluids, multi-channel state), define your
/// own struct and implement this trait.
pub trait Cell: Copy + Default + Send + Sync + 'static {
    /// A compact backend-defined dispatch key for this cell.
    ///
    /// Some solvers use exact-state matching and ignore this value entirely;
    /// others may still use it to bucket rules or choose fast paths.
    fn rule_id(&self) -> u8;

    /// Whether this cell counts as "alive" for `Neighborhood::count_alive()`.
    fn is_alive(&self) -> bool;
}

/// Default Cell implementation for u32.
/// Low byte = rule_id, non-zero = alive.
impl Cell for u32 {
    #[inline]
    fn rule_id(&self) -> u8 {
        (*self & 0xFF) as u8
    }

    #[inline]
    fn is_alive(&self) -> bool {
        *self != 0
    }
}

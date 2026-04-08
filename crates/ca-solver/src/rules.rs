//! Rule and world pass storage types.

use hyle_ca_core::{Action, Cell, GridReader, GridWriter, Neighborhood, Rng};

/// A boxed rule closure.
pub(crate) type BoxedRule<C> = Box<dyn Fn(&Neighborhood<C>, Rng) -> Action<C>>;

/// A boxed world pass closure.
pub(crate) type BoxedWorldPass<C> = Box<dyn Fn(&GridReader<C>, &mut GridWriter<C>)>;

/// A per-cell rule with its pre-built neighborhood buffer.
pub(crate) struct RegisteredRule<C: Cell> {
    /// Reusable neighborhood buffer, built at registration time.
    pub neighborhood: Neighborhood<C>,
    /// The rule closure.
    pub rule: BoxedRule<C>,
}

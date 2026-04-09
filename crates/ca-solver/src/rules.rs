//! Rule and world pass storage types.

use hyle_ca_contracts::{Action, Cell, NeighborhoodShape, NeighborhoodSpec, NeighborhoodWeight};

use crate::{
    inverse_square, moore, spherical, unweighted, von_neumann, GridReader, GridWriter,
    Neighborhood, Rng, ShapeFn, WeightFn,
};

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

impl<C: Cell> RegisteredRule<C> {
    /// Build a registered rule from neighborhood configuration and a boxed closure.
    pub(crate) fn new(radius: u32, shape: ShapeFn, weight: WeightFn, rule: BoxedRule<C>) -> Self {
        assert!(radius >= 1, "radius must be >= 1");
        RegisteredRule {
            neighborhood: Neighborhood::new(radius, shape, weight),
            rule,
        }
    }

    /// Build a registered rule with the default radius-1 Moore neighborhood.
    pub(crate) fn with_default_neighborhood(rule: BoxedRule<C>) -> Self {
        RegisteredRule::new(1, moore, unweighted, rule)
    }

    /// Build a registered rule from a declarative neighborhood specification.
    pub(crate) fn with_spec(spec: NeighborhoodSpec, rule: BoxedRule<C>) -> Self {
        assert!(spec.radius >= 1, "radius must be >= 1");
        RegisteredRule::new(
            spec.radius,
            shape_fn(spec.shape),
            weight_fn(spec.weight),
            rule,
        )
    }
}

pub(crate) fn shape_fn(shape: NeighborhoodShape) -> ShapeFn {
    match shape {
        NeighborhoodShape::Moore => moore,
        NeighborhoodShape::VonNeumann => von_neumann,
        NeighborhoodShape::Spherical => spherical,
    }
}

pub(crate) fn weight_fn(weight: NeighborhoodWeight) -> WeightFn {
    match weight {
        NeighborhoodWeight::Unweighted => unweighted,
        NeighborhoodWeight::InverseSquare => inverse_square,
    }
}

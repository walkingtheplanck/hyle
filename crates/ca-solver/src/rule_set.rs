//! Named groups of rule and world-pass registrations.

use hyle_ca_core::{
    moore, unweighted, Action, Cell, GridReader, GridWriter, Neighborhood, Rng, ShapeFn, WeightFn,
};

use crate::rules::{BoxedRule, BoxedWorldPass, RegisteredRule};

enum RuleRegistration<C: Cell> {
    Default {
        cell_type: u8,
        rule: BoxedRule<C>,
    },
    Radius {
        cell_type: u8,
        radius: u32,
        rule: BoxedRule<C>,
    },
    Shape {
        cell_type: u8,
        radius: u32,
        shape: ShapeFn,
        weight: WeightFn,
        rule: BoxedRule<C>,
    },
}

impl<C: Cell> RuleRegistration<C> {
    fn cell_type(&self) -> u8 {
        match self {
            RuleRegistration::Default { cell_type, .. }
            | RuleRegistration::Radius { cell_type, .. }
            | RuleRegistration::Shape { cell_type, .. } => *cell_type,
        }
    }

    fn into_registered_rule(self) -> RegisteredRule<C> {
        match self {
            RuleRegistration::Default { rule, .. } => {
                RegisteredRule::with_default_neighborhood(rule)
            }
            RuleRegistration::Radius { radius, rule, .. } => {
                RegisteredRule::new(radius, moore, unweighted, rule)
            }
            RuleRegistration::Shape {
                radius,
                shape,
                weight,
                rule,
                ..
            } => RegisteredRule::new(radius, shape, weight, rule),
        }
    }
}

/// A named batch of per-cell rules and world passes.
///
/// `RuleSet` is an optional higher-level API on top of direct solver
/// registration. It lets callers describe a cohesive automaton configuration
/// and then install it onto a solver in one step.
///
/// Installing a rule set does not clear existing solver configuration.
/// Per-cell rules still follow the same "last registration wins" behavior as
/// direct `register_rule*` calls, and world passes are appended in order.
pub struct RuleSet<C: Cell> {
    name: String,
    rules: Vec<RuleRegistration<C>>,
    world_passes: Vec<BoxedWorldPass<C>>,
}

impl<C: Cell> RuleSet<C> {
    /// Create an empty named rule set.
    pub fn new(name: impl Into<String>) -> Self {
        RuleSet {
            name: name.into(),
            rules: Vec::new(),
            world_passes: Vec::new(),
        }
    }

    /// The rule set name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add a per-cell rule with radius 1 and a Moore neighborhood.
    pub fn rule(
        mut self,
        cell_type: u8,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) -> Self {
        self.rules.push(RuleRegistration::Default {
            cell_type,
            rule: Box::new(rule),
        });
        self
    }

    /// Add a per-cell rule with a custom radius and Moore neighborhood.
    pub fn rule_with_radius(
        mut self,
        cell_type: u8,
        radius: u32,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) -> Self {
        assert!(radius >= 1, "radius must be >= 1");
        self.rules.push(RuleRegistration::Radius {
            cell_type,
            radius,
            rule: Box::new(rule),
        });
        self
    }

    /// Add a per-cell rule with a custom radius, shape, and weight.
    pub fn rule_with_shape(
        mut self,
        cell_type: u8,
        radius: u32,
        shape: ShapeFn,
        weight: WeightFn,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) -> Self {
        assert!(radius >= 1, "radius must be >= 1");
        self.rules.push(RuleRegistration::Shape {
            cell_type,
            radius,
            shape,
            weight,
            rule: Box::new(rule),
        });
        self
    }

    /// Add a world pass to run after per-cell rules.
    pub fn world_pass(
        mut self,
        pass: impl Fn(&GridReader<C>, &mut GridWriter<C>) + 'static,
    ) -> Self {
        self.world_passes.push(Box::new(pass));
        self
    }

    fn into_parts(self) -> (Vec<RuleRegistration<C>>, Vec<BoxedWorldPass<C>>) {
        (self.rules, self.world_passes)
    }
}

pub(crate) fn install_rule_set<C: Cell>(
    rules: &mut [Option<RegisteredRule<C>>],
    world_passes: &mut Vec<BoxedWorldPass<C>>,
    rule_set: RuleSet<C>,
) {
    let (registrations, set_world_passes) = rule_set.into_parts();

    for registration in registrations {
        let cell_type = registration.cell_type() as usize;
        rules[cell_type] = Some(registration.into_registered_rule());
    }

    world_passes.extend(set_world_passes);
}

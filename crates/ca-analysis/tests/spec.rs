use hyle_ca_analysis::analyze_spec;
use hyle_ca_interface::{
    neighbors, Cell, Hyle, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

impl Cell for LifeCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }
}

#[test]
fn summarizes_rules_and_neighborhoods() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .neighborhood(
            "far",
            NeighborhoodSpec::new(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform),
        )
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(3))
                .becomes(LifeCell::Alive);
            rules
                .when(LifeCell::Alive)
                .using("far")
                .require(neighbors(LifeCell::Alive).count().at_least(1))
                .keep();
        })
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);

    assert_eq!(analysis.summary.rule_count, 2);
    assert_eq!(analysis.summary.neighborhood_count, 2);
    assert_eq!(analysis.summary.max_radius, 2);
    assert_eq!(analysis.neighborhoods[0].used_by_rules, 1);
    assert_eq!(analysis.neighborhoods[1].used_by_rules, 1);
    assert_eq!(analysis.neighborhoods[1].neighbor_count, 124);
    assert_eq!(analysis.all_diagnostics().count(), 0);
}

#[test]
fn detects_shadowed_and_duplicate_rules() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .rules(|rules| {
            rules.when(LifeCell::Alive).keep();
            rules.when(LifeCell::Alive).keep();
            rules
                .when(LifeCell::Alive)
                .require(neighbors(LifeCell::Alive).any())
                .becomes(LifeCell::Dead);
        })
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);

    assert_eq!(analysis.rules[1].duplicate_of, Some(0));
    assert_eq!(analysis.rules[1].shadowed_by, Some(0));
    assert_eq!(analysis.rules[2].shadowed_by, Some(0));
    assert!(analysis.rules[1]
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "duplicate_rule"));
    assert!(analysis.rules[2]
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "shadowed_rule"));
}

#[test]
fn warns_about_unused_named_neighborhoods() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .neighborhood(
            "unused",
            NeighborhoodSpec::new(NeighborhoodShape::Moore, 3, NeighborhoodFalloff::Uniform),
        )
        .rules(|rules| {
            rules.when(LifeCell::Alive).keep();
        })
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);

    assert!(analysis
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "unused_neighborhood"));
}

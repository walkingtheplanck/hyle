use hyle_ca_interface::resolved::{expand_neighborhood, neighbor_count, offsets, Offset3};
use hyle_ca_interface::{
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Sample,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Sample]
    }

    fn label(self) -> &'static str {
        "sample"
    }
}

fn spec(shape: NeighborhoodShape, radius: u32) -> NeighborhoodSpec {
    NeighborhoodSpec::new(
        N::Sample,
        shape,
        NeighborhoodRadius::new(radius),
        NeighborhoodFalloff::Uniform,
    )
}

#[test]
fn expands_moore_offsets_in_canonical_order() {
    let expanded = expand_neighborhood(spec(NeighborhoodShape::Moore, 1));
    let offsets: Vec<_> = expanded.offsets().collect();

    assert_eq!(expanded.neighbor_count(), 26);
    assert_eq!(offsets.first(), Some(&Offset3::new(-1, -1, -1)));
    assert_eq!(offsets.last(), Some(&Offset3::new(1, 1, 1)));
}

#[test]
fn returns_exact_spherical_counts_for_precomputed_and_fallback_radii() {
    assert_eq!(neighbor_count(spec(NeighborhoodShape::Spherical, 100)), 4_187_856);
    assert_eq!(neighbor_count(spec(NeighborhoodShape::Spherical, 101)), 4_314_770);
}

#[test]
fn offsets_match_neighbor_count() {
    let spec = spec(NeighborhoodShape::VonNeumann, 3);
    let offsets = offsets(spec);

    assert_eq!(offsets.len(), neighbor_count(spec) as usize);
    assert!(offsets.contains(&Offset3::new(0, 0, -3)));
    assert!(!offsets.contains(&Offset3::new(3, 3, 0)));
}

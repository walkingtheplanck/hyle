use hyle_ca_interface::{NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};
use hyle_ca_semantics::{expand_neighborhood, neighbor_count, offsets, Offset3};

#[test]
fn expands_moore_offsets_in_canonical_order() {
    let spec = NeighborhoodSpec::new(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);

    let expanded = expand_neighborhood(spec);
    let offsets: Vec<_> = expanded.offsets().collect();

    assert_eq!(expanded.neighbor_count(), 26);
    assert_eq!(offsets.first(), Some(&Offset3::new(-1, -1, -1)));
    assert_eq!(offsets.last(), Some(&Offset3::new(1, 1, 1)));
}

#[test]
fn returns_exact_spherical_counts_for_precomputed_and_fallback_radii() {
    let precomputed = NeighborhoodSpec::new(
        NeighborhoodShape::Spherical,
        100,
        NeighborhoodFalloff::Uniform,
    );
    let fallback = NeighborhoodSpec::new(
        NeighborhoodShape::Spherical,
        101,
        NeighborhoodFalloff::Uniform,
    );

    assert_eq!(neighbor_count(precomputed), 4_187_856);
    assert_eq!(neighbor_count(fallback), 4_314_770);
}

#[test]
fn offsets_match_neighbor_count() {
    let spec = NeighborhoodSpec::new(
        NeighborhoodShape::VonNeumann,
        3,
        NeighborhoodFalloff::Uniform,
    );

    let offsets = offsets(spec);

    assert_eq!(offsets.len(), neighbor_count(spec) as usize);
    assert!(offsets.contains(&Offset3::new(0, 0, -3)));
    assert!(!offsets.contains(&Offset3::new(3, 3, 0)));
}

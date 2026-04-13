//! Tests for the deterministic RNG.

use hyle_ca_interface::Rng;

#[test]
fn deterministic_same_input_same_output() {
    let a = Rng::new(10, 20, 30, 0);
    let b = Rng::new(10, 20, 30, 0);
    assert_eq!(a.raw(), b.raw());
}

#[test]
fn different_positions_differ() {
    let a = Rng::new(0, 0, 0, 0);
    let b = Rng::new(1, 0, 0, 0);
    let c = Rng::new(0, 1, 0, 0);
    let d = Rng::new(0, 0, 1, 0);
    assert_ne!(a.raw(), b.raw());
    assert_ne!(a.raw(), c.raw());
    assert_ne!(a.raw(), d.raw());
}

#[test]
fn different_steps_differ() {
    let a = Rng::new(5, 5, 5, 0);
    let b = Rng::new(5, 5, 5, 1);
    assert_ne!(a.raw(), b.raw());
}

#[test]
fn chance_1_always_true() {
    // n % 1 == 0 is always true
    for x in 0..100 {
        assert!(Rng::new(x, 0, 0, 0).chance(1));
    }
}

#[test]
fn range_stays_in_bounds() {
    for x in 0..1000 {
        let r = Rng::new(x, 0, 0, 0);
        assert!(r.range(10) < 10);
        assert!(r.range(1) == 0);
        assert!(r.range(256) < 256);
    }
}

#[test]
fn chance_is_roughly_uniform() {
    // Over 10000 samples, chance(10) should hit ~10% of the time
    let hits: u32 = (0..10000)
        .filter(|&x| Rng::new(x, 0, 0, 0).chance(10))
        .count() as u32;
    // Allow 7-13% (generous tolerance for hash distribution)
    assert!(hits > 700 && hits < 1300, "expected ~1000, got {hits}");
}

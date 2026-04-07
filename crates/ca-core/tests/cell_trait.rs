//! Tests for the Cell trait and built-in u32 implementation.

use hyle_ca_core::Cell;

#[test]
fn u32_rule_id_is_low_byte() {
    assert_eq!(0u32.rule_id(), 0);
    assert_eq!(1u32.rule_id(), 1);
    assert_eq!(255u32.rule_id(), 255);
    assert_eq!(256u32.rule_id(), 0); // wraps
    assert_eq!(0x0000_FF01u32.rule_id(), 1);
}

#[test]
fn u32_is_alive() {
    assert!(!0u32.is_alive());
    assert!(1u32.is_alive());
    assert!(255u32.is_alive());
    assert!(u32::MAX.is_alive());
}

#[test]
fn u32_default_is_dead() {
    let cell = u32::default();
    assert!(!cell.is_alive());
    assert_eq!(cell.rule_id(), 0);
}

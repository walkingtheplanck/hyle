//! Tests for explicit runtime cell implementations.

use hyle_ca_interface::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestCell(u32);

impl Cell for TestCell {
    fn rule_id(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    fn is_alive(&self) -> bool {
        self.0 != 0
    }
}

#[test]
fn explicit_rule_id_can_use_low_byte_dispatch() {
    assert_eq!(TestCell(0).rule_id(), 0);
    assert_eq!(TestCell(1).rule_id(), 1);
    assert_eq!(TestCell(255).rule_id(), 255);
    assert_eq!(TestCell(256).rule_id(), 0);
    assert_eq!(TestCell(0x0000_FF01).rule_id(), 1);
}

#[test]
fn explicit_cell_can_define_alive_state() {
    assert!(!TestCell(0).is_alive());
    assert!(TestCell(1).is_alive());
    assert!(TestCell(255).is_alive());
    assert!(TestCell(u32::MAX).is_alive());
}

#[test]
fn default_value_behavior_comes_from_the_user_cell_type() {
    let cell = TestCell::default();
    assert!(!cell.is_alive());
    assert_eq!(cell.rule_id(), 0);
}

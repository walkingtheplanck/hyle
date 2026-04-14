//! Tests for runtime cell marker behavior.

use hyle_ca_interface::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestCell(u32);

fn assert_runtime_cell<T: Cell>(_cell: T) {}

#[test]
fn cell_state_types_automatically_implement_runtime_cell() {
    assert_runtime_cell(TestCell(7));
}

#[test]
fn default_value_behavior_comes_from_the_user_cell_type() {
    let cell = TestCell::default();
    assert_eq!(cell, TestCell(0));
}

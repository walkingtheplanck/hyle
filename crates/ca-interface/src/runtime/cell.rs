//! Solver-facing cell model trait.

use crate::contracts::CellState;

/// A runtime cell model used by solvers.
///
/// This is a marker trait for values that can appear in runtime solver grids.
/// Any [`CellState`] automatically implements it.
pub trait Cell: CellState {}

impl<T> Cell for T where T: CellState {}

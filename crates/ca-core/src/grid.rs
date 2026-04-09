//! GridReader / GridWriter - full grid access for world passes.

use crate::Cell;

type ResolveFn<'a> = dyn Fn(i32, i32, i32) -> usize + 'a;

/// Immutable view of the grid. Used by world passes to read cell state.
pub struct GridReader<'a, C: Cell> {
    cells: &'a [C],
    /// Grid width in cells.
    pub width: u32,
    /// Grid height in cells.
    pub height: u32,
    /// Grid depth in cells.
    pub depth: u32,
    guard_idx: usize,
    resolve: &'a ResolveFn<'a>,
}

impl<'a, C: Cell> GridReader<'a, C> {
    /// Create a new grid reader over the given cell slice.
    pub fn new(
        cells: &'a [C],
        width: u32,
        height: u32,
        depth: u32,
        resolve: &'a ResolveFn<'a>,
    ) -> Self {
        let guard_idx = cells
            .len()
            .checked_sub(1)
            .expect("grid readers require a guard cell");
        GridReader {
            cells,
            width,
            height,
            depth,
            guard_idx,
            resolve,
        }
    }

    /// Get the cell at (x, y, z) according to the supplied coordinate resolver.
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> C {
        let index = (self.resolve)(x, y, z);
        self.cells[index]
    }

    /// Iterate all cells as `(x, y, z, cell)`.
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, u32, C)> + '_ {
        let w = self.width;
        let h = self.height;
        self.cells
            .iter()
            .take(self.guard_idx)
            .enumerate()
            .map(move |(i, &c)| {
                let x = (i as u32) % w;
                let y = ((i as u32) / w) % h;
                let z = (i as u32) / (w * h);
                (x, y, z, c)
            })
    }
}

/// Write-only access to the next-state grid. Used by world passes.
///
/// Intentionally has no `get()` method - this prevents world passes from
/// reading their own output, which would cause order-dependent bugs.
pub struct GridWriter<'a, C: Cell> {
    cells: &'a mut [C],
    /// Grid width in cells.
    pub width: u32,
    /// Grid height in cells.
    pub height: u32,
    /// Grid depth in cells.
    pub depth: u32,
    guard_idx: usize,
    resolve: &'a ResolveFn<'a>,
}

impl<'a, C: Cell> GridWriter<'a, C> {
    /// Create a new grid writer over the given cell slice.
    pub fn new(
        cells: &'a mut [C],
        width: u32,
        height: u32,
        depth: u32,
        resolve: &'a ResolveFn<'a>,
    ) -> Self {
        let guard_idx = cells
            .len()
            .checked_sub(1)
            .expect("grid writers require a guard cell");
        GridWriter {
            cells,
            width,
            height,
            depth,
            guard_idx,
            resolve,
        }
    }

    /// Set the cell at (x, y, z) according to the supplied coordinate resolver.
    pub fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        let index = (self.resolve)(x, y, z);
        if index != self.guard_idx {
            self.cells[index] = cell;
        }
    }
}

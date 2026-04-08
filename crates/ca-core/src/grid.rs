//! GridReader / GridWriter - full grid access for world passes.

use crate::Cell;

type ResolveFn<'a> = dyn Fn(i32, i32, i32) -> Option<(u32, u32, u32)> + 'a;

/// Immutable view of the grid. Used by world passes to read cell state.
pub struct GridReader<'a, C: Cell> {
    cells: &'a [C],
    /// Grid width in cells.
    pub width: u32,
    /// Grid height in cells.
    pub height: u32,
    /// Grid depth in cells.
    pub depth: u32,
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
        GridReader {
            cells,
            width,
            height,
            depth,
            resolve,
        }
    }

    /// Get the cell at (x, y, z) according to the supplied coordinate resolver.
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> C {
        match (self.resolve)(x, y, z) {
            Some((x, y, z)) => self.cells[self.idx(x, y, z)],
            None => C::default(),
        }
    }

    /// Iterate all cells as `(x, y, z, cell)`.
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, u32, C)> + '_ {
        let w = self.width;
        let h = self.height;
        self.cells.iter().enumerate().map(move |(i, &c)| {
            let x = (i as u32) % w;
            let y = ((i as u32) / w) % h;
            let z = (i as u32) / (w * h);
            (x, y, z, c)
        })
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
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
        GridWriter {
            cells,
            width,
            height,
            depth,
            resolve,
        }
    }

    /// Set the cell at (x, y, z) according to the supplied coordinate resolver.
    pub fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        if let Some((x, y, z)) = (self.resolve)(x, y, z) {
            let i = self.idx(x, y, z);
            self.cells[i] = cell;
        }
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}

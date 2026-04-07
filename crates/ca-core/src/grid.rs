//! GridReader / GridWriter — full grid access for world passes.

use crate::Cell;

/// Immutable view of the grid. Used by world passes to read cell state.
pub struct GridReader<'a, C: Cell> {
    cells: &'a [C],
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl<'a, C: Cell> GridReader<'a, C> {
    pub fn new(cells: &'a [C], width: u32, height: u32, depth: u32) -> Self {
        GridReader {
            cells,
            width,
            height,
            depth,
        }
    }

    /// Get the cell at (x, y, z). Returns `C::default()` for out-of-bounds.
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> C {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return C::default();
        }
        self.cells[self.idx(x as u32, y as u32, z as u32)]
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
/// Intentionally has no `get()` method — this prevents world passes from
/// reading their own output, which would cause order-dependent bugs.
pub struct GridWriter<'a, C: Cell> {
    cells: &'a mut [C],
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl<'a, C: Cell> GridWriter<'a, C> {
    pub fn new(cells: &'a mut [C], width: u32, height: u32, depth: u32) -> Self {
        GridWriter {
            cells,
            width,
            height,
            depth,
        }
    }

    /// Set the cell at (x, y, z). No-op for out-of-bounds.
    pub fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return;
        }
        let i = self.idx(x as u32, y as u32, z as u32);
        self.cells[i] = cell;
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}

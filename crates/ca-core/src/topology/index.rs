#[inline]
pub(super) fn linear_index(x: u32, y: u32, z: u32, width: u32, height: u32) -> usize {
    (x as usize)
        + (y as usize) * (width as usize)
        + (z as usize) * (width as usize) * (height as usize)
}

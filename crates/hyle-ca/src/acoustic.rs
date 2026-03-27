//! Acoustic simulation — sparse pressure field and stub wave-propagation system.

use std::collections::HashMap;

use glam::IVec3;

// -- AcousticField ------------------------------------------------------------

/// Sparse acoustic pressure field.
#[derive(Default)]
pub struct AcousticField {
    pub pressure_cur:  HashMap<IVec3, f32>,
    pub pressure_prev: HashMap<IVec3, f32>,
}

impl AcousticField {
    /// Inject an acoustic impulse at `pos` with the given `amplitude`.
    pub fn emit(&mut self, pos: IVec3, amplitude: f32) {
        *self.pressure_cur.entry(pos).or_insert(0.0) += amplitude;
    }
}

// -- acoustic_propagation_step ------------------------------------------------

/// Stub wave-propagation step.
pub fn acoustic_propagation_step(field: &mut AcousticField) {
    std::mem::swap(&mut field.pressure_cur, &mut field.pressure_prev);
    field.pressure_cur.clear();

    const SPREAD: f32 = 0.15;
    const EPSILON: f32 = 1e-4;

    let active: Vec<(IVec3, f32)> = field
        .pressure_prev
        .iter()
        .filter(|(_, p)| p.abs() > EPSILON)
        .map(|(pos, p)| (*pos, *p))
        .collect();

    const NEIGHBOURS: [(i32, i32, i32); 6] = [
        ( 1, 0, 0), (-1, 0, 0),
        ( 0, 1, 0), ( 0,-1, 0),
        ( 0, 0, 1), ( 0, 0,-1),
    ];

    for (pos, pressure) in active {
        for (dx, dy, dz) in NEIGHBOURS {
            let neighbour = IVec3::new(pos.x + dx, pos.y + dy, pos.z + dz);
            *field.pressure_cur.entry(neighbour).or_insert(0.0) += pressure * SPREAD;
        }
    }
}

//! Intra-island bond definitions.

use crate::props::MaterialDef;

/// The mechanical interface between two voxels of the same structural island.
#[derive(Debug, Clone)]
pub struct BondDef {
    pub tensile_strength: f32,
    pub fracture_toughness: f32,
    pub ductility: f32,
    pub contact_area: u16,
}

impl BondDef {
    /// Compute the bond between two materials using the weaker partner's properties.
    pub fn from_pair(a: &MaterialDef, b: &MaterialDef) -> Self {
        Self {
            tensile_strength:   a.structural.yield_strength
                                    .min(b.structural.yield_strength),
            fracture_toughness: a.structural.impact_toughness
                                    .min(b.structural.impact_toughness),
            ductility:          a.structural.plasticity
                                    .min(b.structural.plasticity),
            contact_area:       1,
        }
    }

    pub fn with_contact_area(mut self, area: u16) -> Self {
        self.contact_area = area;
        self
    }

    #[inline]
    pub fn effective_strength(&self) -> f32 {
        self.tensile_strength * self.contact_area as f32
    }

    #[inline]
    pub fn fractures_under(&self, impulse: f32) -> bool {
        impulse > self.fracture_toughness * self.contact_area as f32
    }
}

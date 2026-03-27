//! Voxel property definitions — the full physical description of every voxel type.
//!
//! # Design: Property Composition
//! A material is not a named enum with hardcoded behavior.  It is a **bundle
//! of orthogonal physical property groups**.  Engine systems react to those
//! properties generically.

use crate::voxel::MaterialId;

// -- Visual -------------------------------------------------------------------

/// Rendering properties.  Read by the mesher and future PBR shader.
#[derive(Debug, Clone)]
pub struct VisualProps {
    /// Linear-sRGB RGBA base colour.
    pub color:        [f32; 4],
    /// Perceptual roughness: `0.0` = mirror, `1.0` = fully matte.
    pub roughness:    f32,
    /// Metallicity: `0.0` = dielectric, `1.0` = pure metal.
    pub metallic:     f32,
    /// Self-emission (lava, fire, glowing crystals). Linear RGB.
    pub emissive:     [f32; 3],
    /// `0.0` = fully opaque, `1.0` = fully transparent.
    pub transmittance: f32,
    /// Index of refraction (glass ~1.5, water ~1.33, air = 1.0).
    pub ior:          f32,
}

impl Default for VisualProps {
    fn default() -> Self {
        Self {
            color:        [0.5, 0.5, 0.5, 1.0],
            roughness:    0.8,
            metallic:     0.0,
            emissive:     [0.0; 3],
            transmittance: 0.0,
            ior:          1.5,
        }
    }
}

// -- Structural ---------------------------------------------------------------

/// Mechanical / structural properties.
#[derive(Debug, Clone)]
pub struct StructuralProps {
    /// Mass per unit volume at full density (kg/m^3).
    pub density: f32,
    /// Maximum sustained tensile/shear stress (Pa) before a voxel detaches.
    pub yield_strength: f32,
    /// Maximum instantaneous impulse (N*s/m^2) before fracture propagates.
    pub impact_toughness: f32,
    /// Permanent-deformation ratio before fracture.
    pub plasticity: f32,
    /// Resistance to surface wear from friction over time.
    pub abrasion_resistance: f32,
    /// Optional compaction behaviour.
    pub compaction: Option<CompactionDef>,
}

impl Default for StructuralProps {
    fn default() -> Self {
        Self {
            density:            1000.0,
            yield_strength:     0.0,
            impact_toughness:   0.5,
            plasticity:         0.0,
            abrasion_resistance: 0.5,
            compaction:         None,
        }
    }
}

/// Describes how a material compacts under sustained pressure.
#[derive(Debug, Clone)]
pub struct CompactionDef {
    /// Minimum pressure (Pa) needed to begin compaction.
    pub pressure_threshold: f32,
    /// Density increase rate (kg/m^3 per tick per Pa above threshold).
    pub compaction_rate: f32,
    /// Material this voxel becomes once fully compacted (`None` = stays same type).
    pub product: Option<MaterialId>,
}

// -- Thermal ------------------------------------------------------------------

/// Heat and phase-transition properties.
#[derive(Debug, Clone)]
pub struct ThermalProps {
    pub specific_heat: f32,
    pub thermal_conductivity: f32,
    pub ignition_point: Option<f32>,
    pub melting_point: Option<f32>,
    pub vaporisation_point: Option<f32>,
    pub freezing_point: Option<f32>,
    pub melt_product: Option<MaterialId>,
    pub freeze_product: Option<MaterialId>,
    pub burn_product: Option<MaterialId>,
    pub combustion_energy: f32,
    pub incandescence_threshold: Option<f32>,
    pub flame_emission_color: [f32; 3],
    pub flame_emission_radius: f32,
}

impl Default for ThermalProps {
    fn default() -> Self {
        Self {
            specific_heat:           1000.0,
            thermal_conductivity:    1.0,
            ignition_point:          None,
            melting_point:           None,
            vaporisation_point:      None,
            freezing_point:          None,
            melt_product:            None,
            freeze_product:          None,
            burn_product:            None,
            combustion_energy:       0.0,
            incandescence_threshold: None,
            flame_emission_color:    [1.0, 0.5, 0.1],
            flame_emission_radius:   0.0,
        }
    }
}

// -- Chemical -----------------------------------------------------------------

/// Chemical interaction properties.
#[derive(Debug, Clone)]
pub struct ChemicalProps {
    pub ph: f32,
    pub acid_solubility: f32,
    pub base_solubility: f32,
    pub oxidation_resistance: f32,
    pub corrosion_product: Option<MaterialId>,
}

impl Default for ChemicalProps {
    fn default() -> Self {
        Self {
            ph:                   7.0,
            acid_solubility:      0.0,
            base_solubility:      0.0,
            oxidation_resistance: 1.0,
            corrosion_product:    None,
        }
    }
}

// -- Hydraulic ----------------------------------------------------------------

/// Fluid / moisture properties.
#[derive(Debug, Clone)]
pub struct HydraulicProps {
    pub porosity: f32,
    pub permeability: f32,
    pub saturation_ignition_modifier: f32,
    pub saturation_conductivity_modifier: f32,
}

impl Default for HydraulicProps {
    fn default() -> Self {
        Self {
            porosity:                      0.0,
            permeability:                  0.0,
            saturation_ignition_modifier:  0.0,
            saturation_conductivity_modifier: 0.0,
        }
    }
}

// -- Phase --------------------------------------------------------------------

/// State-of-matter and flow properties.
#[derive(Debug, Clone)]
pub struct PhaseProps {
    pub state: PhaseState,
    pub repose_angle: f32,
    pub viscosity: f32,
    pub buoyancy: f32,
}

impl Default for PhaseProps {
    fn default() -> Self {
        Self {
            state:        PhaseState::Solid,
            repose_angle: 90.0,
            viscosity:    0.0,
            buoyancy:     1.0,
        }
    }
}

/// Fundamental state of matter for a material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhaseState {
    Solid,
    Granular,
    Liquid,
    Gas,
}

// -- Acoustic -----------------------------------------------------------------

/// Acoustic (sound propagation) properties of a material.
#[derive(Debug, Clone)]
pub struct AcousticProps {
    pub propagation_speed: f32,
    pub damping: f32,
    pub reflectance: f32,
}

impl Default for AcousticProps {
    fn default() -> Self {
        Self {
            propagation_speed: 343.0,
            damping:           0.1,
            reflectance:       0.5,
        }
    }
}

// -- MaterialDef --------------------------------------------------------------

/// Complete physical and visual description of a voxel material type.
#[derive(Debug, Clone)]
pub struct MaterialDef {
    pub name:       String,
    pub visual:     VisualProps,
    pub structural: StructuralProps,
    pub thermal:    ThermalProps,
    pub chemical:   ChemicalProps,
    pub hydraulic:  HydraulicProps,
    pub phase:      PhaseProps,
    pub acoustic:   AcousticProps,
}

impl Default for MaterialDef {
    fn default() -> Self {
        Self {
            name:       "unnamed".to_string(),
            visual:     VisualProps::default(),
            structural: StructuralProps::default(),
            thermal:    ThermalProps::default(),
            chemical:   ChemicalProps::default(),
            hydraulic:  HydraulicProps::default(),
            phase:      PhaseProps::default(),
            acoustic:   AcousticProps::default(),
        }
    }
}

//! Material interaction rule system.

use std::ops::RangeInclusive;

use crate::voxel::MaterialId;

// -- MaterialMatcher ----------------------------------------------------------

#[derive(Debug, Clone)]
pub enum MaterialMatcher {
    Exact(MaterialId),
    WithProperty {
        property: MaterialProperty,
        range:    RangeInclusive<f32>,
    },
    All(Vec<MaterialMatcher>),
    Any(Vec<MaterialMatcher>),
}

// -- MaterialProperty ---------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialProperty {
    Roughness,
    Metallic,
    Transmittance,
    Ior,
    Density,
    YieldStrength,
    ImpactToughness,
    Plasticity,
    AbrasionResistance,
    SpecificHeat,
    ThermalConductivity,
    IgnitionPoint,
    MeltingPoint,
    VaporisationPoint,
    FreezingPoint,
    CombustionEnergy,
    Ph,
    AcidSolubility,
    BaseSolubility,
    OxidationResistance,
    Porosity,
    Permeability,
    ReposeAngle,
    Viscosity,
    Buoyancy,
    Temperature,
    Saturation,
    Stress,
}

// -- InteractionTrigger -------------------------------------------------------

#[derive(Debug, Clone)]
pub enum InteractionTrigger {
    Adjacency {
        a: MaterialMatcher,
        b: MaterialMatcher,
    },
    TemperatureThreshold {
        material: MaterialMatcher,
        celsius:  f32,
    },
    PressureThreshold {
        material: MaterialMatcher,
        pascals:  f32,
    },
    SaturationThreshold {
        material: MaterialMatcher,
        level:    f32,
    },
}

// -- InteractionCondition -----------------------------------------------------

#[derive(Debug, Clone)]
pub struct InteractionCondition {
    pub target:    ConditionTarget,
    pub property:  MaterialProperty,
    pub op:        CompareOp,
    pub threshold: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionTarget {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
    Equal,
}

impl CompareOp {
    pub fn eval(self, lhs: f32, rhs: f32) -> bool {
        match self {
            Self::LessThan        => lhs < rhs,
            Self::LessThanOrEq    => lhs <= rhs,
            Self::GreaterThan     => lhs > rhs,
            Self::GreaterThanOrEq => lhs >= rhs,
            Self::Equal           => (lhs - rhs).abs() < f32::EPSILON,
        }
    }
}

// -- InteractionEffect --------------------------------------------------------

#[derive(Debug, Clone)]
pub enum InteractionEffect {
    TransformA { into: MaterialTransform },
    TransformB { into: MaterialTransform },
    AddHeat {
        target:               EffectTarget,
        delta_celsius_per_tick: f32,
    },
    AddSaturation {
        target:        EffectTarget,
        delta_per_tick: f32,
    },
    ApplyImpulse {
        target:    EffectTarget,
        direction: ImpulseDir,
        newtons:   f32,
    },
    SpawnInto {
        material: MaterialId,
        target:   EffectTarget,
    },
    WeakenBond { rate: f32 },
}

#[derive(Debug, Clone)]
pub enum MaterialTransform {
    Into(MaterialId),
    UseMeltProduct,
    UseFreezeProduct,
    UseBurnProduct,
    UseCorrosionProduct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectTarget {
    A,
    B,
    Both,
    ANeighbors,
    BNeighbors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpulseDir {
    Up,
    Down,
    Away,
    Toward,
}

// -- InteractionRule ----------------------------------------------------------

#[derive(Debug, Clone)]
pub struct InteractionRule {
    pub id:          String,
    pub trigger:     InteractionTrigger,
    pub conditions:  Vec<InteractionCondition>,
    pub effects:     Vec<InteractionEffect>,
    pub probability: f32,
}

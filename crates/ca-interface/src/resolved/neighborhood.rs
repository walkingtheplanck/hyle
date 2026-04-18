use crate::{NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};

use super::Offset3;
use crate::WEIGHT_SCALE;

/// A single interpreted neighborhood sample offset and its weight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeighborhoodSample {
    offset: Offset3,
    weight: u32,
}

impl NeighborhoodSample {
    /// Construct a new weighted neighborhood sample.
    ///
    /// Samples are the canonical unit solvers iterate over after a neighborhood
    /// spec has been interpreted.
    pub const fn new(offset: Offset3, weight: u32) -> Self {
        Self { offset, weight }
    }

    /// Return the sample offset.
    pub const fn offset(&self) -> Offset3 {
        self.offset
    }

    /// Return the sample weight.
    pub const fn weight(&self) -> u32 {
        self.weight
    }
}

/// A canonical interpreted neighborhood derived from a declarative specification.
#[derive(Clone, Debug, PartialEq)]
pub struct Neighborhood {
    spec: NeighborhoodSpec,
    samples: Vec<NeighborhoodSample>,
}

impl Neighborhood {
    /// Return the declarative spec this value was expanded from.
    pub const fn spec(&self) -> NeighborhoodSpec {
        self.spec
    }

    /// Return the canonical weighted samples included by the neighborhood.
    ///
    /// Samples preserve both offset and weight so uniform and weighted
    /// neighborhoods share one execution representation.
    pub fn samples(&self) -> &[NeighborhoodSample] {
        &self.samples
    }

    /// Return the canonical offsets included by the neighborhood.
    ///
    /// This is mainly a convenience view for callers that do not care about
    /// falloff weights.
    pub fn offsets(&self) -> impl Iterator<Item = Offset3> + '_ {
        self.samples.iter().map(|sample| sample.offset())
    }

    /// Return the number of neighbor positions included by the neighborhood.
    pub fn neighbor_count(&self) -> u32 {
        self.samples.len() as u32
    }

    /// Construct an interpreted neighborhood directly from a declarative spec.
    ///
    /// Interpretation is eager so later callers can treat neighborhoods as plain
    /// offset/weight tables.
    pub fn from_spec(spec: NeighborhoodSpec) -> Self {
        Self {
            spec,
            samples: samples(spec),
        }
    }
}

/// Expand a declarative neighborhood into canonical offsets and metadata.
///
/// This is a named free-function mirror of [`Neighborhood::from_spec`] for
/// callers that prefer a more functional style.
pub fn expand_neighborhood(spec: NeighborhoodSpec) -> Neighborhood {
    Neighborhood::from_spec(spec)
}

/// Return the exact number of neighbor positions included by a neighborhood spec.
///
/// This counts samples after center-cell exclusion and shape/radius expansion.
pub fn neighbor_count(spec: NeighborhoodSpec) -> u32 {
    shape_neighbor_count(spec.shape(), spec.radius().get())
}

/// Return the maximum weighted sum for a fully matching neighborhood spec.
///
/// This is useful for validating rule thresholds against the largest possible
/// weighted total a neighborhood can contribute.
pub fn max_weighted_sum(spec: NeighborhoodSpec) -> u64 {
    samples(spec)
        .into_iter()
        .map(|sample| sample.weight() as u64)
        .sum()
}

/// Return the canonical weighted neighborhood samples for a declarative spec.
///
/// Sample order is deterministic and follows x-major iteration over the
/// neighborhood cube, which keeps solver preprocessing stable.
pub fn samples(spec: NeighborhoodSpec) -> Vec<NeighborhoodSample> {
    let radius = spec.radius().get() as i32;
    let mut samples = Vec::with_capacity(neighbor_count(spec) as usize);

    for dz in -radius..=radius {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if includes(spec.shape(), dx, dy, dz, spec.radius().get()) {
                    let offset = Offset3::new(dx, dy, dz);
                    samples.push(NeighborhoodSample::new(
                        offset,
                        weight(spec.falloff(), offset),
                    ));
                }
            }
        }
    }

    samples
}

/// Return the canonical neighborhood offsets for a declarative spec.
///
/// This drops weights but preserves the same canonical sample order as
/// [`samples`].
pub fn offsets(spec: NeighborhoodSpec) -> Vec<Offset3> {
    samples(spec)
        .into_iter()
        .map(|sample| sample.offset())
        .collect()
}

fn includes(shape: NeighborhoodShape, dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
    if dx == 0 && dy == 0 && dz == 0 {
        return false;
    }

    let radius = radius as i32;

    match shape {
        NeighborhoodShape::Moore => true,
        NeighborhoodShape::VonNeumann => dx.abs() + dy.abs() + dz.abs() <= radius,
        NeighborhoodShape::Spherical => dx * dx + dy * dy + dz * dz <= radius * radius,
    }
}

fn weight(falloff: NeighborhoodFalloff, offset: Offset3) -> u32 {
    match falloff {
        NeighborhoodFalloff::Uniform => WEIGHT_SCALE,
        NeighborhoodFalloff::InverseSquare => {
            let d_sq =
                (offset.dx * offset.dx + offset.dy * offset.dy + offset.dz * offset.dz) as u32;
            WEIGHT_SCALE / d_sq
        }
    }
}

fn shape_neighbor_count(shape: NeighborhoodShape, radius: u32) -> u32 {
    const MAX_PRECOMPUTED_RADIUS: u32 = 100;

    // Small radii dominate normal schema usage, so table lookup keeps common
    // neighborhood queries branch-light while still allowing exact fallback for
    // very large radii.
    if radius <= MAX_PRECOMPUTED_RADIUS {
        return match shape {
            NeighborhoodShape::Moore => MOORE_NEIGHBOR_COUNTS[radius as usize],
            NeighborhoodShape::VonNeumann => VON_NEUMANN_NEIGHBOR_COUNTS[radius as usize],
            NeighborhoodShape::Spherical => SPHERICAL_NEIGHBOR_COUNTS[radius as usize],
        };
    }

    exact_neighbor_count(shape, radius)
}

fn exact_neighbor_count(shape: NeighborhoodShape, radius: u32) -> u32 {
    let radius = radius as i32;
    let mut count = 0u32;

    for dz in -radius..=radius {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if includes(shape, dx, dy, dz, radius as u32) {
                    count += 1;
                }
            }
        }
    }

    count
}

const MOORE_NEIGHBOR_COUNTS: [u32; 101] = [
    0, 26, 124, 342, 728, 1330, 2196, 3374, 4912, 6858, 9260, 12166, 15624, 19682, 24388, 29790,
    35936, 42874, 50652, 59318, 68920, 79506, 91124, 103822, 117648, 132650, 148876, 166374,
    185192, 205378, 226980, 250046, 274624, 300762, 328508, 357910, 389016, 421874, 456532, 493038,
    531440, 571786, 614124, 658502, 704968, 753570, 804356, 857374, 912672, 970298, 1030300,
    1092726, 1157624, 1225042, 1295028, 1367630, 1442896, 1520874, 1601612, 1685158, 1771560,
    1860866, 1953124, 2048382, 2146688, 2248090, 2352636, 2460374, 2571352, 2685618, 2803220,
    2924206, 3048624, 3176522, 3307948, 3442950, 3581576, 3723874, 3869892, 4019678, 4173280,
    4330746, 4492124, 4657462, 4826808, 5000210, 5177716, 5359374, 5545232, 5735338, 5929740,
    6128486, 6331624, 6539202, 6751268, 6967870, 7189056, 7414874, 7645372, 7880598, 8120600,
];

const VON_NEUMANN_NEIGHBOR_COUNTS: [u32; 101] = [
    0, 6, 24, 62, 128, 230, 376, 574, 832, 1158, 1560, 2046, 2624, 3302, 4088, 4990, 6016, 7174,
    8472, 9918, 11520, 13286, 15224, 17342, 19648, 22150, 24856, 27774, 30912, 34278, 37880, 41726,
    45824, 50182, 54808, 59710, 64896, 70374, 76152, 82238, 88640, 95366, 102424, 109822, 117568,
    125670, 134136, 142974, 152192, 161798, 171800, 182206, 193024, 204262, 215928, 228030, 240576,
    253574, 267032, 280958, 295360, 310246, 325624, 341502, 357888, 374790, 392216, 410174, 428672,
    447718, 467320, 487486, 508224, 529542, 551448, 573950, 597056, 620774, 645112, 670078, 695680,
    721926, 748824, 776382, 804608, 833510, 863096, 893374, 924352, 956038, 988440, 1021566,
    1055424, 1090022, 1125368, 1161470, 1198336, 1235974, 1274392, 1313598, 1353600,
];

const SPHERICAL_NEIGHBOR_COUNTS: [u32; 101] = [
    0, 6, 32, 122, 256, 514, 924, 1418, 2108, 3070, 4168, 5574, 7152, 9170, 11512, 14146, 17076,
    20478, 24404, 28670, 33400, 38910, 44472, 50882, 57776, 65266, 73524, 82518, 91964, 101942,
    113080, 124486, 137064, 150554, 164516, 179578, 195268, 212094, 229548, 248438, 267760, 288358,
    310176, 332778, 356636, 381914, 407596, 434550, 462780, 492566, 523304, 555490, 588816, 623442,
    659756, 696506, 735316, 775774, 816576, 860078, 904088, 950674, 997840, 1047330, 1097916,
    1149650, 1204468, 1259302, 1316424, 1376262, 1436384, 1498754, 1562464, 1628754, 1697436,
    1767062, 1838660, 1912318, 1987440, 2064774, 2143640, 2225666, 2308576, 2395090, 2482212,
    2571710, 2663924, 2758342, 2854024, 2951646, 3053616, 3156602, 3261028, 3369442, 3478572,
    3590862, 3705092, 3821854, 3941728, 4064562, 4187856,
];

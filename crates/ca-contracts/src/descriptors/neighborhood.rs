/// Declarative description of how a rule samples nearby cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodSpec {
    shape: NeighborhoodShape,
    radius: u32,
    falloff: NeighborhoodFalloff,
}

impl NeighborhoodSpec {
    /// Construct a new neighborhood specification.
    pub const fn new(shape: NeighborhoodShape, radius: u32, falloff: NeighborhoodFalloff) -> Self {
        Self {
            shape,
            radius,
            falloff,
        }
    }

    /// Construct the standard adjacent neighborhood: radius-1 Moore, unweighted.
    pub const fn adjacent() -> Self {
        Self::new(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform)
    }

    /// Return the declared neighborhood shape.
    pub const fn shape(&self) -> NeighborhoodShape {
        self.shape
    }

    /// Return the declared neighborhood radius.
    pub const fn radius(&self) -> u32 {
        self.radius
    }

    /// Return the declared neighborhood falloff.
    pub const fn falloff(&self) -> NeighborhoodFalloff {
        self.falloff
    }

    /// Return whether this neighborhood uses a weighted falloff.
    pub const fn is_weighted(&self) -> bool {
        !matches!(self.falloff, NeighborhoodFalloff::Uniform)
    }

    /// Return the number of neighbor positions included by this specification.
    pub const fn neighbor_count(&self) -> u32 {
        self.shape.neighbor_count(self.radius)
    }
}

/// Declarative neighborhood shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodShape {
    /// All offsets in the surrounding cube.
    Moore,
    /// Only axis-aligned offsets within the radius.
    VonNeumann,
    /// Offsets within a Euclidean sphere.
    Spherical,
}

impl NeighborhoodShape {
    /// Return whether the offset belongs to this shape at the given radius.
    pub const fn includes(self, dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
        if dx == 0 && dy == 0 && dz == 0 {
            return false;
        }

        let radius = radius as i32;

        match self {
            Self::Moore => true,
            Self::VonNeumann => dx.abs() + dy.abs() + dz.abs() <= radius,
            Self::Spherical => dx * dx + dy * dy + dz * dz <= radius * radius,
        }
    }

    /// Return the number of neighbor positions included by this shape at the given radius.
    pub const fn neighbor_count(self, radius: u32) -> u32 {
        const MAX_PRECOMPUTED_RADIUS: u32 = 100;

        if radius <= MAX_PRECOMPUTED_RADIUS {
            return match self {
                Self::Moore => MOORE_NEIGHBOR_COUNTS[radius as usize],
                Self::VonNeumann => VON_NEUMANN_NEIGHBOR_COUNTS[radius as usize],
                Self::Spherical => SPHERICAL_NEIGHBOR_COUNTS[radius as usize],
            };
        }

        self.exact_neighbor_count(radius)
    }

    const fn exact_neighbor_count(self, radius: u32) -> u32 {
        let radius = radius as i32;
        let mut count = 0u32;
        let mut dz = -radius;

        while dz <= radius {
            let mut dy = -radius;
            while dy <= radius {
                let mut dx = -radius;
                while dx <= radius {
                    if self.includes(dx, dy, dz, radius as u32) {
                        count += 1;
                    }

                    dx += 1;
                }
                dy += 1;
            }
            dz += 1;
        }

        count
    }
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
    0, 5, 18, 43, 84, 145, 230, 343, 488, 669, 890, 1155, 1468, 1833, 2254, 2735, 3280, 3893, 4578,
    5339, 6180, 7105, 8118, 9223, 10424, 11725, 13130, 14643, 16268, 18009, 19870, 21855, 23968,
    26213, 28594, 31115, 33780, 36593, 39558, 42679, 45960, 49405, 53018, 56803, 60764, 64905,
    69230, 73743, 78448, 83349, 88450, 93755, 99268, 104993, 110934, 117095, 123480, 130093,
    136938, 144019, 151340, 158905, 166718, 174783, 183104, 191685, 200530, 209643, 219028, 228689,
    238630, 248855, 259368, 270173, 281274, 292675, 304380, 316393, 328718, 341359, 354320, 367605,
    381218, 395163, 409444, 424065, 439030, 454343, 470008, 486029, 502410, 519155, 536268, 553753,
    571614, 589855, 608480, 627493, 646898, 666699, 686900,
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

/// Declarative neighborhood falloff strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodFalloff {
    /// Every included offset has uniform influence.
    Uniform,
    /// Weight falls off as inverse squared distance.
    InverseSquare,
}

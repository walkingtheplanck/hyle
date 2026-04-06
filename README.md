# Hyle

A general-purpose **3D cellular automaton library** in Rust.

No physics knowledge required. Define rules, register them, run the simulation.

---

## Core concept

```rust
use hyle::{CaWorld, Neighborhood, Action, Cell};

// A cell is just a u32. The CA knows nothing about what it means.
const ALIVE: Cell = 1;
const DEAD:  Cell = 0;

// A rule is a plain function — no allocation, no trait objects.
fn born_rule(n: Neighborhood, _rng: u32) -> Action {
    if n.count_alive() == 3 { Action::Become(ALIVE) } else { Action::Keep }
}

fn survive_rule(n: Neighborhood, _rng: u32) -> Action {
    match n.count_alive() {
        2 | 3 => Action::Keep,
        _ => Action::Become(DEAD),
    }
}

let mut world = CaWorld::new(64, 64, 64);
world.register_rule(DEAD  as u8, born_rule);
world.register_rule(ALIVE as u8, survive_rule);
world.step();
```

---

## API

```rust
// Cell is opaque — pack anything into a u32
type Cell = u32;

// The 26 neighbors in a 3×3×3 Moore neighborhood
struct Neighborhood {
    center: Cell,
    neighbors: [Cell; 26],
}

impl Neighborhood {
    fn get(&self, dx: i32, dy: i32, dz: i32) -> Cell;  // access by offset
    fn count(&self, pred: impl Fn(Cell) -> bool) -> u32;
    fn count_alive(&self) -> u32;                       // count non-zero neighbors
}

// What a rule returns
enum Action {
    Keep,               // leave center cell unchanged
    Become(Cell),       // replace center with new value
    Swap(Direction),    // exchange with a face neighbor
    Set(Direction, Cell), // overwrite a face neighbor
}

// Per-cell deterministic RNG — use for probabilistic rules
fn cell_rng(x: u32, y: u32, z: u32, step: u32) -> u32;
```

---

## Viewer demo — 3D Conway's Game of Life

```
cargo run --release -p hyle-viewer
```

Renders a 64×64×64 3D GoL simulation with GPU raytracing.

**Controls:**

| Input | Action |
|---|---|
| Right-drag | Orbit camera |
| Middle-drag | Pan |
| Scroll | Zoom |
| WASD / QE | Move camera target |
| R | Reset simulation |
| Tab | Toggle mouse capture |

---

## Crates

| Crate | Description |
|---|---|
| `hyle` | Re-exports `hyle-ca` — use this as the entry point |
| `hyle-ca` | Core CA engine: `CaWorld`, `Neighborhood`, `Action`, `Rule` |
| `hyle-core` | Voxel/material types used by the viewer renderer |
| `hyle-viewer` | Interactive 3D viewer with GPU raytracing |

---

## License

MIT

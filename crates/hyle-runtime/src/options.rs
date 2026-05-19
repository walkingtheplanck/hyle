#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadOptions {
    /// Crashes the solver if any var goes out of bounds
    pub bound_checks: bool,
}

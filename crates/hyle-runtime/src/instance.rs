/// A backend-owned simulation instance handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instance {
    /// Logical module name for debugging and inspection.
    pub module_name: String,
    /// Number of logical steps executed.
    pub steps: u64,
}

impl Instance {
    /// Creates a new instance handle.
    pub fn new(module_name: impl Into<String>) -> Self {
        Self {
            module_name: module_name.into(),
            steps: 0,
        }
    }

    /// Records one logical step.
    pub fn advance(&mut self) {
        self.steps += 1;
    }
}

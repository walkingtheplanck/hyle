use hyle_runtime::{IOHandler, Instance, LoadOptions, RuntimeError, Solver};
use hyle_sole::{decode_sole_json_bytes, SoleModule};

/// Placeholder GPU solver used to validate runtime wiring.
#[derive(Default)]
pub struct GpuSolver;

impl Solver for GpuSolver {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn load(&self, sole: &[u8], _options: LoadOptions) -> Result<Box<dyn Instance>, RuntimeError> {
        let module = decode_sole_json_bytes(sole)
            .map_err(|error| RuntimeError::ModuleLoad(error.to_string()))?;
        Ok(Box::new(GpuInstance { module, steps: 0 }))
    }
}

struct GpuInstance {
    module: SoleModule,
    steps: u64,
}

impl Instance for GpuInstance {
    fn step(&mut self) -> Result<(), RuntimeError> {
        let _ = &self.module;
        self.steps += 1;
        Ok(())
    }

    fn field(
        &mut self,
        model_name: &str,
        field_name: &str,
    ) -> Result<Box<dyn IOHandler>, RuntimeError> {
        Err(RuntimeError::FieldAccess(format!(
            "gpu field access is not implemented for `{model_name}.{field_name}`"
        )))
    }

    fn input(&mut self, input_name: &str) -> Result<Box<dyn IOHandler>, RuntimeError> {
        Err(RuntimeError::FieldAccess(format!(
            "gpu input access is not implemented for `{input_name}`"
        )))
    }
}

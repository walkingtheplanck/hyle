use hyle_runtime::{IOHandler, Instance, LoadOptions, RuntimeError, Solver};
use hyle_sole::{decode_sole_json_bytes, SoleModule};

/// Placeholder CPU solver used to validate runtime wiring.
#[derive(Default)]
pub struct CpuSolver;

impl Solver for CpuSolver {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn load(&self, sole: &[u8], _options: LoadOptions) -> Result<Box<dyn Instance>, RuntimeError> {
        let module = decode_sole_json_bytes(sole)
            .map_err(|error| RuntimeError::ModuleLoad(error.to_string()))?;
        Ok(Box::new(CpuInstance { module, steps: 0 }))
    }
}

struct CpuInstance {
    module: SoleModule,
    steps: u64,
}

impl Instance for CpuInstance {
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
            "cpu field access is not implemented for `{model_name}.{field_name}`"
        )))
    }

    fn input(&mut self, input_name: &str) -> Result<Box<dyn IOHandler>, RuntimeError> {
        Err(RuntimeError::FieldAccess(format!(
            "cpu input access is not implemented for `{input_name}`"
        )))
    }
}

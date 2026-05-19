use crate::io_handler::IOHandler;
use crate::options::LoadOptions;
use crate::RuntimeError;

pub trait Solver: Send + Sync {
    fn name(&self) -> &'static str;

    fn load(&self, sole: &[u8], options: LoadOptions) -> Result<Box<dyn Instance>, RuntimeError>;
}

pub trait Instance: Send {
    fn step(&mut self) -> Result<(), RuntimeError>;

    fn field(
        &mut self,
        model_name: &str,
        field_name: &str,
    ) -> Result<Box<dyn IOHandler>, RuntimeError>;

    fn input(&mut self, input_name: &str) -> Result<Box<dyn IOHandler>, RuntimeError>;
}

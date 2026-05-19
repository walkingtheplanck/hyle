use crate::RuntimeError;

pub trait IOHandler: Send {
    fn set_bool(&mut self, name: &str, value: bool) -> Result<(), RuntimeError>;
    fn get_bool(&self, name: &str) -> Result<bool, RuntimeError>;

    fn set_i32(&mut self, name: &str, value: i32) -> Result<(), RuntimeError>;
    fn get_i32(&self, name: &str) -> Result<i32, RuntimeError>;

    fn set_u32(&mut self, name: &str, value: u32) -> Result<(), RuntimeError>;
    fn get_u32(&self, name: &str) -> Result<u32, RuntimeError>;

    fn set_f32(&mut self, name: &str, value: f32) -> Result<(), RuntimeError>;
    fn get_f32(&self, name: &str) -> Result<f32, RuntimeError>;

    fn set_f64(&mut self, name: &str, value: f64) -> Result<(), RuntimeError>;
    fn get_f64(&self, name: &str) -> Result<f64, RuntimeError>;

    fn set_u64(&mut self, name: &str, value: u64) -> Result<(), RuntimeError>;
    fn get_u64(&self, name: &str) -> Result<u64, RuntimeError>;

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), RuntimeError>;
    fn get_i64(&self, name: &str) -> Result<i64, RuntimeError>;
}

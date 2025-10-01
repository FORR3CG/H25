use std::any::Any;

pub trait Ferd {
    fn as_any(&self) -> &dyn Any;
    fn id(&self) -> u32;
    fn set_fjoldi_bokadra(&mut self, fjoldi: u16) -> Result<(), String>;
}
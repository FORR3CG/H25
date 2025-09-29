use std::{any::Any, fmt::Display};

pub trait Farartaeki: Display {
    fn id(&self) -> u32;
    fn verd(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

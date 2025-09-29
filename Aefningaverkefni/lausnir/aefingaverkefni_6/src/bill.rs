use crate::farartaeki::Farartaeki;
use std::{any::Any, fmt::Display};

pub struct Bill {
    id: u32,
    tegund: String,
    verd: u32,
}

impl Bill {
    pub fn new(id: u32, tegund: &str, verd: u32) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
            verd,
        }
    }
}

impl Farartaeki for Bill {
    fn id(&self) -> u32 {
        self.id
    }

    fn verd(&self) -> u32 {
        self.verd
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bíll, id: {}, tegund: {}, verð: {} kr.",
            self.id, self.tegund, self.verd
        )
    }
}

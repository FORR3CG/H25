
use std::fmt::Display;

use crate::tegund::Tegund;

pub struct Bill {
    id: u32,
    gerd: String,
    tegund: Tegund,
    verd: u32,
}

impl Bill {
    pub fn new(id: u32, gerd: &str, tegund: &str, verd: u32) -> Self {
        Self {
            id,
            gerd: String::from(gerd),
            tegund: Tegund::from(tegund),
            verd,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn verd(&self) -> u32 {
        self.verd
    }

    pub fn set_verd(&mut self, nytt_verd: u32) {
        self.verd = nytt_verd
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nr: {}, gerð: {}, tegund: {}, verð: {} kr.", 
                    self.id, self.gerd, self.tegund, self.verd)
    }
}
use std::{any::Any, fmt::Display};

use crate::farartaeki::Farartaeki;

pub struct Batur {
    id: u32,
    fjoldi_farthega: u16,
    verd: u32,
}

impl Batur {
    pub fn new(id: u32, fjoldi_farthega: u16, verd: u32) -> Self {
        Self {
            id,
            fjoldi_farthega,
            verd,
        }
    }
}

impl Farartaeki for Batur {
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

impl Display for Batur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bátur, id {}, fjöldi farþega: {}, verð: {} kr.",
            self.id, self.fjoldi_farthega, self.verd
        )
    }
}

use std::{any::Any, fmt::Display};

use crate::{farartaeki::Farartaeki, soluvara::Soluvara};

pub struct Batur {
    soluvara: Soluvara,
    fjoldi_farthega: u16,
}

impl Batur {
    pub fn new(soluvara: Soluvara, fjoldi_farthega: u16) -> Self {
        Self {
            soluvara,
            fjoldi_farthega,
        }
    }
}

impl Farartaeki for Batur {
    fn id(&self) -> u32 {
        self.soluvara.id()
    }

    fn verd(&self) -> u32 {
        self.soluvara.verd()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Batur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bátur, fjöldi farþega: {}, {}",
            self.fjoldi_farthega, self.soluvara
        )
    }
}

impl TryFrom<&str> for Batur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 3 {
            Err("Ekki nógu margir liðir til að bua til bát!".to_string())
        } else if let Ok(fjoldi_farthega) = lidir[2].parse::<u16>() {
            Ok(Self {
                soluvara: Soluvara::try_from(lidir[..2].join(" ").as_str())?,
                fjoldi_farthega,
            })
        } else {
            Err(format!("Gat ekki breytt '{}' í fjölda farþega!", lidir[2]))
        }
        
    }
}

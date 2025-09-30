use crate::{farartaeki::Farartaeki, soluvara::Soluvara};
use std::{any::Any, fmt::Display};

pub struct Bill {
    soluvara: Soluvara,
    tegund: String,
}

impl Bill {
    pub fn new(soluvara: Soluvara, tegund: &str) -> Self {
        Self {
            soluvara,
            tegund: tegund.to_string(),
        }
    }
}

impl Farartaeki for Bill {
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

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bíll, tegund: {} {}",
            self.tegund, self.soluvara
        )
    }
}

impl TryFrom<&str> for Bill {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 3 {
            Err("Ekki nógu margir liðir til að bua til bíl!".to_string())
        } else {
            Ok(Self {
                soluvara: Soluvara::try_from(lidir[..2].join(" ").as_str())?,
                tegund: lidir[2].to_string(),
            })
        }
        
    }
}

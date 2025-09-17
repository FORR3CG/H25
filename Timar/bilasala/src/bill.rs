
use std::fmt::Display;

use crate::tegund::{self, Tegund};

pub struct Bill {
    id: u32,
    gerd: String,
    tegund: Tegund,
    verd: u32,
}

impl Bill {
    pub fn new(id: u32, gerd: &str, tegund: &str, verd: u32) -> Result<Self, String> {
        Ok(Self {
            id,
            gerd: String::from(gerd),
            tegund: Tegund::try_from(tegund)?,
            verd,
        })
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

impl TryFrom<&str> for Bill {
    type Error = String;
    //   id gerð  tegund verð
    // value = "101 Volvo    j   2000"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 4 {
            Err(format!("Ekki nægur fjöldi orða í '{}' til að búa til bíl!", value))
        } else {
            let str_2_u32 = |str_tala: &str| -> Result<u32, String> {
                match str_tala.parse::<u32>() {
                    Ok(tala) => Ok(tala),
                    Err(_) => Err(format!("Gat ekki breytt '{}' í tölu!", str_tala))
                }
            };

            Ok(Self {
                id: str_2_u32(lidir[0])?,
                gerd: lidir[1].to_string(),
                tegund: Tegund::try_from(lidir[2])?,
                verd: str_2_u32(lidir[3])?,
            })           
        }

    }
}
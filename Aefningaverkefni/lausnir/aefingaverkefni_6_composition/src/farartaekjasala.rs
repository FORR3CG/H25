use std::fmt::Display;

use crate::batur::Batur;
use crate::bill::Bill;
use crate::farartaeki::Farartaeki;

pub struct Farartaekjasala {
    farartaeki: Vec<Box<dyn Farartaeki>>,
}

impl Farartaekjasala {
    pub fn new() -> Self {
        Self {
            farartaeki: Vec::new(),
        }
    }

    fn skra_farartaeki(&mut self, farartaeki: Box<dyn Farartaeki>) {
        self.farartaeki.push(farartaeki);
    }

    pub fn skra_bil(&mut self, bill: &str) -> Result<(), String>{
        Ok(self.skra_farartaeki(Box::new(Bill::try_from(bill)?)))
    }

    pub fn skra_bat(&mut self, batur: &str) -> Result<(), String>{
        Ok(self.skra_farartaeki(Box::new(Batur::try_from(batur)?)))
    }

    pub fn verdmaeti_allra_farartaekja(&self) -> u32 {
        self.farartaeki
            .iter()
            .fold(0, |summa, f| summa + f.verd())
    }

    pub fn skoda_alla_bila(&self) -> String {
        self.farartaeki
            .iter()
            .filter(|f| f.as_any().downcast_ref::<Bill>().is_some())
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn skoda_alla_bata(&self) -> String {
        let mut batar = String::new();
        for f in &self.farartaeki {
            if let Some(batur) = f.as_any().downcast_ref::<Batur>() {
                batar += format!("{}\n", batur).as_str()
            }
        }
        batar.trim().to_string()
    }

    pub fn skoda_farartaeki_med_id(&self, id: u32) -> Option<String> {
        self.farartaeki
            .iter()
            .find(|f| f.id() == id)
            .map(|farartaeki| farartaeki.to_string())
        // eða 
        /*if let Some(farartaeki) = self.farartaeki.iter().find(|f| f.id() == id) {
            Some(farartaeki.to_string())
        } else {
            None
        } */
    }
}

impl Display for Farartaekjasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.farartaeki
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

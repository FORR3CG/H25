use std::fmt::Display;

use crate::{bill::Bill, tegund};

pub struct Bilasala {
    bilar: Vec<Bill>,
}

impl Bilasala {
    pub fn new() -> Self {
        Self {
            bilar: Vec::new(),
        }
    }

    pub fn skra_bil(&mut self, id: u32, gerd: &str, tegund: &str, verd: u32) {
        self.bilar.push(Bill::new(id, gerd, tegund, verd));
    }

    pub fn skoda_bil_med_id(&self, id: u32) {
        for bill in &self.bilar {
            if bill.id() == id {
                println!("{}", bill);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id)
    }

    pub fn eyda_bil_med_id(&mut self, id: u32) {
        for idx in 0..(self.bilar.len()) {
            if self.bilar[idx].id() == id {
                self.bilar.remove(idx);
                println!("Eyddi bíl nr: {}", id);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id)
    }

    pub fn breyta_verdi_a_bil_med_id(&mut self, id: u32, nytt_verd: u32) {
        for bill in &mut self.bilar {
            if bill.id() == id {
                bill.set_verd(nytt_verd);
                println!("Breytti verði á bíl nr: {}", id);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id)
    }

    pub fn prenta_verd_allra_bila(&self) {
        let mut heildarverd = 0_u64;
        for bill in &self.bilar {
            heildarverd += bill.verd() as u64;
        }
        println!("Heildarverð: {}, meðalverð: {:.2}", 
                    heildarverd, heildarverd as f32 / self.bilar.len() as f32)
    }
}

impl Display for Bilasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut listinn = String::new();
        for bill in &self.bilar {
            listinn.push_str(format!("{}\n", bill).as_str());
        }
        write!(f, "{}", listinn.trim())
    }
}
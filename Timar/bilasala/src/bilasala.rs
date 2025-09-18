use std::fmt::Display;

use crate::{bill::Bill, tegund};

pub struct Bilasala {
    bilar: Vec<Bill>,
    id: u32,
}

impl Bilasala {
    pub fn new() -> Self {
        Self {
            bilar: Vec::new(),
            id: 1000,
        }
    }

    fn next_id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    pub fn skra(&mut self, bilastrengur: &str) -> Result<(), String> {
        // "Volvo j 3000" => "103 Volvo j 3000"
        let bill = format!("{} {}", self.next_id(), bilastrengur);
        self.bilar.push(Bill::try_from(bill.as_str())?);
        // self.bilar.sort_by_key(|bill| bill.verd()); // lægsta verð fyrst    
        self.bilar.sort_by(|a, b| b.verd().cmp(&a.verd())); // hæsta verð fyrst
        Ok(())
    }

    pub fn skra_bil(&mut self, id: u32, gerd: &str, tegund: &str, verd: u32) -> Result<(), String> {
        Ok(self.bilar.push(Bill::new(id, gerd, tegund, verd)?))
    }

    pub fn skoda_bil_med_id(&self, id: u32) -> Result<String, String> {
        if let Some(b) = self.bilar.iter().find(|bill| bill.id() == id) {
            Ok(b.to_string())
        } else {
            Err(format!("Fann engan bíl með id: {}", id))
        }
        
        /* for bill in &self.bilar {
            if bill.id() == id {
                println!("{}", bill);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id) */
    }

    pub fn eyda_bil_med_id(&mut self, id: u32) -> Result<(), String> {
        if let Some(idx) = self.bilar
                                      .iter()
                                      .position(|bill| bill.id() == id) {
            self.bilar.remove(idx);
            Ok(())
        } else {
            Err(format!("Fann engan bíl með id: {}", id))
        }
/*         for idx in 0..(self.bilar.len()) {
            if self.bilar[idx].id() == id {
                self.bilar.remove(idx);
                println!("Eyddi bíl nr: {}", id);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id) */
    }

    pub fn breyta_verdi_a_bil_med_id(&mut self, id: u32, nytt_verd: u32) -> Result<(), String> {
        if let Some(b) = self.bilar.iter_mut().find(|bill| bill.id() == id) {
            b.set_verd(nytt_verd);
            Ok(())
        } else {
            Err(format!("Fann engan bíl með id: {}", id))
        }
        /* for bill in &mut self.bilar {
            if bill.id() == id {
                bill.set_verd(nytt_verd);
                println!("Breytti verði á bíl nr: {}", id);
                return;
            }
        }
        println!("Fann engan bíl með id: {}", id) */
    }

    pub fn prenta_verd_allra_bila(&self) -> u64 {
        self.bilar
            .iter()           // heildarverd = heildarverd + bill.verd()
            .fold(0, |heildarverd, bill| heildarverd + bill.verd() as u64)
/*         let mut heildarverd = 0_u64;
        for bill in &self.bilar {
            heildarverd += bill.verd() as u64;
        }
        println!("Heildarverð: {}, meðalverð: {:.2}", 
                    heildarverd, heildarverd as f32 / self.bilar.len() as f32) */
    }
}

impl Display for Bilasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*         for bill in &self.bilar {
            listinn.push_str(format!("{}\n", bill).as_str());
            }
            write!(f, "{}", listinn.trim()) */
        let listinn = self.bilar
                                     .iter()
                                     .map(|bill| bill.to_string())
                                     .collect::<Vec<String>>()
                                     .join("\n");
        writeln!(f, "{}", listinn)
    }
}
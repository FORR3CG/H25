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
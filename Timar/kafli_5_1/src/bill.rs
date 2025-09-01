use std::fmt::Display;

// unitstruct
pub struct Villumelding;

pub struct Reikniadgerdir;

impl Reikniadgerdir {
    pub fn leggja_saman(x: i32, y: i32) -> i32 {
        x + y
    }
}

// tuple struct
#[derive(Debug)]
pub struct Litur(u8, u8, u8);

impl Litur {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn rautt(&self) -> u8 {
        self.0
    }
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}", self.0, self.1, self.2)
    }
}

// venjulegt struct
#[derive(Debug)]
pub struct Bill {
    nr: u32,
    tegund: String,
    pub litur: Litur,
}

impl Bill {
    pub fn new(nr: u32, tegund: &str, r: u8, g: u8, b: u8) -> Self {
        Self {
            nr,
            tegund: tegund.to_string(),
            litur: Litur::new(r, g, b),
        }
    }

    pub fn get_nr(&self) -> u32 {
        self.nr
    }

    pub fn set_tegund(&mut self, ny_tegund: &str) {
        let k = Reikniadgerdir::leggja_saman(10, 20);
        self.tegund = ny_tegund.to_string()
    }

    pub fn prenta(&self) {
        println!("nr: {}, tegund: {}, litur: {:?}", self.nr, self.tegund, self.litur)
    }

    pub fn prenta_tekur_ownership(self) -> Self {
        self.prenta();
        self
    }

}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bíll, nr: {}, tegund: {}, litur: {}", 
                        self.nr, self.tegund, self.litur)
    }
}
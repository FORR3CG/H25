// unitstruct
pub struct Villumelding;

// tuple struct
#[derive(Debug)]
pub struct Litur(u8, u8, u8);

impl Litur {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
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
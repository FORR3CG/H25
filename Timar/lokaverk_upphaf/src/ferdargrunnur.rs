pub struct FerdarGrunnur {
    id: u32,
    fjoldi_bokadra: u16,
    hamarksfjoldi: u16,
}

impl FerdarGrunnur {
    // pub new() kannski??

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_fjoldi_bokadra(&mut self, fjoldi: u16) -> Result<(), String> {
        if fjoldi > self.hamarksfjoldi {
            Err("Of margir bókaðir!!".to_string())
        } else {
            self.fjoldi_bokadra = fjoldi;
            Ok(())
        }
    }
}
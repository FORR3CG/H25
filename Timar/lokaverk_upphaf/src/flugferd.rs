use crate::ferdargrunnur::FerdarGrunnur;
use crate::ferd::Ferd;
use std::any::Any;

pub struct Flugferd {
    ferd: FerdarGrunnur, // id, fjoldi_bokadra, hamarksfjoldi
    afangastadur: String,
}

impl Flugferd {
    //
}

impl Ferd for Flugferd {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        self.ferd.id()
    }

    fn set_fjoldi_bokadra(&mut self, fjoldi: u16) -> Result<(), String> {
        Ok(self.ferd.set_fjoldi_bokadra(fjoldi)?)
    }
}
use std::fmt::Display;

pub enum Tegund {
    Folksbill,
    Jeppi,
    Annad,
}

impl Display for Tegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tegund::Folksbill => write!(f, "Fólksbíll"),
            Tegund::Jeppi => write!(f, "Jeppi"),
            Tegund::Annad => write!(f, "Annað"),
        }
    }
}
// FB\n fb => Tegund::Folksbill
impl From<&str> for Tegund {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "f" | "fb" | "folks" => Tegund::Folksbill,
            "j" | "jeppi" => Tegund::Jeppi,
            _ => Tegund::Annad,
        }
    }
}
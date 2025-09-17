use std::fmt::Display;

pub enum Tegund {
    Folksbill,
    Jeppi,
    Vorubill,
}

impl Display for Tegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tegund::Folksbill => write!(f, "Fólksbíll"),
            Tegund::Jeppi => write!(f, "Jeppi"),
            Tegund::Vorubill => write!(f, "Vörubíll"),
        }
    }
}
// FB\n fb => Tegund::Folksbill
/* impl From<&str> for Tegund {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "f" | "fb" | "folks" => Tegund::Folksbill,
            "j" | "jeppi" => Tegund::Jeppi,
            "v" => Tegund::Vorubill,
            _ => 
        }
    }
} */

impl TryFrom<&str> for Tegund {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "f" | "fb" => Ok(Tegund::Folksbill),
            "j" => Ok(Tegund::Jeppi),
            "v" => Ok(Tegund::Vorubill),
            _ => Err(format!("Gat ekki breytt '{}' í tegund!", value)),
        }
    }
}
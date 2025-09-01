use std::fmt::Display;

pub enum Stada {
    Leigdur,
    Laus,
    EkkiTilLeigu,
    Othekkt,
}

impl Display for Stada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stada = match self {
            Stada::Leigdur => "Leigður",
            Stada::Laus => "Laus",
            Stada::EkkiTilLeigu => "Ekki til útleigu",
            Stada::Othekkt => "Óþekkt",
        };
        write!(f, "{}", stada)
    }
}

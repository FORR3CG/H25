struct Hundur {
    nafn: String,
    einkunn: u32,
}

impl Hundur {
    fn new(nafn: &str, einkunn: u32) -> Self {
        Self {
            nafn: nafn.to_string(),
            einkunn
        }
    }
}

impl Display for Hundur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hundur, nafn: {}, einkunn: {}", self.nafn, self.einkunn)
    }
}

impl Display for Kottur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Köttur: nafn: {}, aldur: {}", self.nafn, self.aldur)
    }
}

impl Dyr for Hundur {
    fn prenta_nafn(&self) {
        println!("Hundurinn {}", self.nafn)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn tegund(&self) -> String {
        "Hundur".to_string()
    }
}

struct Kottur {
    nafn: String,
    aldur: u8,
}

impl Dyr for Kottur {
    fn prenta_nafn(&self) {
        println!("Kötturinn {}", self.nafn)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn tegund(&self) -> String {
        "Köttur".to_string()
    }
}

impl Kottur {
    fn new(nafn: &str, aldur: u8) -> Self {
        Self {
            nafn: nafn.to_string(),
            aldur,
        }
    }
}

use std::{any::Any, fmt::Display};

trait Dyr : Display {
    fn prenta_nafn(&self);
    fn as_any(&self) -> &dyn Any;
    fn tegund(&self) -> String;
}

struct Dyragardur {
    dyrin: Vec<Box<dyn Dyr>>,
}

impl Dyragardur {
    fn new() -> Self {
        Self {
            dyrin: Vec::new(),
        }
    }

    fn skra_hund(&mut self, nafn: &str, einkunn: u32) {
        self.skra_dyr(Box::new(Hundur::new(nafn, einkunn)));
    }

    fn skra_kott(&mut self, nafn: &str, aldur: u8) {
        self.skra_dyr(Box::new(Kottur::new(nafn, aldur)));
    }

    fn prenta_allt(&self) {
        for d in &self.dyrin {
            //d.prenta_nafn()
            println!("{}", d)
        }
    }

    fn skra_dyr(&mut self, dyr: Box::<dyn Dyr>) {
        self.dyrin.push(dyr);
    }

    fn prenta_alla_ketti(&self) {
        for d in &self.dyrin {
            if let Some(kottur) = d.as_any().downcast_ref::<Kottur>() {
                //println!("Köttur, nafn: {}, aldur: {}", kottur.nafn, kottur.aldur)
                println!("{}", kottur)
            }
        }
    }

    fn prenta_alla_hunda(&self) {
        for d in &self.dyrin {
            match d.as_any().downcast_ref::<Hundur>() {
                Some(hundur) 
                    //=> println!("Hundur, nafn: {}, einkunn: {}", hundur.nafn, hundur.einkunn),
                    => println!("{}", hundur),
                None => println!("Hérna er köttur en ekki hundur!!!"), // eða () eða nota sömu aðferð og í prenta_alla_ketti
            }
        }
    }

    fn prenta_tegund(&self, tegund: &str) {
        for d in &self.dyrin {
            if d.tegund() == tegund {
                println!("{}", d)
            }
        }
    }
}

impl Display for Dyragardur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, 
                 "{}",
                 self.dyrin
                 .iter()
                 .map(|dyr| dyr.to_string())
                 .collect::<Vec<String>>()
                 .join("\n")
                )
    }
}

fn main() {
    let h = Hundur::new("Snati", 8);
    //let k = Kottur::new("Grettir", 9);
    let mut dg = Dyragardur::new();
    dg.skra_dyr(Box::new(h));
    dg.skra_dyr(Box::new(Kottur::new("Grettir", 9)));
    dg.skra_dyr(Box::new(Kottur::new("Snotra", 3)));
    dg.skra_dyr(Box::new(Hundur::new("Assa", 7)));
    dg.skra_hund("Lotta", 3);
    dg.skra_kott("Brandur", 12);
    dg.prenta_allt();
    println!("----------------------------------------------");
    dg.prenta_alla_ketti();
    println!("----------------------------------------------");
    dg.prenta_alla_hunda();
    println!("----------------------------------------------");
    println!("{}", dg);
    println!("----------------------------------------------");
    dg.prenta_tegund("Köttur");
    //let k = Box::new(23);
    //let j = Box::new(100_u128);
    //println!("Hello, world!");
}

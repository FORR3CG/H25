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

impl Dyr for Hundur {
    fn prenta_nafn(&self) {
        println!("Hundurinn {}", self.nafn)
    }

    fn as_any(&self) -> &dyn Any {
        self
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
}

impl Kottur {
    fn new(nafn: &str, aldur: u8) -> Self {
        Self {
            nafn: nafn.to_string(),
            aldur,
        }
    }
}

use std::any::Any;

trait Dyr {
    fn prenta_nafn(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Dyragardur {
    dyrin: Vec<Box<dyn Dyr>>,
}

fn main() {
    let k = Box::new(23);
    let j = Box::new(100_u128);
    println!("Hello, world!");
}

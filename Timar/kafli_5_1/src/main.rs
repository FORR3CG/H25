mod bill;
use crate::bill::{Bill, Litur};


fn main() {
    //let b = Bill { nr: 123, tegund: "Toyota".to_string(), litur: "Grænn".to_string()};
    let mut b2 = Bill::new(124, "Honda", 123, 45, 0);
    let k = 10;
    // b2.nr = 1000;
    b2.litur = Litur::new(10, 20, 30);
    println!("Bíllinn er {}", b2.get_nr());
    println!("{:#?}", b2);
    b2.prenta();
    b2 = b2.prenta_tekur_ownership();
    //b2.litur = "Grænn".to_string();
}

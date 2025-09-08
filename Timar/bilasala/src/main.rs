mod bill;
mod tegund;
mod bilasala;

use crate::bill::Bill;
use bilasala::Bilasala;

fn main() {
    //let b = Bill::new(101, "Toyota", "fb", 1000);
    let mut bs = Bilasala::new();
    bs.skra_bil(101, "Toyota", "fb", 1000);
    bs.skra_bil(102, "Nissan", "j", 1000);
    bs.skra_bil(103, "Scania", "vörubíll", 1000);
    println!("{}", bs);
}

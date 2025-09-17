mod bill;
mod tegund;
mod bilasala;

use crate::bill::Bill;
use bilasala::Bilasala;

fn main() {
    //let b = Bill::new(101, "Toyota", "fb", 1000);
    // "101 Volvo j 2000"
    // skrá Toyota fb 1000 
    // selja 101
    // breyta 101 2000 
    let mut bs = Bilasala::new();
    if let Err(e) = bs.skra("Nissan fb 2000") {
        println!("{}", e)
    }
    bs.skra("Komatsu v 5000");
    println!("{}", bs);
    if let Err(e) = bs.skra_bil(101, "Toyota", "bátur", 1000) {
        println!("{}", e)
    } else {
        println!("skráði bíl")
    }
    if let Err(e) = Bill::try_from("102 Tesla abc geir") {
        println!("Villa: {}", e)
    } else {
        println!("Bjó til bíl")
    }
    bs.skra_bil(102, "Nissan", "j", 1000);
    bs.skra_bil(103, "Scania", "vörubíll", 1000);
    bs.skoda_bil_med_id(101);
    bs.skoda_bil_med_id(999);
    bs.eyda_bil_med_id(999);
    bs.eyda_bil_med_id(101);
    println!("{}", bs);
    bs.breyta_verdi_a_bil_med_id(103, 5000);
    bs.prenta_verd_allra_bila();
}

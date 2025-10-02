mod batur;
mod bill;
mod farartaeki;
mod farartaekjasala;
mod soluvara;

use farartaekjasala::Farartaekjasala;

fn main() {
    let mut fs = Farartaekjasala::new();
    // id verð tegund
    fs.skra_bil("geir 2000 Volvo").map_err(|villa| println!("{}", villa));
    fs.skra_bil("101 2500 Toyota").map_err(|villa| println!("{}", villa));
    fs.skra_bil("102 1500 Nizzan").map_err(|villa| println!("{}", villa));
    // id verð fjöldi_farþega
    fs.skra_bat("103 5000 5" ).map_err(|villa| println!("{}", villa));
    fs.skra_bat("104 10000 7").map_err(|villa| println!("{}", villa));
    fs.skra_bat("105 4000 3").map_err(|villa| println!("{}", villa));
    println!("{}", fs);
    // Bíll, tegund: Volvo id: 100, verð: 2000 kr.
    // Bíll, tegund: Toyota id: 101, verð: 2500 kr.
    // Bíll, tegund: Nizzan id: 102, verð: 1500 kr.
    // Bátur, fjöldi farþega: 5, id: 103, verð: 5000 kr.
    // Bátur, fjöldi farþega: 7, id: 104, verð: 10000 kr.
    // Bátur, fjöldi farþega: 3, id: 105, verð: 4000 kr.
    println!("----------------------------------");
    println!("Verðmæti: {}", fs.verdmaeti_allra_farartaekja());
    // Verðmæti: 25000
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bata());
    // Bátur, fjöldi farþega: 5, id: 103, verð: 5000 kr.
    // Bátur, fjöldi farþega: 7, id: 104, verð: 10000 kr.
    // Bátur, fjöldi farþega: 3, id: 105, verð: 4000 kr.
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bila());
    // Bíll, tegund: Volvo id: 100, verð: 2000 kr.
    // Bíll, tegund: Toyota id: 101, verð: 2500 kr.
    // Bíll, tegund: Nizzan id: 102, verð: 1500 kr.
    println!("----------------------------------");
    if let Some(farartaeki) = fs.skoda_farartaeki_med_id(103) {
        println!("{}", farartaeki)
    } else {
        println!("Fann ekki farartæki með þessu id!")
    }
    if let Some(farartaeki) = fs.skoda_farartaeki_med_id(999) {
        println!("{}", farartaeki)
    } else {
        println!("Fann ekki farartæki með þessu id!")
    }
    // Bátur, fjöldi farþega: 5, id: 103, verð: 5000 kr.
    // Fann ekki farartæki með þessu id!
}

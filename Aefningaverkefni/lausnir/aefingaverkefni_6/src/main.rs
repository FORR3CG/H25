mod batur;
mod bill;
mod farartaeki;
mod farartaekjasala;

use farartaekjasala::Farartaekjasala;

fn main() {
    let mut fs = Farartaekjasala::new();
    fs.skra_bil(100, "Volvo", 2000);
    fs.skra_bil(101, "Toyota", 2500);
    fs.skra_bil(102, "Nizzan", 1500);
    fs.skra_bat(103, 5, 5000);
    fs.skra_bat(104, 7, 10000);
    fs.skra_bat(105, 3, 4000);
    println!("{}", fs);
    // Bíll, id: 100, tegund: Volvo, verð: 2000 kr.
    // Bíll, id: 101, tegund: Toyota, verð: 2500 kr.
    // Bíll, id: 102, tegund: Nizzan, verð: 1500 kr.
    // Bátur, id 103, fjöldi farþega: 5, verð: 5000 kr.
    // Bátur, id 104, fjöldi farþega: 7, verð: 10000 kr.
    // Bátur, id 105, fjöldi farþega: 3, verð: 4000 kr.
    println!("----------------------------------");
    println!("Verðmæti: {}", fs.verdmaeti_allra_farartaekja());
    // Verðmæti: 25000
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bata());
    // Bátur, id 103, fjöldi farþega: 5, verð: 5000 kr.
    // Bátur, id 104, fjöldi farþega: 7, verð: 10000 kr.
    // Bátur, id 105, fjöldi farþega: 3, verð: 4000 kr.
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bila());
    // Bíll, id: 100, tegund: Volvo, verð: 2000 kr.
    // Bíll, id: 101, tegund: Toyota, verð: 2500 kr.
    // Bíll, id: 102, tegund: Nizzan, verð: 1500 kr.
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
    // Bátur, id 103, fjöldi farþega: 5, verð: 5000 kr.
    // Fann ekki farartæki með þessu id!
}

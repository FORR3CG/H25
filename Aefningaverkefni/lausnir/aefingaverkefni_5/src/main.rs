use crate::adflugsstjorn::Adflugsstjorn;

mod adflugsstjorn;
mod flugvel;

fn main() {
    let mut fs = Adflugsstjorn::new();
    fs.skra_flugvel("tf-abc 100 300");
    fs.skra_flugvel("tf-ice 200 500");
    fs.skra_flugvel("aal5 100 50");
    fs.skra_flugvel("tf-efg 100 200");
    println!("{}", fs);
    fs.lata_minutur_lida(40);
    println!("{}", fs)
}

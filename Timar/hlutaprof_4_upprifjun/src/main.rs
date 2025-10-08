/* trait Dyr : Display {
    fn prenta_nafn(&self);
    fn as_any(&self) -> &dyn Any;
    fn tegund(&self) -> String;
    fn hallo(&self) {
        println!("halló")
    }
} */


struct Hundur {
    nafn: String,
    aldur: u8,
}

// Option<T>, Result<T, E>
use std::str::FromStr;

fn str_to_tala<T: FromStr>(texti: &str) -> Result<T, String> {
    match texti.parse::<T>() {
        Ok(tala) => Ok(tala),
        Err(_) => Err(format!("Gat ekki breytt '{}' í tölu", texti))
    }
} 

fn main() {
    // i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    let mut i = 20_u128;
    let f = [1,2,3,4,5];
    let idx = 3;
    println!("{}", f[idx]);
    let c = 'A';
    let c = "A"; // &str

    let mut b = Box::new(23);
    *b += 1;
    let mut f = [1,2,3,4,5];
    for i in &mut f {
        *i += 1
    }
    // loop, while
    //let d: Vec<Box<dyn Dyr>> = Vec::new();

    let k = if true {10} else {30}; // js: let j = true ? 10 : 30;

    let s1 = "strengur";
    let s2 = "strengur".to_string();
    let s3 = s1;
    println!("{}", s1);
    let s4 = &s2;
    println!("{}", s2);

    // python: k = lambda: x + y
    let k = |x, y| x + y;
    let k = k(10, 20);

}

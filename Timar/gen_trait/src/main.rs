/*
enum Resut<T, E> {
    Ok(T),
    Err(E),
}
*/



/*
struct Punktur_i32 {
    x: i32,
    y: i32,
}

struct Punktur_f64 {
    x: f64,
    y: f64
}
*/

struct AnnarPunktur<T, U> {
    x: T,
    y: U,
}

use std::{fmt::Display, ops::Add, str::FromStr};

fn leggja_saman<T>(a: T, b: T) -> T where T: Add<Output = T> {
    a + b
}

fn str_to_tala<T: FromStr>(texti: &str) -> Result<T, String> {
    match texti.parse::<T>() {
        Ok(tala) => Ok(tala),
        Err(_) => Err(format!("Gat ekki breytt '{}' í tölu!", texti))
    }
}

fn prenta_i_hastofum<T: Display>(texti: T) {
    println!("{}", texti.to_string().to_uppercase())
}

struct Hundur {
    nafn: String,
    einkunn: u8,
}

impl Dyrahljod for Hundur {
    fn segir(&self) {
        println!("{} segir voff!", self.nafn)
    }
}

struct Kottur {
    nafn: String,
    eigandi: String,
}

impl Dyrahljod for Kottur {
    fn segir(&self) {
        println!("{} segir mjá!", self.nafn)
    }

    fn hallo(&self) {
        println!("Mjalló!")
    }
}

trait Dyrahljod {
    fn segir(&self);

    fn hallo(&self) {
        println!("Halló!")
    }
}

struct Punktur<T> {
    x: T,
    y: T,
}

impl<T> Punktur<T>  {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T> Add for Punktur<T> where T: Add<Output = T> {
    type Output = Punktur<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct EinfaldurPunktur {
    x: i32,
    y: i32,
}

/*
__add__
*/

impl Add for EinfaldurPunktur {
    type Output = EinfaldurPunktur;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = EinfaldurPunktur{x: 10, y: 20};
    let p2 = EinfaldurPunktur{x: 20, y: 30};
    let p3 = p1 + p2;
    println!("{:?}", p3);
    let h = Hundur { nafn: "Snati".to_string(), einkunn: 9 };
    let k = Kottur { nafn: "Grettir".to_string(), eigandi: "Jón".to_string() };
    h.hallo();
    h.segir();
    k.hallo();
    k.segir();
    let p1 = Punktur::new(10, 20);
    let p2 = Punktur::new(1.2, 2.2);
    let p3: Punktur<u8> = Punktur::new(1, 3);
    let tala = str_to_tala::<i32>("34");
    let tala = str_to_tala::<u32>("34");
    let tala = str_to_tala::<f32>("34");
    let tala: Result<u8, String> = str_to_tala("9");
    leggja_saman(10, 20);
    //leggja_saman("abd", "def");
    //leggja_saman(&p1, &p1);
    println!("Hello, world!");
}

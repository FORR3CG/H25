use std::fmt::Display;

// enums
enum Dagar {
    Manudagur, // 0
    Thridjudagur, // 1
    Midvikudagur,
    Fimmtudagur,
    Fostudagur,
    Laugardagur,
    Sunnudagur,
}

#[derive(Debug)]
enum IPtala {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

impl Display for IPtala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IPtala::IPv4(a, b ,c ,d ) => 
                        write!(f, "{}.{}.{}.{}", a, b, c, d),
            IPtala::IPv6(t) => write!(f, "{}", t),
        }
    }
}

/* enum Option<T> {
    Some(T),
    None,
} */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(a) => Some(a + 1),
        None => None,
    }
}

fn main() {
    let k = None;
    println!("{:?}", plus_one(k));
    let mut j = plus_one(k);
    if let Some(t) = plus_one(k) {
        println!("Fékk töluna {}", t)
    } else {
        println!("Fékk enga tölu!!")
    }
    match j {
        Some(l) => {
            let j = l + 10;
            println!("{}", j)
        },
        None => println!("Fékk ekki tölu"),
    }
    //j += 10;
    let localhost = IPtala::IPv4(127, 0, 0, 1); // 127.0.0.1
    let localhostv6 = IPtala::IPv6("::1".to_string());
    println!("{}\n{}", localhost, localhostv6);
    println!("Hello, world!");
    let t = Dagar::Manudagur;
    match t {
        Dagar::Manudagur => println!("Vikan er að byrja!!"),
        Dagar::Thridjudagur | Dagar::Midvikudagur => println!("Vikan byrjaði í gær"),
        _ => println!("Vikan er langt komin"),
    }
}

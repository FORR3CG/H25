use std::fmt::Display;

// unitstruct
struct Reikniadgerdir;

// tuplestruct
struct Punktur(u32, u32);

// "hefðbundið" struct
#[derive(Debug)]
struct Bill {
    id: u32,
    tegund: String,
    verd: u32,
    litur: Litur,
}

impl Bill {
    fn new(id: u32, tegund: &str, verd: u32, litur: &str) -> Self {
        Self {
            id,
            tegund: String::from(tegund), // tegund.to_string()
            verd,
            litur: Litur::from(litur),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, nytt_id: u32) {
        self.id = nytt_id
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, teg: {}, verð: {} kr., {}"
                    , self.id, self.tegund, self.verd, self.litur)
    }
}

#[derive(Debug)]
enum Litur {
    Gulur, // = 0
    Raudur, // 1
    Graenn, // 2
    Blar, // 3
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litur::Gulur => write!(f, "Gulur litur"),
            Litur::Raudur => write!(f, "Rauður litur"),
            Litur::Graenn => write!(f, "Grænn litur"),
            Litur::Blar => write!(f, "Blár litur"),
        }
    }
}

impl From<&str> for Litur {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "rauður" | "rautt" => Litur::Raudur,
            "gulur" => Litur::Gulur,
            "grænn" => Litur::Graenn,
            _ => Litur::Blar,
        }
    }
}

/*
Option (Some/None)
Result (Ok/Err)
*/

fn main() {
    let v = vec![1,2,3,4];
    if let Some(tala) = v.get(2) {
        println!("talan + 10 er {}", tala + 10);
    }
    match v.get(99) {
        Some(tala) => println!("Talan var: {}", tala),
        None => println!("Engin tala!!"),
    }
    //println!("{}", v.get(2));
    let l = Litur::Graenn;
    let litur_string = l.to_string();
    println!("{:?}", l);
    println!("{}", l);                                              //  geir
    let b = Bill::new(100, "Volvo", 1000, "RauÐuR");
    println!("{:?}", b); // Debug
    println!("{}", b); // Display
}

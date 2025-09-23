mod bill;
mod tegund;
mod bilasala;

use std::io::Write;

use crate::bill::Bill;
use bilasala::Bilasala;

fn main() {
    // hjálp
    // hætta
    // skrá Toyota fb 1000 
    // selja 101
    // breyta 101 2000 
    // prenta verð
    // prenta allt
    // prenta 101
    let mut bs = Bilasala::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Gat ekki flush-að!");
        let mut inntak = String::new();
        std::io::stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá notanda!");
        let skipanir: Vec<&str> = inntak.split_whitespace().collect();
        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().as_str() {
                    "hætta" => break,
                    "hjálp" => println!("Skrifum út hjálpina"),
                    "skrá" => {
                        // skrá 101 Volvo fb 1000
                        if let Err(e) = bs.skra(skipanir[1..].join(" ").as_str()) {
                            println!("{}", e)
                        }
                    },
                    "eyða" | "selja" => {
                        if let Some(id_str) = skipanir.get(1) {
                            if let Ok(id) = id_str.parse::<u32>() {
                                if let Err(e) = bs.eyda_bil_med_id(id) {
                                    println!("{}", e)
                                }
                            } else {
                                println!("Gat ekki breytt {} í tölu!", id_str)
                            }
                        } else {
                            println!("Fann ekkert id!")
                        }
                    },
                    "prenta" => {
                        if skipanir.len() != 2 {
                            println!("Þú verður að segja mér hvað ég á að prenta!")
                        } else {
                            if let Ok(id) = skipanir[1].parse::<u32>() {
                                match bs.skoda_bil_med_id(id) {
                                    Ok(bill) => println!("{}", bill),
                                    Err(villa) => println!("{}", villa),
                                }
                            } else {
                                match skipanir[1] {
                                    "allt" => println!("{}", bs),
                                    "verð" => println!("{}", bs.prenta_verd_allra_bila()),
                                    _ => println!("Kann ekki að prenta '{}'!", skipanir[1]),
                                }
                            }
                        }
                    },
                    "breyta" => {
                        if skipanir.len() != 3 {
                            println!("Ég þarf að fá id og verð!")
                        } else {
                            if let Ok(id) = skipanir[1].parse::<u32>() {
                                if let Ok(verd) = skipanir[2].parse::<u32>() {
                                    if let Err(villa) = bs.breyta_verdi_a_bil_med_id(id, verd) {
                                        println!("{}", villa)
                                    }
                                } else {
                                    println!("Gat ekki breytt '{}' í verð!", skipanir[2])
                                }
                            } else {
                                println!("Gat ekki breytt '{}' í id!", skipanir[1])
                            }
                        }
                    },
                    _ => println!("Skil ekki skipunina ´{}´", skipun),
                }
            }
            None => println!("Þú verður að slá eitthvað inn!!"),
        }
    }
}

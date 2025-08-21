fn main() {
    let x = 10;
    let y = 20;
    // python - rust
    //   and     &&
    //    or     ||
    //   not      !
    if !(x < 10 && y < 20) {
        println!("x er minna en 10 og y er minna en 20, ekki");
    } else if x > 10 || y > 20 {
        println!("x er stærra en 10 eða y er stærra en 20");
    } else {
        println!("x er 10");
    }
    let nott = true;
    let k = if nott { 10 } else { 20 };
    let j = if x < 10 { 10 } else { 20 };
    println!("{}{}", k, j);
    let k = 10;
    match k {
        1 => {
            println!("k er einn");
            println!("og gera eihvað meira");
        }
        2..4 => println!("k er 2 eða 3"),
        _ => println!("k er eitthvað annað"),
    }
    let b = true;
    match b {
        true => println!("b er true"),
        false => println!("b er false"),
    }

    let k = match b {
        true => 10,
        false => 20,
    };

    // lykkjur
    let mut k = 5;
    while k > 0 {
        println!("{}", k);
        k -= 1;
    }

    // while True:
    let mut k = 5;
    'ytri: loop {
        'innri: loop {
            println!("{}", k);
            k -= 1;
            if k < 0 {
                break 'ytri;
            }
        }
    }

    let fylki = [1, 2, 3, 4, 5];
    for f in fylki {
        println!("{}", f);
    }
    let t = 1..4;
    for x in (1..=4).rev().step_by(2) {
        println!("{x}");
    }
}

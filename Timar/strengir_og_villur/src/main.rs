use std::hint;

fn deila(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err("deila: Ekki má deila með núll!!!".to_string())
    }
}

fn deiling(a: i32, b: i32) -> Result<i32, String> {
    // gera fullt af hlutum
    if a > 10 {
        return Err("deiling: a má ekki vera stærra en 10!".to_string());
    }
    // gera fullt af hlutum
    let k = deila(a, b)?;
    // gera fullt af hlutum
    Ok(k)
}

fn main() {
    match deiling(9, 0) {
        Ok(nidurstada) => println!("niðurstaðan var: {}", nidurstada),
        Err(e) => println!("{}", e),
    }


    deila(10, 0);
    let s1 = String::from("hello, world!");
    let s2 = &s1[0..5];
    let s3 = &s1[7..];
    println!("{}", &s1[..1]);
    println!("len: {}, cap: {}", s1.len(), s1.capacity());

    let mut v1 = vec![1,2,3,4,5];
    let v2 = &mut v1[0..3];
    v2[1] = 99;
    dbg!(v1);

    let s1 = String::from("halló😊, heimur!");
    for stafur in s1.chars() {
        println!("{}", stafur)
    }

    let s1 = "abc def ghi".to_string();
    let v_s1: Vec<&str> = s1.split_whitespace().collect();
    let v_s1 = s1.split_whitespace().collect::<Vec<&str>>();
    dbg!(&v_s1);
    if let Some(ord) = v_s1.first() {
        println!("orðið er: {}", ord);
    } else {
        println!("Ekkert var fremst!");
    }

    /*
    enum Option<T> { // T stendur fyrir hvað sem er
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    if let Some(heiltala) = v_s1.get(1) {
        if let Ok(tala) = heiltala.parse::<i32>() {
            println!("talan var: {}", tala);
        } else {
            println!("gat ekki parsað: {}", heiltala);
        }
    }

    match v_s1.get(2) {
        Some(tala) => {
            match tala.parse::<f32>() {
                Ok(kommutala) => println!("talan var: {}", kommutala),
                Err(e) => println!("{}", e),
            }
        },
        None => println!("Ekkert í index 2"),
    }

    //v_s1[1]

}

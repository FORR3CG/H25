fn main() {
    // i8 og u8
    // usize og isize
    let i: i8 = 120;
    let j = 120_i8;
    let k = i as i16 + j as i16;
    let mut m = 10;
    m += 1;
    let n = 9;
    let o = 10;
    let p = n / o;  /* eins og heiltöludeiling (//) í python */
    let p = n as f32 / o as f32;
    // char, bool 
    let t = (10_u8, 32, 3.14);
    println!("{}", t.2);
    let (x, y, z) = t;
    let mut f = [1_u8, 2,3,4,5];
    let index = 2_i32;
    f[index as usize] = 99;
    println!("{}", f[2]);
    prenta_tolu(88);
    println!("{}", tvofalda(99));
    let mut t = 99;
    haekka_um_einn(&mut t); // 0xff00 -> 100
    println!("{}", t);

    for i in (0..5).rev().step_by(2) { // range(4, -1, -2) í python
        println!("{}", i);
    }

    let mut nafn = "Geir".to_string();
    nafn = prenta_nafn_hastafir(nafn);

    let n2 = &nafn;
    println!("{}", nafn);

    let n2 = String::from("abc");
    prenta_hastafi(&n2);

}

fn prenta_hastafi(inn: &String) {
    println!("{}", inn.to_uppercase())
}

fn prenta_nafn_hastafir(inn: String) -> String {
    /* let hs = inn.to_uppercase();
    //println!("{}", hs);
    hs */
    inn.to_uppercase()
}



fn haekka_um_einn(tala: &mut i32) { // 0xff00
    *tala = *tala + 1;
}

fn tvofalda(tala: i32) -> i64 {
    tala as i64 * 2
/*     let svar = tala as i64 * 2;
    return svar;
    return tala as i64 * 2; */
}

fn prenta_tolu(tala: i32) {
    println!("tók inn tölu");
    println!("talan er {}", tala)
}

fn prentar_hallo() {
    println!("halló")
}

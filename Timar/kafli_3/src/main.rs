const PI: f64 = 3.14;
//let b = 10;
fn main() {
    let PI2 = 3.14;
    let mut langt_breytunafn = 10;
    langt_breytunafn += 1;
    println!("Hello, world!");
    let i: i8 = 0x10;
    let mut k = 255_u8;
    k = k.wrapping_add(1);
    let j = 1_000_000_000_i128;
    let g = b'g';
    println!("{}", g as char);
    // heiltölur
    // i8 / u8 - i16 / u16 - i32 / u32 - i64 / u64 - i128 / u128 - isize / usize

    // float:
    // f32 og f64
    let p = 3.14;
    let p2: f32 = 3.14;
    let p3 = 3.14_f32;
    let i = 9;
    let j = 10;
    // python: 9 // 10
    let k = i / j; // heiltöludeiling
    let k = i as f64 / j as f64; // float deiling
    let v = 2_u16.pow(5);
    let vf = 3.14_f64.powf(3.2);
    println!("{}", k);
    let b = true; // false
    // túplur
    let t = (10_u8, 3.14, 'g');
    println!("{} {} {}", t.0, t.1, t.2);
    let mut t2 = (10, 20, 30);
    t2.2 = 99;
    let (x, y, z) = t;
    let (_, y, _) = t;
    // array - fylki
    let f = [1_u8, 2, 3, 4, 5];
    let mut f2: [i8; 3] = [0; 3];
    f2[0] = 10;
    let nofn = ["geir", "ari", "bjarki"];
    let mut f2d = [[0; 5]; 4];
    println!("{:#?}", f2d);
    // föll
    fall_sem_prentar_geir();
}

fn fall_sem_skilar_fimm() -> i64 {
    // return 5;
    5    
}

fn fall_sem_tekur_faeribreytu(x: u16, y: f32) {
    println!("Þú sendir mér {} og {}", x, y)
}

fn fall_sem_prentar_geir() {
    println!("Geir")
}

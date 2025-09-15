use std::process::Termination;

fn leggja_saman_fall(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let leggja_saman = |a: i32, b: i32| a + b;

    let prenta_hallo_tskoli = || {
        println!("Halló");
        println!("Tskóli");
    };

    //println!("{}", leggja_saman(10, 20));
    //println!("{}", leggja_saman_fall(10, 20));
    //prenta_hallo_tskoli();

    /*
        listi = [1,2,3]
        for i in listi:
            print(i)
    */

    let texti = "Í tæk".to_string();
    let mut itexti = texti.chars();
    println!("{:?}", itexti.next());
    println!("{:?}", itexti.next());
    println!("{:?}", itexti.next());
    println!("{:?}", itexti.next());
    println!("{:?}", itexti.next());
    println!("{:?}", itexti.next());
    println!("{:?}", itexti);

    let texti = "abcdef".to_string();
    for stafur in texti.chars() {
        println!("{}", stafur)
    }

    let fylki = vec![1,2,3,4,5];
    let mut ifylki = fylki.iter();
    println!("{:?}", ifylki.next());
    println!("{:?}", ifylki.next());
    println!("{:?}", ifylki.next());
    println!("{:?}", ifylki);
    println!("{:?}", fylki)
}

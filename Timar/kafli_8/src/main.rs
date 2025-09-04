use std::collections::VecDeque;

fn main() {
    // ----------------------- Vec -------------------------

    let mut tomur_listi: Vec<i128> = Vec::new();
    for i in 1..=5 {
        tomur_listi.push(i*10);
    }
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);
    let index = 2_i32;
    println!("{}", tomur_listi[index as usize]);
    println!("Aftasta stakið er: {:?}", tomur_listi.last());
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);
    println!("stak nr 2 er {}", tomur_listi[1]);
    println!("stak nr 2 er {:?}", tomur_listi.get(1)); // get(n), first() og last()
    let stak_nr_2 = match tomur_listi.get(1) {
        Some(x) => *x, // verður ref á stakið í vectornum og þurfum því að derefa til að fá töluna sjálfa
        None => 0,
    };
    if let Some(x) = tomur_listi.get(1) {
        println!("Fann töluna {}", x)
    } else {
        println!("Engin tala í þessu staki!")
    }
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);
    println!("{:?}", tomur_listi.pop());
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);
    println!("{}", tomur_listi.remove(0));
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);
    println!("{}", tomur_listi.swap_remove(0));
    println!("len: {}, cap: {}, listinn: {:?}", tomur_listi.len(), tomur_listi.capacity(), tomur_listi);

    let mut listi = vec![1,2,3,4,5,6];

    for l in &mut listi {
        println!("gildi fyrir: {}", l);
        *l += 10;
        println!("gildi eftir: {}", l)
    }
    for (i, l) in listi.iter().enumerate() {
        println!("{}: {}", i, l)
    }

    for l in listi.chunks(2) {
        println!("{:?}", l)
    }
    for l in listi.windows(3) {
        println!("{:?}", l)
    }

    let mut oradadur_listi = vec![9, 3, 34, 17, 4, 1];
    println!("{:?}", oradadur_listi);
    oradadur_listi.sort();
    println!("{:?}", oradadur_listi);
    oradadur_listi.insert(0, 99);

    // ----------------------- VecDeq -------------------------

    let mut vd_listi = VecDeque::from([10, 20, 30, 40]);
    vd_listi.push_front(5);
    vd_listi.push_back(99);
    vd_listi.pop_back();
    vd_listi.pop_front();
    vd_listi.front();
    vd_listi.back();
    vd_listi.insert(2, 22);
    vd_listi.remove(3);
}

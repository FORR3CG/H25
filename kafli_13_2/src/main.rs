fn leggja_saman(a: i32, b: i32) -> i32 {
    a + b
}

fn leggja_saman_2(a: i32, b: i32) -> i32 {
    let summa = a + b;
    return summa;
}


fn main() {
    let mut v = vec![1,2,3,4,5];

    for tala in &v {
        println!("{}", tala)
    }

    v.iter().for_each(|tala| println!("{}", tala));

    v.iter_mut().for_each(|tala| *tala *= 10);

    /* let nytt_v = v.iter().map(|tala| {
        let k = tala * 5;
        return k;
        }); */
    let mut nytt_v = v.iter().map(|tala| tala * 5).collect::<Vec<i32>>();
    let mut nytt_v: Vec<i32> = v.iter().map(|tala| tala * 5).collect();
    let mut nytt_v = v.iter()
                                .map(|tala| tala * 5)
                                .filter(|tala| *tala > 150)
                                .collect::<Vec<i32>>();
    println!("{:?}", nytt_v);
    //println!("{:?}", nytt_v.next())
    // [10, 20, 30, 40, 50]
    let fold = v.iter().fold(1, |summa, tala| summa * tala);
    //let red = v.iter().reduce(|summa, tala| summa + tala);
    println!("{:?}", v);
    v.retain(|tala| *tala < 20);
    println!("{:?}", v);
    let mut v = [5, 7, 2, 1, 0];
    //v.sort();
    //v.reverse();
    v.sort_by(|a, b| b.cmp(a));
    println!("{:?}", v);
}

fn main() {
    let leggja_saman = |a, b| {
        a + b
    };
    println!("{}", leggja_saman(10, 20));
    let margfalda = |a: i32, b: i32| -> i64 {
        a as i64 * b as i64
    };
    let v = vec![1, 2, 3, 4, 5];
    let k = v.iter().for_each(|x| println!("{}", x));
    let k = v.iter().map(|x| x + 3).collect::<Vec<i32>>();
    //                                                          summa = summa <op> x
    let k = v.iter().fold(0, |summa, x| summa + x);
    let k: i32 = v.iter().sum();
    let k = v.iter().find(|x| **x == 9);
    let k = v.iter().filter(|x| **x > 2).collect::<Vec<&i32>>();
    let k = v.iter().position(|x| *x == 4);
    let mut j = vec![10, 20, 20, 30, 40, 50];
    j.retain(|x| *x != 20);
    let mut j = vec![7, 3, 1, 4, 5];
    j.sort_by(|a, b| a.cmp(b));
    //j.sort();
    println!("{:?}", j);
    let inntak = "skrá 100 200";
    let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();
    // ["skrá", "100", "200"]
    let x = skipanir[1..].join(" ");

}

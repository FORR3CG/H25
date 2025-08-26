fn main() {
    let mut stack_array = [0u8; 100];
    let mut sa_2 = stack_array;
    sa_2[5] = 25;
    let heap_array = Box::new([0u128; 1_000]);
    println!("sa_addr : {:p}\nsa2_addr: {:p}\nheapaddr: {:p}", 
                    &stack_array, &sa_2, &heap_array);

}

/* fn main() {
    let mut first = String::from("Ferris");
    first = add_suffix(first);
    println!("{first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
 */

/* fn main() {
    let mut x = "Geir".to_string();
    let y = &x;
    let z = &x;
    println!("x: {}, y: {}, z: {}", x, y, z);
    x.push('X');
}
 */
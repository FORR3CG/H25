use std::io::stdin;

fn main() {
    let tala = rand::random_range(1..=100);
    loop {
        println!("Giskaðu á tölu: ");
        // let inntak = "".to_string();
        let mut inntak = String::new();
        // inntak = input("texti: ")
        //let utkoma = stdin().read_line(&mut inntak);
        //let _ = stdin().read_line(&mut inntak);
        /*     match stdin().read_line(&mut inntak) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        } */
        stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá notanda!");
        //println!("{:?}, {inntak:?}", utkoma);
        let gisk = inntak
            .trim()
            .parse::<i32>()
            .expect("Gat ekki breytt í tölu");

        if gisk == tala {
            println!("Þú giskaðir rétt!!!!");
            break;
        } else if gisk < tala {
            println!("Of lágt!");
        } else {
            println!("Of hátt!");
        }
    }
}

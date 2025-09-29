# FORR3CG - Æfingaverkefni 6

## Verkefnið

Bílasalan er að færa út kvíarnar og ætlar að bæta við möguleikanum að selja báta. Bilar hafa id (u32), tegund (String) og verð (u32). Bátar hafa id (u32), fjölda farþega (u16) og verð (u32).

Búðu svo til trait sem hefur föllin as_any, id og verd. Tryggðu að allir sem nota trait-ið þurfi líka að útfæra Display.

Gerðu svo struct til að halda utan um Vec af trait-inu sem þú gerðir hér fyrir ofan. Það þarf svo að eiga föll til að gera eftirfarandi:
- Skrá bát
- Skrá bíl
- Fá alla bíla sem String
- Fá alla báta sem String
- Sjá verðmæti allra farartækja í listanum (Skila u32)
- Fá farartæki út frá id (Skila Option\<String\>)
- Útfæra Display

Sjá dæmi um notkun hér fyrir neðan.

Hvert struct þarf að vera í sér skrá.

Dæmi um notkun:
```rust 
fn main() {
    let mut fs = Farartaekjasala::new();
    fs.skra_bil(100, "Volvo", 2000);
    fs.skra_bil(101, "Toyota", 2500);
    fs.skra_bil(102, "Nizzan", 1500);
    fs.skra_bat(103, 5, 5000);
    fs.skra_bat(104, 7, 10000);
    fs.skra_bat(105, 3, 4000);
    println!("{}", fs);
    println!("Verðmæti: {}", fs.verdmaeti_allra_farartaekja());
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bata());
    println!("----------------------------------");
    println!("{}", fs.skoda_alla_bila());
    println!("----------------------------------");
    if let Some(farartaeki) = fs.skoda_farartaeki_med_id(103) {
        println!("{}", farartaeki)
    } else {
        println!("Fann ekki farartæki með þessu id!")
    }
    if let Some(farartaeki) = fs.skoda_farartaeki_med_id(999) {
        println!("{}", farartaeki)
    } else {
        println!("Fann ekki farartæki með þessu id!")
    }
}

```

Dæmi um lausn er [hér](https://github.com/FORR3CG/H25/tree/main/Aefningaverkefni/lausnir/aefingaverkefni_6).

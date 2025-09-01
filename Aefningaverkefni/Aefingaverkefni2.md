# FORR3CG - Æfingaverkefni 2 - `struct` og `enum`

Útfærðu `struct` sem geymir upplýsingar um einn **hest**. Upplýsingarnar sem þarf að skrá fyrir hestinn eru eftirfarandi: id (u32), nafn (String), aldur (u8) og **staða**. Staða er `enum` sem getur haft eftirfarandi gildi: *Leigdur*, *Laus*, *EkkiTilLeigu* og *Othekkt*.

Bættu falli við `Hestur` sem getur breytt stöðunni á honum og svo öðru falli sem hækkar aldur hestsins um einn.

Hafðu `Hestur` og `Stöðu` í sér skrám.

Útfærðu `Display` fyrir hestinn og stöðuna.

Dæmi um forrit fyrir ofantalið:
```rust
mod hestur;
mod stada;

use hestur::Hestur;
use stada::Stada;

fn main() {
    let mut sleipnir = Hestur::new(100, "Gráni", 15, Stada::Laus);
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Laus

    sleipnir.breyta_stodu(Stada::Leigdur);
    sleipnir.haekka_aldur();
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 16, staða: Leigður    
}
```

[Lausn á verkefninu](./lausnir/aefingaverkefni_2/)
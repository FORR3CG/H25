# FORR3CG Haust 2025 - Lokaverkefni A (10%)

Áður en lengra er haldið skalt þú stofna nýtt **private** repo á github og bjóða *gestskoli* inn sem samstarfsaðila. Farðu svo inn á Innu og skilaðu slóðinni í **Lokaverkefni A** skilahólfið.


  - [Almennar reglur varðandi verkefnið](#almennar-reglur-varðandi-verkefnið)
    - [Námsmat](#námsmat)
  - [Skil verkefnisins](#skil-verkefnisins)
  - [Verkefnið](#verkefnið)
    - [Ferðaskrifstofan Tskóla-ferðir](#ferðaskrifstofan-tskóla-ferðir)
    - [Kröfur](#kröfur)
  - [Prófunarforrit](#prófunarforrit)
    - [Prófunargögn](#prófunargögn)
    - [Prófunargögn í svigum](#prófunargögn-í-svigum)

## Almennar reglur varðandi verkefnið

- Verkefnið er einstaklingsverkefni. Ef tveir eða fleiri nemendur skila sömu lausnunum er gefið 0 (núll) fyrir þær lausnir.
- Ekki er heimilt að nota gervigreind við lausn þessa verkefnis. Vertu viðbúin(n) því að geta útskýrst hvern einasta staf í kóðanum sem þú skilar. Einnig að gera breytingar á kóðanum ef beðið er um. Getir þú ekki útskýrt eitthvað í kóðanum eða ekki gert þær breytingar sem beðið er um verður gefið 0 (núll) fyrir verkefnið.
- Ef kóði er tekinn af netinu (eða öðrum álíka stöðum t.d. AI) skal taka það fram, benda á hvaðan hann kemur og skrifa skýringar (e. comment) við hverja línu kóðans. Sé það ekki gert verður gefið 0 (núll) fyrir verkefnið í heild.
- Sein skil, ekki er hægt að bjóða upp á sein skil í verkefninu. :exclamation:
- Þýðing (e. compile) verkefnið á að þýðast villu- og viðvaranalaust.

### Námsmat

Við mat á verkefninu er horft til eftirfarandi (ekki tæmandi listi):
- Virkni
- Lýsandi nöfn á:
  - Breytum.
  - Föllum og færibreytum þeirra.
  - Struct og enums.
  - Hefðbundnum [ritunarreglum Rust](https://rust-lang.github.io/api-guidelines/naming.html) fylgt.
- Snyrtilega upp settur kóði.
- Skýringar (e. comment) notaðar þar sem það á við.
- Verkefnið unnið jafnt og þétt (commit á github).

## Skil verkefnisins

Skila skal verkefninu fyrir miðnætti sunnudaginn 5. október 2025. Ekki verður hægt að fá framlengigu á því.

## Verkefnið

### Ferðaskrifstofan Tskóla-ferðir

Ferðaskrifstofan Tskóla-ferðir hefur beðið þig um að skrifa bókunarkerfi fyrir sig. Ferðaskrifstofan býður upp á þrjár mismunandi ferðir; flugferðir, hjólaferðir og bátsferðir. Fyrir allar ferðir þarf að skrá númer (id), hámarskfjöldi í ferðinni og hversu margir eru bókaðir í ferðina. Fyrir flugferðir þarf að auki að skrá hvert er flogið, fyrir hjólaferðina hversu marga klukkutíma hún taki og fyrir bátsferðina hvort báturinn er yfirbyggður eða ekki.

Þegar ferð hefur verið stofnuð þarf svo að vera hægt að uppfæra fjölda bókaðra farþega í ferðunum. 

### Kröfur

- Gerðu struct sem inniheldur sameiginlegu breyturnar í ferðunum (sjá dæmi [hér](../Aefningaverkefni/lausnir/aefingaverkefni_6_composition/src)).
- Gerðu struct fyrir hverja gerð ferðar sem inniheldur sameiginlega struct-ið og svo það sem er sér fyrir hverja gerð ferðar.
- Gerðu getters og setters fyrir gagnabreytur struct-anna þar sem það á við. 
    - Tryggðu að ekki sé hægt að skrá fleiri farþega í ferð en pláss er fyrir.
- Útfærðu Display og TryFrom fyrir öll structin.
- Gerðu trait til að geta skráð mismunandi ferðir í sama Vectorinn.
- Með listastructinu þarf að vera hægt að gera eftirfarandi, útfæra Result þar sem það á við:
  - Það þarf að vera hægt að skrá ferð í listann.
  - Það þarf að vera hægt að skoða ákveðna ferð í listanum.
  - Það þarf að vera hægt að eyða ferð úr listanum.
  - Það þarf að vera hægt að uppfæra fjölda bókaðra farþega í ferðum.
  - það þarf að vera hægt að prenta á skjá lista:
    - Með öllum ferðum.
    - Með öllum bátsferðum.
    - Með öllum hjólaferðum.
    - Með öllum flugferðum.

## Prófunarforrit

Skrifaðu lítið forrit sem sýnir virknina á ofantöldu.

### Prófunargögn

Hafðu þessi gögn harðkóðuð í forritinu sem þið skilið.

Ferð | Nr. | Fjöldi bókaðir | Heildarfjöldi | Hvert/Tími/Yfirbyggður
--- | --- | :-: | :-: | ---
Bátsferð | 201 | 2 | 10 | Nei
Bátsferð | 202 | 20 | 100 | Já
Bátsferð | 203 | 6 | 8 | Nei
Flugferð | 204 | 10 | 50 | Akureyri
Flugferð | 205 | 120 | 200 | Tenerife
Flugferð | 206 | 3 | 10 | Grímsey
Flugferð | 207 | 50 | 250 | Boston
Hjólaferð | 208 | 3 | 10 | 4
Hjólaferð | 209 | 0 | 20 | 10
Hjólaferð | 210 | 3 | 5 | 1

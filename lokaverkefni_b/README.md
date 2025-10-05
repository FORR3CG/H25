# FORR3CG Haust 2025 - Lokaverkefni B (15%)

Áður en lengra er haldið skalt þú stofna nýtt **private** repo á github og bjóða *gestskoli* inn sem samstarfsaðila. Farðu svo inn á Innu og skilaðu slóðinni að repo-inu í **Lokaverkefni B** skilahólfið.

## Efnisyfirlit
- [Verkefnalýsing](#verkefnið---búnaðarlisti-tækniskólans)
- [Ráðleggingar frá AI varðandi verkefnið](#leiðbeiningar-fyrir-nemendur-um-gervigreind-ai)
- [Hverju á að skila](#afurðir-verkefnisins--hverju-á-að-skila)


## Verkefnið - Búnaðarlisti Tækniskólans

Húsumsjón Tækniskólans hefur beðið þig um skrifa hugbúnað í Rust til að halda utan um ýmsan búnað sem skólinn á og hvar sá búnaður er staðsettur. Það þarf að gera myndrænt notendaviðmót (frjálst val, skoðið hvað er til fyrir rust) og gögnin á að geyma í gagnagrunni (SQLite).

Allur búnaður hefur auðkenni, verðmæti og staðsetningu:
- Auðkennið er einkvæm heiltala (sjálfhækkandi (e. auto increment)).
- Verðmætið er í krónum.
- Staðsetningin er samsett gildi sem samanstendur af húsi, hæð og herbergisnúmeri t.d. H-202 sem er þá Háteigsvegur, 2. hæð, herbergi nr. 2 og HA-123 er þá Hafnarfjörður, 1. hæð, herbergi nr. 23. Húsmerkingarnar eru eftirfarandi:
    - HA - Hafnarfjörður,
    - H - Háteigsvegur,
    - S - Skólavörðuholt,

Sá búnaður sem Húsumsjónin vill geta skráð í þessum hugbúnaði eru:
  - Borð, aukalega þarf að skrá hversu margir geta setið við borðið.
  - Stóll, aukalega þarf að skrá hvernig stóllinn er (Hægindastóll, Skólastóll, Skrifstofustóll eða annað).
  - Skjávarpi, aukalega þarf að skrá hversu mörg lumens skjávarpinn er.

Búnaður:

- Hafðu struct fyrir hverja gerð búnaðar og enum þar sem það á við.
- Útfærðu Display og TryFrom fyrir allan búnað.
- Hafðu hvert struct og enum í sér skrá.
- Annað sem eðlilegt getur verið að hafa.

Listinn:

- Úfærðu struct fyrir listann en listinn heldur líka utan um hvaða auðkenni (id) hver hlutur fær.
  - Það þarf að vera hægt að skrá búnað í listann.
  - Það þarf að vera hægt að eyða búnaði úr listanum.
  - Það þarf að vera hægt að uppfæra staðsetningu búnaðar.
  - Það þarf að vera hægt að prenta út ákveðinn búnað útfrá id.
  - það þarf að vera hægt að prenta á skjá lista með öllum búnaði.
  - það þarf að vera hægt að prenta á skjá lista með öllum búnaði í ákveðnu húsi.
  - Prenta á skjá búnað af ákveðinni gerð, t.d. alla stóla.
  - Prenta á skjá allan búnað í:
    - ákveðinni stofu.
    - ákveðinni hæð í ákveðnu húsi.
  - Gögnin eru röðuð í listanum eftir húsi, hæð, herbergi og svo tegund búnaðar.
  - Skrifa og lesa gogn í/úr JSON skrá.
  - Annað sem ykkur dettur í hug í samráði við kennara (allt að 10 stig).

## Ábendingar til ykkar frá ChatGPT

Ég bað ChatGPT um ábendingar til ykkar varðandi verkefnið og þetta er svarið:

### Leiðbeiningar fyrir nemendur um gervigreind (AI)

#### 1. Leyfileg notkun (og væntingar)
- Það má/á að nota **GitHub Copilot**, **Copilot Chat/Agent**, **ChatGPT** o.fl. til hugmyndavinnu, kóðunar, útskýringar, kóðadæma, prófana og skjölunar.
- Þið berið **fulla ábyrgð** á kóðanum sem þið skiljið inn — keyrið, prófið og skiljið áður en þið samþykkið.
- Skráið **hvaða AI-aðstoð** var notuð (stutt lýsing + dæmi um skipanir/spurningar).

---

#### 2. Að fá það besta úr AI-notkun
- **Prófa fyrst** – Skrifið litlar einingaprófanir eða keyrsludæmi áður en þið biðjið um kóða.  
- **Lítil verkþrep** – Biðjið um litlar, skýrar breytingar („Bættu við `add_category()` sem skrifar í SQLite með SQLx“).  
- **Stíll og samfella** – Látið AI laga stíl, nafngiftir og skjölun (docstrings).  
- **Rubber duck** – Látið AI útskýra *af hverju* lausnin virkar, ekki bara *hvað* hún gerir.  
- **Diff-beiðnir** – „Sýndu diff“ til að sjá nákvæmlega hvað breytist.

---

#### 3. Öryggi og friðhelgi
- **Engin viðkvæm gögn** (lyklar, lykilorð, persónuupplýsingar) í promptum.  
- Notið **.env** og **dotenv** fyrir tengistrengi — **aldrei** harðkóða leyndarmál í repo.  
- Látið AI fara yfir **SQL injection**, **error handling** og **boundary cases** (tóm gildi, NULL, röng staðsetning).

---

#### 4. Traust og staðfesting
- Keyrið `cargo check` / `cargo test` eftir breytingar.  
- Biðjið AI: „Skrifaðu lágmarks test fyrir `insert_equipment()`“.  
- Látið AI framkvæma **kóðaskoðun**: „Gerðu code review með áherslu á villumeðhöndlun og lifetimes“.

---

#### 5. Hönnun verkefnisins
- Haldið **skýrri lagskiptingu**: `models.rs`, `database.rs`, GUI-lag.  
- Biðjið AI að virða þessa uppbyggingu.  
- **Staðsetningarkóðar** – Látið AI búa til validator fyrir `HA/H/S-hæðherbergi`.  
- **Flokkar** – Enginn „annað“ flokkur. Látið AI bæta við dýnamísku flokkasniði í GUI (CRUD + schema migration).

---

#### 6. Forðast „hallucinations“
- Spyrjið: „Ertu viss? Vísaðu í Rust docs / crates.io.“  
- Krefjast nákvæmni: „Notaðu `rusqlite`/`sqlx` með réttum örútgáfum; sýndu `Cargo.toml` bút.“

---

#### 7. Heiðarleiki og tilvísanir
- Ef AI skapar efni eða kóða sem byggir á heimildum: **nefnið heimild** (t.d. Rust Book kafli).  
- Setjið í README **AI-notkunaryfirlýsingu** (sjá sniðmát hér að neðan).

---

#### AI-notkunaryfirlýsing — 📄 sniðmát

**AI-notkun**  
Við notuðum eftirfarandi AI-verkfæri í verkefninu:

- **Verkfæri:** GitHub Copilot Agent, ChatGPT. Hvaða model var notað. 
- **Notkun:** kóðauppástungur fyrir CRUD á SQLite, útskýringar á `Result` og villumeðhöndlun, smíði einingaprófa, skjölun.  
- **Dæmi um beiðnir:**  
  - „Búðu til `create_table` fyrir `bunadur` með dálkum id, flokkur, verdmaeti, stadsetning.“  
  - „Skrifaðu validator fyrir staðsetningarkóða `HA/H/S-hæð-herbergi` með regex + prófum.“  
- **Staðfesting:** Keyrðum `cargo check/test`, koðaskoðun með áherslu á SQL injection og villumeðhöndlun.  
- **Takmörk:** Breytingar samþykktar aðeins eftir prófun og handvirka yfirferð.

---

#### 💡 Dæmi um „góðan prompt“

> „Ég er með Rust verkefni með `sqlx` og SQLite.  
> Viltu skrifa fallið `insert_equipment(db: &Pool<Sqlite>, item: NewItem) -> Result<i64>` sem:  
> 1. Fullgildir (e. validate) `stadsetning` með regex (`^(HA|H|S)-[0-9][0-9]+$`)  
> 2. Skilar `Ok(id)` eftir `INSERT` með `RETURNING id`  
> 3. Skilar villu með skilaboðum ef `flokkur` er ekki í `categories` töflu  
> 4. Bætir við lágmarks einingaprófi  
> 5. Sýndu líka diff á `Cargo.toml` ef þarf feature-flögg.“

---

#### ⚠️ Algengar mistök
- Að samþykkja stórar kóðabreytingar í einu.  
- Að setja leyndarmál í repo.  
- Að treysta á ósannaðar fullyrðingar (útgáfur, API nöfn).  
- Að sleppa prófunum og kóðaskoðun.

---

## Afurðir verkefnisins / Hverju á að skila
1. Allur kóði á að vera á Github.
1. Leiðbeiningar um hvernig á að keyra forritið.
1. Allar skipanir til gervigreindarinnar (sjá [hér](#ai-notkunaryfirlýsing---sniðmát))
1. Myndband þar sem þið sýnið helstu virkni forritsins.
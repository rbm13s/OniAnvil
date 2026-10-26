<div align="center">

<img src="assets/icon.jpg" alt="Logotip OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio za piksel grafiku i animaciju prozirnih slojeva (Onion-Skinning) na temelju igraćih atlasa**

[![Izgradnja i izdanje](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Preuzimanja](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Jezici](https://img.shields.io/badge/Jezici-37_Jezika-purple)](#10-35-ergonomskih-tema-i-potpuna-lokalizacija)
[![Platforme](https://img.shields.io/badge/Platforme-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#kako-započeti)
[![Online aplikacija](https://img.shields.io/badge/Online_aplikacija-Isprobajte_u_pregledniku_(Eksperimentalno)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=hr)
[![Licenca: MIT](https://img.shields.io/badge/Licenca-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Isprobajte OniAnvil u pregledniku (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=hr)** *(⚠️ Eksperimentalna verzija: može sadržavati WebGL/WASM ograničenja i programske pogreške)* • **[🌐 Službena stranica dokumentacije](https://rbm13s.github.io/OniAnvil/hr.html)**

<br/>

**Languages / Языки / Мови / Sprachen / 语言:**<br/>
[English](../README.md) •
[Русский](README-ru.md) •
[Українська](README-uk.md) •
[Беларуская](README-be.md) •
[Қазақша](README-kk.md) •
[ქართული](README-ka.md) •
[Հայերեն](README-hy.md) •
[Azərbaycan](README-az.md) •
[Oʻzbekcha](README-uz.md) •
[Deutsch](README-de.md) •
[Français](README-fr.md) •
[Español](README-es.md) •
[Italiano](README-it.md) •
[Português](README-pt.md) •
[Polski](README-pl.md) •
[Čeština](README-cs.md) •
[Slovenčina](README-sk.md) •
[Slovenščina](README-sl.md) •
[Български](README-bg.md) •
[Српски](README-sr.md) •
**Hrvatski** •
[Nederlands](README-nl.md) •
[Dansk](README-da.md) •
[Norsk](README-no.md) •
[Svenska](README-sv.md) •
[Suomi](README-fi.md) •
[Eesti](README-et.md) •
[Latviešu](README-lv.md) •
[Lietuvių](README-lt.md) •
[Magyar](README-hu.md) •
[Română](README-ro.md) •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Sadržaj

- [Uvod](#uvod)
- [Snimke zaslona i vizualni pregled](#snimke-zaslona-i-vizualni-pregled)
- [Sveobuhvatni pregled alata](#sveobuhvatni-pregled-alata)
- [Funkcionalna dokumentacija](#funkcionalna-dokumentacija)
  - [1. Dvojna arhitektura usmjerena na atlas](#1-dvojna-arhitektura-usmjerena-na-atlas)
  - [2. Višekanalna vremenska crta slojeva](#2-višekanalna-vremenska-crta-slojeva)
  - [3. Pretpregled animacije uživo u stvarnom vremenu](#3-pretpregled-animacije-uživo-u-stvarnom-vremenu)
  - [4. Napredni sustav prozirnih slojeva (Onion Skinning)](#4-napredni-sustav-prozirnih-slojeva-onion-skinning)
  - [5. Pametni referentni sloj i crtanje po skicama](#5-pametni-referentni-sloj-i-crtanje-po-skicama)
  - [6. Precizni alati za odabir i marširajući mravi](#6-precizni-alati-za-odabir-i-marširajući-mravi)
  - [7. Efekti uživo, filtri i RotSprite](#7-efekti-uživo-filtri-i-rotsprite)
  - [8. Upravljanje paletama, zamjena boja i preoblike likova](#8-upravljanje-paletama-zamjena-boja-i-preoblike-likova)
  - [9. Rezanje listova, 9-dijelna mreža sučelja i izvoz za igraće motore](#9-rezanje-listova-9-dijelna-mreža-sučelja-i-izvoz-za-igraće-motore)
  - [10. 35 ergonomskih tema i potpuna lokalizacija](#10-35-ergonomskih-tema-i-potpuna-lokalizacija)
- [Kontrole i tipkovnički prečaci](#kontrole-i-tipkovnički-prečaci)
- [Kako započeti](#kako-započeti)
  - [Samostalne binarne datoteke](#samostalne-binarne-datoteke)
  - [Izgradnja iz izvornog koda](#izgradnja-iz-izvornog-koda)
- [Format datoteka i popratna arhitektura](#format-datoteka-i-popratna-arhitektura)
- [Doprinos i zahvale](#doprinos-i-zahvale)
- [Licenca i odricanje od odgovornosti](#licenca-i-odricanje-od-odgovornosti)

---

## Uvod

**OniAnvil** (*naziv potječe od **Oni**on-Skinning (prozirni slojevi) + **Anvil** (nakovanj za kovanje spriteova)*) je otvoreni, visokooptimizirani uređivač piksel arta i paket za animaciju napisan u programskom jeziku Rust. Pokreće se izvorno uz hardverski ubrzano iscrtavanje na sustavima Windows, Linux i macOS, kao i izravno unutar modernih web preglednika putem WebAssembly tehnologije bez potrebe za instalacijom.

Većina postojećih uređivača spriteova tretira animaciju kao skup nepovezanih slikovnih datoteka, što prisiljava programere na zamorne korake izvoza, rezanja i ponovnog pakiranja. **OniAnvil preokreće ovu paradigmu:** teksturni atlas lista spriteova (`atlas_rgba`) primarni je izvor istine. Ćelije okvira, trake slojeva, animacijske oznake i točke sudara u svakom su trenutku nedestruktivno usklađeni s atlasom.

### Zašto OniAnvil?
- 🚀 **Munjevite performanse izvornog Rusta**: Trenutačno pokretanje ispod jedne milisekunde, kristalno čisto iscrtavanje pogonjeno tehnologijama `wgpu` i `egui 0.31`, bez ikakvog zastoja pri kretanju kroz duge vremenske crte.
- 🎬 **Višekanalna vremenska crta slojeva**: Nelinearne trake slojeva (`V1, V2, V3...`) sa slaganjem odozgo prema dolje, spajanjem prema dolje, povezanim okvirima i podesivim trajanjem zadržavanja.
- 🧅 **Vrhunski sustav prozirnih slojeva (Onion Skinning)**: Prikaz više susjednih okvira s neovisnim bojanjem unaprijed/unatrag, prilagodljivim krivuljama prozirnosti i prelijevanjem preko petlje.
- 🎨 **Trenutačne preoblike paleta likova**: Stvarajte dnevne/noćne varijante, timske boje ili elementarne preoblike uz automatsko preslikavanje boja atlasa u stvarnom vremenu te uvoz i izvoz GPL/PAL/HEX formata.
- 📦 **Izravan izvoz za igraće motore**: Izvozite skalirane PNG datoteke s proširenim rubnim pikselima (ekstruzijom) radi uklanjanja neželjenih šavova pri filtriranju tekstura u 2D motorima (Godot, Bevy, Raylib, Defold), uz standardne Sprite Sheet JSON metapodatke i animirane GIF-ove.

---

## Snimke zaslona i vizualni pregled

<div align="center">

### 1. Glavno radno platno i studio
*Visokoperformansno radno okruženje za piksel grafiku s hardverski ubrzanim prikazom, višekanalnom vremenskom crtom, pretpregledom uživo u stvarnom vremenu, uzorcima paleta i preciznim alatnim trakama.*
<img src="screenshots/main.png" alt="Glavno radno okruženje OniAnvil" width="950" />

<br/><br/>

### 2. Višekanalna vremenska crta animacije
*Nelinearna višekanalna vremenska crta sa složenim redovima slojeva (`V1, V2...`), povezanim primjercima okvira (`🔗`), množiteljima zadržavanja i imenovanim obojenim oznakama animacija.*
<img src="screenshots/timeline.png" alt="Višekanalna vremenska crta OniAnvil" width="950" />

<br/><br/>

### 3. Precizni alati i alatna traka brzih radnji
*Potpuni paket od 15 alata za crtanje sa standardnim i kompaktnim načinom prikvačivanja, uz gornju traku za brzo podešavanje veličine kista, simetrije, prelamanja rubova i načina Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Alatna traka brzih radnji OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Standardna traka alata OniAnvil" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Kompaktna traka alata OniAnvil" width="300" />
</p>

<br/><br/>

### 4. Pretpregled animacije uživo u stvarnom vremenu
*Zasebni panel za reprodukciju uživo s filtriranjem po oznakama, prilagodbom brzine izmjene sličica (FPS), klizačem za zumiranje i prekidačima pozadine (šahovnica, tamna, svijetla).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Pretpregled uživo OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zumiranje u pretpregledu uživo OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Upravljanje paletama, zamjena boja i prijelazi
*Sveobuhvatni sustav boja s poljima za prednju i pozadinsku boju, automatskim generiranjem dvotočkastih prijelaza boja i imenovanim preoblikama paleta.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta i zamjena preoblika OniAnvil" width="950" />

<br/><br/>

### 6. Crtanje po referentnim skicama i prilagodljivi prozirni slojevi
*Učitajte idejne skice ili fotografije kao podlogu ili prekrivajući sloj s kontrolama transformacije, pametnim uzorkovanjem boja s prianjanjem na paletu i višeslojnim prozirnim prikazom okvira.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Skica i prozirni slojevi OniAnvil" width="950" />

<br/><br/>

### 7. Točke sidra i akcijske točke igranja
*Definirajte središte spritea (Pivot) i prilagođene imenovane točke za mehaniku igre (hvatišta oružja, izvori projektila, glava, koraci) uz interaktivne oznake na platnu.*
<img src="screenshots/pivot_and_action_points.png" alt="Sidro i točke radnje OniAnvil" width="950" />

<br/><br/>

### 8. Rezanje listova spriteova i prilagodba 9-dijelne mreže (9-Slice)
*Automatsko rezanje prepoznavanjem granica spriteova putem autokorelacije (npr. ciklus hoda mačke) te podešavanje margina skalabilne 9-dijelne mreže za korisničko sučelje.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Rezanje listova OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Podešavanje 9-dijelne mreže OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Efekti i filtri slika u stvarnom vremenu (uz pretpregled uživo)
*Svi dijalozi efekata sadrže pretpregled uživo izravno na platnu, interaktivne pipete i mogućnost poništavanja bez narušavanja povijesti radnji.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generator obruba" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Zamjena boje" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Nijansa, zasićenost i kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrični filtri" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Preslikavanje na paletu" width="460" />
</p>

<br/><br/>

### 10. Mreža platna i načini pozadine
*Besprijekorno prebacujte između mreže pojedinačnih piksela, granica okvira, tamne šahovnice, svijetle šahovnice i prilagođene pune boje pozadine.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Mreža i tamna šahovnica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Svijetla šahovnica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Puna pozadina po izboru" width="300" />
</p>

<br/><br/>

### 11. Čarobnjak za stvaranje novog projekta
*Konfigurirajte dimenzije platna, broj okvira, stupčani raspored mreže atlasa i birajte između više od 20 autentičnih retro paleta pri pokretanju projekta.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Čarobnjak novog projekta" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palete novog projekta" width="460" />
</p>

<br/><br/>

### 12. 35 autentičnih retro i modernih tema
*Uključuje Studio Dark, Studio Light, fosforne CRT monitore (Amber, Green, Cyan), retro konzole (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) i autorske teme.*
<p align="center">
  <img src="screenshots/themes.png" alt="Izbornik tema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Amber" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PSX" width="300" />
</p>

<br/><br/>

### 13. Sustav izbornika i ugrađena pomoć s tipkovničkim prečacima
<p align="center">
  <img src="screenshots/file_menu.png" alt="Izbornik Datoteka" width="180" />
  <img src="screenshots/edit_menu.png" alt="Izbornik Uredi" width="180" />
  <img src="screenshots/select_menu.png" alt="Izbornik Odabir" width="180" />
  <img src="screenshots/view_menu.png" alt="Izbornik Prikaz" width="180" />
  <img src="screenshots/project_menu.png" alt="Izbornik Projekt" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Dijalog pomoći s tipkovničkim prečacima" width="700" />
</p>

</div>

---

## Sveobuhvatni pregled alata

OniAnvil sadrži 15 preciznih alata za crtanje, oblike, odabir i metapodatke. Ikone se prikazuju pomoću biblioteke otvorenog koda **Font Awesome Free** *(ikone: CC BY 4.0, fontovi: SIL OFL 1.1)*.

| Alat | Ikona | Prečac | Veličina kista | Pixel Perfect | Opis |
|:---|:---:|:---:|:---:|:---:|:---|
| **Olovka** | ✏️ | <kbd>B</kbd> | 1..32 px | Da | Glavni alat za crtanje. Način rada 1px podržava funkciju **Pixel-Perfect** (automatski uklanja neželjene stepeničaste L-kutove). Podržava prilagođene kistove iz odabira (`Ctrl/Cmd+B`). |
| **Gumica** | 🧹 | <kbd>E</kbd> | 1..32 px | Ne | Briše piksele do potpune prozirnosti (`[0, 0, 0, 0]`). Ograničeno aktivnom maskom odabira ako postoji. |
| **Pipeta** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ne | Lijevi klik uzima boju u primarnu (prednju); desni klik u sekundarnu (pozadinsku). Može uzimati boju s platna, slojeva ili referentne skice. |
| **Kantica boje** | 🪣 | <kbd>G</kbd> | Nije primjenjivo | Ne | Brzo algoritamsko ispunjavanje redom (BFS) uz podesivi prag tolerancije (`0..=255`). Podržava susjedno i globalno ispunjavanje te ograničenje odabirom. |
| **Gradijent** | 🌈 | <kbd>G</kbd> *(prebacivanje)* | Nije primjenjivo | Ne | Linearni i radijalni prijelazi između prednje i pozadinske boje. Podržava Bayer (2x2, 4x4, 8x8) i Blue Noise dither uz regulaciju jačine. |
| **Crta** | 📏 | <kbd>L</kbd> | 1..32 px | Da | Bresenhamov algoritam ravnih linija s prikazom povlačenja uživo. Držite <kbd>Shift</kbd> za fiksiranje kutova pod koracima od 45°/90°. |
| **Krivulja** | 〰️ | <kbd>C</kbd> | 1..32 px | Da | Kvadratna Bézierova krivulja u 3 klika. Klik 1 postavlja početak, Klik 2 postavlja kraj, pomak miša određuje zakrivljenost, Klik 3 potvrđuje krivulju. |
| **Pravokutnik** | ⬜ | <kbd>U</kbd> | 1..32 px | Ne | Obrezani ili ispunjeni geometrijski pravokutnici. Držite <kbd>Shift</kbd> za zadržavanje omjera kvadrata 1:1. |
| **Elipsa** | ⭕ | <kbd>O</kbd> | 1..32 px | Ne | Obrezane ili ispunjene elipse uz rasterizaciju središnje točke. Držite <kbd>Shift</kbd> za savršen krug. |
| **Kist za dither** | 🏁 | <kbd>D</kbd> | 1..32 px | Ne | Crtanje šahovskog uzorka dither prijelaza između prednje i pozadinske boje bez dodavanja suvišnih unosa u paletu. |
| **Alat za sjenčanje** | 🌓 | <kbd>S</kbd> | 1..32 px | Ne | Postupno posvjetljivanje (Dodge) i potamnjivanje (Burn) piksela na temelju redoslijeda boja u aktivnoj paleti. |
| **Pravokutni odabir** | ⬚ | <kbd>M</kbd> | Nije primjenjivo | Ne | Pravokutni odabir područja s animiranim rubom marširajućih mrava. Podržava dodavanje (<kbd>Shift</kbd>) i oduzimanje (<kbd>Alt</kbd>). |
| **Laso odabir** | ➰ | <kbd>Q</kbd> | Nije primjenjivo | Ne | Slobodni poligonalni laso odabir. Koristi 8-povezanu rasterizaciju granice s 4-povezanim vanjskim ispunom, što jamči 100% puni odabir bez rupa čak i kod petlji. |
| **Čarobni štapić** | 🪄 | <kbd>W</kbd> | Nije primjenjivo | Ne | Odabir susjednih (ili globalnih) područja podudarne boje uz podesivi prag tolerancije. |
| **Sidro i radnje** | 🎯 | <kbd>P</kbd> | Nije primjenjivo | Ne | Interaktivno postavljanje središta spritea (Pivot) i imenovanih akcijskih točaka igranja (`weapon`, `head`, `feet`, `hitbox`) uz izvoz u JSON. |

---

## Funkcionalna dokumentacija

### 1. Dvojna arhitektura usmjerena na atlas

U alatu OniAnvil, glavni teksturni atlas (`atlas_rgba`) predstavlja stvarni list spriteova onako kako ga učitava vaš igraći motor:
- **Način rada okvira (Frame Mode)**: Usmjerava se na jedan aktivni okvir animacije, pružajući neometano platno za rad s pikselima. Sve izmjene odmah ažuriraju odgovarajuću ćeliju u teksturi atlasa.
- **Način rada atlasa (Atlas Mode)**: Prikazuje cjelokupnu mrežu lista spriteova, omogućujući makro prilagodbe, provjeru poravnanja između okvira i skupno uređivanje.
- **Popratna datoteka**: Projekti spremaju čiste `.png` slike praćene laganom popratnom metapodatkovnom datotekom `.onianvil.json` koja sadrži raspored okvira, hijerarhiju slojeva, oznake, množitelje zadržavanja i preoblike paleta.

### 2. Višekanalna vremenska crta slojeva

Donja vremenska crta oblikovana je po uzoru na profesionalne nelinearne video uređivače:
- **Trake slojeva (`V1, V2, V3...`)**: Slojevi su vizualno složeni odozgo prema dolje. Traka postavljena više na vremenskoj crti (`V1`) iscrtava se na vrhu radnog platna.
- **Kontrole slojeva**:
  - Kliknite na oznake `V1` / `V2` za brzu promjenu aktivnog sloja za crtanje.
  - Uključite/isključite vidljivost sloja (ikona oka) i zaključavanje uređivanja (ikona lokota).
  - Prilagodite prozirnost pojedinog sloja (`0%..=100%`).
  - Pomaknite sloj gore/dolje u hijerarhiji, duplicirajte sloj ili ga spojite prema dolje (Merge Down) u sloj ispod.
  - Sigurno brisanje sloja uz dijaloški okvir potvrde.
- **Isječci okvira (Frame Clips)**:
  - Množitelj zadržavanja (Hold): dodijelite prilagođeno trajanje okviru (npr. `2×`, `3×` zadržavanje za stanke ili snažne udarce).
  - Povezani okviri (`🔗`): povežite ponavljajuće okvire duž animacije tako da uređivanje jednog automatski ažurira sve povezane primjerke.
  - Ponavljanje oznaka: organizirajte animacije u imenovane oznake (`idle`, `walk`, `attack`) i reproducirajte određene oznake u petlji.
  - Kontrole reprodukcije: Naprijed, Natrag, Ping-Pong i Ponavljanje (Loop) uz podesivu brzinu kadrova (`1..=120` FPS).

### 3. Pretpregled animacije uživo u stvarnom vremenu

Desni panel za pretpregled reproducira vaše animacije u stvarnom vremenu, potpuno neovisno o zumiranju ili pomaku glavnog platna:
- **Filtriranje oznaka**: Ograničite reprodukciju na određene oznake (`walk`, `jump`) ili reproducirajte cijeli niz animacije.
- **Kontrole reprodukcije**: Pokreni, Pauziraj, Prethodni okvir, Sljedeći okvir, Uključivanje petlje i prilagođene brzine reprodukcije (`0.25x..=4.0x`).
- **Načini prikaza**: Prebacivanje između tamne šahovnice, svijetle šahovnice, čiste bijele, čiste crne ili prilagođene boje pozadine.

### 4. Napredni sustav prozirnih slojeva (Onion Skinning)

Oblikujte fluidne animacije pomoću namjenskog sustava prozirnih slojeva:
- **Prilagođeno nijansiranje**: Neovisne boje nijansiranja za prethodne okvire (zadano: crvena) i sljedeće okvire (zadano: plava).
- **Opadanje prozirnosti**: Podesiva osnovna prozirnost i krivulje postupnog opadanja prozirnosti za svaki sljedeći korak.
- **Prelazak preko petlje**: Omogućuje prozirni prikaz prvog okvira kroz zadnji prilikom izrade bešavnih petlji.
- **Brzi uvid (Quick-Peek)**: Držite tipku <kbd>Razmaknica</kbd> bez povlačenja za trenutni pretpregled susjednih okvira.

### 5. Pametni referentni sloj i crtanje po skicama

Ubrzajte svoj rad crtanjem preko ručno nacrtanih skica ili referentnih fotografija:
- **Formati**: Učitavanje referentnih slika u formatima PNG, JPEG i WebP.
- **Načini stapanja**: Podloga (ispod vaših pikselnih slojeva) ili Prekrivanje (iznad vaših slojeva).
- **Kontrole transformacije**: Neovisno zumiranje, pomak položaja, rotacija i klizač prozirnosti.
- **Pametno uzorkovanje boja**: Koristite pipetu na sloju skice uz neobavezno automatsko privlačenje najblizoj boji u aktivnoj paleti projekta.
- **Pretvaranje u piksele (1-Click Bake)**: Jednim klikom spojite skaliranu referentnu sliku izravno u piksel grafiku aktivnog okvira.

### 6. Precizni alati za odabir i marširajući mravi

- **Animirani marširajući mravi**: Aktivni odabiri prikazuju dinamičan, dvobojni isprekidani rub visoke vidljivosti (poput programa Photoshop ili Aseprite).
- **Puni laso odabir s vanjskim ispunom**: Za razliku od jednostavnih algoritama praćenja zraka koji stvaraju rupe kod preklapajućih petlji, OniAnvilov laso gradi zatvorenu 8-povezanu granicu i ispunjava vanjski prostor, čime osigurava 100% puni odabir bez šupljina.
- **Radnje s odabirom**: Proširi, Skupi, Rub (s kružnim, kvadratnim ili romboidnim oblikom jezgre), Premjesti, Preokreni, Obriši, Slobodno zakreni te Kopiraj/Izreži/Zalijepi putem međuspremnika.
- **Kopiranje i lijepljenje između okvira**: Odaberite područje na jednom okviru, pritisnite <kbd>Ctrl/Cmd+C</kbd>, promijenite okvir i pritisnite <kbd>Ctrl/Cmd+V</kbd> — pikseli se lijepe točno na istim relativnim koordinatama bez obzira na raspored tipkovnice.

### 7. Efekti uživo, filtri i RotSprite

Svi slikovni efekti uključuju potvrdni okvir **Pretpregled uživo** koji prikazuje promjene na platnu u stvarnom vremenu i omogućuje jednostavno odustajanje bez zagađivanja povijesti radnji:
- **Zamjena boje (`Shift+R`)**:
  - Interaktivna pipeta: kliknite ikonu pipete i odaberite boje izravno s platna ili iz panela palete.
  - Traka za brzi odabir iz palete: lijevi klik bira boju "Iz", desni klik bira boju "U".
  - Klizač tolerancije i skupna zamjena na svim okvirima animacije odjednom.
- **Prilagodba boja (`Ctrl/Cmd+U`)**: Podešavanje HSV nijanse (Hue), zasićenosti (Saturation), svjetline (Value) i kontrasta u stvarnom vremenu.
- **Generator automatskog obruba**: Stvaranje čistih pikselnih obruba likova (Izvana, Iznutra, Po sredini) s pravokutnim ili kosim (romb) kutovima.
- **Matrični filtri 3×3**: Zamućivanje (Blur), Izoštravanje (Sharpen), Prepoznavanje rubova (Edge Detect) i Reljef (Emboss) uz očuvanje alfa kanala.
- **Slobodno zakretanje (`Ctrl/Cmd+Shift+R`)**: Rotirajte odabrana područja ili cijele okvire pod bilo kojim kutom pomoću klasičnog algoritma **RotSprite** ili Nearest Neighbor.
- **Preslikavanje na paletu**: Prebacuje boje platna u aktivnu paletu pomoću Bayer dither uzorka (2x2, 4x4, 8x8) ili Blue Noise algoritma.

### 8. Upravljanje paletama, zamjena boja i preoblike likova

- **Ugrađene palete**: Više od 20 autentičnih retro paleta uključujući Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized i Sweetie 16.
- **Uvoz i izvoz**: Podrška za GIMP palete (`.gpl`), PAL (`.pal`), Hex zapise (`.hex`) i slikovne trake uzoraka (`.png`).
- **Zamjena paleta (Preoblike)**: Definirajte više varijanti izgleda za jedan lik (npr. Zadano, Zlatni oklop, Sjena, Ledeno). Promjena preoblike trenutačno preslikava sve boje okvira u cijelom atlasu.
- **Generator prijelaza boja**: Automatski stvara glatke gradijente između dvije odabrane nijanse uz podesiv broj međukoraka.

### 9. Rezanje listova, 9-dijelna mreža sučelja i izvoz za igraće motore

- **Rezanje listova (Sheet Slicer)**: Automatsko otkrivanje granica autokorelacijom reže sirove listove spriteova u ujednačene ili točno obrezane okvire animacije.
- **9-dijelno skaliranje (9-Slice)**: Postavite podesive margine (gore, dolje, lijevo, desno) za pretpregled prilagodljivih okvira sučelja i gumba bez izobličenja kutova.
- **Izvoz PNG-a s proširenim rubnim pikselima**: Proširuje boje rubnih piksela u prozirna područja čime se uklanjaju neželjeni šavovi i artefakti filtriranja tekstura u grafičkim sustavima OpenGL, DirectX, Vulkan i motorima za 2D igre (kao što je Godot).
- **Izvoz animiranih GIF-ova**: Generira optimizirane animirane GIF datoteke s prilagođenim brojem ponavljanja, kašnjenjem okvira i prozirnim pozadinama.
- **Standardni Sprite Sheet JSON**: Izvozi koordinate okvira, oznake, trajanja i točke sidra usklađene sa standardnom JSON shemom koja se koristi u modernim igraćim motorima.

### 10. 35 ergonomskih tema i potpuna lokalizacija

- **Kolekcija tema**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, fosforni CRT monitori (Amber, Green, Cyan), retro konzole (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) i retro računala (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Potpuna višejezična podrška**: Trenutačno prebacivanje između 37 jezika uz 100% pokrivenost svih izraza i opisa.

---

## Kontrole i tipkovnički prečaci

| Prečac | Alat / Radnja | Opis |
|:---|:---|:---|
| <kbd>B</kbd> | **Olovka** | Precizno crtanje 1px uz opcionalni način Pixel-Perfect |
| <kbd>E</kbd> | **Gumica** | Brisanje piksela do prozirnosti |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipeta** | Uzimanje boje s piksela platna ili referentne skice |
| <kbd>G</kbd> | **Kantica boje / Gradijent** | Prebacivanje između kantice boje i dither gradijenta |
| <kbd>L</kbd> | **Crta** | Crtanje ravnih linija uz pretpregled uživo i fiksiranje kuta uz Shift |
| <kbd>C</kbd> | **Krivulja** | Alat za kvadratnu Bézierovu krivulju u 3 klika |
| <kbd>U</kbd> | **Pravokutnik** | Geometrijski pravokutnik i kvadrat s obrubom ili ispunom |
| <kbd>O</kbd> | **Elipsa** | Geometrijska elipsa i krug s obrubom ili ispunom |
| <kbd>D</kbd> | **Kist za dither** | Crtanje šahovskog uzorka dither prijelaza |
| <kbd>S</kbd> | **Alat za sjenčanje** | Postupno posvjetljivanje ili potamnjivanje duž prijelaza palete |
| <kbd>M</kbd> | **Pravokutni odabir** | Pravokutni odabir područja uz marširajuće mrave |
| <kbd>Q</kbd> | **Laso odabir** | Slobodni puni poligonalni odabir područja |
| <kbd>W</kbd> | **Čarobni štapić** | Odabir susjednih područja iste boje |
| <kbd>P</kbd> | **Točke sidra** | Postavljanje središta spritea i točaka radnje |
| <kbd>X</kbd> | **Zamijeni boje** | Zamjena primarne (prednje) i sekundarne (pozadinske) boje |
| <kbd>[</kbd> / <kbd>]</kbd> | **Veličina kista** | Smanjivanje / Povećavanje veličine kista (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopiraj** | Kopiranje odabranih piksela u sistemski međuspremnik |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Izreži** | Izrezivanje odabranih piksela u međuspremnik |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Zalijepi** | Lijepljenje piksela iz međuspremnika u aktivni okvir |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Obriši** | Brisanje piksela unutar aktivnog odabira |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Odaberi sve** | Odabir svih piksela na aktivnom okviru |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Poništi odabir** | Uklanjanje trenutne maske odabira |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Preokreni odabir** | Preokretanje trenutne maske odabira |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dupliciraj okvir** | Dupliciranje aktivnog okvira na vremenskoj crti |
| <kbd>Razmaknica</kbd> | **Pokreni / Pauziraj** | Pokretanje ili zaustavljanje reprodukcije animacije |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Zamjena boje** | Otvaranje dijaloga za zamjenu boje uz pretpregled uživo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Prilagodba boja** | Otvaranje dijaloga za prilagodbu HSV nijanse i kontrasta |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Slobodno zakretanje** | Otvaranje dijaloga za proizvoljno zakretanje (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Poništi** | Poništavanje zadnjeg poteza na platnu ili radnje alata |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Ponovi** | Ponovno primjenjivanje poništene radnje |
| <kbd>Razmaknica</kbd> + Povlačenje / Srednji klik | **Pomicanje platna** | Glatko beskonačno kretanje koordinatama platna |
| <kbd>Ctrl/Cmd</kbd> + Kotačić / <kbd>+</kbd> / <kbd>-</kbd> | **Zumiranje** | Čisto zumiranje piksela centrirano oko pokazivača miša |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Način prikaza** | Prebacivanje između prikaza okvira (<kbd>1</kbd>) i prikaza atlasa (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Prilagodi platno** | Automatsko centriranje i prilagodba okvira središtu radnog prostora |
| <kbd>F1</kbd> | **Pomoć** | Otvaranje pregleda tipkovničkih prečaca |

---

## Kako započeti

### Samostalne binarne datoteke

Preuzmite unaprijed kompilirane, samostalne izvršne datoteke sa **[stranice izdanja na GitHubu](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Preuzmite `OniAnvil-windows-x86_64.zip`, raspakirajte arhivu i dvokliknite `OniAnvil.exe`.
- **Linux**: Preuzmite `OniAnvil-linux-x86_64.tar.gz` (prijenosna verzija) ili instalirajte Debian/Ubuntu paket:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Preuzmite `OniAnvil-macos-universal.zip`, raspakirajte i povucite `OniAnvil.app` u mapu Applications. Univerzalna binarna datoteka izvorno radi i na Apple Silicon čipovima (M1/M2/M3/M4) i na Intel Mac računalima.
- **Web**: Pokrenite izravno u pregledniku bez ikakve instalacije: **[Pokreni WebAssembly aplikaciju](https://rbm13s.github.io/OniAnvil/app/?lang=hr)**.
  > [!WARNING]
  > WebAssembly (internetska) verzija ima status **Eksperimentalno**. Zbog ograničenja sigurnosnog pješčanika preglednika, razlika u WebGL/WebGPU grafičkim podsustavima i ograničenja međuspremnika, mogu se pojaviti manje grafičke nepravilnosti ili slabije performanse u usporedbi s izvornom verzijom za radnu površinu. Za profesionalni rad snažno se preporučuje izvorna inačica za računala.

---

### Izgradnja iz izvornog koda

#### Preduvjeti
- **Rust razvojni alati**: Instalirajte Rust (preporučuje se verzija 1.80 ili novija) putem [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Zavisnosti za izgradnju na Linuxu** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompiliranje i pokretanje izvorne verzije za računala
```bash
# Klonirajte repozitorij
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Pokrenite testove
cargo test

# Izgradite i pokrenite optimiziranu distribucijsku verziju
cargo run --release
```

#### 2. Kompiliranje WebAssembly verzije (putem alata Trunk)
```bash
# Dodajte ciljnu platformu wasm32 i alat Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Pokrenite lokalni razvojni poslužitelj s automatskim osvježavanjem
trunk serve

# Izgradite optimizirane datoteke za objavu u mapu docs/app/
./build_wasm.sh        # Na Linuxu/macOS-u
./build_wasm.ps1       # Na sustavu Windows PowerShell
```

---

## Format datoteka i popratna arhitektura

OniAnvil izbjegava zatvorene vlasničke binarne formate. Svaki projekt sastoji se od dvije čiste datoteke pogodne za verzioniranje (Git):

```
my_character/
├── my_character.png            # Standardni PNG teksturni atlas spriteova (otvara se u bilo kojem motoru ili pregledniku)
└── my_character.onianvil.json   # Popratna JSON datoteka s metapodacima projekta
```

Popratna datoteka `.onianvil.json` pohranjuje:
- Konfiguraciju mreže platna: dimenzije okvira, broj stupaca, ukupan broj okvira.
- Hijerarhiju slojeva: nazive traka, razine prozirnosti, zastavice vidljivosti i stanja zaključavanja.
- Oznake animacija: naziv, početni/završni okvir, smjer reprodukcije (Naprijed, Natrag, Ping-Pong), trajanje okvira.
- Definicije paleta: aktivne uzorke boja, imenovane varijante preoblika, indeks trenutne preoblike.
- Stanje referentne skice: putanju slike, mjerilo, pomak, kut rotacije, prozirnost, način rada sloja.
- Točke sidra i radnje: prilagođene točke za mehaniku, hvatišta oružja i centre sudara.

---

## Doprinos i zahvale

Doprinosi, prijave grešaka i prijedlozi novih mogućnosti toplo su dobrodošli!
1. Napravite fork repozitorija na GitHubu.
2. Stvorite granu za novu mogućnost: `git checkout -b feature/moja-nova-znacajka`.
3. Spremite promjene: `git commit -am 'Dodan novi alat'`.
4. Pošaljite granu na svoj fork: `git push origin feature/moja-nova-znacajka`.
5. Otvorite zahtjev za spajanje (Pull Request).

### Zahvale
- Početni pokazni animirani sprite mačke **"Cat 2D Pixel Art"** izradio je autor **xzany** ([stranica projekta na itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Izgrađeno uz pomoć izvrsne biblioteke za korisničko sučelje **[egui](https://github.com/emilk/egui)** i grafičkog sustava **[wgpu](https://wgpu.rs/)**.
- Ikone sučelja omogućuje otvoreni paket **[Font Awesome Free](https://fontawesome.com)** autora @fontawesome (ikone: CC BY 4.0, fontovi: SIL OFL 1.1, kod: MIT licenca).

---

## Licenca i odricanje od odgovornosti

Ovaj projekt licenciran je pod uvjetima **[MIT licence](../LICENSE)**. Slobodni ste koristiti, modificirati, distribuirati i integrirati OniAnvil u komercijalne i osobne razvojne projekte.

### Odricanje od jamstva i ograničenje odgovornosti
> [!IMPORTANT]
> Program **OniAnvil** distribuira se **besplatno** po načelu **"KAKO JEST" ("AS IS")** i **"KAKO JE DOSTUPNO" ("AS AVAILABLE")**, bez ikakvih izričitih ili podrazumijevanih jamstava ili uvjeta.
> 
> Ni pod kojim okolnostima autor(i), nositelji autorskih prava ili suradnici na projektu **ne snose odgovornost** za bilo kakve izravne, neizravne, slučajne, posebne, kaznene ili posljedične štete (uključujući, ali ne ograničavajući se na, gubitak podataka, oštećenje datoteka projekta, kvar hardvera, izgubljenu dobit ili prekid poslovanja) koje su na bilo koji način proizašle iz korištenja ili nemogućnosti korištenja ovog softvera, čak i ako su bili obaviješteni o mogućnosti takve štete. Korisnik preuzima svu odgovornost i rizik korištenja ovog programa.

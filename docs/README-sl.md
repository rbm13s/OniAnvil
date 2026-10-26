<div align="center">

<img src="assets/icon.jpg" alt="Logotip OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio za slikovno pikčasto risanje in animacijo čebulne lupine (Onion-Skinning) za igralne atlase**

[![Gradnja in izdaja](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Prenosi](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Izrisovalnik](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Jeziki](https://img.shields.io/badge/Jeziki-37_Jezikov-purple)](#10-35-ergonomskih-tem-in-popolna-lokalizacija)
[![Platforme](https://img.shields.io/badge/Platforme-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#kako-začeti)
[![Spletna aplikacija](https://img.shields.io/badge/Spletna_aplikacija-Preizkusi_v_brskalniku_(Eksperimentalno)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sl)
[![Licenca: MIT](https://img.shields.io/badge/Licenca-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Preizkusite OniAnvil v brskalniku (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sl)** *(⚠️ Eksperimentalna različica: lahko vsebuje omejitve WebGL/WASM in hrošče)* • **[🌐 Uradna spletna stran dokumentacije](https://rbm13s.github.io/OniAnvil/sl.html)**

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
**Slovenščina** •
[Български](README-bg.md) •
[Српски](README-sr.md) •
[Hrvatski](README-hr.md) •
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

## Kazalo vsebine

- [Uvod](#uvod)
- [Posnetki zaslona in vizualni pregled](#posnetki-zaslona-in-vizualni-pregled)
- [Celovit pregled orodij](#celovit-pregled-orodij)
- [Funkcionalna dokumentacija](#funkcionalna-dokumentacija)
  - [1. Dvojna arhitektura, osredotočena na atlas](#1-dvojna-arhitektura-osredotočena-na-atlas)
  - [2. Večtirna časovnica plasti](#2-večtirna-časovnica-plasti)
  - [3. Predogled animacije v živo v realnem času](#3-predogled-animacije-v-živo-v-realnem-času)
  - [4. Napredni mehanizem čebulne lupine (Onion Skinning)](#4-napredni-mehanizem-čebulne-lupine-onion-skinning)
  - [5. Pametna referenčna plast in risanje po skicah](#5-pametna-referenčna-plast-in-risanje-po-skicah)
  - [6. Natančna orodja za izbiro in premikajoče se mravlje](#6-natančna-orodja-za-izbiro-in-premikajoče-se-mravlje)
  - [7. Učinki v živo, filtri in RotSprite](#7-učinki-v-živo-filtri-in-rotsprite)
  - [8. Upravljanje palet, zamenjava barv in preobleke likov](#8-upravljanje-palet-zamenjava-barv-in-preobleke-likov)
  - [9. Razrezovalnik listov, 9-delna mreža UI in izvoz za igralne pogone](#9-razrezovalnik-listov-9-delna-mreža-ui-in-izvoz-za-igralne-pogone)
  - [10. 35 ergonomskih tem in popolna lokalizacija](#10-35-ergonomskih-tem-in-popolna-lokalizacija)
- [Nadzor in tipkovnične bližnjice](#nadzor-in-tipkovnične-bližnjice)
- [Kako začeti](#kako-začeti)
  - [Samostojne dvojiške datoteke](#samostojne-dvojiške-datoteke)
  - [Gradnja iz izvorne kode](#gradnja-iz-izvorne-kode)
- [Oblika datotek in spremljevalna arhitektura](#oblika-datotek-in-spremljevalna-arhitektura)
- [Prispevanje in zahvale](#prispevanje-in-zahvale)
- [Licenca in omejitev odgovornosti](#licenca-in-omejitev-odgovornosti)

---

## Uvod

**OniAnvil** (*ime izhaja iz **Oni**on-Skinning (čebulna lupina) + **Anvil** (nakovalo za kovanje spriteov)*) je odprtokodni, visoko zmogljiv urejevalnik slikovnih pik in paket za animacijo, zgrajen v programskem jeziku Rust. Izvorno se izvaja s strojno pospešenim upodabljanjem v sistemih Windows, Linux in macOS ter kot spletna aplikacija WebAssembly brez namestitve neposredno v sodobnih spletnih brskalnikih.

Večina obstoječih urejevalnikov spriteov obravnava animacijo kot sklad nepovezanih slikovnih datotek, kar razvijalce sili v zamudne postopke izvoza, razreza in ponovnega pakiranja. **OniAnvil ta pristop postavlja na glavo:** teksturni atlas lista spriteov za igre (`atlas_rgba`) je primarni vir resnice. Celice sličic, sledi plasti, animacijske oznake in točke trkov se ves čas nedestruktivno sinhronizirajo z atlasom.

### Zakaj OniAnvil?
- 🚀 **Izjemna izvorna zmogljivost v Rustu**: Takojšen zagon pod milisekundo, ostro upodabljanje z `wgpu` in `egui 0.31`, ničelni zamik pri pomikanju po dolgih časovnicah.
- 🎬 **Večtirna časovnica plasti**: Nelinearne plasti (`V1, V2, V3...`) s sestavljanjem od zgoraj navzdol, združevanjem navzdol, povezanimi sličicami in nastavljivim trajanjem zadržanja.
- 🧅 **Prvovrstna čebulna lupina (Onion Skinning)**: Prikaz več sosednjih sličic z neodvisnim obarvanjem naprej/nazaj, prilagodljivimi krivuljami prosojnosti in ovijanjem zank.
- 🎨 **Takojšnje preobleke palet likov**: Ustvarite dnevne/nočne različice, ekipe ali elementarne preobleke s sprotnim prekodiranjem atlasa ter uvozom in izvozom formatov GPL/PAL/HEX.
- 📦 **Neposreden izvoz za igralne pogone**: Izvozite umerjene PNG-je z raztegnjenimi robovi (ekstruzijo robnih slikovnih pik), ki preprečujejo neželeno prelivanje tekstur v 2D pogonih (Godot, Bevy, Raylib, Defold), skupaj s standardnimi metapodatki Sprite Sheet JSON in animiranimi GIF-i.

---

## Posnetki zaslona in vizualni pregled

<div align="center">

### 1. Glavno delovno platno in studio
*Visokozmogljivo delovno okolje za slikovne pike s strojno pospešenim pogledom, večtirno časovnico, predogledom v živo v realnem času, vzorci palet in natančnimi orodnimi vrsticami.*
<img src="screenshots/main.png" alt="Glavno delovno okolje OniAnvil" width="950" />

<br/><br/>

### 2. Večtirna časovnica animacije
*Nelinearna večtirna časovnica z naloženimi vrsticami plasti (`V1, V2...`), povezanimi primerki sličic (`🔗`), množitelji zadržanja in poimenovanimi barvnimi oznakami animacij.*
<img src="screenshots/timeline.png" alt="Večtirna časovnica OniAnvil" width="950" />

<br/><br/>

### 3. Natančna orodja in orodna vrstica dejanj
*Popoln nabor 15 orodij za risanje s standardnim in strnjenim načinom pripenjanja ter zgornjo vrstico za hitre nastavitve velikosti čopiča, simetrije, ovijanja robov in stikala Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Orodna vrstica za hitre nastavitve OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Standardna orodna vrstica OniAnvil" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Strnjena orodna vrstica OniAnvil" width="300" />
</p>

<br/><br/>

### 4. Predogled animacije v živo v realnem času
*Namensko podokno za predvajanje v živo s filtriranjem po oznakah, prilagodljivim nadzorom hitrosti sličic (FPS), drsnikom za povečavo in preklapljanjem ozadja (šahovnica, temno, svetlo).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Predogled v živo OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Povečava predogleda v živo OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Upravljanje palet, zamenjava barv in barvni prehodi
*Izpopolnjen barvni mehanizem s polji za osprednjo/ozadno barvo, samodejnim ustvarjanjem dvotočkovnih barvnih prehodov in poimenovanimi preoblekami palet likov.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta in zamenjava preoblek OniAnvil" width="950" />

<br/><br/>

### 6. Risanje po referenčnih skicah in prilagodljiva čebulna lupina
*Naložite idejne skice ali fotografije kot podlago ali prekrivno plast s transformacijskimi kontrolami, pametnim vzorčenjem barv s pripenjanjem na paleto in večslično čebulno lupino.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Skica in čebulna lupina OniAnvil" width="950" />

<br/><br/>

### 7. Vrtišča in točke trkov ter dejanj
*Določite središče spritea (Pivot) in prilagojene poimenovane točke za mehaniko iger (sidra za orožje, izstrelke, glavo, korake) z interaktivnimi prijemali na platnu.*
<img src="screenshots/pivot_and_action_points.png" alt="Vrtišče in točke dejanj OniAnvil" width="950" />

<br/><br/>

### 8. Razrezovalnik listov spriteov in prilagajanje 9-delne mreže (9-Slice)
*Samodejni razrez z zaznavanjem meja spriteov prek avtokorelacije (npr. cikel hoje mačke) ter nastavljanje robov raztegljive 9-delne mreže za uporabniški vmesnik.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Razrezovalnik listov OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Prilagajanje 9-delne mreže OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Učinki in filtri slik v realnem času (s predogledom v živo)
*Vsa pogovorna okna z učinki ponujajo predogled v živo neposredno na platnu, interaktivne kapalke in možnost preklica brez popačenja zgodovine razveljavitev.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generator obrisov" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Zamenjava barve" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Otenek, nasičenost in kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrični filtri" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Prekodiranje v paleto" width="460" />
</p>

<br/><br/>

### 10. Mreža platna in načini ozadja
*Nemoteno preklapljajte med mrežo posameznih slikovnih pik, mrežo sličic, temno šahovnico, svetlo šahovnico in polnimi enobarvnimi ozadji po meri.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Mreža in temna šahovnica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Svetla šahovnica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Enobarvno ozadje po meri" width="300" />
</p>

<br/><br/>

### 11. Čarovnik za ustvarjanje novega projekta
*Nastavite mere platna projekta, število sličic, stolpično postavitev mreže in ob zagonu izberite med več kot 20 pristnimi retro paletami.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Čarovnik za nov projekt" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palete novega projekta" width="460" />
</p>

<br/><br/>

### 12. 35 pristnih retro in sodobnih tem
*Vključuje Studio Dark, Studio Light, CRT Amber/Green/Cyan fosforne zaslone, retro konzole (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) in dizajnerske teme.*
<p align="center">
  <img src="screenshots/themes.png" alt="Meni tem" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Amber" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PSX" width="300" />
</p>

<br/><br/>

### 13. Menijski sistem in vgrajena pomoč s tipkovničnimi bližnjicami
<p align="center">
  <img src="screenshots/file_menu.png" alt="Meni Datoteka" width="180" />
  <img src="screenshots/edit_menu.png" alt="Meni Uredi" width="180" />
  <img src="screenshots/select_menu.png" alt="Meni Izbira" width="180" />
  <img src="screenshots/view_menu.png" alt="Meni Pogled" width="180" />
  <img src="screenshots/project_menu.png" alt="Meni Projekt" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Pomoč z bližnjicami na tipkovnici" width="700" />
</p>

</div>

---

## Celovit pregled orodij

OniAnvil ponuja 15 natančnih orodij za risanje, oblike, izbiro in metapodatke. Ikone uporabljajo odprtokodno zbirko **Font Awesome Free** *(ikone: CC BY 4.0, pisave: SIL OFL 1.1)*.

| Orodje | Ikona | Bližnjica | Velikost čopiča | Pixel Perfect | Opis |
|:---|:---:|:---:|:---:|:---:|:---|
| **Svinčnik** | ✏️ | <kbd>B</kbd> | 1..32 px | Da | Glavno risalno orodje. Način 1px podpira funkcijo **Pixel-Perfect** (samodejno odpravlja neželene stopničaste L-vogale). Podpira lastne čopiče iz izbire (`Ctrl/Cmd+B`). |
| **Radirka** | 🧹 | <kbd>E</kbd> | 1..32 px | Ne | Izbriše slikovne pike nazaj v prosojnost (`[0, 0, 0, 0]`). Omejeno z masko izbire, če je ta aktivna. |
| **Kapalka** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ne | Levi klik vzorči barvo v primarno (ospredje); desni klik vzorči v sekundarno (ozadje). Vzorči lahko s platna, plasti ali referenčne skice. |
| **Vedro z barvo** | 🪣 | <kbd>G</kbd> | Ni na voljo | Ne | Hitro algoritemsko zapolnjevanje z vrsto (BFS) z nastavljivo barvno toleranco (`0..=255`). Podpira zvezno in globalno polnjenje ter omejitev z izbiro. |
| **Barvni prehod** | 🌈 | <kbd>G</kbd> *(preklop)* | Ni na voljo | Ne | Linearni in radialni barvni prehodi med barvo ospredja in ozadja. Podpira Bayerjev (2x2, 4x4, 8x8) in Blue Noise dither z nastavitvijo intenzivnosti. |
| **Črta** | 📏 | <kbd>L</kbd> | 1..32 px | Da | Bresenhamov algoritem za ravne črte s sprotnim predogledom vlečenja. Pridržite <kbd>Shift</kbd> za zaklepanje kotov na korake po 45°/90°. |
| **Krivulja** | 〰️ | <kbd>C</kbd> | 1..32 px | Da | Kvadratna Bezierova krivulja s 3 kliki. Klik 1 določi začetek, Klik 2 določi konec, premik miške upogne ukrivljenost, Klik 3 potrdi krivuljo. |
| **Pravokotnik** | ⬜ | <kbd>U</kbd> | 1..32 px | Ne | Obrisani ali zapolnjeni geometrijski pravokotniki. Pridržite <kbd>Shift</kbd> za ohranitev razmerja kvadrata 1:1. |
| **Elipsa** | ⭕ | <kbd>O</kbd> | 1..32 px | Ne | Obrisane ali zapolnjene elipse z algoritmom sredinske točke. Pridržite <kbd>Shift</kbd> za popoln krog. |
| **Čopič za dither** | 🏁 | <kbd>D</kbd> | 1..32 px | Ne | Riše izmenične šahovnične dither vzorce med osprednjo in ozadno barvo brez dodajanja odvečnih barv v paleto. |
| **Orodje za senčenje** | 🌓 | <kbd>S</kbd> | 1..32 px | Ne | Postopno posvetljevanje (Dodge) in temnenje (Burn) glede na zaporedje barvnih odtenkov v aktivni paleti. |
| **Pravokotna izbira** | ⬚ | <kbd>M</kbd> | Ni na voljo | Ne | Pravokotna izbira območja z animirano mejo premikajočih se mravelj. Podpira dodajanje (<kbd>Shift</kbd>) in odštevanje (<kbd>Alt</kbd>). |
| **Lasasta izbira** | ➰ | <kbd>Q</kbd> | Ni na voljo | Ne | Prosto večkotno laso. Uporablja 8-povezano rasterizacijo meje s 4-povezanim zunanjim polnjenjem, kar zagotavlja 100% polno izbiro brez lukenj tudi pri zankah. |
| **Čarobna paličica** | 🪄 | <kbd>W</kbd> | Ni na voljo | Ne | Izbere zvezna (ali globalna) območja ujemajoče se barve z nastavljivim pragom tolerance. |
| **Vrtišče in dejanja** | 🎯 | <kbd>P</kbd> | Ni na voljo | Ne | Interaktivno postavljanje izhodišča spritea (Pivot) in poimenovanih akcijskih točk igranja (`weapon`, `head`, `feet`, `hitbox`) z izvozom v JSON. |

---

## Funkcionalna dokumentacija

### 1. Dvojna arhitektura, osredotočena na atlas

V OniAnvilu glavno teksturo atlasa (`atlas_rgba`) predstavlja dejanski list spriteov, kot ga naloži vaš igralni pogon:
- **Način sličice (Frame Mode)**: Osredotoči se na posamezno aktivno sličico animacije in ponuja nemoteno platno za delo s slikovnimi pikami. Vsaka sprememba nemudoma posodobi ustrezno celico v teksturi atlasa.
- **Način atlasa (Atlas Mode)**: Prikaže celotno mrežo lista spriteov, kar omogoča makro prilagoditve, preverjanje poravnave med sličicami in paketno urejanje.
- **Spremljevalna datoteka**: Projekti shranjujejo čiste slikovne datoteke `.png`, ki jih spremlja lahka metapodatkovna datoteka `.onianvil.json` z razporeditvijo sličic, hierarhijo plasti, oznakami, množitelji zadržanja in preoblekami palet.

### 2. Večtirna časovnica plasti

Spodnja časovnica je zasnovana po vzoru profesionalnih nelinearnih video urejevalnikov:
- **Sledi plasti (`V1, V2, V3...`)**: Plasti so vizualno zložene od zgoraj navzdol. Sled, postavljena višje na časovnici (`V1`), se na platnu izriše na vrhu.
- **Nadzor plasti**:
  - Kliknite na oznake `V1` / `V2` za hiter preklop aktivne risalne plasti.
  - Vklop/izklop vidnosti plasti (ikona očesa) in zaklepanje urejanja (ikona ključavnice).
  - Nastavitev prosojnosti posamezne plasti (`0%..=100%`).
  - Premikanje plasti navzgor/navzdol v hierarhiji, podvajanje in združevanje navzdol (Merge Down) v spodnjo sled.
  - Varno brisanje plasti s potrditvenim oknom.
- **Posamezne sličice (Frame Clips)**:
  - Množitelj zadržanja (Hold): dodelite trajanje sličicam po meri (npr. `2×`, `3×` zadržanje za premore ali poudarjene udarce).
  - Povezane sličice (`🔗`): povežite ponavljajoče se sličice vzdolž animacije, tako da urejanje ene samodejno posodobi vse povezane primerke.
  - Zankanje oznak: organizirajte animacije v poimenovane oznake (`idle`, `walk`, `attack`) in predvajajte določene oznake v zanki.
  - Nadzor predvajanja: naprej, nazaj, ping-pong in zanka z nastavljivo hitrostjo sličic (`1..=120` FPS).

### 3. Predogled animacije v živo v realnem času

Desno podokno za predogled predvaja vaše animacije v realnem času, povsem neodvisno od povečave ali položaja glavnega platna:
- **Filtriranje oznak**: Omejite predvajanje na posamezne oznake (`walk`, `jump`) ali predvajajte celotno zaporedje animacije.
- **Nadzor predvajanja**: Predvajaj, Premor, Prejšnja sličica, Naslednja sličica, Vklop zanke in hitrosti predvajanja po meri (`0.25x..=4.0x`).
- **Načini prikaza**: Preklapljajte med temno šahovnico, svetlo šahovnico, čisto belo, čisto črno ali ozadjem po meri.

### 4. Napredni mehanizem čebulne lupine (Onion Skinning)

Ustvarjajte tekoče animacije z namenskim sistemom čebulne lupine:
- **Barvanje po meri**: Neodvisni barvni odtenki za prejšnje sličice (privzeto: rdeča) in naslednje sličice (privzeto: modra).
- **Pojemanje prosojnosti**: Nastavljiva začetna prosojnost in postopne krivulje zmanjševanja prosojnosti za vsak naslednji korak.
- **Ovijanje zanke**: Omogoča prosojni prikaz prve sličice skozi zadnjo pri ustvarjanju neskončnih zank.
- **Hitri ogled (Quick-Peek)**: Pridržite tipko <kbd>Preslednica</kbd> brez vlečenja za takojšen predogled sosednjih sličic.

### 5. Pametna referenčna plast in risanje po skicah

Pospešite svoje delo z risanjem prek ročno narisanih skic ali referenčnih fotografij:
- **Formati**: Nalaganje referenčnih slik v formatih PNG, JPEG in WebP.
- **Načini mešanja**: Podloga (pod vašimi piksel plastmi) ali Prekritje (nad vašimi plastmi).
- **Transformacije**: Neodvisna povečava, zamik položaja, vrtenje in drsnik za prosojnost.
- **Pametno vzorčenje barv**: Uporabite kapalko na plasti skice z možnostjo samodejnega pripenjanja na najbližjo barvo v aktivni paleti projekta.
- **Pretvorba v slikovne pike (1-Click Bake)**: Z enim klikom zapečete umerjeno referenčno sliko neposredno v slikovne pike aktivne sličice.

### 6. Natančna orodja za izbiro in premikajoče se mravlje

- **Animirane premikajoče se mravlje**: Aktivne izbire prikazujejo dinamično, visoko vidno dvobarvno črtkano mejo (kot v programih Photoshop ali Aseprite).
- **Polno laso z zunanjim polnjenjem**: V nasprotju z naivnimi algoritmi sledenja žarkom, ki ustvarjajo luknje pri prekrivajočih se zankah, OniAnvilovo laso zgradi sklenjeno 8-povezano mejo in zapolni zunanjost, kar zagotavlja 100% polno izbiro brez lukenj.
- **Operacije z izbiro**: Razširi, Skrči, Rob (z okroglo, kvadratno ali romboidno obliko jedra), Premakni, Preobrni, Izbriši, Prosto zavrti ter Kopiraj/Izreži/Prilepi prek sistemskega odložišča.
- **Kopiranje in lepljenje med sličicami**: Izberite območje na eni sličici, pritisnite <kbd>Ctrl/Cmd+C</kbd>, preklopite na drugo sličico in pritisnite <kbd>Ctrl/Cmd+V</kbd> — slikovne pike se prilepijo na natanko enakih relativnih koordinatah ne glede na razporeditev tipkovnice.

### 7. Učinki v živo, filtri in RotSprite

Vsi slikovni učinki vključujejo potrditveno polje **Predogled v živo**, ki sprotno prikazuje spremembe na platnu in omogoča brezhiben preklic brez smetenja po zgodovini razveljavitev:
- **Zamenjava barve (`Shift+R`)**:
  - Interaktivna kapalka: kliknite ikono kapalke in izberite barvo neposredno s platna ali podokna palete.
  - Trak za hitro izbiro palete: z levim klikom izberite barvo »Iz«, z desnim klikom barvo »V«.
  - Drsnik za toleranco in paketna zamenjava po vseh sličicah animacije hkrati.
- **Prilagoditev barv (`Ctrl/Cmd+U`)**: Sprotno spreminjanje odtenka HSV (Hue), nasičenosti (Saturation), svetlosti (Value) in kontrasta.
- **Generator samodejnih obrisov**: Ustvarite ostre piksel obrise likov (Zunaj, Znotraj, Na sredini) s kvadratnimi ali romboidnimi vogali.
- **Matrični filtri 3×3**: Zameglitev (Blur), Ostrenje (Sharpen), Zaznavanje robov (Edge Detect) in Izbočenje (Emboss) z ohranjanjem alfa kanala.
- **Prosto vrtenje (`Ctrl/Cmd+Shift+R`)**: Zavrtite izbrana območja ali celotne sličice pod poljubnim kotom z uporabo klasičnega algoritma **RotSprite** ali Nearest Neighbor.
- **Prekodiranje v paleto**: Pretvori barve platna v aktivno paleto z uporabo Bayerjevega ditherja (2x2, 4x4, 8x8) ali Blue Noise.

### 8. Upravljanje palet, zamenjava barv in preobleke likov

- **Vgrajene palete**: Več kot 20 pristnih retro palet, vključno z Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized in Sweetie 16.
- **Uvoz in izvoz**: Podpora za formate GIMP Palette (`.gpl`), PAL (`.pal`), Hex palete (`.hex`) in slikovne vzorce (`.png`).
- **Zamenjava palet (Preobleke)**: Določite več barvnih preoblek za enega lika (npr. Privzeto, Zlati oklep, Senca, Ledeno). Zamenjava preobleke v trenutku prekodira barve vseh sličic v atlasu.
- **Generator barvnih prehodov**: Samodejno interpolira gladke barvne prehode med dvema izbranima odtenkoma z nastavljivim številom vmesnih stopenj.

### 9. Razrezovalnik listov, 9-delna mreža UI in izvoz za igralne pogone

- **Razrezovalnik listov (Sheet Slicer)**: Zaznavanje meja z avtokorelacijo samodejno razreže neobdelane liste spriteov v enakomerne ali natančno obrezane sličice animacije.
- **Raztegovanje z 9-delno mrežo (9-Slice)**: Nastavite poljubne robove (zgoraj, spodaj, levo, desno) za preizkušanje raztegljivih oken uporabniškega vmesnika in gumbov brez popačenja vogalov.
- **Izvoz PNG z raztegnjenimi robovi**: Raztegne robne slikovne pike v prosojna območja, kar odpravi vizualne šive pri filtriranju tekstur v standardih OpenGL, DirectX, Vulkan in igralnih pogonih 2D (kot je Godot).
- **Izvoz animiranih slik GIF**: Ustvari optimizirane animirane GIF-e z nastavljivim številom zank, zamiki sličic in prosojnimi ozadji.
- **Standardni Sprite Sheet JSON**: Izvozi koordinate celic sličic, oznake, trajanja in točke vrtišča, ki ustrezajo standardni shemi Sprite Sheet JSON, uporabljeni v sodobnih igralnih pogonih.

### 10. 35 ergonomskih tem in popolna lokalizacija

- **Izbrana zbirka tem**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, fosforni CRT monitorji (Amber CRT, Green CRT, Cyan CRT), retro konzole (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) ter retro računalniki (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Popolna večjezična podpora**: Nemoteno preklapljanje med 37 jeziki s 100% pokritostjo vseh besedil in orodnih namigov.

---

## Nadzor in tipkovnične bližnjice

| Bližnjica | Orodje / Dejanje | Opis |
|:---|:---|:---|
| <kbd>B</kbd> | **Svinčnik** | Natančno risanje 1px z možnostjo načina Pixel-Perfect |
| <kbd>E</kbd> | **Radirka** | Brisanje slikovnih pik nazaj v prosojnost |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Kapalka** | Vzorčenje barve s slikovnih pik platna ali referenčne skice |
| <kbd>G</kbd> | **Vedro z barvo / Prehod** | Preklop med vedrom z barvo in dither barvnim prehodom |
| <kbd>L</kbd> | **Črta** | Risanje ravnih črt s predogledom v živo in poravnavo s Shift |
| <kbd>C</kbd> | **Krivulja** | Orodje za kvadratno Bezierovo krivuljo s 3 kliki |
| <kbd>U</kbd> | **Pravokotnik** | Risanje obrisanih ali zapolnjenih pravokotnikov in kvadratov |
| <kbd>O</kbd> | **Elipsa** | Risanje obrisanih ali zapolnjenih elips in krogov |
| <kbd>D</kbd> | **Čopič za dither** | Slikanje s šahovničnimi vzorci ditherja |
| <kbd>S</kbd> | **Orodje za senčenje** | Postopno posvetljevanje ali temnenje vzdolž barvne rampe palete |
| <kbd>M</kbd> | **Pravokotna izbira** | Izbira pravokotnega območja s premikajočimi se mravljami |
| <kbd>Q</kbd> | **Lasasta izbira** | Prosta sklenjena večkotna izbira območja |
| <kbd>W</kbd> | **Čarobna paličica** | Izbira zveznih območij iste barve |
| <kbd>P</kbd> | **Točke vrtišča** | Postavitev izhodišča spritea in akcijskih točk igranja |
| <kbd>X</kbd> | **Zamenjaj barvi** | Zamenjava primarne (ospredje) in sekundarne (ozadje) barve |
| <kbd>[</kbd> / <kbd>]</kbd> | **Velikost čopiča** | Zmanjšaj / Povečaj velikost čopiča (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopiraj** | Kopiranje izbranih slikovnih pik v sistemsko odložišče |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Izreži** | Izrezovanje izbranih slikovnih pik v sistemsko odložišče |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Prilepi** | Lepljenje slikovnih pik iz odložišča v aktivno sličico |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Izbriši** | Brisanje slikovnih pik znotraj aktivne izbire |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Izberi vse** | Izbira vseh slikovnih pik na aktivni sličici |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Prekliči izbiro** | Počisti trenutno masko izbire |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Preobrni izbiro** | Preobrne trenutno masko izbire |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Podvoji sličico** | Podvoji trenutno aktivno sličico v časovnici |
| <kbd>Preslednica</kbd> | **Predvajaj / Premor** | Preklop predvajanja animacije v časovnici |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Zamenjaj barvo** | Odpre pogovorno okno za zamenjavo barv s predogledom v živo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Prilagoditev barv** | Odpre pogovorno okno za nastavitev HSV in kontrasta |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Prosto vrtenje** | Odpre okno za vrtenje pod poljubnim kotom (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Razveljavi** | Razveljavi zadnjo potezo na platnu ali dejanje orodja |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Uveljavi** | Ponovno uveljavi predhodno razveljavljeno dejanje |
| <kbd>Preslednica</kbd> + Vlečenje / Srednji klik | **Premikanje platna** | Gladko neskončno pomikanje po koordinatah platna |
| <kbd>Ctrl/Cmd</kbd> + Kolesce / <kbd>+</kbd> / <kbd>-</kbd> | **Povečava** | Ostra povečava slikovnih pik s središčem ob kazalcu miške |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Način pogleda** | Preklop med načinom sličice (<kbd>1</kbd>) in načinom atlasa (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Prilagodi platno** | Samodejno prilagodi aktivno sličico središču delovnega prostora |
| <kbd>F1</kbd> | **Pomoč** | Odpre preglednico bližnjic na tipkovnici |

---

## Kako začeti

### Samostojne dvojiške datoteke

Prenesite vnaprej prevedene, samostojne namestitvene datoteke s **[strani izdaj na GitHubu](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Prenesite `OniAnvil-windows-x86_64.zip`, razpakirajte arhiv in dvokliknite `OniAnvil.exe`.
- **Linux**: Prenesite `OniAnvil-linux-x86_64.tar.gz` (prenosna različica) ali namestite paket `.deb` za sisteme Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Prenesite `OniAnvil-macos-universal.zip`, razpakirajte in povlecite datoteko `OniAnvil.app` v mapo Applications. Univerzalna dvojiška datoteka deluje izvorno tako na procesorjih Apple Silicon (M1/M2/M3/M4) kot na računalnikih Mac s procesorji Intel.
- **Splet (Web)**: Zaženite neposredno v brskalniku brez namestitve: **[Zaženi spletno aplikacijo WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=sl)**.
  > [!WARNING]
  > Spletna gradnja WebAssembly ima status **Eksperimentalno**. Zaradi omejitev varnostnega peskovnika brskalnikov, razlik v gonilnikih WebGL/WebGPU in omejitev odložišča lahko pride do manjših grafičnih napak ali zmanjšane zmogljivosti v primerjavi z izvorno namizno različico. Za resno in produkcijsko delo toplo priporočamo namestitev izvorne namizne različice.

---

### Gradnja iz izvorne kode

#### Predpogoji
- **Orodja Rust**: Namestite Rust (priporočena različica 1.80 ali novejša) prek [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Gradbene odvisnosti za Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Prevajanje in zagon izvorne namizne različice
```bash
# Klonirajte repozitorij
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Zaženite preizkuse
cargo test

# Zgradite in zaženite optimizirano izdajo
cargo run --release
```

#### 2. Prevajanje različice WebAssembly (prek orodja Trunk)
```bash
# Dodajte ciljno platformo wasm32 in orodje Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Zaženite lokalni razvojni strežnik s samodejnim vročim ponovnim nalaganjem
trunk serve

# Zgradite optimizirane datoteke za objavo v mapo docs/app/
./build_wasm.sh        # V sistemih Linux/macOS
./build_wasm.ps1       # V sistemu Windows PowerShell
```

---

## Oblika datotek in spremljevalna arhitektura

OniAnvil se izogiba zaprtim lastniškim dvojiškim formatom. Vsak projekt sestavljata dve čisti datoteki, primerni za sisteme za nadzor različic (Git):

```
my_character/
├── my_character.png            # Standardni teksturni atlas spriteov v formatu PNG (odpre se v vsakem pogonu ali pregledovalniku)
└── my_character.onianvil.json   # Lahka spremljevalna datoteka z metapodatki projekta
```

Spremljevalna datoteka `.onianvil.json` hrani:
- Nastavitve mreže platna: dimenzije sličice, število stolpcev, skupno število sličic.
- Hierarhijo plasti: imena sledi, stopnje prosojnosti, oznake vidnosti in stanja zaklepanja.
- Oznake animacij: ime, začetno in končno sličico, smer predvajanja (Naprej, Nazaj, Ping-Pong), trajanje sličic.
- Opredelitve palet: aktivne barvne vzorce, poimenovane različice preoblek, indeks trenutne preobleke.
- Stanje referenčne skice: pot do slike, merilo, zamik položaja, kot vrtenja, prosojnost, način plasti.
- Točke vrtišča in dejanja: prilagojene akcijske točke, sidra za orožje in središča trkov.

---

## Prispevanje in zahvale

Prispevki kode, poročila o hroščih in predlogi novih funkcij so zelo dobrodošli!
1. Ustvarite vilice (fork) repozitorija na GitHubu.
2. Ustvarite novo vejo za funkcionalnost: `git checkout -b feature/nova-funkcija`.
3. Uveljavite svoje spremembe: `git commit -am 'Dodano novo orodje'`.
4. Potisnite vejo v svojo vilico: `git push origin feature/nova-funkcija`.
5. Odprite zahtevek za združitev (Pull Request).

### Zahvale
- Začetni predstavitveni animirani sprite mačke **"Cat 2D Pixel Art"** je ustvaril avtor **xzany** ([stran projekta na itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Zgrajeno z izjemno grafično knjižnico **[egui](https://github.com/emilk/egui)** in grafičnim ekosistemom **[wgpu](https://wgpu.rs/)**.
- Ikone vmesnika omogoča odprtokodna zbirka **[Font Awesome Free](https://fontawesome.com)** avtorjev @fontawesome (ikone: CC BY 4.0, pisave: SIL OFL 1.1, koda: licenca MIT).

---

## Licenca in omejitev odgovornosti

Ta projekt je licenciran pod pogoji **[licence MIT](../LICENSE)**. OniAnvil lahko prosto uporabljate, spreminjate, razširjate in vključujete v komercialne in osebne cevovode za razvoj iger.

### Zavrnitev jamstva in omejitev odgovornosti
> [!IMPORTANT]
> Program **OniAnvil** se razširja **brezplačno** na osnovi **"KOT JE" ("AS IS")** in **"KOT JE NA VOLJO" ("AS AVAILABLE")**, brez kakršnih koli izrecnih ali implicitnih jamstev ali pogojev.
> 
> Avtor(ji), imetniki avtorskih pravic ali sodelavci v nobenem primeru ne odgovarjajo za kakršno koli neposredno, posredno, naključno, posebno, svarilno ali posledično škodo (vključno z, vendar ne omejeno na, izgubo podatkov, poškodovanje datotek projektov, okvaro strojne opreme, izgubo dobička ali prekinitev poslovanja), ki bi kakor koli nastala zaradi uporabe ali nezmožnosti uporabe te programske opreme, tudi če so bili obveščeni o možnosti nastanka take škode. Z uporabo tega programa prevzemate vso odgovornost in tveganje.

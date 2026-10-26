<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pikslikunsti ja sibulakooreanimatsiooni (Onion-Skinning) stuudio mänguvalmis tekstuuriatlastele**

[![Koostamine ja väljalase](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Allalaadimised](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Kasutajaliides](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderdaja](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Keeled](https://img.shields.io/badge/Keeled-37_Keelt-purple)](#10-35-ergonoomilist-teemat-ja-mitmekeelne-lokaliseerimine)
[![Platvorm](https://img.shields.io/badge/Platvorm-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#alustamine)
[![Veebirakendus](https://img.shields.io/badge/Veebirakendus-Proovi_brauseris_(Eksperimentaalne)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=et)
[![Litsents: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Proovi OniAnvilit veebibrauseris (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=et)** *(⚠️ Eksperimentaalne veebiversioon: võib esineda WebGL/WASM piiranguid ja vigu)* • **[🌐 Ametlik dokumentatsiooni veebisait](https://rbm13s.github.io/OniAnvil/et.html)**

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
[Hrvatski](README-hr.md) •
[Nederlands](README-nl.md) •
[Dansk](README-da.md) •
[Norsk](README-no.md) •
[Svenska](README-sv.md) •
[Suomi](README-fi.md) •
**Eesti** •
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

## Sisukord

- [Sissejuhatus](#sissejuhatus)
- [Ekraanipildid ja visuaalne ringkäik](#ekraanipildid-ja-visuaalne-ringkäik)
- [Põhjalik tööriistade teatmik](#põhjalik-tööriistade-teatmik)
- [Funktsionaalne dokumentatsioon](#funktsionaalne-dokumentatsioon)
  - [1. Kaksikatlasel põhinev arhitektuur](#1-kaksikatlasel-põhinev-arhitektuur)
  - [2. Mitmerealine kihtide ajajoon](#2-mitmerealine-kihtide-ajajoon)
  - [3. Reaalajas animatsiooni reaalne eelvaade](#3-reaalajas-animatsiooni-reaalne-eelvaade)
  - [4. Täiustatud sibulakoore mootor (Onion Skinning)](#4-täiustatud-sibulakoore-mootor-onion-skinning)
  - [5. Nutikas eskiisi ja referentsi kiht](#5-nutikas-eskiisi-ja-referentsi-kiht)
  - [6. Täpne valikute komplekt ja marssivad sipelgad](#6-täpne-valikute-komplekt-ja-marssivad-sipelgad)
  - [7. Reaalajas efektid, filtrid ja RotSprite](#7-reaalajas-efektid-filtrid-ja-rotsprite)
  - [8. Palettide haldus, vahetus ja tegelase nahad (Skins)](#8-palettide-haldus-vahetus-ja-tegelase-nahad-skins)
  - [9. Leheviilutaja, 9-Grid UI ja eksport mängumootoritesse](#9-leheviilutaja-9-grid-ui-ja-eksport-mängumootoritesse)
  - [10. 35 ergonoomilist teemat ja mitmekeelne lokaliseerimine](#10-35-ergonoomilist-teemat-ja-mitmekeelne-lokaliseerimine)
- [Juhtimine ja kiirklahvid](#juhtimine-ja-kiirklahvid)
- [Alustamine](#alustamine)
  - [Eelkoostatud käivitusfailid](#eelkoostatud-käivitusfailid)
  - [Lähtekoodist kompileerimine](#lähtekoodist-kompileerimine)
- [Failivorming ja kaaslasfailide arhitektuur](#failivorming-ja-kaaslasfailide-arhitektuur)
- [Panustamine ja tänuavaldused](#panustamine-ja-tänuavaldused)
- [Litsents ja lahtiütlus](#litsents-ja-lahtiütlus)

---

## Sissejuhatus

**OniAnvil** (*nimi tuleneb sõnadest **Oni**on-Skinning (sibulakooretehnika) + **Anvil** (alasi spraitide sepistamiseks)*) on avatud lähtekoodiga ülikiire pikslikunsti redaktor ja animatsioonistuudio, mis on kirjutatud programmeerimiskeeles Rust. See töötab riistvaralise kiirendusega Windowsi, Linuxi ja macOS-i keskkondades ning brauseripõhise WebAssembly rakendusena ilma installimiseta otse tänapäevastes veebibrauserites.

Enamik olemasolevaid spraidiredaktoreid käsitleb animatsiooni lahusolevate pildifailide virnana, sundides arendajaid korduvatesse eksportimise, viilutamise ja taaslaadimise etappidesse. **OniAnvil pöörab selle paradigma ümber:** mängu spraidilehe tekstuuriatlas (`atlas_rgba`) on esmane tõeallikas. Kaadrielemendid, kihiread, animatsioonisildid ja põrkepunktid püsivad pidevalt ja kadudeta tekstuuriatlasega sünkroonis.

### Miks OniAnvil?
- 🚀 **Hämmastav natiivne Rusti jõudlus**: Hetkeline millisekundist kiirem käivitus, terav renderdus tänu `wgpu` ja `egui 0.31` raamistikele ning sujuv kerimine pikkadel ajajoontel ilma viivituseta.
- 🎬 **Mitmerealine kihtide ajajoon**: Mittelineaarsetest videotöötlusprogrammidest inspireeritud kihiread (`V1, V2, V3...`) ülalt-alla kompositsiooniga, kihtide allapoole ühendamine, lingitud kaadrid ja paindlikud hoidmiskestused.
- 🧅 **Esmaklassiline sibulakooretehnika (Onion Skinning)**: Mitmekaadriline poolläbipaistvus sõltumatu esi- ja tagasisuuna toonimisega, kohandatavad läbipaistvuskõverad ning sujuv tsükliline üleminek.
- 🎨 **Tegelaste kiired paletivahetused (Skins)**: Loo hõlpsalt päeva-/öövariante, meeskonnavärve või elementaarseid stiile reaalajas atlase ümberkaardistamise ja GPL-, PAL- ning HEX-toega.
- 📦 **Otsene eksport mängumootoritesse**: Ekspordi laiendatud servapikslitega (extrude) PNG-faile tekstuuride veritsemise vältimiseks 2D-mootorites (Godot, Bevy, Raylib, Defold) koos standardse JSON-metaandmete ja animeeritud GIF-idega.

---

## Ekraanipildid ja visuaalne ringkäik

<div align="center">

### 1. Peamine lõuend ja stuudio tööruum
*Suure jõudlusega pikslitööruum riistvarakiirendusega vaateakna, mitmerealise ajajoone, reaalajas eelvaate, paletiruutude ja täppistööriistaribadega.*
<img src="screenshots/main.png" alt="OniAnvili peamine tööruum" width="950" />

<br/><br/>

### 2. Mitmerealine animatsiooni ajajoon
*Mittelineaarne ajajoon kihtide virnaga (`V1, V2...`), lingitud kaadrieksemplaridega (`🔗`), hoidmise kordajate ja värviliste nimesiltidega.*
<img src="screenshots/timeline.png" alt="OniAnvili mitmerealine ajajoon" width="950" />

<br/><br/>

### 3. Täppistööriistad ja tegevuste ülariba
*Täielik 15-tööriistaline joonistuskomplekt tava- ja kompaktse dokkimisega ning ülemine kiirriba pintsli suuruse, sümmeetria, tsüklilisuse ja Pixel-Perfecti juhtimiseks.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Tegevuste tööriistariba" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Tavapärane tööriistariba" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Kompaktne tööriistariba" width="300" />
</p>

<br/><br/>

### 4. Reaalajas animatsiooni reaalne eelvaade
*Eraldi eelvaatedokk sildifiltreerimise, muudetava kaadrisageduse (FPS), suumiliuguri ja lõuendi taustade vahetamisega (malelaud, tume, hele).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Reaalajas eelvaade" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Eelvaate suumimine" width="460" />
</p>

<br/><br/>

### 5. Palettide haldus, vahetus ja värviüleminekud
*Põhjalik värvimootor esi- ja taustavärvi lahtrite, automaatse kahepunktilise värviskaala genereerimise ja tegelase nimega nahkade vahetamisega.*
<img src="screenshots/palette_and_swaps.png" alt="Palett ja nahkade vahetus" width="950" />

<br/><br/>

### 6. Eskiisi joonistamine ja seadistatav sibulakoor
*Laadi eskiisid või fotod alus- või pealiskihiks teisendustööriistade, paletti lukustuva nutika pipeti ja mitmekaadrilise sibulakoorekumaga.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Eskiis ja sibulakoor" width="950" />

<br/><br/>

### 7. Pöördepunktid (Pivot) ja tegevuspunktid (Action Points)
*Määra spraidi keskpunkt ja kohandatud mängupunktid (relvaankrud, laskepunktid, pea, sammud) interaktiivsete ekraanitööriistadega.*
<img src="screenshots/pivot_and_action_points.png" alt="Pöörde- ja tegevuspunktid" width="950" />

<br/><br/>

### 8. Spraidilehe viilutaja ja 9-Grid (9-Slice) skaleerija
*Automaatne spraidi piiride tuvastus autokorrelatsiooniga (nt kõndiv kass) ja 9-osalise skaleeritava kasutajaliidese kasti veeriste häälestus.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Leheviilutaja" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="9-Grid skaleerija" width="460" />
</p>

<br/><br/>

### 9. Reaalajas pildiefektid ja filtrid (Live Preview)
*Kõikidel efektidialoogidel on reaalajas lõuendi eelvaade, interaktiivsed pipetid ja kadudeta tühistamine ilma ajalugu risustamata.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Kontuuri generaator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Värvi asendamine" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Värvitoon, küllastus ja kontrastsus" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Maatriksfilter" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Paletti ümberkaardistamine" width="460" />
</p>

<br/><br/>

### 10. Lõuendi ruudustik ja taustarežiimid
*Vaheta sujuvalt piksliruudustiku, kaadriruudustiku, tumeda malelaua, heleda malelaua ja kohandatud ühevärvilise tausta vahel.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Ruudustik ja tume malelaud" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Hele malelaud ilma ruudustikuta" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Kohandatud ühevärviline taust" width="300" />
</p>

<br/><br/>

### 11. Uue projekti loomise viisard
*Määra lõuendi mõõtmed, kaadrite arv, veergude paigutus ja vali projekti käivitamisel rohkem kui 20 autentse retropaleti hulgast.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Uue projekti viisard" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Projekti paletid" width="460" />
</p>

<br/><br/>

### 12. 35 autentset retro- ja kaasaegset teemat
*Sisaldab Studio Dark, Studio Light, CRT-fosforeid (Amber, Green, Cyan), retrokonsoole (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) ja disaineriteemasid.*
<p align="center">
  <img src="screenshots/themes.png" alt="Teemade menüü" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber teema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX teema" width="300" />
</p>

<br/><br/>

### 13. Menüüsüsteem ja sisseehitatud kiirklahvide abi
<p align="center">
  <img src="screenshots/file_menu.png" alt="Faili menüü" width="180" />
  <img src="screenshots/edit_menu.png" alt="Muutmise menüü" width="180" />
  <img src="screenshots/select_menu.png" alt="Valiku menüü" width="180" />
  <img src="screenshots/view_menu.png" alt="Vaate menüü" width="180" />
  <img src="screenshots/project_menu.png" alt="Projekti menüü" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Kiirklahvide abiaken" width="700" />
</p>

</div>

---

## Põhjalik tööriistade teatmik

OniAnvil sisaldab 15 professionaalset joonistus-, geomeetria-, valiku- ja metaandmete tööriista. Ikoonid kasutavad avatud lähtekoodiga **Font Awesome Free** ikoonikomplekti *(Ikoonid: CC BY 4.0, Fondid: SIL OFL 1.1)*.

| Tööriist | Ikoon | Kiirklahv | Pintsli suurus | Pixel Perfect | Kirjeldus |
|:---|:---:|:---:|:---:|:---:|:---|
| **Pliiats** | ✏️ | <kbd>B</kbd> | 1..32 px | Jah | Peamine joonistustööriist. 1 px režiimis toetab **Pixel-Perfect** joonistamist (eemaldab automaatselt trepilaadsed L-nurgad). Toetab valikust templipintsleid (`Ctrl/Cmd+B`). |
| **Kustutuskumm** | 🧹 | <kbd>E</kbd> | 1..32 px | Ei | Kustutab pikslid täiesti läbipaistvaks (`[0, 0, 0, 0]`). Aktiivse valiku korral piiratud valikumaskiga. |
| **Pipett** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ei | Vasak klõps võtab esiplaanivärvi (Primary); parem klõps taustavärvi (Secondary). Võtab värvi lõuendilt, kihtidelt või eskiisilt. |
| **Täiteämber** | 🪣 | <kbd>G</kbd> | — | Ei | Kiire järjekorrapõhine (BFS) täitmine reguleeritava värvitaluvusega (`0..=255`). Toetab külgnevat ja globaalset täitmist ning valikuga piiramist. |
| **Gradient** | 🌈 | <kbd>G</kbd> *(lüliti)* | — | Ei | Lineaarsed ja radiaalsed üleminekud esiplaani ja taustavärvi vahel. Toetab Bayeri (2x2, 4x4, 8x8) ja Blue Noise puidrit tugevuse reguleerimisega. |
| **Joon** | 📏 | <kbd>L</kbd> | 1..32 px | Jah | Bresenhami joonealgoritm reaalajas eelvaatega. <kbd>Shift</kbd> fikseerib nurgad 45°/90° sammudega. |
| **Kõver** | 〰️ | <kbd>C</kbd> | 1..32 px | Jah | 3 klõpsuga ruut-Bézier' kõver. 1. klõps määrab alguspunkti, 2. klõps lõpp-punkti, hiire liigutus painutab kaart ja 3. klõps kinnitab kõvera. |
| **Ristkülik** | ⬜ | <kbd>U</kbd> | 1..32 px | Ei | Kontuuriga või täidetud geomeetrilised ristkülikud. <kbd>Shift</kbd> fikseerib 1:1 ruuduks. |
| **Ellips** | ⭕ | <kbd>O</kbd> | 1..32 px | Ei | Kontuuriga või täidetud ellipsid keskpunkti rasterdamisega. <kbd>Shift</kbd> fikseerib täpseks ringiks. |
| **Puiderpintsel** | 🏁 | <kbd>D</kbd> | 1..32 px | Ei | Maalib korrapäraseid malelauakujulisi puidermustreid esi- ja taustavärvi vahel ilma lisavärve paletti toomata. |
| **Varjutustööriist** | 🌓 | <kbd>S</kbd> | 1..32 px | Ei | Järkjärguline heledamaks muutmine (Dodge) ja tumendamine (Burn) aktiivse paleti järjekorra alusel. |
| **Ristkülikukujuline valik** | ⬚ | <kbd>M</kbd> | — | Ei | Ristkülikukujuline alavalik animeeritud marssivate sipelgate piirjoonega. Lisamine (<kbd>Shift</kbd>) ja lahutamine (<kbd>Alt</kbd>). |
| **Lassovalik** | ➰ | <kbd>Q</kbd> | — | Ei | Vabakäe hulknurkne lasso. Kasutab 8-sidususega piirjoone rasterdamist ja 4-sidususega välist täitmist, tagades 100% tiheda, aukudeta valiku ka silmuste korral. |
| **Võlusau** | 🪄 | <kbd>W</kbd> | — | Ei | Valib külgnevad (või globaalsed) sama värvi piirkonnad reguleeritava taluvuslävega. |
| **Pöörde- ja tegevuspunktid** | 🎯 | <kbd>P</kbd> | — | Ei | Spraidi alguspunkti ja kohandatud tegevuspunktide (`weapon`, `head`, `feet`, `hitbox`) interaktiivne paigutamine lõuendil ja eksport JSON-metaandmetesse. |

---

## Funktsionaalne dokumentatsioon

### 1. Kaksikatlasel põhinev arhitektuur

OniAnvilis esindab peamine tekstuuriatlas (`atlas_rgba`) spraidilehe tegelikku tekstuuri täpselt sellisena, nagu see laaditakse mängumootorisse:
- **Kaadrirežiim (Frame Mode)**: Keskendub ühele aktiivsele animatsioonikaadrile, pakkudes segajateta pikslilõuendit. Muudatused uuendavad koheselt vastavat lahtrit tekstuuriatlases.
- **Atlaserežiim (Atlas Mode)**: Kuvab kogu spraidilehe ruudustikku, võimaldades makrotaseme kohandamist, kaadrite joonduskontrolli ja hulgitöötlust.
- **Kaaslasfail (Companion File)**: Projektid salvestavad puhta `.png` spraidilehe koos kerge `.onianvil.json` kaaslasfailiga, mis talletab kaadrite paigutuse, kihtide hierarhia, sildid, hoidmise kordajad ja paletinahad.

### 2. Mitmerealine kihtide ajajoon

Alumine ajajoon on üles ehitatud professionaalse mittelineaarse videotöötlustarkvara põhimõttel:
- **Kihiread (`V1, V2, V3...`)**: Kihid asetsevad visuaalselt ülalt alla. Ajajoonel kõrgemal asuv rida (`V1`) renderdatakse kaadril teiste kihtide peale.
- **Rea juhtelemendid**:
  - Klõpsa `V1` / `V2` märkidel, et aktiivset joonistuskihti kiiresti vahetada.
  - Lülita kihi nähtavust (silmaikoon) ja muutmise lukku (lukuikoon).
  - Reguleeri üksiku kihi läbipaistvust (`0%..=100%`).
  - Liiguta kihti hierarhias üles/alla, dubleeri kiht ja liida kiht allasuunas alumise reaga (Merge Down).
  - Kustuta kiht kinnitusdialoogiga.
- **Kaadrik Lipid**:
  - Hoidmise kordaja: määra kohandatud kaadri kestus (nt `2×`, `3×` hoidmine pauside või löögikaadrite jaoks).
  - Lingitud kaadrid (`🔗`): lingi korduvad kaadrid üle kogu animatsiooni nii, et ühe redigeerimine uuendab kohe kõiki teisi.
  - Siltide tsükeldamine: rühmita animatsioonid nimega siltidesse (`idle`, `walk`, `attack`) ja esita kindlaid silte korduvalt.
  - Esituse juhtelemendid: Edasi, Tagasi, Ping-Pong ja Pidev tsükkel reguleeritava kaadrisagedusega (`1..=120` FPS).

### 3. Reaalajas animatsiooni reaalne eelvaade

Parempoolne eelvaatedokk esitab animatsiooni reaalajas, olles täielikult sõltumatu lõuendi suumist või asukohast:
- **Sildifiltreerimine**: Eralda esitus konkreetsete siltide (`walk`, `jump`) jaoks või vaata kogu animatsioonijada.
- **Esituse juhtelemendid**: Esita, Peata, Eelmine kaader, Järgmine kaader, Tsükli lüliti ja kohandatud kiirused (`0.25x..=4.0x`).
- **Kuvarežiimid**: Vali tumeda malelaua, heleda malelaua, puhta valge, puhta musta või kohandatud taustavärvi vahel.

### 4. Täiustatud sibulakoore mootor (Onion Skinning)

Loo sujuvaid animatsioone spetsiaalse sibulakooresüsteemiga:
- **Kohandatud toonimine**: Sõltumatud toonivärvid eelmistele kaadritele (vaikimisi: punane) ja järgmistele kaadritele (vaikimisi: sinine).
- **Läbipaistvuse hääbumine**: Seadistatav baasläbipaistvus ja astmeline hääbumiskõver eemaldumisel.
- **Tsükliline üleminek**: Luba poolläbipaistvus esimese ja viimase kaadri vahel sujuvate silmuste loomiseks.
- **Kiirpilk (Quick-Peek)**: Hoia all <kbd>Tühikut</kbd> ilma lohistamata, et näha hetkega naaberkaadreid.

### 5. Nutikas eskiisi ja referentsi kiht

Kiirenda oma töövoogu käsitsi joonistatud visandite või referentsfotode ülejoonistamisega:
- **Vormingud**: Laadi PNG, JPEG ja WebP vormingus referentspilte.
- **Segamisrežiimid**: Aluskiht (pikslikihtide all) või Pealiskiht (pikslikihtide peal).
- **Teisendustööriistad**: Sõltumatu suum, nihutamine, pööramine ja läbipaistvuse liugur.
- **Nutikas värvivalik**: Kasuta pipetti eskiisikihil koos valikulise automaatse haakumisega aktiivse projekti paleti lähima värviga.
- **1-klõpsuga küpsetamine (Bake)**: Ühenda skaleeritud referentspilt ühe klõpsuga otse puhtaks pikslikunstiks aktiivsele kaadrile.

### 6. Täpne valikute komplekt ja marssivad sipelgad

- **Animeeritud marssivad sipelgad**: Aktiivsed valikud kuvavad animeeritud kontrastset kahevärvilist punktiirjoont.
- **Tihe polügonaalne lasso**: Erinevalt lihtsatest kiirteviskamise algoritmidest, mis tekitavad kattuvatel silmustel tühimikke, rasterdab OniAnvili lasso suletud 8-sidususega piiri ja täidab välisala, tagades 100% tiheda, aukudeta valiku.
- **Valikutehted**: Laienda, Ahenda, Ääris (ringikujulise, ruudukujulise ja rombikujulise tuumaga), Teisalda, Pööra ümber (Invert), Kustuta, Vabapööramine ja süsteemi lõikelaua Kopeeri/Lõika/Kleebi.
- **Kaadriteülene kopeerimine ja kleepimine**: Vali ala ühel kaadril, vajuta <kbd>Ctrl/Cmd+C</kbd>, vaheta kaadrit ja vajuta <kbd>Ctrl/Cmd+V</kbd>, et kleepida täpselt samadele suhtelistele koordinaatidele mistahes klaviatuuripaigutusega.

### 7. Reaalajas efektid, filtrid ja RotSprite

Kõik pildiefektid sisaldavad reaalajas **Live Preview** märkeruutu, mis kuvab muudatused otse lõuendil ja võimaldab tühistada tegevuse ilma tagasivõtmise ajalugu risustamata:
- **Värvi asendamine (`Shift+R`)**:
  - Interaktiivne pipett: klõpsa pipetiikoonil ja vali värvid otse lõuendi pikslitelt või paletiribalt.
  - Kiirvalikuriba: vasak klõps valib lähte- ("Kust"), parem klõps sihtvärvi ("Kuhu").
  - Taluvuse liugur ja hulgiasendus korraga üle kõigi animatsioonikaadrite.
- **Värvi korrigeerimine (`Ctrl/Cmd+U`)**: Reaalajas HSV tooni nihe, küllastus, heledus ja kontrastsuse reguleerimine.
- **Automaatne kontuurigeneraator**: Loo tegelasele terav pikslikontuur (Väljaspool, Seespool, Keskel) ruut- või rombnurkadega.
- **3×3 konvolutsioonifiltrid**: Hägusus, Teravustamine, Servade tuvastus ja Reljeef koos alfa-kanali säilitamisega.
- **Vabapööramine (`Ctrl/Cmd+Shift+R`)**: Pööra valitud alasid või terveid kaadreid mistahes nurga all klassikalise **RotSprite** või lähima naabri algoritmiga.
- **Paletti ümberkaardistamine**: Kvantiseeri lõuendi värvid aktiivsesse paletti, kasutades Bayer 2x2, 4x4, 8x8 või Blue Noise puidrit.

### 8. Palettide haldus, vahetus ja tegelase nahad (Skins)

- **Sisseehitatud paletid**: Üle 20 autentse retropaleti, sealhulgas Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized ja Sweetie 16.
- **Import ja eksport**: GIMP-paleti (`.gpl`), PAL-failide (`.pal`), HEX-failide (`.hex`) ja pildinäidiste (`.png`) tugi.
- **Paletivahetus (Nahad)**: Määra ühele tegelasele mitu värvinahka (nt Vaikimisi, Kuldne rüü, Vari, Jäine). Naha vahetamine viib reaalajas läbi kõigi kaadrite värvide ümberkaardistamise kogu atlases.
- **Värviskaala generaator**: Interpoleeri sujuvaid värvigradiente kahe valitud tooni vahel reguleeritava vahesammude arvuga.

### 9. Leheviilutaja, 9-Grid UI ja eksport mängumootoritesse

- **Leheviilutaja (Sheet Slicer)**: Autokorrelatsioonil põhinev servatuvastus viilutab toored spraidilehed automaatselt ühtlasteks või tihedalt piiratud animatsioonikaadriteks.
- **9-Grid (9-Slice) skaleerimine**: Seadista kohandatavad viilutusveerised (ülemine, alumine, vasak, parem), et vaadata skaleeritavaid kasutajaliidese kaste ja nuppe ilma nurki moonutamata.
- **Laiendatud servapikslitega PNG eksport (Extruded Border)**: Kordab servapiksleid läbipaistvatesse veeristesse, kõrvaldades tekstuurifiltreerimise liitekohad ja servavead OpenGL-is, DirectX-is, Vulkanis ja 2D-mängumootorites (nt Godot).
- **Animeeritud GIF-i eksport**: Loo optimeeritud animeeritud GIF-e kohandatud korduste arvu, kaadriviivituste ja läbipaistva taustaga.
- **Standardne Sprite Sheet JSON**: Ekspordib kaadrite lõigud, sildid, kestused ja pöördepunktid vastavalt kaasaegsetes 2D-mängumootorites kasutatavale standardsele skeemile.

### 10. 35 ergonoomilist teemat ja mitmekeelne lokaliseerimine

- **Valitud teemad**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT-fosforid (Amber, Green, Cyan), retrokonsoolid (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) ja retroarvutid (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Mitmekeelne kasutajaliides**: Vaheta sujuvalt lennult keelt täieliku 37 keele tõlkepariteediga.

---

## Juhtimine ja kiirklahvid

| Kiirklahv | Tööriist / Tegevus | Kirjeldus |
|:---|:---|:---|
| <kbd>B</kbd> | **Pliiats** | 1 px täpne joonistamine valikulise Pixel-Perfect režiimiga |
| <kbd>E</kbd> | **Kustutuskumm** | Kustutab pikslid läbipaistvaks |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipett** | Võtab värvi lõuendi pikslitelt või eskiisilt |
| <kbd>G</kbd> | **Täiteämber / Gradient** | Lülita täiteämbri ja puidriga gradiendi vahel |
| <kbd>L</kbd> | **Joon** | Sirgete joonte joonistamine eelvaate ja Shift-nurgafikseerimisega |
| <kbd>C</kbd> | **Kõver** | 3 klõpsuga ruut-Bézier' kõvera tööriist |
| <kbd>U</kbd> | **Ristkülik** | Geomeetriline ristkülik ja ruut (kontuur või täidetud) |
| <kbd>O</kbd> | **Ellips** | Geomeetriline ellips ja ring (kontuur või täidetud) |
| <kbd>D</kbd> | **Puiderpintsel** | Maalib malelauakujulisi puidermustreid |
| <kbd>S</kbd> | **Varjutustööriist** | Järkjärguline heledamaks / tumedamaks muutmine paletil |
| <kbd>M</kbd> | **Ristkülikukujuline valik** | Ristkülikukujuline ala valimine marssivate sipelgatega |
| <kbd>Q</kbd> | **Lassovalik** | Vabakäe tihe polügonaalne valik |
| <kbd>W</kbd> | **Võlusau** | Vali külgnevad sama värvi alad |
| <kbd>P</kbd> | **Pöördepunktid** | Paiguta spraidi alguspunkt ja tegevuspunktid |
| <kbd>X</kbd> | **Vaheta värvid** | Vaheta esiplaani ja taustavärvid omavahel |
| <kbd>[</kbd> / <kbd>]</kbd> | **Pintsli suurus** | Vähenda / Suurenda pintsli suurust (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopeeri** | Kopeeri valitud pikslid süsteemi lõikelauale |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Lõika** | Lõika valitud pikslid süsteemi lõikelauale |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Kleebi** | Kleebi lõikelaua pikslid aktiivsele kaadrile |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Kustuta** | Kustuta pikslid aktiivse valiku seest |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Vali kõik** | Vali aktiivse kaadri kõik pikslid |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Tühista valik** | Eemalda aktiivne valikumask |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Pööra valik ümber** | Pööra aktiivne valikumask vastupidiseks |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dubleeri kaader** | Dubleeri aktiivne kaader ajajoonel |
| <kbd>Tühik</kbd> | **Esita / Peata** | Käivita või peata animatsiooni esitus ajajoonel |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Asenda värv** | Ava värvi asendamise aken reaalajas eelvaatega |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Korrigeeri värvi** | Ava HSV värvitooni, küllastuse ja kontrastsuse aken |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Vabapööramine** | Ava suvalise nurga all pööramise aken (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Võta tagasi** | Tühista eelmine pintslitõmme või tegevus |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Tee uuesti** | Taasta varem tühistatud tegevus |
| <kbd>Tühik</kbd> + Lohistamine / Hiire keskmine nupp | **Nihuta lõuendit** | Sujuv lõputu liikumine lõuendi koordinaatidel |
| <kbd>Ctrl/Cmd</kbd> + Ratas / <kbd>+</kbd> / <kbd>-</kbd> | **Suumi** | Pikslitäpne suumimine hiirekursori asukohas |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Vaaterežiim** | Lülita kaadrivaate (<kbd>1</kbd>) ja atlasevaate (<kbd>2</kbd>) vahel |
| <kbd>F</kbd> | **Mahuta lõuend** | Tsentreeri ja mahuta aktiivne kaader tööruumi |
| <kbd>F1</kbd> | **Abi** | Ava kiirklahvide abiaken |

---

## Alustamine

### Eelkoostatud käivitusfailid

Laadi alla eelkoostatud, iseseisvad käivitusfailid **[GitHubi väljalasete lehelt](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Laadi alla `OniAnvil-windows-x86_64.zip`, paki lahti ja tee topeltklõps failil `OniAnvil.exe`.
- **Linux**: Laadi alla `OniAnvil-linux-x86_64.tar.gz` (kaasaskantav) või paigalda `onianvil_*_amd64.deb` Debian/Ubuntu jaoks:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Laadi alla `OniAnvil-macos-universal.zip`, paki lahti ja lohista `OniAnvil.app` kausta Applications. Universaalne binaarfail töötab natiivselt nii Apple Siliconil (M1/M2/M3/M4) kui ka Inteli Macidel.
- **Veebiversioon**: Käivita otse brauseris ilma midagi installimata: **[Käivita WebAssembly rakendus](https://rbm13s.github.io/OniAnvil/app/?lang=et)**.
  > [!WARNING]
  > WebAssembly (brauseri) versioonil on **eksperimentaalne (Experimental)** staatus. Brauseri liivakasti piirangute, WebGL/WebGPU erinevuste ja lõikelaua piirangute tõttu võib esineda väiksemaid graafilisi tõrkeid või jõudluse kõikumisi võrreldes töölauaversiooniga. Tootmistööks on tungivalt soovitatav kasutada natiivseid töölauaversioone.

---

### Lähtekoodist kompileerimine

#### Eeldused
- **Rusti tööriistakomplekt**: Paigalda Rust (soovitatav 1.80+) veebisaidilt [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linuxi kompileerimise sõltuvused** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Natiivse töölauaversiooni kompileerimine ja käivitamine
```bash
# Klooni repositoorium
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Käivita testid
cargo test

# Koosta ja käivita optimeeritud väljalase
cargo run --release
```

#### 2. WebAssembly versiooni kompileerimine (Trunkiga)
```bash
# Lisa wasm32 sihtplatvorm ja paigalda Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Käivita kohalik arendusserver kuumtaaskäivitusega
trunk serve

# Koosta optimeeritud väljalaskefailid kausta docs/app/
./build_wasm.sh        # Linuxis / macOS-is
./build_wasm.ps1       # Windows PowerShellis
```

---

## Failivorming ja kaaslasfailide arhitektuur

OniAnvil väldib suletud binaarvorminguid. Iga projekt koosneb kahest puhtast, versioonihalduseks ideaalselt sobivast failist:

```
my_character/
├── my_character.png            # Standardne PNG spraidilehe atlas (avatav igas mootoris või vaaturis)
└── my_character.onianvil.json   # Kaaslasfail projekti metaandmetega
```

Fail `.onianvil.json` talletab:
- Lõuendi ruudustiku seaded: kaadri mõõtmed, veergude arv, kaadrite koguarv.
- Kihtide hierarhiad: radade nimed, läbipaistvustasemed, nähtavuse lipud, lukustatud olekud.
- Animatsiooni sildid: nimi, algus-/lõpukaadri indeksid, esituse suund (Edasi, Tagasi, Ping-Pong), kaadrite kestused.
- Palettide definitsioonid: aktiivsed värviruudud, nimega nahkade variandid (skins), aktiivse naha indeks.
- Eskiisikihi olek: pildi tee, skaala, nihutus, pööramine, läbipaistvus, kiharerežiim.
- Pöördepunktid: kohandatud tegevuspunktid, relvaankrud ja kokkupõrkekeskmed.

---

## Panustamine ja tänuavaldused

Kõik panused, veateated ja funktsioonietepanekud on soojalt oodatud!
1. Tee repositooriumist GitHubis hargnemine (Fork).
2. Loo funktsiooniharu: `git checkout -b feature/minu-uus-tooriist`.
3. Salvesta muudatused: `git commit -am 'Lisa uus pikslitööriist'`.
4. Lükka haru serverisse: `git push origin feature/minu-uus-tooriist`.
5. Ava Pull Request.

### Tänuavaldused
- Algne demosprait **"Cat 2D Pixel Art"** on loonud autor **xzany** ([itch.io projekti leht](https://xzany.itch.io/cat-2d-pixel-art)).
- Ehitatud suurepärase **[egui](https://github.com/emilk/egui)** kasutajaliidese raamistiku ja **[wgpu](https://wgpu.rs/)** graafikaökosüsteemi baasil.
- Kasutajaliidese ikoonid pärinevad paketist **[Font Awesome Free](https://fontawesome.com)** (@fontawesome) (Ikoonid: CC BY 4.0, Fondid: SIL OFL 1.1, Kood: MIT litsents).

---

## Litsents ja lahtiütlus

See projekt on litsentsitud **[MIT litsentsi](LICENSE)** alusel. Teil on vabadus OniAnvilit kasutada, muuta, levitada ja integreerida ärilistesse ja isiklikesse mänguarenduse projektidesse.

### Garantii lahtiütlus ja vastutuse piirang
> [!IMPORTANT]
> **OniAnvil** levitatakse **tasuta** põhimõttel **"NAGU ON" ("AS IS")** ja **"NAGU SAADAVAL" ("AS AVAILABLE")**, ilma mistahes otseste või kaudsete garantiide või tingimusteta.
> 
> Mitte mingil juhul ei vastuta autor(id), autoriõiguste omanikud ega kaastöölised otseste, kaudsete, juhuslike, eriliste, hoiatavate või tulenevate kahjude eest (sealhulgas, kuid mitte ainult, andmete kaotus, failide kahjustumine, riistvara rike, saamata jäänud tulu või äritegevuse katkemine), mis tulenevad mistahes viisil selle tarkvara kasutamisest või võimetusest seda kasutada, isegi kui sellise kahju võimalikkusest on teavitatud. Te võtate endale kogu vastutuse ja riski selle programmi kasutamisel.

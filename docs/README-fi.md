<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pikselitaide- ja sipulinkuorianimaatiostudio (Onion-Skinning) pelivalmiille tekstuuriatlaksille**

[![Koontiversio ja julkaisu](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Lataukset](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Käyttöliittymä](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderöijä](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Kielet](https://img.shields.io/badge/Kielet-37_Kieltä-purple)](#10-35-ergonomista-teemaa-ja-monikielinen-lokalisointi)
[![Alusta](https://img.shields.io/badge/Alusta-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#aloitusopas)
[![Verkkoversio](https://img.shields.io/badge/Verkkosovellus-Kokeile_selaimessa_(Kokeellinen)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=fi)
[![Lisenssi: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Kokeile OniAnvilia selaimessa (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=fi)** *(⚠️ Kokeellinen verkkoversio: saattaa sisältää WebGL/WASM-rajoituksia ja virheitä)* • **[🌐 Virallinen dokumentaatiosivusto](https://rbm13s.github.io/OniAnvil/fi.html)**

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
**Suomi** •
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

## Sisällysluettelo

- [Johdanto](#johdanto)
- [Kuvakaappaukset ja visuaalinen esittely](#kuvakaappaukset-ja-visuaalinen-esittely)
- [Kattava työkalujen opas](#kattava-työkalujen-opas)
- [Toiminnallinen dokumentaatio](#toiminnallinen-dokumentaatio)
  - [1. Kaksoisatlakseen perustuva arkkitehtuuri](#1-kaksoisatlakseen-perustuva-arkkitehtuuri)
  - [2. Moniraitainen tasojen aikajana](#2-moniraitainen-tasojen-aikajana)
  - [3. Reaaliaikainen animaation suora esikatselu](#3-reaaliaikainen-animaation-suora-esikatselu)
  - [4. Edistynyt sipulinkuorimoottori (Onion Skinning)](#4-edistynyt-sipulinkuorimoottori-onion-skinning)
  - [5. Älykäs viitekuva- ja luonnoskerros](#5-älykäs-viitekuva--ja-luonnoskerros)
  - [6. Tarkka valintatyökalusto ja marssivat muurahaiset](#6-tarkka-valintatyökalusto-ja-marssivat-muurahaiset)
  - [7. Reaaliaikaiset tehosteet, suodattimet ja RotSprite](#7-reaaliaikaiset-tehosteet-suodattimet-ja-rotsprite)
  - [8. Palettien hallinta, vaihto ja hahmon ulkoasut (Skins)](#8-palettien-hallinta-vaihto-ja-hahmon-ulkoasut-skins)
  - [9. Arkkileikkuri, 9-alueinen skaalaus ja pelimoottorivienti](#9-arkkileikkuri-9-alueinen-skaalaus-ja-pelimoottorivienti)
  - [10. 35 ergonomista teemaa ja monikielinen lokalisointi](#10-35-ergonomista-teemaa-ja-monikielinen-lokalisointi)
- [Ohjaus ja pikanäppäimet](#ohjaus-ja-pikanäppäimet)
- [Aloitusopas](#aloitusopas)
  - [Valmiit suoritettavat tiedostot](#valmiit-suoritettavat-tiedostot)
  - [Kääntäminen lähdekoodista](#kääntäminen-lähdekoodista)
- [Tiedostomuoto ja oheistiedostoarkkitehtuuri](#tiedostomuoto-ja-oheistiedostoarkkitehtuuri)
- [Osallistuminen ja kiitokset](#osallistuminen-ja-kiitokset)
- [Lisenssi ja vastuuvapauslauseke](#lisenssi-ja-vastuuvapauslauseke)

---

## Johdanto

**OniAnvil** (*nimi tulee sanoista **Oni**on-Skinning (sipulinkuorikalvo) + **Anvil** (alasin spritien takomiseen)*) on avoimen lähdekoodin erittäin suorituskykyinen pikselitaide-editori ja animaatiostudio, joka on ohjelmoitu Rust-kielellä. Se toimii natiivisti laitteistokiihdytetyllä grafiikalla Windows-, Linux- ja macOS-alustoilla sekä suoraan nykyaikaisissa verkkoselaimissa WebAssembly-sovelluksena ilman asennusta.

Useimmat perinteiset sprite-editorit käsittelevät animaatiota erillisten kuvatiedostojen pinoina, mikä pakottaa pelinkehittäjät työläisiin vienti-, leikkaus- ja uudelleenpaketointivaiheisiin. **OniAnvil kääntää tämän toimintamallin toisinpäin:** pelin sprite-arkkitekstuuri (`atlas_rgba`) on ensisijainen totuuden lähde. Ruutujen solut, tasoraidat, animaatiotunnisteet ja törmäyskiintopisteet pidetään jatkuvasti ja häviöttömästi synkronoituna tekstuuriatlaksen kanssa.

### Miksi OniAnvil?
- 🚀 **Salamannopea natiivi Rust-suorituskyky**: Välitön käynnistyminen alle millisekunnissa, tarkka renderöinti `wgpu`- ja `egui 0.31` -kirjastoilla sekä viiveetön toiminta pitkiä aikajanoja selattaessa.
- 🎬 **Moniraitainen tasojen aikajana**: Epälineaarisista videoeditoreista tutut tasoraidat (`V1, V2, V3...`), joissa tasot yhdistyvät ylhäältä alas, sisältäen tasojen yhdistämisen alaspäin, linkitetyt ruudut ja säädettävät pitoajat.
- 🧅 **Ensiluokkainen sipulinkuori (Onion Skinning)**: Moniruutuinen haamukuva itsenäisillä eteen- ja taaksepäin sävytyksillä, mukautettavilla peittävyyskäyrillä ja saumattomalla silmukan läpikierrolla.
- 🎨 **Hahmojen välittömät palettiulkoasut (Skins)**: Luo vaivattomasti päivä-/yöversioita, tiimivärejä tai elementtiteemoja reaaliaikaisella atlaksen uudelleenvärityksellä sekä GPL-, PAL- ja HEX-tuella.
- 📦 **Suora vienti pelimoottoreille**: Vie skaalattuja PNG-kuvia laajennetuilla reunapikseleillä (extrude), mikä ehkäisee tekstuurien saumavirheitä 2D-moottoreissa (Godot, Bevy, Raylib, Defold), yhdessä standardin JSON-metatiedon ja animoitujen GIF-kuvien kanssa.

---

## Kuvakaappaukset ja visuaalinen esittely

<div align="center">

### 1. Päätyötila ja piirtoalusta
*Huippusuorituskykyinen pikselityötila, jossa on laitteistokiihdytetty näkymä, moniraitainen aikajana, reaaliaikainen esikatselu, palettiruudut ja tarkkuustyökalupalkit.*
<img src="screenshots/main.png" alt="OniAnvilin päätyötila" width="950" />

<br/><br/>

### 2. Moniraitainen animaation aikajana
*Epälineaarinen aikajana pinotuilla tasoriveillä (`V1, V2...`), linkitetyillä ruutuinstansseilla (`🔗`), pidon kertoimilla ja nimetyillä värikoodatuilla animaatiotageilla.*
<img src="screenshots/timeline.png" alt="OniAnvilin moniraitainen aikajana" width="950" />

<br/><br/>

### 3. Tarkkuustyökalut ja ylätyökalupalkki
*Kaikki 15 piirtotyökalua vakio- ja kompaktilla telakoinnilla sekä yläpalkki siveltimen koon, symmetrian, jatkuvuuden ja pikselintarkan piirron säätöön.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Toimintotyökalupalkki" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Vakiotyökalupaneeli" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Kompakti työkalupaneeli" width="300" />
</p>

<br/><br/>

### 4. Reaaliaikainen animaation suora esikatselu
*Erillinen esikatselutelakka tagisuodatuksella, säädettävällä toistonopeudella (FPS), zoomaussäätimellä ja kankaan taustavaihtoehdoilla (shakkiruudukko, tumma, vaalea).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Suora esikatselu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Esikatselun zoomaus" width="460" />
</p>

<br/><br/>

### 5. Palettien hallinta, vaihto ja väriliu'ut
*Kattava värijärjestelmä etuala- ja taustavärilaatikoilla, automaattisella kahden pisteen väriliukujen generoinnilla ja nimetyillä hahmon palettivaihdoksilla.*
<img src="screenshots/palette_and_swaps.png" alt="Paletit ja ulkoasujen vaihto" width="950" />

<br/><br/>

### 6. Malliluonnoskerros ja konfiguroitava sipulinkuori
*Lataa konseptiluonnoksia tai valokuvia aluskerrokseksi tai päällikerrokseksi muunnostyökaluilla, älykkäällä palettiin kiinnittyvällä pipetillä ja moniruutuisella sipulinkuorikuultavuudella.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Luonnos ja sipulinkuori" width="950" />

<br/><br/>

### 7. Kiintopisteet (Pivot) ja toimintopisteet (Action Points)
*Määritä spriten origokeskipiste ja mukautetut nimetyt pelitoimintopisteet (aseankkurit, ammusten lähtöpisteet, pää, askeleet) interaktiivisilla ohjaimilla.*
<img src="screenshots/pivot_and_action_points.png" alt="Kiintopisteet ja toimintopisteet" width="950" />

<br/><br/>

### 8. Arkkileikkuri (Sheet Slicer) ja 9-alueinen (9-Slice) skaalaus
*Automaattinen spriten rajojen autokorrelaatioleikkaus (esim. kävelevä kissa) sekä skaalattava 9-alueinen käyttöliittymälaatikon reunamarginaalien konfigurointi.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Arkkileikkuri" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="9-Grid-skaalaus" width="460" />
</p>

<br/><br/>

### 9. Reaaliaikaiset kuvatehosteet ja suodatinikkunat (Live Preview)
*Kaikki tehosteikkunat sisältävät reaaliaikaisen esikatselun kankaalla, interaktiivisen pipetin ja häviöttömän peruutuksen ilman historian sotkemista.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Reunaviivageneraattori" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Värin korvaus" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Sävy, kylläisyys ja kontrasti" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matriisisuodatin" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Kiinnitys palettiin rasteroinnilla" width="460" />
</p>

<br/><br/>

### 10. Piirtoalustan ruudukko- ja taustavaihtoehdot
*Vaihda vaivattomasti pikseliruudukon, ruutujen rajojen, tumman ruutukuvion, vaalean ruutukuvion ja mukautetun kiinteän taustavärin välillä.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Ruudukko ja tumma shakkikuvio" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Vaalea shakkikuvio ilman ruudukkoa" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Mukautettu yksivärinen tausta" width="300" />
</p>

<br/><br/>

### 11. Uuden projektin ohjattu luominen
*Määritä kankaan mitat, ruutujen määrä, sarakeruudukko ja valitse yli 20 aidosta retropaletista projektin käynnistyksen yhteydessä.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Uuden projektin luonti" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Projektin paletit" width="460" />
</p>

<br/><br/>

### 12. 35 aitoa retro- ja modernia teemaa
*Mukana Studio Dark, Studio Light, CRT-fosforit (Amber, Green, Cyan), retrokonsolit (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) ja suunnittelijateemat.*
<p align="center">
  <img src="screenshots/themes.png" alt="Teemavalikko" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber -teema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX-teema" width="300" />
</p>

<br/><br/>

### 13. Valikkojärjestelmä ja sisäänrakennettu pikanäppäinohje
<p align="center">
  <img src="screenshots/file_menu.png" alt="Tiedosto-valikko" width="180" />
  <img src="screenshots/edit_menu.png" alt="Muokkaa-valikko" width="180" />
  <img src="screenshots/select_menu.png" alt="Valinta-valikko" width="180" />
  <img src="screenshots/view_menu.png" alt="Näytä-valikko" width="180" />
  <img src="screenshots/project_menu.png" alt="Projekti-valikko" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Pikanäppäinten ohjeikkuna" width="700" />
</p>

</div>

---

## Kattava työkalujen opas

OniAnvil sisältää 15 ammattimaista piirto-, muoto-, valinta- ja metatietotyökalua. Kuvakkeet käyttävät avoimen lähdekoodin **Font Awesome Free** -kuvakekokoelmaa *(Kuvakkeet: CC BY 4.0, Fontit: SIL OFL 1.1)*.

| Työkalu | Kuvake | Pikanäppäin | Siveltimen koko | Pixel Perfect | Kuvaus |
|:---|:---:|:---:|:---:|:---:|:---|
| **Lyijykynä** | ✏️ | <kbd>B</kbd> | 1..32 px | Kyllä | Ensisijainen piirtotyökalu. 1 px -tilassa tuki **Pixel-Perfect** -piirrolle (poistaa automaattisesti epäsiistit porrasmaiset L-kulmat). Tukee valinnasta luotuja leimasinsiveltimiä (`Ctrl/Cmd+B`). |
| **Pyyhekumi** | 🧹 | <kbd>E</kbd> | 1..32 px | Ei | Pyyhkii pikselit täysin läpinäkyviksi (`[0, 0, 0, 0]`). Rajautuu aktiiviseen valintaan, jos sellainen on olemassa. |
| **Pipetti** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ei | Vasen napsautus poimii etualavärin (Primary); oikea napsautus taustavärin (Secondary). Poimii kankaalta, tasoilta tai luonnoskuvasta. |
| **Täyttö** | 🪣 | <kbd>G</kbd> | — | Ei | Nopea jonopohjainen (BFS) täyttö säädettävällä väritoleranssilla (`0..=255`). Tukee yhtenäistä ja globaalia täyttöä sekä valintaan rajaamista. |
| **Liukuväri** | 🌈 | <kbd>G</kbd> *(vaihto)* | — | Ei | Lineaariset ja säteittäiset liukuvärit etuala- ja taustavärin välillä. Tukee Bayer- (2x2, 4x4, 8x8) ja Blue Noise -rasterointia voimakkuudensäädöllä. |
| **Viiva** | 📏 | <kbd>L</kbd> | 1..32 px | Kyllä | Bresenhamin suoraviiva-algoritmi reaaliaikaisella esikatselulla. <kbd>Shift</kbd> pohjassa lukitsee kulmat 45°/90° asteen portaisiin. |
| **Käyrä** | 〰️ | <kbd>C</kbd> | 1..32 px | Kyllä | 3 napsautuksen neliöllinen Bézier-käyrä. Napsautus 1 asettaa alkupisteen, Napsautus 2 päätepisteen, hiiren liike taivuttaa kaarta ja Napsautus 3 vahvistaa käyrän. |
| **Suorakulmio** | ⬜ | <kbd>U</kbd> | 1..32 px | Ei | Ääriviivalliset tai täytetyt geometriset suorakulmiot. <kbd>Shift</kbd> pohjassa pakottaa 1:1 neliöksi. |
| **Ellipsi** | ⭕ | <kbd>O</kbd> | 1..32 px | Ei | Ääriviivalliset tai täytetyt ellipsit keskipisterasteroinnilla. <kbd>Shift</kbd> pohjassa pakottaa täydelliseksi ympyräksi. |
| **Rasterisivellin** | 🏁 | <kbd>D</kbd> | 1..32 px | Ei | Maalaa shakkiruutuista rasterikuviota etuala- ja taustavärien välillä lisäämättä uusia palettivärejä. |
| **Varjostustyökalu** | 🌓 | <kbd>S</kbd> | 1..32 px | Ei | Asteittainen vaalennus (Dodge) ja tummennus (Burn) aktiivisen paletin järjestyksen mukaan. |
| **Suorakulmainen valinta** | ⬚ | <kbd>M</kbd> | — | Ei | Suorakulmainen aluevalinta animoidulla marssivien muurahaisten reunuksella. Tukee lisäystä (<kbd>Shift</kbd>) ja vähennystä (<kbd>Alt</kbd>). |
| **Lassovalinta** | ➰ | <kbd>Q</kbd> | — | Ei | Vapaamuotoinen monikulmiolasso. Käyttää 8-kytkettyä reunarasterointia ja 4-kytkettyä ulkopuolista täyttöä taaten 100 % ehyen valinnan ilman reikiä myös silmukoissa. |
| **Taikasauva** | 🪄 | <kbd>W</kbd> | — | Ei | Valitsee yhtenäisen (tai globaalin) samanvärisen alueen säädettävällä toleranssikynnyksellä. |
| **Nivel- ja toimintopisteet** | 🎯 | <kbd>P</kbd> | — | Ei | Spriten origon ja mukautettujen toimintopisteiden (`weapon`, `head`, `feet`, `hitbox`) interaktiivinen sijoittelu kankaalle ja vienti JSON-metatietoihin. |

---

## Toiminnallinen dokumentaatio

### 1. Kaksoisatlakseen perustuva arkkitehtuuri

OniAnvilissa päätekstuuriatlas (`atlas_rgba`) edustaa todellista sprite-arkkitekstuuria sellaisena kuin se ladataan pelimoottoriin:
- **Ruututila (Frame Mode)**: Keskittyy yhteen aktiiviseen animaatioruutuun tarjoten häiriöttömän pikselikankaan. Muokkaukset päivittyvät välittömästi vastaavaan soluun atlastestuurissa.
- **Atlastila (Atlas Mode)**: Näyttää koko sprite-arkin ruudukon, mahdollistaen makrotason säädöt, ruutujen kohdistustarkistukset ja massamuokkaukset.
- **Oheistiedosto (Companion File)**: Projektit tallentavat puhtaan `.png`-sprite-arkin ja sen rinnalla kevyen `.onianvil.json`-oheistiedoston, joka sisältää ruutujärjestelyt, tasohierarkiat, tagit, pidon kertoimet ja palettiulkoasut.

### 2. Moniraitainen tasojen aikajana

Alareunan aikajana on mallinnettu ammattimaisten epälineaaristen videoeditointiohjelmien mukaisesti:
- **Tasoraidat (`V1, V2, V3...`)**: Tasot pinoutuvat visuaalisesti ylhäältä alas. Aikajanalla ylempänä sijaitseva raita (`V1`) piirtyy kankaalla alimpien päälle.
- **Raidan säätimet**:
  - Napsauta `V1` / `V2` -merkkejä vaihtaaksesi aktiivista piirtotasoa nopeasti.
  - Kytke tason näkyvyys (silmäkuvake) ja muokkauksen lukitus (lukkokuvake).
  - Säädä yksittäisen tason peittävyyttä (`0%..=100%`).
  - Siirrä tasoa ylös/alas hierarkiassa, monista taso ja yhdistä taso alaspäin alempaan raitaan (Merge Down).
  - Poista taso vahvistusikkunan kautta.
- **Ruutuleikkeet (Frame Clips)**:
  - Pidon kerroin: määritä mukautettuja ruutujen kestoja (esim. `2×`, `3×` pito taukoja tai iskuruutuja varten).
  - Linkitetyt ruudut (`🔗`): linkitä toistuvia ruutuja animaation läpi niin, että yhden muokkaaminen päivittää välittömästi kaikki linkitetyt instanssit.
  - Tagien silmukointi: järjestä animaatiot nimetyiksi tunnisteiksi (`idle`, `walk`, `attack`) ja toista tiettyjä tageja silmukassa.
  - Toistosäätimet: Eteenpäin, Taaksepäin, Ping-Pong ja Jatkuva silmukka säädettävällä ruudunpäivitysnopeudella (`1..=120` FPS).

### 3. Reaaliaikainen animaation suora esikatselu

Oikeanpuoleinen esikatselutelakka toistaa animaatioita reaaliajassa, täysin riippumattomana kankaan zoomauksesta tai panoroinnista:
- **Tagisuodatus**: Eristä toisto tiettyihin tageihin (`walk`, `jump`) tai esikatsele koko animaatiosarjaa.
- **Toistosäätimet**: Toisto, Tauko, Edellinen ruutu, Seuraava ruutu, Silmukan kytkentä ja mukautetut toistonopeudet (`0.25x..=4.0x`).
- **Näyttötilat**: Vaihda tumman shakkiruudukon, vaalean shakkiruudukon, puhtaan valkoisen, puhtaan mustan tai mukautetun kiinteän taustavärin välillä.

### 4. Edistynyt sipulinkuorimoottori (Onion Skinning)

Muotoile sujuvia animaatioita erikoistuneella sipulinkuorijärjestelmällä:
- **Mukautettu sävytys**: Erilliset sävyvärit edellisille ruuduille (oletus: punainen) ja seuraaville ruuduille (oletus: sininen).
- **Peittävyyden vaimennus**: Määritettävä peruspeittävyys ja asteittainen vaimennuskäyrä etäisyyden kasvaessa.
- **Silmukan läpikierto**: Ota käyttöön kuultavuus ensimmäisen ja viimeisen ruudun välillä saumattomien silmukoiden luomiseksi.
- **Pikavilkaisu (Quick-Peek)**: Pidä <kbd>Välilyöntiä</kbd> pohjassa ilman raahausta nähdäksesi viereiset ruudut välittömästi.

### 5. Älykäs viitekuva- ja luonnoskerros

Nopeuta työskentelyäsi piirtämällä käsintehtyjen luonnosten tai referenssikuvien päältä:
- **Tuetut tiedostomuodot**: Lataa PNG-, JPEG- ja WebP-viitekuvia.
- **Sekoitustilat**: Aluskerros (pikselitasojen alla) tai Päällikerros (pikselitasojen päällä).
- **Muunnostyökalut**: Itsenäinen mittakaava, sijaintisiirtymä, kierto ja peittävyyden liukusäädin.
- **Älykäs värinpoiminta**: Käytä pipettiä luonnostasolla valinnaisella automaattisella kiinnityksellä projektin aktiivisen paletin lähimpään väriin.
- **1 napsautuksen poltto (Bake)**: Yhdistä skaalattu referenssikuva suoraan puhtaaksi pikselitaiteeksi aktiiviselle ruudulle yhdellä napsautuksella.

### 6. Tarkka valintatyökalusto ja marssivat muurahaiset

- **Animoidut marssivat muurahaiset**: Aktiiviset valinnat näyttävät animoidun, kaksisävyisen ja erittäin erottuvan katkoviivoreunuksen.
- **Umpinainen monikulmiolasso**: Toisin kuin naiivit säteenheijastusalgoritmit, jotka luovat reikiä päällekkäisissä silmukoissa, OniAnvilin lasso rasteroi suljetun 8-kytketyn reunan ja täyttää ulkopuolen taaten 100 % umpeen täytetyn, reiättömän valinnan.
- **Valintatoiminnot**: Laajenna, Supista, Reunaviiva (pyöreällä, neliöllisellä ja vinoneliöllisellä ytimellä), Siirrä, Käännä ympäri (Invert), Poista, Vapaa kierto sekä järjestelmän leikepöydän Kopioi/Leikkaa/Liitä.
- **Ruutujen välinen kopiointi ja liittäminen**: Valitse alue yhdellä ruudulla, paina <kbd>Ctrl/Cmd+C</kbd>, vaihda ruutua ja paina <kbd>Ctrl/Cmd+V</kbd> liittääksesi täsmälleen samoihin suhteellisiin koordinaatteihin riippumatta näppäimistöasettelusta.

### 7. Reaaliaikaiset tehosteet, suodattimet ja RotSprite

Kaikki kuvatehosteet sisältävät reaaliaikaisen **Live Preview** -valintaruudun, joka esikatselee muutokset suoraan kankaalla ja sallii peruutuksen ilman kumoamishistorian sotkemista:
- **Värin korvaus (`Shift+R`)**:
  - Interaktiivinen pipetti: napsauta pipettikuvaketta ja valitse korvattava väri suoraan kankaalta tai palettipalkista.
  - Pikavalintapalkki: vasen napsautus valitsee lähtövärin ("Mistä"), oikea napsautus kohdevärin ("Mihin").
  - Toleranssisäädin ja massakorvaus kerralla kaikille animaation ruuduille.
- **Värisäätö (`Ctrl/Cmd+U`)**: Reaaliaikainen HSV-sävyn siirto, kylläisyys, kirkkausarvo ja kontrastin säätö.
- **Automaattinen reunaviivageneraattori**: Luo terävät pikselireunat hahmolle (Ulkopuolelle, Sisäpuolelle, Keskitetty) neliö- tai timanttikulmilla.
- **3×3-konvoluutiosuodattimet**: Sumennus, Terävöitys, Reunantunnistus ja Kohokuviointi (Emboss) alfan säilytyksellä.
- **Vapaa kierto (`Ctrl/Cmd+Shift+R`)**: Kierrä valittuja alueita tai kokonaisia ruutuja mihin tahansa kulmaan käyttäen klassista **RotSprite**- tai lähimmän naapurin (**Nearest Neighbor**) algoritmia.
- **Kiinnitys palettiin**: Kvantisoi kankaan värit aktiiviseen palettiin käyttäen Bayer 2x2-, 4x4-, 8x8- tai Blue Noise -rasterointia.

### 8. Palettien hallinta, vaihto ja hahmon ulkoasut (Skins)

- **Sisäänrakennetut paletit**: Yli 20 aitoa retropalettia, mukaan lukien Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized ja Sweetie 16.
- **Tuonti ja vienti**: Tuki GIMP-paleteille (`.gpl`), PAL-tiedostoille (`.pal`), heksadesimaalipaleteille (`.hex`) ja kuvanäytteille (`.png`).
- **Palettien vaihto (Skins)**: Määritä useita palettiulkoasuja samalle hahmolle (esim. Oletus, Kultainen haarniska, Varjo, Jää). Ulkoasun vaihtaminen remppaa välittömästi kaikki ruutuvärit koko atlaksessa.
- **Väriliukugeneraattori**: Interpoloitu pehmeä liukuväri kahden valitun sävyn välille säädettävällä välivaiheiden määrällä.

### 9. Arkkileikkuri, 9-alueinen skaalaus ja pelimoottorivienti

- **Arkkileikkuri (Sheet Slicer)**: Autokorrelaatiopohjainen reunantunnistus leikkaa raa'at sprite-arkit automaattisesti tasaisiksi tai tarkasti rajatuiksi animaatioruuduiksi.
- **9-alueinen skaalaus (9-Slice)**: Aseta mukautettavat leikkausmarginaalit (ylä, ala, vasen, oikea) esikatsellaksesi skaalautuvia käyttöliittymälaatikoita ja painikepaneeleja kulmia venyttämättä.
- **PNG-vienti laajennetuilla reunapikseleillä (Extruded Border)**: Monistaa reunapikselit läpinäkyvään marginaaliin, mikä estää tekstuurisuodatuksen saumavirheet ja värivuodot OpenGL-, DirectX-, Vulkan- ja 2D-pelimoottoreissa (kuten Godot).
- **Animoitu GIF-vienti**: Luo optimoituja animoituja GIF-tiedostoja mukautetuilla silmukkamäärillä, ruutuviiveillä ja läpinäkyvyydellä.
- **Standardi Sprite Sheet JSON**: Vie ruutujen koordinaatit, tagit, kestot ja kiintopisteet standardissa muodossa, jota nykyaikaiset 2D-pelimoottorit tukevat suoraan.

### 10. 35 ergonomista teemaa ja monikielinen lokalisointi

- **Valikoidut teemat**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Green CRT, Cyan CRT, Retro-konsolit (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) ja Retro-tietokoneet (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Monikielinen käyttöliittymä**: Vaihda kieltä saumattomasti lennosta täydellisellä käännösten vastaavuudella 37 kielen välillä.

---

## Ohjaus ja pikanäppäimet

| Pikanäppäin | Työkalu / Toiminto | Kuvaus |
|:---|:---|:---|
| <kbd>B</kbd> | **Lyijykynä** | 1 px tarkka piirto valinnaisella Pixel-Perfect-tilalla |
| <kbd>E</kbd> | **Pyyhekumi** | Pyyhkii pikselit läpinäkyviksi |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipetti** | Poimii värin kankaalta tai luonnoksesta |
| <kbd>G</kbd> | **Täyttö / Liukuväri** | Vaihda täyttöämpärin ja rasteroidun liukuvärin välillä |
| <kbd>L</kbd> | **Viiva** | Piirrä suoria viivoja esikatselulla ja Shift-kulmalukituksella |
| <kbd>C</kbd> | **Käyrä** | 3 napsautuksen neliöllinen Bézier-käyrätyökalu |
| <kbd>U</kbd> | **Suorakulmio** | Suorakulmion ja neliön ääriviiva tai täyttö |
| <kbd>O</kbd> | **Ellipsi** | Ellipsin ja ympyrän ääriviiva tai täyttö |
| <kbd>D</kbd> | **Rasterisivellin** | Maalaa shakkiruutuista rasterimatriisikuviota |
| <kbd>S</kbd> | **Varjostustyökalu** | Asteittainen vaalennus / tummennus palettirampin mukaan |
| <kbd>M</kbd> | **Suorakulmainen valinta** | Suorakulmainen aluevalinta marssivilla muurahaisilla |
| <kbd>Q</kbd> | **Lassovalinta** | Vapaamuotoinen yhtenäinen monikulmiovalinta |
| <kbd>W</kbd> | **Taikasauva** | Valitse yhtenäiset samanväriset alueet |
| <kbd>P</kbd> | **Kiintopisteet** | Sijoita spriten origo ja toimintopisteet |
| <kbd>X</kbd> | **Vaihda värit** | Vaihda etuala- ja taustavärien paikkaa |
| <kbd>[</kbd> / <kbd>]</kbd> | **Siveltimen koko** | Pienennä / Suurenna siveltimen kokoa (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopioi** | Kopioi valitut pikselit leikepöydälle |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Leikkaa** | Leikkaa valitut pikselit leikepöydälle |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Liitä** | Liitä leikepöydän pikselit aktiiviselle ruudulle |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Poista** | Tyhjennä pikselit aktiivisen valinnan sisältä |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Valitse kaikki** | Valitse kaikki aktiivisen ruudun pikselit |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Poista valinta** | Tyhjennä aktiivinen valintamaski |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Käännä valinta** | Käännä aktiivinen valintamaski päinvastaiseksi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Monista ruutu** | Monista aktiivinen ruutu aikajanalla |
| <kbd>Välilyönti</kbd> | **Toista / Tauko** | Käynnistä tai pysäytä animaation toisto aikajanalla |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Korvaa väri** | Avaa värin korvausikkuna suoralla esikatselulla |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Säädä väriä** | Avaa HSV-sävyn, kylläisyyden ja kontrastin ikkuna |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Vapaa kierto** | Avaa vapaan kulman kiertoikkuna (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Kumoa** | Kumoa edellinen siveltimenveto tai toiminto |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Tee uudelleen** | Tee aiemmin kumottu toiminto uudelleen |
| <kbd>Välilyönti</kbd> + Raahaus / Hiiren keskipainike | **Panoroi kangasta** | Sulava ääretön kankaan koordinaattien siirtäminen |
| <kbd>Ctrl/Cmd</kbd> + Rulla / <kbd>+</kbd> / <kbd>-</kbd> | **Zoomaa** | Pikselintarkka zoomaus hiiren osoittimen kohdalla |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Näkymätila** | Vaihda ruutunäkymän (<kbd>1</kbd>) ja atlasnäkymän (<kbd>2</kbd>) välillä |
| <kbd>F</kbd> | **Sovita kankaalle** | Keskitä ja sovita aktiivinen ruutu työtilaan |
| <kbd>F1</kbd> | **Ohje** | Avaa pikanäppäinten ohjeikkuna |

---

## Aloitusopas

### Valmiit suoritettavat tiedostot

Lataa valmiiksi käännetyt, itsenäiset suoritettavat tiedostot **[GitHub-julkaisusivulta](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Lataa `OniAnvil-windows-x86_64.zip`, pura paketti ja kaksoisnapsauta `OniAnvil.exe`.
- **Linux**: Lataa `OniAnvil-linux-x86_64.tar.gz` (siirrettävä paketti) tai asenna `onianvil_*_amd64.deb` Debian/Ubuntu-jakeluille:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Lataa `OniAnvil-macos-universal.zip`, pura paketti ja vedä `OniAnvil.app` Ohjelmat-kansioon. Universaali binääri toimii natiivisti sekä Apple Siliconilla (M1/M2/M3/M4) että Intel-pohjaisilla Mac-tietokoneilla.
- **Verkkoversio**: Käytä suoraan selaimessasi ilman asennusta: **[Käynnistä WebAssembly-sovellus](https://rbm13s.github.io/OniAnvil/app/?lang=fi)**.
  > [!WARNING]
  > WebAssembly-selainversiolla on **kokeellinen (Experimental)** tila. Selaimen hiekkalaatikkorajoitusten, WebGL/WebGPU-erojen ja leikepöydän rajoitusten vuoksi saatat kohdata pieniä grafiikkavirheitä tai suorituskykyeroja työpöytäversioon verrattuna. Tuotantotyöhön suositellaan lämpimästi natiiveja työpöytäasennuksia.

---

### Kääntäminen lähdekoodista

#### Esivaatimukset
- **Rust-työkaluketju**: Asenna Rust (suositus 1.80+) osoitteesta [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linuxin käännösriippuvuudet** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Käännä ja suorita natiivi työpöytäversio
```bash
# Kloonaa repositorio
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Suorita testit
cargo test

# Rakenna ja käynnistä optimoitu julkaisuversio
cargo run --release
```

#### 2. Käännä WebAssembly-versio (Trunkilla)
```bash
# Lisää wasm32-käännöskohde ja asenna Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Käynnistä paikallinen kehityspalvelin kuumapäivityksellä
trunk serve

# Rakenna optimoidut julkaisutiedostot kansioon docs/app/
./build_wasm.sh        # Linuxilla / macOS:llä
./build_wasm.ps1       # Windows PowerShellissä
```

---

## Tiedostomuoto ja oheistiedostoarkkitehtuuri

OniAnvil välttää suljettuja binäärimuotoja. Jokainen projekti koostuu kahdesta puhtaasta, versionhallintaan erinomaisesti soveltuvasta tiedostosta:

```
my_character/
├── my_character.png            # Standardi PNG-tekstuuriatlas (avautuu missä tahansa moottorissa tai katselimessa)
└── my_character.onianvil.json   # Oheistiedosto projektin metatiedoille
```

`.onianvil.json`-tiedosto sisältää:
- Kankaan ruudukkomääritykset: ruudun mitat, sarakkeiden määrä, ruutujen kokonaismäärä.
- Tasohierarkiat: raitojen nimet, peittävyystasot, näkyvyysliput, lukitustilat.
- Animaatiotunnisteet: nimi, alku-/loppuruudun indeksit, toistosuunta (Eteenpäin, Taaksepäin, Ping-Pong), ruutujen kestot.
- Palettimääritykset: aktiiviset värilaatat, nimetyt ulkoasumuunnelmat (skins), aktiivisen ulkoasun indeksi.
- Malliluonnoksen tila: kuvan polku, mittakaava, panorointisiirtymä, kierto, peittävyys, tasotila.
- Kiintopisteet: mukautetut toimintopisteet, aseankkurit ja törmäyskeskipisteet.

---

## Osallistuminen ja kiitokset

Kaikki parannusehdotukset, virheraportit ja ominaisuustoiveet ovat erittäin tervetulleita!
1. Tee repositoriosta oma haarukka (Fork) GitHubissa.
2. Luo uusi ominaisuushaara: `git checkout -b feature/uusi-pikselityokalu`.
3. Tee muutokset ja commitoi: `git commit -am 'Lisää uusi pikselityökalu'`.
4. Lähetä haara omaan repositorioosi: `git push origin feature/uusi-pikselityokalu`.
5. Avaa Pull Request.

### Kiitokset
- Aloituksen demosprite **"Cat 2D Pixel Art"** on tekijän **xzany** luoma ([itch.io-projektisivu](https://xzany.itch.io/cat-2d-pixel-art)).
- Rakennettu upean **[egui](https://github.com/emilk/egui)** -käyttöliittymäkirjaston ja **[wgpu](https://wgpu.rs/)** -grafiikkaekosysteemin päälle.
- Käyttöliittymäkuvakkeet tarjoaa **[Font Awesome Free](https://fontawesome.com)** (@fontawesome) (Kuvakkeet: CC BY 4.0, Fontit: SIL OFL 1.1, Koodi: MIT-lisenssi).

---

## Lisenssi ja vastuuvapauslauseke

Tämä projekti on lisensoitu **[MIT-lisenssillä](LICENSE)**. Voit vapaasti käyttää, muokata, levittää ja integroida OniAnvilia kaupallisiin ja henkilökohtaisiin pelinkehitysprojekteihin.

### Takuun vastuuvapauslauseke ja vastuunrajoitus
> [!IMPORTANT]
> **OniAnvil** jaetaan **maksutta** sellaisenaan (**"AS IS"**) ja saatavuuden mukaan (**"AS AVAILABLE"**), ilman minkäänlaisia nimenomaisia tai oletettuja takuita tai ehtoja.
> 
> Tekijä(t), tekijänoikeuden haltijat tai osallistujat eivät missään olosuhteissa ole vastuussa mistään suorista, epäsuorista, satunnaisista, erityisistä, varoittavista tai välillisistä vahingoista (mukaan lukien, mutta ei rajoittuen, tietojen menetys, tiedostojen korruptoituminen, laitteistoviat, saamatta jääneet voitot tai liiketoiminnan keskeytyminen), jotka johtuvat millään tavalla tämän ohjelmiston käytöstä tai kyvyttömyydestä käyttää sitä, vaikka tällaisen vahingon mahdollisuudesta olisi ilmoitettu. Käyttäjä ottaa kaiken vastuun ja riskin tämän ohjelmiston käytöstä.

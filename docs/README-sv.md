<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixelgrafik- och lökbladsanimationsstudio (Onion-Skinning) byggd för spelklara texturatlaser**

[![Bygge och release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Nedladdningar](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderare](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Språk](https://img.shields.io/badge/Språk-37_språk-purple)](#10-35-ergonomiska-teman-och-fullständig-lokalisering)
[![Plattform](https://img.shields.io/badge/Plattform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#kom-igång)
[![Webbdemo](https://img.shields.io/badge/Online-Kör_i_webbläsaren_(Experimentell)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sv)
[![Licens: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Kör OniAnvil i webbläsaren (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sv)** *(⚠️ Experimentell version: kan innehålla WebGL/WASM-begränsningar och buggar)* • **[🌐 Officiell dokumentationswebbplats](https://rbm13s.github.io/OniAnvil/sv.html)**

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
**Svenska** •
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

## Innehållsförteckning

- [Introduktion](#introduktion)
- [Skärmdumpar och visuell rundtur](#skärmdumpar-och-visuell-rundtur)
- [Omfattande verktygsreferens](#omfattande-verktygsreferens)
- [Funktionsdokumentation](#funktionsdokumentation)
  - [1. Atlascentrerad dubbelarkitektur](#1-atlascentrerad-dubbelarkitektur)
  - [2. Flerspårig lagertidslinje](#2-flerspårig-lagertidslinje)
  - [3. Förhandsvisning i realtid (Live Preview)](#3-förhandsvisning-i-realtid-live-preview)
  - [4. Avancerad lökbladsmotor (Onion Skinning)](#4-avancerad-lökbladsmotor-onion-skinning)
  - [5. Smart referens- och skisskalkeringslager](#5-smart-referens--och-skisskalkeringslager)
  - [6. Precisionsmarkeringssvit och vandrande myror (Marching Ants)](#6-precisionsmarkeringssvit-och-vandrande-myror-marching-ants)
  - [7. Live-effekter, filter och RotSprite](#7-live-effekter-filter-och-rotsprite)
  - [8. Paletthantering, färgbyten och karaktärsskal (Skins)](#8-paletthantering-färgbyten-och-karaktärsskal-skins)
  - [9. Spritearksdelare, 9-Grid UI och export till spelmotorer](#9-spritearksdelare-9-grid-ui-och-export-till-spelmotorer)
  - [10. 35 ergonomiska teman och fullständig lokalisering](#10-35-ergonomiska-teman-och-fullständig-lokalisering)
- [Kontroller och kortkommandon](#kontroller-och-kortkommandon)
- [Kom igång](#kom-igång)
  - [Fristående binärer](#fristående-binärer)
  - [Bygga från källkod](#bygga-från-källkod)
- [Filformat och följeslagararkitektur](#filformat-och-följeslagararkitektur)
- [Bidra och tack](#bidra-och-tack)
- [Licens och ansvarsfriskrivning](#licens-och-ansvarsfriskrivning)

---

## Introduktion

**OniAnvil** (*från **Oni**on-Skinning [lökbladsanimering] + **Anvil** [städ för spritetillverkning]*) är en öppen, högpresterande pixelgrafiks- och animationsstudio skriven i Rust. Den körs helt integrerat med hårdvaruaccelererad grafik på Windows, Linux och macOS, samt som en nollinstallations WebAssembly-applikation direkt i moderna webbläsare.

De flesta befintliga sprite-redigerare betraktar animation som en samling fristående bildfiler, vilket tvingar utvecklare till tröttsamma arbetsflöden med export, uppdelning och packning. **OniAnvil vänder på detta paradigm:** spelatlasens textur (`atlas_rgba`) är den primära källan till sanning. Bildrutor, lagerspår, animationstaggar och kollisionspunkter hålls kontinuerligt synkroniserade med atlasen på ett icke-destruktivt sätt.

### Varför OniAnvil?
- 🚀 **Blixtsnabb prestanda i ren Rust**: Omedelbar uppstart på under millisekunden, knivskarp grafik driven av `wgpu` och `egui 0.31`, noll fördröjning vid navigering i långa tidslinjer.
- 🎬 **Flerspårig lagertidslinje**: NLE-inspirerade lagerspår (`V1, V2, V3...`) med lagerkomposition uppifrån och ned, sammanfogning, länkade bildrutor och variabla bildrutedurationer.
- 🧅 **Förstklassigt lökblad (Onion Skinning)**: Flerbildrutors genomskinlighet med oberoende färgtoning framåt och bakåt, anpassningsbara opacitetskurvor och sömlös loop-avrundning.
- 🎨 **Omedelbara palettskal (Skins)**: Skapa dag-/nattvarianter, lagfärger eller elementarskal med direkt ommappning i atlasen samt import och export av GPL-, PAL- och HEX-filer.
- 📦 **Direkt export till spelmotorer**: Exportera skalade PNG-bilder med extruderade kanter för att eliminera texturblödning i 2D-motorer (Godot, Bevy, Raylib, Defold), tillsammans med standardiserad Sprite Sheet JSON-metadata och animerade GIF-bilder.

---

## Skärmdumpar och visuell rundtur

<div align="center">

### 1. Huvudduk och studioarbetsyta
*Högpresterande pixelarbetsyta med hårdvaruaccelererad vy, flerspårig tidslinje, förhandsvisning i realtid, palettrutor och precisionsverktygsfält.*
<img src="screenshots/main.png" alt="OniAnvil Huvudarbetsyta" width="950" />

<br/><br/>

### 2. Flerspårig animationstidslinje
*Ickelinjär flerspårig tidslinje med staplade lagerrader (`V1, V2...`), länkade bildruteinstanser (`🔗`), hållmultiplikatorer och namngivna färgkodade animationstaggar.*
<img src="screenshots/timeline.png" alt="OniAnvil Flerspårig tidslinje" width="950" />

<br/><br/>

### 3. Precisionsverktyg och snabbåtgärdsfält
*Komplett ritverktygssvit med 15 verktyg i standard- och kompaktdockningslägen, tillsammans med det övre snabbåtgärdsfältet för penselstorlek, symmetri, kantupprepning och Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Åtgärdsfält" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Verktyg standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Verktyg kompakt" width="300" />
</p>

<br/><br/>

### 4. Förhandsvisning av animation i realtid (Live Preview)
*Dedikerad uppspelningsdocka med taggfiltrering, variabel FPS-hastighetskontroll, zoomreglage och bakgrundsväxling för vyn (schackmönster, mörk enfärgad, ljus enfärgad).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Förhandsvisning" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Förhandsvisning zoom" width="460" />
</p>

<br/><br/>

### 5. Paletthantering, färgbyten och färgövergångar (Ramps)
*Omfattande färgmotor med rutor för förgrunds-/bakgrundsfärger, automatisk generering av tvåpunkts färgövergångar och namngivna palettskal (Skins).*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palett och skal" width="950" />

<br/><br/>

### 6. Skisskalkering och konfigurerbart lökblad (Onion Skinning)
*Läs in konceptskisser eller fotografier som underlägg/överlägg med transformeringskontroller, smart färgprovtagning med palettfästning och flerbildrutors lökbladsskuggor.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Skiss och lökblad" width="950" />

<br/><br/>

### 7. Fästpunkter (Pivots) och kollisions-/åtgärdspunkter
*Definiera spritens mittpunkt/origo och anpassade namngivna speldynamiska punkter (vapenfästen, projektilpunkter, huvud, fotsteg) med interaktiva verktygsmarkörer på duken.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Fästpunkter och åtgärdspunkter" width="950" />

<br/><br/>

### 8. Spritearksdelare (Sheet Slicer) och 9-Grid (9-Slice) skalare
*Automatisk avgränsningsidentifiering med autokorrelation (t.ex. kattens gångcykel) samt skalbar 9-slice UI-rutemarginalskonfiguration.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Spritearksdelare" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Slice skalare" width="460" />
</p>

<br/><br/>

### 9. Bild-FX och filterdialoger i realtid (med direkt förhandsvisning)
*Alla effektdialoger har direkt förhandsvisning på duken, interaktiva pipetter och icke-destruktiv avbrytning.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Konturgenerator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Ersätt färg" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Nyans Mättnad Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrisfilter" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Mappa till palett" width="460" />
</p>

<br/><br/>

### 10. Rutnäts- och bakgrundslägen för duken
*Växla sömlöst mellan pixelrutnät, rutmönster, mörkt schackmönster, ljust schackmönster och anpassade enfärgade bakgrunder.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Rutnät och mörkt schackmönster" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Ljust schackmönster utan rutnät" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Anpassad enfärgad bakgrund" width="300" />
</p>

<br/><br/>

### 11. Guide för skapande av nya projekt
*Konfigurera dukdimensioner, bildrutor, kolumnlayout och välj bland över 20 autentiska retropaletter vid projektstart.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Guide för nytt projekt" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletter för nytt projekt" width="460" />
</p>

<br/><br/>

### 12. 35 autentiska retro- och moderna teman
*Inkluderar Studio Dark, Studio Light, CRT Amber/Green/Cyan-fosfor, retrokonsoler (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) och designerteman.*
<p align="center">
  <img src="screenshots/themes.png" alt="Temameny" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber-tema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX-tema" width="300" />
</p>

<br/><br/>

### 13. Menysystem och inbyggd genvägshjälp
<p align="center">
  <img src="screenshots/file_menu.png" alt="Arkivmeny" width="180" />
  <img src="screenshots/edit_menu.png" alt="Redigerameny" width="180" />
  <img src="screenshots/select_menu.png" alt="Markerameny" width="180" />
  <img src="screenshots/view_menu.png" alt="Visameny" width="180" />
  <img src="screenshots/project_menu.png" alt="Projektmeny" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Hjälpfönster för kortkommandon" width="700" />
</p>

</div>

---

## Omfattande verktygsreferens

OniAnvil innehåller 15 precisionsverktyg för ritning, geometri, markering och metadata. Ikonerna återges med den öppna ikonsviten **Font Awesome Free** *(Ikoner: CC BY 4.0, Typsnitt: SIL OFL 1.1)*.

| Verktyg | Ikon | Kortkommando | Penselstorlek | Pixel Perfect | Beskrivning |
|:---|:---:|:---:|:---:|:---:|:---|
| **Penna** | ✏️ | <kbd>B</kbd> | 1..32 px | Ja | Primärt ritverktyg. 1px-läget stöder **Pixel-Perfect**-ritning (eliminerar automatiskt fula trappstegs-L-hörn). Stöder anpassade stämpelpenslar från markeringar (`Ctrl/Cmd+B`). |
| **Suddgummi** | 🧹 | <kbd>E</kbd> | 1..32 px | Nej | Raderar pixlar tillbaka till full genomskinlighet (`[0, 0, 0, 0]`). Begränsas av markeringsmasken om den är aktiv. |
| **Pipett** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nej | Vänsterklick väljer färg till Förgrund (Primär); Högerklick väljer till Bakgrund (Sekundär). Hämtar färg från dukpixlar, lager eller referensskiss. |
| **Färghink** | 🪣 | <kbd>G</kbd> | Ej tillämpligt | Nej | Snabbt köbaserat BFS-fyllningsverktyg med justerbar färgtolerans (`0..=255`). Stöder sammanhängande vs global fyllning och markeringsbegränsning. |
| **Toning** | 🌈 | <kbd>G</kbd> *(växla)* | Ej tillämpligt | Nej | Linjära och radiella färgövergångar mellan förgrunds- och bakgrundsfärger. Stöder Bayer- (2x2, 4x4, 8x8) och Blue Noise-dithering med styrkekontroll. |
| **Linje** | 📏 | <kbd>L</kbd> | 1..32 px | Ja | Bresenham-linjealgoritm med direkt förhandsvisning vid dragning. Håll <kbd>Shift</kbd> för att låsa vinklar till steg om 45°/90°. |
| **Kurva** | 〰️ | <kbd>C</kbd> | 1..32 px | Ja | Kvadratisk Bézier-kurva med 3 klick. Klick 1 sätter startpunkt, Klick 2 sätter slutpunkt, musrörelse böjer kurvaturen, Klick 3 bekräftar kurvan. |
| **Rektangel** | ⬜ | <kbd>U</kbd> | 1..32 px | Nej | Geometriska rektanglar med kontur eller fyllning. Håll <kbd>Shift</kbd> för att låsa till en proportionell 1:1-kvadrat. |
| **Ellips** | ⭕ | <kbd>O</kbd> | 1..32 px | Nej | Ellipser och cirklar med kontur eller fyllning via mittpunktsrasterisering. Håll <kbd>Shift</kbd> för en perfekt cirkel. |
| **Dithering-pensel** | 🏁 | <kbd>D</kbd> | 1..32 px | Nej | Målar alternerande schackmönstrade dithering-mönster mellan förgrunds- och bakgrundsfärger utan att lägga till extra palettfärger. |
| **Skuggningsverktyg** | 🌓 | <kbd>S</kbd> | 1..32 px | Nej | Stegvis ljusning (Dodge) och mörkning (Burn) baserat på färgordningen i den aktiva paletten. |
| **Rektangulär markering** | ⬚ | <kbd>M</kbd> | Ej tillämpligt | Nej | Rektangulär markeringsram med animerad kantlinje av vandrande myror (marching ants). Stöder lägg till (<kbd>Shift</kbd>) och dra ifrån (<kbd>Alt</kbd>). |
| **Lassomarkering** | ➰ | <kbd>Q</kbd> | Ej tillämpligt | Nej | Frihandspolygonlasso. Använder 8-kopplad kantrasterisering med 4-kopplad yttre flödesfyllning, vilket garanterar 100 % heltäckande markeringar utan hål även vid överlappande öglor. |
| **Trollspö** | 🪄 | <kbd>W</kbd> | Ej tillämpligt | Nej | Markerar sammanhängande (eller globala) områden med matchande färg och justerbar toleranströskel. |
| **Fästpunkt & åtgärdspunkt** | 🎯 | <kbd>P</kbd> | Ej tillämpligt | Nej | Interaktiv dukplacering av spritens mittpunkt (Pivot) och anpassade speldynamiska åtgärdspunkter (`weapon`, `head`, `feet`, `hitbox`) som exporteras till JSON-metadata. |

---

## Funktionsdokumentation

### 1. Atlascentrerad dubbelarkitektur

I OniAnvil representerar huvudtexturen (`atlas_rgba`) den verkliga spritearksstrukturen så som den läses in av din spelmotor:
- **Bildruteläge (Frame Mode)**: Fokuserar på en enda aktiv animationsbildruta, vilket ger en distraktionsfri pixelyta. Ändringar uppdaterar omedelbart motsvarande cell i atlastexturen.
- **Atlasläge (Atlas Mode)**: Visar hela spritearkets rutnät för makrojusteringar, inriktningskontroll över flera rutor och massredigering.
- **Följeslagarfil (Companion File)**: Projekt sparar rena `.png`-spriteark tillsammans med en lätt `.onianvil.json`-metadatafil som innehåller bildrutelayouter, lagerhierarkier, taggar, bildrutedurationer och palettskal.

### 2. Flerspårig lagertidslinje

Den nedre tidslinjen är utformad efter principer från professionella videoredigeringsprogram:
- **Lagerspår (`V1, V2, V3...`)**: Lagren staplas visuellt uppifrån och ned. Spåret som ligger högst upp i tidslinjen (`V1`) renderas överst på bildruteduken.
- **Spårkontroller**:
  - Klicka på märkena `V1` / `V2` för att snabbt byta aktivt ritlager.
  - Växla lagersynlighet (ögonikon) och redigeringslås (hänglåsikon).
  - Justera individuellt lagers opacitet (`0%..=100%`).
  - Flytta lager upp/ner i hierarkin, duplicera lager och slå samman lager nedåt i underliggande spår.
  - Ta bort lager med en säkerhetsbekräftelsedialog.
- **Bildrutor och klipp**:
  - Hållmultiplikator (Hold): tilldela anpassade tidslängder per bildruta (t.ex. `2×`, `3×` för pauser eller effektbildrutor).
  - Länkade bildrutor: länka repetitiva bildrutor genom animationen så att en ändring i en automatiskt uppdaterar alla länkade instanser.
  - Tagglooping: organisera animationer i namngivna taggar (`idle`, `walk`, `attack`) och loopa specifika taggar vid uppspelning.
  - Uppspelningskontroller: Framåt, Bakåt, Ping-pong och Loop med justerbar bildfrekvens (`1..=120` FPS).

### 3. Förhandsvisning i realtid (Live Preview)

Den högra dockningspanelen för förhandsvisning spelar upp dina animationer i realtid, helt oberoende av dukens zoomnivå och panoreringsposition:
- **Taggfiltrering**: Isolera uppspelningen till specifika taggar (`walk`, `jump`) eller förhandsgranska hela animationssekvensen.
- **Uppspelningskontroller**: Spela, Pausa, Föregående bildruta, Nästa bildruta, Loop-växling och anpassade uppspelningshastigheter (`0.25x..=4.0x`).
- **Visningslägen**: Växla mellan mörkt schackmönster, ljust schackmönster, helvit, helsvart eller anpassade bakgrundsfärger.

### 4. Avancerad lökbladsmotor (Onion Skinning)

Skapa mjuka och följsamma animationer med det dedikerade lökbladssystemet:
- **Anpassad färgtoning**: Oberoende färgtoner för föregående bildrutor (standard: röd) och nästa bildrutor (standard: blå).
- **Opacitetsavtagande**: Konfigurerbar grundopacitet och stegvis dämpningskurva för intilliggande faser.
- **Loop-avrundning**: Aktivera lökbladsvisning mellan första och sista bildrutan i cykliska animationer.
- **Snabbtitt (Quick-Peek)**: Håll ned <kbd>Mellanslag</kbd> utan att dra med musen för att omedelbart förhandsgranska intilliggande bildrutor.

### 5. Smart referens- och skisskalkeringslager

Snabbspara ditt arbetsflöde genom att kalkera handritade skisser eller referensfotografier:
- **Format**: Läs in PNG-, JPEG- och WebP-referensbilder.
- **Blandningslägen**: Underlägg (under dina pixellager) eller Överlägg (ovanpå dina pixellager).
- **Transformeringskontroller**: Oberoende zoom, positionsförskjutning, rotation och opacitetsreglage.
- **Smart färgprovtagning**: Använd pipetten på skisslagret med valfri automatisk fästning till närmaste färg i din aktiva projektpalett.
- **1-klicks-bakning (Bake)**: Platta ut den skalade referensbilden direkt till pixelgrafik på den aktiva bildrutan.

### 6. Precisionsmarkeringssvit och vandrande myror (Marching Ants)

- **Animerade vandrande myror**: Aktiva markeringar visar en animerad tvåfärgad streckad omkrets med hög kontrast.
- **Heltäckande lasso med flödesfyllning**: Till skillnad från enkla strålningsalgoritmer som skapar oönskade hål vid överlappande öglor, rasteriserar OniAnvils lasso en sluten 8-kopplad kontur och fyller utsidan, vilket garanterar 100 % heltäckande fyllning utan interna hål.
- **Markeringsoperationer**: Utöka, Minska, Ram (med cirkulär, fyrkantig och rombisk kärnform), Förflytta, Invertera, Ta bort, Fri rotation och Urklippskopiering (Kopiera/Klipp ut/Klistra in).
- **Kopiera och klistra in mellan bildrutor**: Markera områden på en bildruta, tryck på <kbd>Ctrl/Cmd+C</kbd>, byt bildruta och tryck på <kbd>Ctrl/Cmd+V</kbd> för att klistra in på exakt samma relativa koordinater oavsett tangentbordslayout.

### 7. Live-effekter, filter och RotSprite

Alla bildeffekter har en kryssruta för **Direkt förhandsvisning (Live Preview)** som visar ändringar direkt på duken och möjliggör avbrytning utan att skräpa ner ångringshistoriken:
- **Ersätt färg (`Shift+R`)**:
  - Interaktiv pipett: klicka på pipettikonen och hämta färger direkt från dukens pixlar eller palettpanelen.
  - Snabbvalsremsa: vänsterklicka för "Från"-färg, högerklicka för "Till"-färg.
  - Toleransreglage och massersättning över alla animationsbildrutor.
- **Färgjustering (`Ctrl/Cmd+U`)**: Justering av HSV-nyans, mättnad, ljusstyrka och kontrast i realtid.
- **Automatisk konturgenerator**: Skapa skarpa pixelkonturer runt karaktärer (Utsida, Insida, Centrerad) med fyrkantiga eller rombiska hörnfogar.
- **3×3-faltningsfilter**: Oskärpa, Skärpa, Kantidentifiering och Relief (Emboss) med bevarande av alfakanalen.
- **Fri rotation (`Ctrl/Cmd+Shift+R`)**: Rotera markerade områden eller hela bildrutan med valfri vinkel med den klassiska **RotSprite**-algoritmen eller Nearest Neighbor.
- **Mappa till palett**: Dithera dukens färger till aktiv palett med Bayer 2x2, 4x4, 8x8 eller Blue Noise.

### 8. Paletthantering, färgbyten och karaktärsskal (Skins)

- **Inbyggda paletter**: Över 20 autentiska retropaletter inklusive Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized och Sweetie 16.
- **Import och export**: Stöd för GIMP Palette (`.gpl`), PAL (`.pal`), Hex-paletter (`.hex`) och bildremsor (`.png`).
- **Palettskal (Skins)**: Definiera flera palettvarianter för en enskild karaktär (t.ex. Standard, Guldrostning, Skugga, Is). Att byta skal mappar omedelbart om alla färger över hela atlasen.
- **Färgövergångsgenerator (Ramp Generator)**: Interpolera automatiskt mjuka färggradienter mellan två valda nyanser med ett justerbart antal mellansteg.

### 9. Spritearksdelare, 9-Grid UI och export till spelmotorer

- **Spritearksdelare (Sheet Slicer)**: Autokorrelationsbaserad avgränsningsidentifiering delar automatiskt upp råa spriteark i jämna eller tätt avgränsade animationsbildrutor.
- **9-Grid (9-Slice) skalning**: Ställ in anpassningsbara marginaler (topp, botten, vänster, höger) för att förhandsgranska skalbara användargränssnittselement och knappaneler.
- **Extruderad PNG-kantexport**: Extruderar kantpixlars färger in i genomskinliga marginaler, vilket eliminerar texturfiltreringsskarvar och färgblödning i OpenGL, DirectX, Vulkan och 2D-motorer (såsom Godot).
- **Animerad GIF-export**: Skapa optimerade animerade GIF-filer med anpassat loopantal, bildrutedurationer och genomskinlig bakgrund.
- **Standardiserad Sprite Sheet JSON**: Exporterar bildruteutskärningar, taggar, varaktigheter och fästpunkter som matchar standardformatet för spriteark-JSON i moderna 2D-spelmotorer.

### 10. 35 ergonomiska teman och fullständig lokalisering

- **Utvalda teman**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT Amber, CRT Green, CRT Cyan, Retrokonsoler (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) och Retrodatorer (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Flerspråksstöd**: Växla sömlöst mellan språken i gränssnittet med 100 % paritet över alla gränssnittssträngar och verktygstips.

---

## Kontroller och kortkommandon

| Kortkommando | Verktyg / Åtgärd | Beskrivning |
|:---|:---|:---|
| <kbd>B</kbd> | **Penna** | 1px precisionsritning med valfritt Pixel-Perfect-läge |
| <kbd>E</kbd> | **Suddgummi** | Radera pixlar tillbaka till genomskinlighet |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipett** | Hämta färg från dukens pixlar eller referensskissen |
| <kbd>G</kbd> | **Färghink / Toning** | Växla mellan färghink och dithering-toning |
| <kbd>L</kbd> | **Linje** | Rita raka linjer med direkt förhandsvisning och Shift-vinkellås |
| <kbd>C</kbd> | **Kurva** | Kvadratisk Bézier-pixelkurva med 3 klick |
| <kbd>U</kbd> | **Rektangel** | Geometrisk rektangel och kvadrat med kontur eller fyllning |
| <kbd>O</kbd> | **Ellips** | Geometrisk ellips och cirkel med kontur eller fyllning |
| <kbd>D</kbd> | **Dithering-pensel** | Måla schackmönstrade matriser |
| <kbd>S</kbd> | **Skuggningsverktyg** | Stegvis ljusning (Dodge) och mörkning (Burn) längs palettrampen |
| <kbd>M</kbd> | **Rektangulär markering** | Rektangulär områdesmarkering med vandrande myror |
| <kbd>Q</kbd> | **Lassomarkering** | Heltäckande frihandspolygonmarkering |
| <kbd>W</kbd> | **Trollspö** | Markera sammanhängande färgregioner |
| <kbd>P</kbd> | **Fästpunkter** | Placera spritens origo och speldynamiska åtgärdspunkter |
| <kbd>X</kbd> | **Växla färger** | Byt plats på primär och sekundär färg |
| <kbd>[</kbd> / <kbd>]</kbd> | **Penselstorlek** | Minska / öka penselstorlek (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopiera** | Kopiera markerade pixlar till systemets urklipp |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Klipp ut** | Klipp ut markerade pixlar till urklipp |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Klistra in** | Klistra in pixlar från urklipp i aktiv bildruta |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Ta bort** | Rensa pixlar inom aktiv markering |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Markera allt** | Markera alla pixlar i den aktiva bildrutan |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Avmarkera** | Rensa aktiv markeringsmask |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invertera markering** | Invertera aktiv markeringsmask |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplicera bildruta** | Skapa en kopia av aktuell bildruta i tidslinjen |
| <kbd>Mellanslag</kbd> | **Spela / Pausa** | Starta eller pausa animationsuppspelning i tidslinjen |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Ersätt färg** | Öppna dialogrutan för färgersättning med förhandsvisning |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Färgjustering** | Öppna dialogrutan för HSV-nyans, mättnad och kontrast |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Fri rotation** | Öppna dialogrutan för fri vinkelrotation (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Ångra** | Ångra senaste penseldrag eller verktygsåtgärd |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Gör om** | Återställ tidigare ångrad åtgärd |
| <kbd>Mellanslag</kbd> + Drag / Musknapp 3 | **Panorera duk** | Mjuk och oändlig panorering över dukens koordinater |
| <kbd>Ctrl/Cmd</kbd> + Hjul / <kbd>+</kbd> / <kbd>-</kbd> | **Zooma** | Pixelskarp zoomning centrerad runt muspekaren |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Visningsläge** | Växla mellan bildruteläge (<kbd>1</kbd>) och atlasläge (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Anpassa till vy** | Centrera och anpassa aktiv bildruta till arbetsytan |
| <kbd>F1</kbd> | **Hjälp** | Öppna hjälpfönstret för kortkommandon |

---

## Kom igång

### Fristående binärer

Ladda ner färdigkompilerade, fristående körbara filer från **[GitHub Releases-sidan](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Ladda ner `OniAnvil-windows-x86_64.zip`, packa upp och dubbelklicka på `OniAnvil.exe`.
- **Linux**: Ladda ner `OniAnvil-linux-x86_64.tar.gz` (portabel) eller installera paketet `onianvil_*_amd64.deb` för Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Ladda ner `OniAnvil-macos-universal.zip`, packa upp och dra `OniAnvil.app` till mappen Program. Den universella binären körs med inbyggd prestanda på både Apple Silicon (M1/M2/M3/M4) och Intel-Macar.
- **Webb**: Kör direkt i webbläsaren utan installation: **[Starta WebAssembly-appen](https://rbm13s.github.io/OniAnvil/app/?lang=sv)**.
  > [!WARNING]
  > WebAssembly-versionen (webbläsaren) har statusen **Experimentell**. På grund av webbläsarens säkerhetssandlåda, WebGL/WebGPU-skillnader och begränsningar i urklippshantering kan mindre grafiska artefakter eller prestandaproblem uppstå jämfört med skrivbordsversionen. För produktionsarbete rekommenderas de inbyggda skrivbordsversionerna starkt.

---

### Bygga från källkod

#### Förutsättningar
- **Rust-verktygskedja**: Installera Rust (version 1.80+ rekommenderas) via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Byggberoenden för Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompilera och kör den inbyggda skrivbordsversionen
```bash
# Klona databasen
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Kör testsviten
cargo test

# Bygg och kör den optimerade release-binären
cargo run --release
```

#### 2. Kompilera WebAssembly-versionen (via Trunk)
```bash
# Installera kompileringsmålet wasm32 och byggverktyget Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Kör lokal utvecklingsserver med automatisk omladdning
trunk serve

# Bygg optimerade distributionsfiler till docs/app/
./build_wasm.sh        # På Linux/macOS
./build_wasm.ps1       # På Windows PowerShell
```

---

## Filformat och följeslagararkitektur

OniAnvil undviker proprietära binärformat. Varje projekt består av två rena filer under versionshantering:

```
my_character/
├── my_character.png            # Standardiserad PNG-atlas (kan öppnas i alla motorer och bildvisare)
└── my_character.onianvil.json   # Lättviktig JSON-metadatafil
```

Filen `.onianvil.json` lagrar:
- Rutnätskonfiguration för duken: bildrutedimensioner, kolumnantal, totalt antal bildrutor.
- Lagerhierarkier: spårnamn, opacitetsnivåer, synlighetsflaggor, låsningsstatus.
- Animationstaggar: namn, start- och slutbildruteindex, uppspelningsriktning (Framåt, Bakåt, Ping-pong), bildrutedurationer.
- Palettdefinitioner: aktiva färgrutor, namngivna skalvarianter, index för aktivt skal.
- Referensskissläge: bildsökväg, skalning, panoreringsförskjutning, rotation, opacitet, lagringsläge.
- Fästpunkter: anpassade åtgärdspunkter, vapenfästen och kollisionspunkter.

---

## Bidra och tack

Bidrag, felrapporter och funktionsförslag är varmt välkomna!
1. Förgrena (fork) databasen på GitHub.
2. Skapa en funktionsgren: `git checkout -b feature/min-nya-funktion`.
3. Checka in dina ändringar: `git commit -am 'Lägg till nytt pixelverktyg'`.
4. Skicka upp till grenen: `git push origin feature/min-nya-funktion`.
5. Skapa en Pull Request.

### Tack till
- Startdemospriten **"Cat 2D Pixel Art"** skapad av **xzany** ([projektsida på itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Byggd med det fantastiska GUI-biblioteket **[egui](https://github.com/emilk/egui)** och grafiksystemet **[wgpu](https://wgpu.rs/)**.
- Gränssnittsikoner från **[Font Awesome Free](https://fontawesome.com)** av @fontawesome (Ikoner: CC BY 4.0, Typsnitt: SIL OFL 1.1, Kod: MIT-licens).

---

## Licens och ansvarsfriskrivning

Detta projekt är licensierat under **[MIT-licensen](../LICENSE)**. Du har full frihet att använda, modifiera, distribuera och integrera OniAnvil i såväl kommersiella som privata spelutvecklingsprojekt.

### Friskrivning från garantier och ansvarsbegränsning
> [!IMPORTANT]
> **OniAnvil** distribueras **kostnadsfritt** i befintligt skick **("AS IS")** och **"I MÅN AV TILLGÅNG"**, utan några uttryckliga eller underförstådda garantier eller villkor av något slag.
> 
> Inte under några omständigheter ska upphovsrättsinnehavaren/författarna eller bidragsgivarna hållas ansvariga för några direkta, indirekta, tillfälliga, särskilda, exemplariska eller följdskador (inklusive, men inte begränsat till, förlust av data, filkorruption, hårdvarufel, utebliven vinst eller verksamhetsavbrott) som på något sätt orsakats av användning av eller oförmåga att använda denna programvara, även om underrättelse om risken för sådan skada har lämnats. Du påtar dig allt ansvar och all risk för användningen av detta program.

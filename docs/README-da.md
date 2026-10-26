<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixelgrafik- og løgskindanimationsstudie (Onion-Skinning) bygget til spilklare teksturatlasser**

[![Byg og udgivelse](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Sprog](https://img.shields.io/badge/Sprog-37_sprog-purple)](#10-35-ergonomiske-temaer-og-fuldstændig-lokalisering)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#kom-godt-i-gang)
[![Webdemo](https://img.shields.io/badge/Online-Prøv_i_browseren_(Eksperimentel)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=da)
[![Licens: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Prøv OniAnvil i browseren (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=da)** *(⚠️ Eksperimentel version: kan indeholde WebGL/WASM-begrænsninger og fejl)* • **[🌐 Officiel dokumentationshjemmeside](https://rbm13s.github.io/OniAnvil/da.html)**

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
**Dansk** •
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

## Indholdsfortegnelse

- [Introduktion](#introduktion)
- [Skærmbilleder og visuel rundvisning](#skærmbilleder-og-visuel-rundvisning)
- [Omfattende værktøjsreference](#omfattende-værktøjsreference)
- [Funktionsdokumentation](#funktionsdokumentation)
  - [1. Dobbelt atlas-centreret arkitektur](#1-dobbelt-atlas-centreret-arkitektur)
  - [2. Multispors lagtidslinje](#2-multispors-lagtidslinje)
  - [3. Live forhåndsvisning i realtid](#3-live-forhåndsvisning-i-realtid)
  - [4. Avanceret løgskindsmotor (Onion Skinning)](#4-avanceret-løgskindsmotor-onion-skinning)
  - [5. Smart reference- og skitsetegningslag](#5-smart-reference--og-skitsetegningslag)
  - [6. Præcisionsmarkeringsværktøjer og marcherende myrer (Marching Ants)](#6-præcisionsmarkeringsværktøjer-og-marcherende-myrer-marching-ants)
  - [7. Live-effekter, filtre og RotSprite](#7-live-effekter-filtre-og-rotsprite)
  - [8. Paletstyring, farveudskiftning og karakterskins](#8-paletstyring-farveudskiftning-og-karakterskins)
  - [9. Spritearkopdeler, 9-Grid UI og eksport til spilmotorer](#9-spritearkopdeler-9-grid-ui-og-eksport-til-spilmotorer)
  - [10. 35 ergonomiske temaer og fuldstændig lokalisering](#10-35-ergonomiske-temaer-og-fuldstændig-lokalisering)
- [Tastaturgenveje og styring](#tastaturgenveje-og-styring)
- [Kom godt i gang](#kom-godt-i-gang)
  - [Fritstående binære filer](#fritstående-binære-filer)
  - [Kompilering fra kildekode](#kompilering-fra-kildekode)
- [Filformat og ledsagerarkitektur](#filformat-og-ledsagerarkitektur)
- [Bidrag og tak](#bidrag-og-tak)
- [Licens og ansvarsfraskrivelse](#licens-og-ansvarsfraskrivelse)

---

## Introduktion

**OniAnvil** (*fra **Oni**on-Skinning [løgskindsteknik] + **Anvil** [ambolt til smedning af sprites]*) er et open source, højtydende pixelgrafik- og animationsstudie skrevet i Rust. Det kører integreret med hardwareaccelereret rendering på Windows, Linux og macOS samt som en installationsfri WebAssembly-applikation direkte i moderne browsere.

De fleste eksisterende sprite-editorer behandler animationer som en stak adskilte billedfiler, hvilket tvinger udviklere ud i tidskrævende processer med eksport, opdeling og pakning. **OniAnvil vender denne tankegang på hovedet:** spillets teksturatlas (`atlas_rgba`) er den primære kilde til sandhed. Billedrammer, lagspor, animationstags og kollisionspunkter holdes konstant synkroniseret med atlasset på en ikke-destruktiv måde.

### Hvorfor OniAnvil?
- 🚀 **Lynhurtig ydelse i ren Rust**: Øjeblikkelig opstart på under ét millisekund, knivskarp grafik drevet af `wgpu` og `egui 0.31`, ingen forsinkelse ved navigering i lange tidslinjer.
- 🎬 **Multispors lagtidslinje**: NLE-inspirerede lagspor (`V1, V2, V3...`) med lagkomposition oppefra og ned, lagfletning, kædede billedrammer og variable varigheder.
- 🧅 **Førsteklasses løgskind (Onion Skinning)**: Gennemsigtighed over flere billedrammer med uafhængig farvetoning fremad og bagud, tilpasningsdygtige opacitetskurver og ubrudt loop-understøttelse.
- 🎨 **Øjeblikkelige karakterskins**: Opret dag-/natvarianter, holdfarver eller elementskins med direkte atlas-remappning samt import og eksport af GPL-, PAL- og HEX-filer.
- 📦 **Direkte eksport til spilmotorer**: Eksporter skalerede PNG-filer med ekstruderede kanter for at eliminere teksturblødning i 2D-motorer (Godot, Bevy, Raylib, Defold), sammen med standardiseret Sprite Sheet JSON-metadata og animerede GIF-filer.

---

## Skærmbilleder og visuel rundvisning

<div align="center">

### 1. Hovedlærred og studiearbejdsområde
*Højtydende pixelarbejdsområde med hardwareaccelereret visning, multispors tidslinje, live forhåndsvisning i realtid, paletfelter og præcisionsværktøjslinjer.*
<img src="screenshots/main.png" alt="OniAnvil Hovedarbejdsområde" width="950" />

<br/><br/>

### 2. Multispors animationstidslinje
*Ikke-lineær multispors tidslinje med stablede lagrækker (`V1, V2...`), kædede billedfeltforekomster (`🔗`), hold-multiplikatorer og navngivne farvekodede animationstags.*
<img src="screenshots/timeline.png" alt="OniAnvil Multispors tidslinje" width="950" />

<br/><br/>

### 3. Præcisionsværktøjer og hurtighandlingslinje
*Komplet tegnesuite med 15 værktøjer i standard- og kompaktdocking-tilstand samt øverste værktøjslinje til penselstørrelse, symmetri, kantombrydning og Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Handlingslinje" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Værktøjer standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Værktøjer kompakt" width="300" />
</p>

<br/><br/>

### 4. Live animationsforhåndsvisning i realtid (Live Preview)
*Dedikeret afspilningspanel med tag-filtrering, variable FPS-hastighedskontroller, zoomskyder og baggrundsskift (skaktern, mørk ensfarvet, lys ensfarvet).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Forhåndsvisning" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Forhåndsvisning zoom" width="460" />
</p>

<br/><br/>

### 5. Paletstyring, farveudskiftning og farveramper
*Omfattende farvemotor med felter til forgrunds-/baggrundsfarve, automatisk generering af topunkts farveramper og navngivne paletskins.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palet og skins" width="950" />

<br/><br/>

### 6. Skitsesporing og konfigurerbart løgskind (Onion Skinning)
*Indlæs konceptskitser eller fotografier som underlag/overlag med transformeringskontroller, smart farveopsamling med palettilpasning og flerrammers løgskindskygger.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Skitse og løgskind" width="950" />

<br/><br/>

### 7. Drejepunkter (Pivots) og gameplay-handlingspunkter (Action Points)
*Definer spritens centrum/origo og brugerdefinerede gameplay-punkter (våbenfæster, projektilpunkter, hoved, fodtrin) med interaktive lærredsmarkører.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Drejepunkter og handlingspunkter" width="950" />

<br/><br/>

### 8. Spritearkopdeler (Sheet Slicer) og 9-Grid (9-Slice) skalering
*Automatisk grænsegenkendelse via autokorrelation (f.eks. kattens gangcyklus) samt skalerbar 9-slice UI-kassemarginkonfiguration.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Spritearkopdeler" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Slice skalering" width="460" />
</p>

<br/><br/>

### 9. Live billed-FX og filterdialoger (med forhåndsvisning i realtid)
*Alle effektdialoger har direkte lærredsforhåndsvisning, interaktive pipetter og ikke-destruktiv annullering.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Konturgenerator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Erstat farve" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Farvetone Mætning Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrixfiltre" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Knyt til palet" width="460" />
</p>

<br/><br/>

### 10. Lærredsgitter og baggrundstilstande
*Skift problemfrit mellem pixelgitter, flisegitter, mørkt skaktern, lyst skaktern og brugerdefineret ensfarvet baggrund.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Gitter og mørkt skaktern" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Lyst skaktern uden gitter" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Brugerdefineret ensfarvet baggrund" width="300" />
</p>

<br/><br/>

### 11. Guide til oprettelse af nyt projekt
*Konfigurer lærredsdimensioner, billeder, kolonnegitter og vælg mellem over 20 autentiske retropaletter ved opstart.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Guide til nyt projekt" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletter til nyt projekt" width="460" />
</p>

<br/><br/>

### 12. 35 autentiske retro- og moderne temaer
*Indeholder Studio Dark, Studio Light, CRT Amber/Green/Cyan-fosfor, retrokonsoller (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) og designertemaer.*
<p align="center">
  <img src="screenshots/themes.png" alt="Temamenu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber-tema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX-tema" width="300" />
</p>

<br/><br/>

### 13. Menusystem og indbygget genvejshjælp
<p align="center">
  <img src="screenshots/file_menu.png" alt="Filmenu" width="180" />
  <img src="screenshots/edit_menu.png" alt="Redigeringsmenu" width="180" />
  <img src="screenshots/select_menu.png" alt="Markeringsmenu" width="180" />
  <img src="screenshots/view_menu.png" alt="Visningsmenu" width="180" />
  <img src="screenshots/project_menu.png" alt="Projektmenu" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Hjælpevindue til tastaturgenveje" width="700" />
</p>

</div>

---

## Omfattende værktøjsreference

OniAnvil indeholder 15 præcisionsværktøjer til tegning, geometri, markering og metadata. Ikonerne vises ved hjælp af open source-pakken **Font Awesome Free** *(Ikoner: CC BY 4.0, Skrifttyper: SIL OFL 1.1)*.

| Værktøj | Ikon | Genvej | Penselstørrelse | Pixel Perfect | Beskrivelse |
|:---|:---:|:---:|:---:|:---:|:---|
| **Blyant** | ✏️ | <kbd>B</kbd> | 1..32 px | Ja | Primært tegneværktøj. 1px-tilstand understøtter **Pixel-Perfect**-tegning (fjerner automatisk grimme trappetrins-L-hjørner). Understøtter brugerdefinerede stempelpensler fra markeringer (`Ctrl/Cmd+B`). |
| **Viskelæder** | 🧹 | <kbd>E</kbd> | 1..32 px | Nej | Sletter pixels tilbage til fuld gennemsigtighed (`[0, 0, 0, 0]`). Begrænses af markeringsmasken, hvis den er aktiv. |
| **Pipette** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nej | Venstreklik optager farve til Forgrund (Primær); Højreklik optager til Baggrund (Sekundær). Kan hente farve fra lærredets pixels, lag eller referenceskitse. |
| **Malingbøtte** | 🪣 | <kbd>G</kbd> | Ikke relevant | Nej | Hurtig kø-baseret BFS-udfyldning med justerbar farvetolerance (`0..=255`). Understøtter sammenhængende vs. global udfyldning samt markeringsbegrænsning. |
| **Farveovergang** | 🌈 | <kbd>G</kbd> *(skift)* | Ikke relevant | Nej | Lineære og radiale overgange mellem forgrunds- og baggrundsfarver. Understøtter Bayer- (2x2, 4x4, 8x8) og Blue Noise-dithering med styrkekontrol. |
| **Linje** | 📏 | <kbd>L</kbd> | 1..32 px | Ja | Bresenham-linjealgoritme med live forhåndsvisning under træk. Hold <kbd>Shift</kbd> nede for at låse vinkler til trin på 45°/90°. |
| **Kurve** | 〰️ | <kbd>C</kbd> | 1..32 px | Ja | Kvadratisk Bézier-pixelkurve med 3 klik. Klik 1 angiver startpunkt, Klik 2 angiver slutpunkt, musebevægelse justerer krumning, Klik 3 fastlægger kurven. |
| **Rektangel** | ⬜ | <kbd>U</kbd> | 1..32 px | Nej | Geometriske rektangler med omrids eller fyld. Hold <kbd>Shift</kbd> for at fastholde et 1:1 kvadrat. |
| **Ellipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Nej | Ellipser og cirkler med omrids eller fyld via midtpunktsrasterisering. Hold <kbd>Shift</kbd> for at gennemtvinge en perfekt cirkel. |
| **Dithering-pensel** | 🏁 | <kbd>D</kbd> | 1..32 px | Nej | Maler alternerende skakbrætmønstret dithering mellem forgrunds- og baggrundsfarver uden at introducere ekstra paletfarver. |
| **Skyggeværktøj** | 🌓 | <kbd>S</kbd> | 1..32 px | Nej | Trinvis lysning (Dodge) og mørkning (Burn) baseret på farverækkefølgen i den aktive palet. |
| **Rektangulær markering** | ⬚ | <kbd>M</kbd> | Ikke relevant | Nej | Rektangulær markeringsramme med animeret kantlinje af marcherende myrer. Understøtter tilføjelse (<kbd>Shift</kbd>) og fratrækning (<kbd>Alt</kbd>). |
| **Lassomarkering** | ➰ | <kbd>Q</kbd> | Ikke relevant | Nej | Frihåndspolygonlasso. Anvender 8-forbundet kantrasterisering med 4-forbundet ydre flodfyldning, hvilket garanterer 100 % massive markeringer uden huller selv ved overlappende løkker. |
| **Tryllestav** | 🪄 | <kbd>W</kbd> | Ikke relevant | Nej | Vælger sammenhængende (eller globale) områder af matchende farve med justerbar tolerancetærskel. |
| **Drejepunkt & handlingspunkt** | 🎯 | <kbd>P</kbd> | Ikke relevant | Nej | Interaktiv placering på lærredet af spritens origo (Pivot) og brugerdefinerede gameplay-handlingspunkter (`weapon`, `head`, `feet`, `hitbox`), der eksporteres til JSON-metadata. |

---

## Funktionsdokumentation

### 1. Dobbelt atlas-centreret arkitektur

I OniAnvil udgør hovedteksturen (`atlas_rgba`) den faktiske spritearktekstur, som den indlæses af din spilmotor:
- **Billedrammetilstand (Frame Mode)**: Fokuserer på en enkelt aktiv animationsramme, hvilket giver et distraktionsfrit pixelarbejdsområde. Redigeringer opdaterer øjeblikkeligt den tilhørende celle i atlasstrukturen.
- **Atlastilstand (Atlas Mode)**: Viser hele spritearkets gitter for overblik, justering på tværs af billedrammer og batch-redigering.
- **Ledsagerfil (Companion File)**: Projekter gemmes som rene `.png`-spriteark ledsaget af en let `.onianvil.json`-metadatafil, der indeholder rammeopsætninger, laghierarkier, tags, varigheder og paletskins.

### 2. Multispors lagtidslinje

Den nederste tidslinje er opbygget efter principper fra professionel videoredigeringssoftware:
- **Lagspor (`V1, V2, V3...`)**: Lagene stables visuelt oppefra og ned. Det spor, der ligger øverst i tidslinjen (`V1`), tegnes øverst på lærredet.
- **Lagstyring**:
  - Klik på `V1` / `V2`-mærkerne for hurtigt at skifte aktivt tegnelag.
  - Slå lagsynlighed (øjeikon) og redigeringslås (hængelåsikon) til og fra.
  - Juster individuel lagopacitet (`0%..=100%`).
  - Flyt lag op/ned i hierarkiet, dupliker lag og flet lag nedad i underliggende spor.
  - Slet lag med en sikkerhedsbekræftelsesdialog.
- **Billedrammer og klip**:
  - Hold-multiplikator: tildel tilpassede varigheder pr. ramme (f.eks. `2×`, `3×` til pauser eller effektrammer).
  - Kædede rammer: kæd gentagne rammer sammen gennem animationen, så en ændring i én automatisk opdaterer alle forbundne forekomster.
  - Tag-looping: organiser animationer i navngivne tags (`idle`, `walk`, `attack`) og loop specifikke tags under afspilning.
  - Afspilningsknapper: Fremad, Baglæns, Ping-pong og Loop med justerbar billedfrekvens (`1..=120` FPS).

### 3. Live forhåndsvisning i realtid

Det højre forhåndsvisningspanel afspiller dine animationer i realtid, fuldstændig uafhængigt af lærredets zoomniveau og panoreringsposition:
- **Tag-filtrering**: Isoler afspilningen til bestemte tags (`walk`, `jump`) eller forhåndsvis hele animationssekvensen.
- **Afspilningsstyring**: Afspil, Pause, Forrige ramme, Næste ramme, Loop-skift og tilpassede hastigheder (`0.25x..=4.0x`).
- **Visningstilstande**: Skift mellem mørkt skaktern, lyst skaktern, ren hvid, ren sort eller tilpassede baggrundsfarver.

### 4. Avanceret løgskindsmotor (Onion Skinning)

Skab flydende og levende animationer med det dedikerede løgskindssystem:
- **Tilpasset farvetoning**: Uafhængige farvetoner for forrige rammer (standard: rød) og næste rammer (standard: blå).
- **Opacitetsaftagning**: Justerbar basisopacitet og trinvis dæmpningskurve for tilstødende faser.
- **Loop-understøttelse**: Aktiver løgskindsvisning på tværs af første og sidste ramme i cykliske animationer.
- **Hurtigt kig (Quick-Peek)**: Hold <kbd>Mellemrum</kbd> nede uden at trække med musen for øjeblikkeligt at inspicere tilstødende billedrammer.

### 5. Smart reference- og skitsetegningslag

Gør dit arbejdsforløb hurtigere ved at overføre håndtegnede skitser eller referencefotos:
- **Formater**: Indlæs PNG-, JPEG- og WebP-referencebilleder.
- **Blandingstilstande**: Underlag (under dine pixellag) eller Overlag (over dine pixellag).
- **Transformeringsstyring**: Uafhængig zoom, positionsforskydning, rotation og opacitetsskyder.
- **Smart farveopsamling**: Brug pipetten på skitselaget med valgfri automatisk tilpasning til den nærmeste farve i den aktive projektpalet.
- **1-klik bagning (Bake)**: Fladgør det skalerede referencebillede direkte til pixelgrafik på den aktive ramme med et enkelt klik.

### 6. Præcisionsmarkeringsværktøjer og marcherende myrer (Marching Ants)

- **Animerede marcherende myrer**: Aktive markeringer vises med en animeret tofarvet stiplet omkreds med høj kontrast.
- **Massiv lasso med flodfyldning**: I modsætning til simple stråleafsøgningsalgoritmer, der skaber uønskede huller ved overlappende sløjfer, rasteriserer OniAnvils lasso en lukket 8-forbundet grænse og udfylder det ydre rum, hvilket sikrer 100 % massive markeringer uden indre huller.
- **Markeringshandlinger**: Udvid, Formindsk, Ramme (med cirkulær, kvadratisk og rombisk kerneform), Flyt, Inverter, Slet, Fri rotation samt Udklipsholder (Kopier/Klip/Sæt ind).
- **Kopier og indsæt på tværs af rammer**: Vælg områder på én ramme, tryk <kbd>Ctrl/Cmd+C</kbd>, skift ramme, og tryk <kbd>Ctrl/Cmd+V</kbd> for at indsætte på nøjagtig samme relative koordinater uanset tastaturlayout.

### 7. Live-effekter, filtre og RotSprite

Alle billedeffekter indeholder et afkrydsningsfelt til **Live forhåndsvisning (Live Preview)**, der viser ændringer direkte på lærredet og muliggør problemfri annullering uden at forurene fortrydelseshistorikken:
- **Erstat farve (`Shift+R`)**:
  - Interaktiv pipette: klik på pipetteikonet og vælg farver direkte fra lærredets pixels eller paletpanelet.
  - Hurtigvalgsstribe: venstreklik for "Fra"-farve, højreklik for "Til"-farve.
  - Toleranceskyder og batch-udskiftning på tværs af alle animationsrammer.
- **Farvejustering (`Ctrl/Cmd+U`)**: Justering af HSV-farvetone, mætning, lysstyrke og kontrast i realtid.
- **Automatisk konturgenerator**: Skab skarpe pixelkonturer om figurer (Udvendig, Indvendig, Centreret) med firkantede eller rombiske hjørnesamlinger.
- **3×3-konvolutionsfiltre**: Sløring, Skarphed, Kantgenkendelse og Prægning (Emboss) med bevarelse af alfakanalen.
- **Fri rotation (`Ctrl/Cmd+Shift+R`)**: Roter markerede områder eller hele rammer i vilkårlige vinkler ved hjælp af den klassiske **RotSprite**-algoritme eller Nearest Neighbor.
- **Knyt til palet**: Dither lærredets farver ind i den aktive palet med Bayer 2x2, 4x4, 8x8 eller Blue Noise.

### 8. Paletstyring, farveudskiftning og karakterskins

- **Indbyggede paletter**: Over 20 autentiske retropaletter, herunder Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized og Sweetie 16.
- **Import og eksport**: Understøttelse af GIMP Palette (`.gpl`), PAL (`.pal`), Hex-paletter (`.hex`) og billedstriber (`.png`).
- **Paletskins**: Definer flere farvevarianter for en enkelt figur (f.eks. Standard, Guldrustning, Skygge, Is). Ved at skifte skin opdateres alle farver øjeblikkeligt på tværs af hele atlasset.
- **Farveramper (Ramp Generator)**: Beregn automatisk glatte farveovergange mellem to valgte nuancer med et justerbart antal mellemtrin.

### 9. Spritearkopdeler, 9-Grid UI og eksport til spilmotorer

- **Spritearkopdeler (Sheet Slicer)**: Autokorrelationsbaseret grænsegenkendelse opdeler automatisk rå spriteark i ensartede eller tæt afgrænsede animationsrammer.
- **9-Grid (9-Slice) skalering**: Indstil tilpassede margener (top, bund, venstre, højre) til forhåndsvisning af skalerbare UI-bokse og knappaneler.
- **Ekstruderet PNG-kanteksport**: Ekstruderer kantpixels farver ind i gennemsigtige margener, hvilket forhindrer teksturfiltreringsstriber og blødning i OpenGL, DirectX, Vulkan og 2D-spilmotorer (såsom Godot).
- **Animeret GIF-eksport**: Generer optimerede animerede GIF-filer med brugerdefineret gentagelsesantal, rammevarigheder og gennemsigtig baggrund.
- **Standard Sprite Sheet JSON**: Eksporterer rammeudsnit, tags, varigheder og drejepunkter, der matcher standardformatet for spriteark-JSON i moderne 2D-spilmotorer.

### 10. 35 ergonomiske temaer og fuldstændig lokalisering

- **Udvalgte temaer**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT Amber, CRT Green, CRT Cyan, Retrokonsoller (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) og Retrocomputere (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Flersproget brugerflade**: Skift problemfrit mellem sprogene i brugerfladen med 100 % paritet over alle tekststrenge og værktøjstip.

---

## Tastaturgenveje og styring

| Genvej | Værktøj / Handling | Beskrivelse |
|:---|:---|:---|
| <kbd>B</kbd> | **Blyant** | 1px præcisionstegning med valgfri Pixel-Perfect-tilstand |
| <kbd>E</kbd> | **Viskelæder** | Slet pixels tilbage til gennemsigtighed |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipette** | Optag farve fra lærredets pixels eller referenceskitse |
| <kbd>G</kbd> | **Malingbøtte / Farveovergang** | Skift mellem malingbøtte og dithering-farveovergang |
| <kbd>L</kbd> | **Linje** | Tegn lige linjer med live forhåndsvisning og Shift-vinkellås |
| <kbd>C</kbd> | **Kurve** | Kvadratisk Bézier-pixelkurve med 3 klik |
| <kbd>U</kbd> | **Rektangel** | Geometrisk rektangel og kvadrat med omrids eller fyld |
| <kbd>O</kbd> | **Ellipse** | Geometrisk ellipse og cirkel med omrids eller fyld |
| <kbd>D</kbd> | **Dithering-pensel** | Mal med skakbrætmønstrede matricer |
| <kbd>S</kbd> | **Skyggeværktøj** | Trinvis lysning (Dodge) og mørkning (Burn) langs paletrampen |
| <kbd>M</kbd> | **Rektangulær markering** | Rektangulær områdemarkering med marcherende myrer |
| <kbd>Q</kbd> | **Lassomarkering** | Frihånds massiv polygonmarkering |
| <kbd>W</kbd> | **Tryllestav** | Vælg sammenhængende farveregioner |
| <kbd>P</kbd> | **Drejepunkter** | Placer spritens origo og gameplay-handlingspunkter |
| <kbd>X</kbd> | **Byt farver** | Byt om på primær og sekundær farve |
| <kbd>[</kbd> / <kbd>]</kbd> | **Penselstørrelse** | Formindsk / forøg penselstørrelse (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopier** | Kopier markerede pixels til systemets udklipsholder |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Klip** | Klip markerede pixels til udklipsholderen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Sæt ind** | Sæt pixels fra udklipsholderen ind i aktiv ramme |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Slet** | Ryd pixels inden for den aktive markering |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Vælg alt** | Vælg alle pixels i den aktive ramme |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Fravælg** | Ryd aktiv markeringsmaske |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Inverter markering** | Inverter aktiv markeringsmaske |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dupliker ramme** | Opret en kopi af aktuel ramme i tidslinjen |
| <kbd>Mellemrum</kbd> | **Afspil / Pause** | Start eller sæt animationsafspilning på pause |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Erstat farve** | Åbn dialogboksen til farveudskiftning med forhåndsvisning |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Farvejustering** | Åbn dialogboksen til justering af HSV og kontrast |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Fri rotation** | Åbn dialogboksen til fri vinkelrotation (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Fortryd** | Fortryd seneste penselstrøg eller værktøjshandling |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Gentag** | Gentag tidligere fortrudt handling |
| <kbd>Mellemrum</kbd> + Træk / MMB | **Panorer lærred** | Jævn og uendelig panorering over lærredets koordinater |
| <kbd>Ctrl/Cmd</kbd> + Hjul / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Pixelskarp zoom centreret omkring musemarkøren |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Visningstilstand** | Skift mellem rammetilstand (<kbd>1</kbd>) og atlastilstand (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Tilpas lærred** | Centrer og tilpas aktiv ramme til arbejdsområdet |
| <kbd>F1</kbd> | **Hjælp** | Åbn hjælpevinduet for tastaturgenveje |

---

## Kom godt i gang

### Fritstående binære filer

Download færdigkompilerede, fritstående eksekverbare filer fra **[GitHub Releases-siden](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Download `OniAnvil-windows-x86_64.zip`, pak ud, og dobbeltklik på `OniAnvil.exe`.
- **Linux**: Download `OniAnvil-linux-x86_64.tar.gz` (portabel) eller installer pakken `onianvil_*_amd64.deb` til Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Download `OniAnvil-macos-universal.zip`, pak ud, og træk `OniAnvil.app` til mappen Programmer. Den universelle binære fil kører med direkte native ydelse på både Apple Silicon (M1/M2/M3/M4) og Intel-Macs.
- **Web**: Kør direkte i browseren uden installation: **[Start WebAssembly-appen](https://rbm13s.github.io/OniAnvil/app/?lang=da)**.
  > [!WARNING]
  > WebAssembly-versionen (browser) har status som **Eksperimentel**. Grundet browserens sandkassebegrænsninger, WebGL/WebGPU-forskelle og begrænsninger i udklipsholderen kan der opstå mindre grafiske fejl eller ydeevneforskelle sammenlignet med skrivebordsversionen. Til produktionsarbejde anbefales de native desktop-versioner kraftigt.

---

### Kompilering fra kildekode

#### Forudsætninger
- **Rust-værktøjskæde**: Installer Rust (version 1.80+ anbefales) via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux-byggeafhængigheder** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompiler og kør den native skrivebordsversion
```bash
# Klon arkivet
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Kør testpakken
cargo test

# Byg og kør den optimerede release-binære fil
cargo run --release
```

#### 2. Kompiler WebAssembly-versionen (via Trunk)
```bash
# Installer kompileringsmålet wasm32 og byggeværktøjet Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Kør lokal udviklingsserver med hot-reload
trunk serve

# Byg optimerede udgivelsesfiler til docs/app/
./build_wasm.sh        # På Linux/macOS
./build_wasm.ps1       # På Windows PowerShell
```

---

## Filformat og ledsagerarkitektur

OniAnvil undgår proprietære binære formater. Hvert projekt består af to rene filer under versionsstyring:

```
my_character/
├── my_character.png            # Standard PNG-atlas (kan åbnes i enhver motor eller billedfremviser)
└── my_character.onianvil.json   # Letvægts JSON-metadatafil
```

Filen `.onianvil.json` gemmer:
- Lærredsgitteropsætning: rammedimensioner, kolonneantal, samlet antal rammer.
- Laghierarkier: spornavne, opacitetsniveauer, synlighedsflag, låsestatus.
- Animationstags: navn, start- og slutrammeindeks, afspilningsretning (Fremad, Baglæns, Ping-pong), rammevarigheder.
- Paletdefinitioner: aktive farvefelter, navngivne skin-varianter, indeks for aktivt skin.
- Referenceskitsetilstand: billedsti, skalering, panoreringsforskydning, rotation, opacitet, lagtilstand.
- Drejepunkter: brugerdefinerede handlingspunkter, våbenfæster og kollisionscentre.

---

## Bidrag og tak

Bidrag, fejlrapporter og forslag til nye funktioner er altid meget velkomne!
1. Fork arkivet på GitHub.
2. Opret en funktionsgren: `git checkout -b feature/min-nye-funktion`.
3. Gem dine ændringer: `git commit -am 'Tilføj nyt pixelværktøj'`.
4. Push til grenen: `git push origin feature/min-nye-funktion`.
5. Opret en Pull Request.

### Tak til
- Startdemospriten **"Cat 2D Pixel Art"** skabt af **xzany** ([projektside på itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Bygget med det fantastiske GUI-bibliotek **[egui](https://github.com/emilk/egui)** og grafiksystemet **[wgpu](https://wgpu.rs/)**.
- Brugerfladeikoner fra **[Font Awesome Free](https://fontawesome.com)** af @fontawesome (Ikoner: CC BY 4.0, Skrifttyper: SIL OFL 1.1, Kode: MIT-licens).

---

## Licens og ansvarsfraskrivelse

Dette projekt er licenseret under **[MIT-licensen](../LICENSE)**. Du har fuld frihed til at anvende, ændre, distribuere og integrere OniAnvil i både kommercielle og private spiludviklingsprojekter.

### Fraskrivelse af garanti og ansvarsbegrænsning
> [!IMPORTANT]
> **OniAnvil** distribueres **gratis** på **"SOM DET ER OG FOREFINDES" ("AS IS")** og **"SOM TILGÆNGELIGT"** grundlag, uden garantier eller betingelser af nogen art, hverken udtrykkelige eller underforståede.
> 
> Under ingen omstændigheder kan forfatteren/forfatterne, ophavsretsindehaverne eller bidragyderne holdes ansvarlige for direkte, indirekte, hændelige, særlige, pønalt begrundede eller følgeskader (herunder, men ikke begrænset til, tab af data, filbeskadigelse, hardwarefejl, driftstab eller driftsforstyrrelser), der på nogen måde måtte opstå som følge af brugen af eller manglende evne til at bruge denne software, selvom der er blevet underrettet om muligheden for en sådan skade. Du påtager dig ethvert ansvar og enhver risiko ved brugen af dette program.

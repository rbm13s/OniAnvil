<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixel-art- & onion-skin-animatiestudio gebouwd rond speelklare atlassen**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Talen](https://img.shields.io/badge/Talen-37_talen-purple)](#10-35-ergonomische-themas--volledige-lokalisatie)
[![Platformen](https://img.shields.io/badge/Platformen-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#aan-de-slag)
[![Web-demo](https://img.shields.io/badge/Online_app-Probeer_in_browser_(Experimenteel)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=nl)
[![Licentie: MIT](https://img.shields.io/badge/Licentie-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Probeer OniAnvil in de browser (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=nl)** *(⚠️ Experimentele web-build: kan WebGL/WASM-beperkingen en bugs bevatten)* • **[🌐 Officiële documentatiewebsite](https://rbm13s.github.io/OniAnvil/nl.html)**

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
**Nederlands** •
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

## Inhoudsopgave

- [Inleiding](#inleiding)
- [Screenshots & Visuele rondleiding](#screenshots--visuele-rondleiding)
- [Uitgebreide gereedschapsreferentie](#uitgebreide-gereedschapsreferentie)
- [Functionele documentatie](#functionele-documentatie)
  - [1. Dubbele atlas-centrische architectuur](#1-dubbele-atlas-centrische-architectuur)
  - [2. Meerlagige tijdlijn](#2-meerlagige-tijdlijn)
  - [3. Realtime live preview van animaties](#3-realtime-live-preview-van-animaties)
  - [4. Geavanceerde onion-skinning-engine](#4-geavanceerde-onion-skinning-engine)
  - [5. Slimme referentie- & schetstracing-laag](#5-slimme-referentie---schetstracing-laag)
  - [6. Precisieselectiesuite & Marching Ants](#6-precisieselectiesuite--marching-ants)
  - [7. Live FX, filters & RotSprite](#7-live-fx-filters--rotsprite)
  - [8. Paletbeheer, kleurwissels & personageskins](#8-paletbeheer-kleurwissels--personageskins)
  - [9. Sheet slicer, 9-grid UI & export naar game-engines](#9-sheet-slicer-9-grid-ui--export-naar-game-engines)
  - [10. 35 ergonomische thema's & tweetalige lokalisatie](#10-35-ergonomische-themas--tweetalige-lokalisatie)
- [Besturing & sneltoetsen](#besturing--sneltoetsen)
- [Aan de slag](#aan-de-slag)
  - [Kant-en-klare binaire bestanden](#kant-en-klare-binaire-bestanden)
  - [Bouwen vanaf broncode](#bouwen-vanaf-broncode)
- [Bestandsformaat & begeleidende architectuur](#bestandsformaat--begeleidende-architectuur)
- [Bijdragen & dankbetuigingen](#bijdragen--dankbetuigingen)
- [Licentie & disclaimer](#licentie--disclaimer)

---

## Inleiding

**OniAnvil** (*samengesteld uit **Oni**on-Skinning [uienhuid-animatie] en **Anvil** [het aambeeld om sprites te smeden]*) is een open-source, krachtige pixel-art-editor en animatiestudio geschreven in Rust. Het draait native met hardware-versnelde rendering op Windows, Linux en macOS, en als een installatievrije WebAssembly-toepassing rechtstreeks in moderne webbrowsers.

De meeste bestaande sprite-editors behandelen animaties als een verzameling losstaande afbeeldingsbestanden, wat game-ontwikkelaars dwingt tot tijdrovende export-, snijd- en atlas-herverpakkingsstappen. **OniAnvil draait dit paradigma om:** de spritesheet-atlas-textuur (`atlas_rgba`) van de game is de primaire bron van de waarheid. Framecellen, lagensporen, animatietags en botsingsdraaipunten blijven te allen tijde non-destructief en synchroon met de atlas.

### Waarom OniAnvil?
- 🚀 **Bliksemsnelle native Rust-prestaties**: Vrijwel onmiddellijke opstart in milliseconden, haarscherpe weergave aangedreven door `wgpu` en `egui 0.31`, geen vertraging bij het doorbladeren van lange tijdlijnen.
- 🎬 **Meerlagige montagetijdlijn**: Geïnspireerd op NLE-videobewerkers, met lagensporen (`V1, V2, V3...`), compositing van boven naar beneden, omlaag samenvoegen, gekoppelde frames en variabele weergaveduur (hold).
- 🧅 **Eersteklas onion skinning**: Ghosting over meerdere frames met onafhankelijke kleurtinten voor voorwaarts/achterwaarts, aanpasbare dekkingscurven en automatische lusovergang.
- 🎨 **Directe kleurenpaletskins voor personages**: Maak dag/nacht-varianten, teamkleuren of elementaire skins met live herindeling van de textuuratlas en GPL/PAL/HEX-import en -export.
- 📦 **Rechtstreekse export naar game-engines**: Exporteer geschaalde PNG's met geëxtrudeerde randen om textuurfilteringsfouten (naden) in 2D-engines (Godot, Bevy, Raylib, Defold) te voorkomen, samen met standaard sprite sheet JSON-metadata en geanimeerde GIF's.

---

## Screenshots & Visuele rondleiding

<div align="center">

### 1. Hoofdcanvas & studiowerkruimte
*Krachtige pixelwerkruimte met hardware-versnelde viewport, meerlagige tijdlijn, realtime live preview, paletstalen en precisiewerkbalken.*
<img src="screenshots/main.png" alt="OniAnvil hoofdwerkruimte" width="950" />

<br/><br/>

### 2. Meerlagige animatietijdlijn
*Niet-lineaire tijdlijn met gestapelde lagenrijen (`V1, V2...`), gekoppelde framecellen (`🔗`), hold-vermenigvuldigers en benoemde, kleurgecodeerde animatietags.*
<img src="screenshots/timeline.png" alt="OniAnvil meerlagige tijdlijn" width="950" />

<br/><br/>

### 3. Precisiegereedschappen & actiewerkbalk
*Volledige suite van 15 tekengereedschappen met standaard en compacte dockmodi, naast de bovenste snelle-actiewerkbalk voor penseelgrootte, symmetrie, randterugloop en pixel-perfect-modi.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil actiewerkbalk" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil gereedschappen standaard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil gereedschappen compact" width="300" />
</p>

<br/><br/>

### 4. Realtime live preview van animaties
*Speciaal live-afspeeldock met tagfiltering, variabele FPS-snelheidsregeling, zoomschuifregelaar en achtergrondwissels (dambordpatroon, egaal donker, egaal licht).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil live preview" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil live preview zoom" width="460" />
</p>

<br/><br/>

### 5. Paletbeheer, kleurwissels & kleurverlopen
*Uitgebreide kleurenmotor met voorgrond-/achtergrondkleurvakken, automatische tweepuntsverloopgeneratie en benoemde paletskinwissels.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil palet en kleurwissels" width="950" />

<br/><br/>

### 6. Referentieschetsen overtrekken & Configureerbare onion skinning
*Laad conceptschetsen of foto's als onderlaag/bovenlaag met transformatieknoppen, slimme kleurbemonstering met paletuitlijning en meervoudige onion-skin-ghosting.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil schets en onion skinning" width="950" />

<br/><br/>

### 7. Draaipunten & Interactieve actiepunten
*Plaats het sprite-nulpunt (pivot) en aangepaste gameplay-actiepunten (wapenankers, projectieluitgangen, hoofd, voetstappen) met interactieve canvas-gizmo's.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil draaipunt en actiepunten" width="950" />

<br/><br/>

### 8. Spritesheet-slicer & 9-Grid (9-Slice) schaalregelaar
*Automatische detectie van framegrenzen via autocorrelatie (bijv. loopcyclus van een kat) en configuratie van schaalbare 9-slice UI-kaders.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil sheet-slicer" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-slice schaalregelaar" width="460" />
</p>

<br/><br/>

### 9. Realtime beeldeffecten & filtervensters (met live preview)
*Alle effectvensters bieden een directe canvas-live-preview, interactieve pipetten en niet-destructieve annulering zonder vervuiling van de geschiedenis.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Omlijningsgenerator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Kleur vervangen" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Tint verzadiging contrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrixfilter" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Toewijzen aan palet" width="460" />
</p>

<br/><br/>

### 10. Canvasrooster & achtergrondmodi
*Schakel naadloos tussen pixelroosters, tegelroosters, donker dambordpatroon, licht dambordpatroon en aangepaste egale achtergrondkleuren.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Rooster en donker dambord" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Licht dambord zonder rooster" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Aangepaste egale achtergrond" width="300" />
</p>

<br/><br/>

### 11. Wizard voor nieuw project
*Configureer frame-afmetingen, aantal frames, kolomroosterindeling en kies uit meer dan 20 authentieke retropaletten bij het starten van een project.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Nieuw-projectwizard" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletkeuze in wizard" width="460" />
</p>

<br/><br/>

### 12. 35 authentieke retro- & moderne thema's
*Inclusief Studio Dark, Studio Light, CRT Amber/Groen/Cyaan fosfor, retroconsoles (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) en ontwerpersthema's.*
<p align="center">
  <img src="screenshots/themes.png" alt="Thema-menu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber-thema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX-thema" width="300" />
</p>

<br/><br/>

### 13. Menusysteem & Ingebouwde sneltoetshulp
<p align="center">
  <img src="screenshots/file_menu.png" alt="Bestand-menu" width="180" />
  <img src="screenshots/edit_menu.png" alt="Bewerken-menu" width="180" />
  <img src="screenshots/select_menu.png" alt="Selecteren-menu" width="180" />
  <img src="screenshots/view_menu.png" alt="Beeld-menu" width="180" />
  <img src="screenshots/project_menu.png" alt="Project-menu" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Sneltoetsen-hulpvenster" width="700" />
</p>

</div>

---

## Uitgebreide gereedschapsreferentie

OniAnvil bevat 15 precisietekengereedschappen voor rasteren, geometrie, selectie en metadata. De pictogrammen maken gebruik van de open-source iconensuite **Font Awesome Free** *(Iconen: CC BY 4.0, Fonts: SIL OFL 1.1)*.

| Gereedschap | Icoon | Sneltoets | Penseelgrootte | Pixel-Perfect | Beschrijving |
|:---|:---:|:---:|:---:|:---:|:---|
| **Potlood** | ✏️ | <kbd>B</kbd> | 1..32 px | Ja | Primair tekengereedschap. De 1px-modus ondersteunt **Pixel-Perfect** tekenen (elimineert automatisch lelijke trap-hoeken). Ondersteunt aangepaste selectiestempelpenselen (`Ctrl/Cmd+B`). |
| **Gum** | 🧹 | <kbd>E</kbd> | 1..32 px | Nee | Wist pixels terug naar volledige transparantie (`[0, 0, 0, 0]`). Beperkt tot het actieve selectiemasker indien aanwezig. |
| **Pipet** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nee | Linksklikken kiest de kleur als Voorgrond (Primary); rechtsklikken als Achtergrond (Secondary). Kan kleur opnemen van canvas, lagen of referentieschets. |
| **Vulemmer** | 🪣 | <kbd>G</kbd> | N.v.t. | Nee | Razendsnelle BFS-vulling op wachtrijbasis met instelbare kleurtolerantie (`0..=255`). Ondersteunt aaneengesloten versus globale vulling en selectiebegrenzing. |
| **Kleurverloop** | 🌈 | <kbd>G</kbd> *(wissel)* | N.v.t. | Nee | Lineaire en radiale verlopen tussen voorgrond- en achtergrondkleur. Ondersteunt Bayer- (2x2, 4x4, 8x8) en Blue Noise-dithering met intensiteitsregeling. |
| **Lijn** | 📏 | <kbd>L</kbd> | 1..32 px | Ja | Bresenham-lijnalgoritme met live sleepvoorbeeld. Houd <kbd>Shift</kbd> ingedrukt om hoeken vast te zetten op stappen van 45°/90°. |
| **Kromme** | 〰️ | <kbd>C</kbd> | 1..32 px | Ja | Kwadratische Bézier-kromme in 3 klikken: klik 1 zet startpunt, klik 2 zet eindpunt, muisbeweging buigt de curve, klik 3 bevestigt de lijn. |
| **Rechthoek** | ⬜ | <kbd>U</kbd> | 1..32 px | Nee | Omlijnde of gevulde geometrische rechthoeken. Houd <kbd>Shift</kbd> ingedrukt om een perfect 1:1 vierkant te forceren. |
| **Ellips** | ⭕ | <kbd>O</kbd> | 1..32 px | Nee | Omlijnde of gevulde ellipsen via middelpunt-rasterisatie. Houd <kbd>Shift</kbd> ingedrukt om een perfecte cirkel te forceren. |
| **Ditherpenseel** | 🏁 | <kbd>D</kbd> | 1..32 px | Nee | Schildert afwisselende dambord-ditherpatronen tussen voorgrond- en achtergrondkleur zonder extra paletkleuren toe te voegen. |
| **Schakeergereedschap** | 🌓 | <kbd>S</kbd> | 1..32 px | Nee | Stapsgewijs lichter maken (Dodge) en donkerder maken (Burn) op basis van de volgorde van de stalen in het actieve palet. |
| **Rechthoekige selectie** | ⬚ | <kbd>M</kbd> | N.v.t. | Nee | Rechthoekige gebiedsselectie met geanimeerde marching-ants-rand. Ondersteunt toevoegen (<kbd>Shift</kbd>) en aftrekken (<kbd>Alt</kbd>). |
| **Lassoselectie** | ➰ | <kbd>Q</kbd> | N.v.t. | Nee | Veelhoekselectie uit de vrije hand. Maakt gebruik van 8-verbonden randrastering met 4-verbonden externe vulling: 100% solide en gatvrij, zelfs bij lussen. |
| **Toverstaf** | 🪄 | <kbd>W</kbd> | N.v.t. | Nee | Selecteert aaneengesloten (of globale) gebieden met dezelfde kleur met een instelbare tolerantiedrempel. |
| **Draai- & actiepunten** | 🎯 | <kbd>P</kbd> | N.v.t. | Nee | Interactieve canvasplaatsing van het spritenulpunt (Pivot) en aangepaste gameplay-actiepunten (`weapon`, `head`, `feet`, `hitbox`) met JSON-export. |

---

## Functionele documentatie

### 1. Dubbele atlas-centrische architectuur

In OniAnvil vertegenwoordigt de master-textuuratlas (`atlas_rgba`) de werkelijke spritesheet-textuur zoals deze door uw game-engine wordt geladen:
- **Frame-modus (Frame Mode)**: Focust op één actief animatieframe voor ongestoord pixeltekenen. Bewerkingen werken onmiddellijk de bijbehorende cel in de atlastextuur bij.
- **Atlas-modus (Atlas Mode)**: Toont het volledige spritesheet-rooster en maakt macro-aanpassingen, frame-uitlijncontroles en batchbewerkingen mogelijk.
- **Begeleidend bestand (Companion File)**: Projecten slaan schone `.png`-spritesheets op, vergezeld van een lichtgewicht `.onianvil.json`-metadatabestand met frame-indelingen, lagenhiërarchieën, tags, weergaveduren en paletskins.

### 2. Meerlagige tijdlijn

De onderste tijdlijn is gemodelleerd naar professionele niet-lineaire videobewerkingssoftware (NLE):
- **Lagensporen (`V1, V2, V3...`)**: Lagen worden visueel van boven naar beneden gestapeld. Het spoor dat hoger in de tijdlijn staat (`V1`), wordt bovenop in het canvas weergegeven.
- **Spoorbediening**:
  - Klik op de badges `V1` / `V2` om snel van actieve tekenlaag te wisselen.
  - Schakel laagzichtbaarheid (oogpictogram) en bewerkingsvergrendeling (slotpictogram) in of uit.
  - Pas de dekking van individuele lagen aan (`0%..=100%`).
  - Verplaats lagen omhoog/omlaag in de hiërarchie, dupliceer lagen en voeg lagen omlaag samen (Merge Down).
  - Verwijder lagen veilig met een bevestigingsvenster.
- **Frameclips**:
  - Hold-vermenigvuldiger: Wijs aangepaste frameduren toe (bijv. `2×`, `3×` hold voor pauzes of impactframes).
  - Gekoppelde frames (`🔗`): Koppel herhalende frames in de animatie, zodat het bewerken van één instantie automatisch alle gekoppelde exemplaren bijwerkt.
  - Tag-lussen: Organiseer animaties in benoemde tags (`idle`, `walk`, `attack`) en speel specifieke tags in een lus af.
  - Afspeelbediening: Vooruit, Achteruit, Ping-pong en Herhalend afspelen met instelbare beeldsnelheid (`1..=120` FPS).

### 3. Realtime live preview van animaties

Het rechter preview-dock speelt uw animaties in realtime af, volledig onafhankelijk van het zoomniveau of de panpositie van uw canvas:
- **Tagfiltering**: Isoleer het afspelen naar specifieke animatietags (`walk`, `jump`) of bekijk de volledige reeks frames.
- **Afspeelknoppen**: Afspelen, Pauzeren, Vorige stap, Volgende stap, Luswissel en aangepaste afspeelsnelheden (`0.25x..=4.0x`).
- **Weergavemodi**: Schakel eenvoudig tussen een donker dambordpatroon, een licht dambordpatroon, zuiver wit, zuiver zwart of een aangepaste canvasachtergrondkleur.

### 4. Geavanceerde onion-skinning-engine

Boetseer vloeiende animaties met het gespecialiseerde onion-skinning-systeem:
- **Aangepaste kleurtinten**: Onafhankelijke tintkleuren voor voorgaande frames (standaard: Rood) en opeenvolgende frames (standaard: Blauw).
- **Dekkingsafval**: Configureerbare basisdekking en stapsgewijze vervalcurven voor opeenvolgende schaduwbeelden.
- **Lusovergang (Wrap-Around)**: Maakt ghosting mogelijk over het eerste en laatste frame bij gesloten animatielussen.
- **Snelle weergave (Quick-Peek)**: Houd de <kbd>Spatiebalk</kbd> ingedrukt (zonder te slepen) om direct aangrenzende frames in te kijken.

### 5. Slimme referentie- & schetstracing-laag

Versnel uw workflow door handgetekende schetsen of referentiefoto's over te trekken:
- **Formaten**: Laad PNG-, JPEG- en WebP-referentieafbeeldingen.
- **Overvloeimodi**: Onderlaag (onder uw pixellagen) of Bovenlaag (boven uw pixellagen).
- **Transformatieknoppen**: Onafhankelijke zoom, positieverschuiving, rotatie en dekkingsschuifregelaar.
- **Slimme kleurbemonstering**: Gebruik de pipet op de schetslaag met optionele automatische uitlijning naar de dichtstbijzijnde kleur in uw actieve projectpalet.
- **1-klik inbakken (Bake)**: Zet de geschaalde referentieafbeelding met één klik direct om in pixel-art op het actieve frame.

### 6. Precisieselectiesuite & Marching Ants

- **Geanimeerde Marching Ants**: Actieve selecties worden aangeduid met een dynamische, contrastrijke tweekleurige stippellijn.
- **Solide vullasso**: In tegenstelling tot naïeve straalalgoritmen die gaten achterlaten bij overlappende lussen, rastert OniAnvil's lasso een gesloten 8-verbonden rand en vult het de buitenruimte, wat een 100% solide en gatvrije selectie garandeert.
- **Selectiebewerkingen**: Uitbreiden, Verkleinen, Rand (met cirkel-, vierkant- en ruitvormen), Verplaatsen, Inverteren, Verwijderen, Vrij roteren en kopiëren/knippen/plakken via het klembord.
- **Frame-overschrijdend kopiëren & plakken**: Selecteer gebieden op één frame, druk op <kbd>Ctrl/Cmd+C</kbd>, schakel naar een ander frame en druk op <kbd>Ctrl/Cmd+V</kbd> om pixels op exact dezelfde relatieve coördinaten te plakken, ongeacht uw toetsenbordindeling.

### 7. Live FX, filters & RotSprite

Alle beeldeffecten bevatten een realtime selectievakje **Live preview** dat wijzigingen direct op het canvas toont en een naadloze annulering mogelijk maakt zonder de geschiedenis te belasten:
- **Kleur vervangen (`Shift+R`)**:
  - Interactieve pipet: klik op het pipetpictogram en kies kleuren direct uit canvaspixels of de paletbalk.
  - Snelle paletstrip: linksklikken selecteert de bronkleur ("Van"), rechtsklikken selecteert de doelkleur ("Naar").
  - Tolerantieschuifregelaar en batchgewijze vervanging over alle animatieframes tegelijk.
- **Kleuraanpassing (`Ctrl/Cmd+U`)**: Realtime HSV-tintverschuiving (Hue), verzadiging (Saturation), helderheid (Value) en contrastinstelling.
- **Automatische omlijningsgenerator**: Genereer haarscherpe personagecontouren (Buiten, Binnen, Gecentreerd) met rechte (Vierkant) of afgeschuinde (Ruit) hoeken.
- **3×3-convolutiefilters**: Vervagen (Blur), Verscherpen (Sharpen), Randdetectie (Edge Detect) en Reliëf (Emboss) met behoud van het alfakanaal.
- **Vrije rotatie (`Ctrl/Cmd+Shift+R`)**: Roteer geselecteerde gebieden of volledige frames over een willekeurige hoek met de klassieke **RotSprite**- of **Nearest Neighbor**-algoritmen.
- **Toewijzen aan palet**: Zet canvaskleuren om naar het actieve palet met behulp van Bayer-dithering (2x2, 4x4, 8x8) of Blue Noise.

### 8. Paletbeheer, kleurwissels & personageskins

- **Ingebouwde paletten**: Meer dan 20 authentieke retropaletten, waaronder Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized en Sweetie 16.
- **Import & export**: Ondersteuning voor GIMP-paletten (`.gpl`), PAL (`.pal`), hexadecimale bestanden (`.hex`) en afbeeldingsstalen (`.png`).
- **Paletwissels (Skins)**: Definieer meerdere kleurvarianten voor één enkel personage (bijv. Standaard, Gouden pantser, Schaduw, IJs). Het wisselen van skins herberekent direct alle framekleuren over de gehele atlas.
- **Kleurverloopgenerator (Ramps)**: Interpoleer automatisch vloeiende kleurverlopen tussen twee gekozen tinten met een instelbaar aantal tussenstappen.

### 9. Sheet slicer, 9-grid UI & export naar game-engines

- **Sheet slicer**: Autocorrelatiedetectie snijdt ruwe spritesheets automatisch op in uniforme of strak omsloten animatieframes.
- **9-Grid (9-Slice) schaling**: Stel aanpasbare marges in (boven, onder, links, rechts) om rekbare UI-vensters en knoppen nauwkeurig te testen.
- **PNG-export met geëxtrudeerde randen**: Dupliceert randpixels naar transparante marges, wat textuurfilteringfouten en naden in OpenGL, DirectX, Vulkan en 2D-engines (zoals Godot) effectief voorkomt.
- **Geanimeerde GIF-export**: Genereer geoptimaliseerde geanimeerde GIF's met aangepaste herhalingsaantallen, framevertragingen en transparantie.
- **Standaard sprite sheet JSON**: Exporteert framesegmenten, tags, weergaveduren en nulpuntcoördinaten conform het standaard sprite sheet JSON-schema van moderne 2D-game-engines.

### 10. 35 ergonomische thema's & tweetalige lokalisatie

- **Samengestelde thema's**: Studio Dark, Studio Light, Gitsterzwart, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Groen CRT, Cyaan CRT, Retroconsoles (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) en Retrocomputers (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Tweetalige ondersteuning**: Schakel naadloos tussen **English** en **Nederlands** (evenals vele andere talen) met 100% pariteit van alle vertaalsleutels.

---

## Besturing & sneltoetsen

| Sneltoets | Gereedschap / Actie | Beschrijving |
|:---|:---|:---|
| <kbd>B</kbd> | **Potlood** | 1px precisietekenen met optionele Pixel-Perfect-modus |
| <kbd>E</kbd> | **Gum** | Pixels wissen naar transparantie |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipet** | Kleur kiezen van canvaspixels of referentieschets |
| <kbd>G</kbd> | **Vulemmer / Kleurverloop** | Wisselen tussen vulemmer en dither-kleurverloop |
| <kbd>L</kbd> | **Lijn** | Rechte lijnen met live sleepvoorbeeld en Shift-uitlijning |
| <kbd>C</kbd> | **Kromme** | Kwadratische Bézier-kromme in 3 muisklikken |
| <kbd>U</kbd> | **Rechthoek** | Geometrische rechthoek en vierkant (omlijnd/gevuld) |
| <kbd>O</kbd> | **Ellips** | Geometrische ellips en cirkel (omlijnd/gevuld) |
| <kbd>D</kbd> | **Ditherpenseel** | Schilderen met dambord-ditherpatronen |
| <kbd>S</kbd> | **Schakeergereedschap** | Stapsgewijs lichter (Dodge) of donkerder (Burn) maken via het palet |
| <kbd>M</kbd> | **Rechthoekige selectie** | Rechthoekige gebiedsselectie met marching ants |
| <kbd>Q</kbd> | **Lassoselectie** | Solide veelhoekselectie uit de vrije hand |
| <kbd>W</kbd> | **Toverstaf** | Aaneengesloten kleurgebieden selecteren |
| <kbd>P</kbd> | **Draai- & actiepunten** | Spritenulpunt en actiepunten interactief plaatsen |
| <kbd>X</kbd> | **Kleuren wisselen** | Voorgrond- en achtergrondkleur omwisselen |
| <kbd>[</kbd> / <kbd>]</kbd> | **Penseelgrootte** | Penseelgrootte verkleinen / vergroten (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopiëren** | Geselecteerde pixels kopiëren naar het klembord |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Knippen** | Geselecteerde pixels knippen naar het klembord |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Plakken** | Klembordpixels op het actieve frame plakken |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Wissen** | Pixels binnen de actieve selectie wissen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Alles selecteren** | Alle pixels op het actieve frame selecteren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Deselecteren** | Actieve selectiemasker wissen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Selectie omkeren** | Actieve selectiemasker inverteren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Frame dupliceren** | Actieve frame in de tijdlijn dupliceren |
| <kbd>Spatiebalk</kbd> | **Afspelen / Pauzeren** | Animatie afspelen in tijdlijn in-/uitschakelen |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Kleur vervangen** | Venster voor kleurvervanging openen met live preview |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Kleuraanpassing** | Venster voor HSV-tint / verzadiging / contrast openen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Vrij roteren** | Venster voor vrije rotatie openen (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Ongedaan maken** | Laatste penseelstreek of gereedschapsactie ongedaan maken |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Opnieuw uitvoeren** | Eerder ongedaan gemaakte actie opnieuw uitvoeren |
| <kbd>Spatiebalk</kbd> + Slepen / MMB | **Canvas pannen** | Vloeiende oneindige verschuiving over canvascoördinaten |
| <kbd>Ctrl/Cmd</kbd> + Wiel / <kbd>+</kbd> / <kbd>-</kbd> | **Zoomen** | Pixelscherp zoomen gecentreerd rond de muisaanwijzer |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Weergavemodus** | Schakelen tussen frame-weergave (<kbd>1</kbd>) en atlas-weergave (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Canvas passend maken** | Actieve frame automatisch in het midden van het canvas centreren |
| <kbd>F1</kbd> | **Hulp** | Sneltoetsenoverzicht openen |

---

## Aan de slag

### Kant-en-klare binaire bestanden

Download vooraf gecompileerde, zelfstandige binaire bestanden rechtstreeks vanaf de **[GitHub Releases-pagina](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Download `OniAnvil-windows-x86_64.zip`, pak het bestand uit en dubbelklik op `OniAnvil.exe`.
- **Linux**: Download `OniAnvil-linux-x86_64.tar.gz` (draagbaar) of installeer het Debian/Ubuntu-pakket `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Download `OniAnvil-macos-universal.zip`, pak het uit en sleep `OniAnvil.app` naar de map Programma's. Het universele binaire bestand draait native op zowel Apple Silicon (M1/M2/M3/M4) als Intel Macs.
- **Web**: Draai direct in uw webbrowser zonder installatie: **[WebAssembly-toepassing starten](https://rbm13s.github.io/OniAnvil/app/?lang=nl)**.
  > [!WARNING]
  > De WebAssembly-browserversie heeft een **experimentele** status. Vanwege browsersandbox-beperkingen, WebGL/WebGPU-verschillen en klembordbeperkingen kunnen er kleine grafische foutjes of prestatieverminderingen optreden ten opzichte van de native desktopversie. Voor productiewerk worden native desktopversies ten zeerste aanbevolen.

---

### Bouwen vanaf broncode

#### Vereisten
- **Rust Toolchain**: Installeer Rust (versie 1.80+ aanbevolen) via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux-build-afhankelijkheden** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compileer & start de native desktopversie
```bash
# Kloon de repository
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Voer de testsuite uit
cargo test

# Bouw en start de geoptimaliseerde release-binary
cargo run --release
```

#### 2. Compileer de WebAssembly-versie (via Trunk)
```bash
# Voeg het wasm32-compilatiedoel toe en installeer Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Start een lokale ontwikkelserver met hot-reload
trunk serve

# Bouw geoptimaliseerde release-bestanden in docs/app/
./build_wasm.sh        # Op Linux/macOS
./build_wasm.ps1       # In Windows PowerShell
```

---

## Bestandsformaat & begeleidende architectuur

OniAnvil vermijdt gesloten binaire formaten. Elk project bestaat uit twee schone, onder versiebeheer te houden bestanden:

```
my_character/
├── my_character.png            # Standaard PNG-spritesheet-atlas (te openen in elke game-engine of viewer)
└── my_character.onianvil.json   # Begeleidend metadatabestand
```

Het `.onianvil.json`-bestand bewaart:
- Canvasroosterconfiguratie: frame-afmetingen, aantal kolommen, totaal aantal frames.
- Lagenhiërarchieën: spoornamen, dekkingsniveaus, zichtbaarheidsvlaggen, vergrendelingsstatussen.
- Animatietags: naam, start-/eindframe-indices, afspeelrichting (Vooruit, Achteruit, Ping-pong), frameduren.
- Paletdefinities: actieve kleurstalen, benoemde skinvarianten, actieve skin-index.
- Status van referentieschets: afbeeldingspad, schaal, positieverschuiving, rotatie, dekking, laagmodus.
- Draaipunten: aangepaste actiepunten, wapenankers en botsingscentra.

---

## Bijdragen & dankbetuigingen

Bijdragen, bugmeldingen en functieverzoeken zijn van harte welkom!
1. Fork de repository op GitHub.
2. Maak een feature-branch aan: `git checkout -b feature/mijn-nieuwe-functie`.
3. Leg uw wijzigingen vast: `git commit -am 'Voeg nieuw pixelgereedschap toe'`.
4. Push naar de branch: `git push origin feature/mijn-nieuwe-functie`.
5. Open een Pull Request.

### Dankbetuigingen
- Starter-demosprite **"Cat 2D Pixel Art"** gemaakt door **xzany** ([itch.io projectpagina](https://xzany.itch.io/cat-2d-pixel-art)).
- Gebouwd met de fantastische **[egui](https://github.com/emilk/egui)** GUI-bibliotheek en het **[wgpu](https://wgpu.rs/)** grafische ecosysteem.
- UI-pictogrammen verzorgd door **[Font Awesome Free](https://fontawesome.com)** van @fontawesome (Iconen: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License).

---

## Licentie & disclaimer

Dit project is gelicentieerd onder de **[MIT-licentie](../LICENSE)**. Het staat u vrij om OniAnvil te gebruiken, aan te passen, te verspreiden en te integreren in commerciële en persoonlijke game-ontwikkelingspipelines.

### Garantie-uitsluiting & aansprakelijkheidsbeperking
> [!IMPORTANT]
> **OniAnvil** wordt **gratis** verspreid op een **"ZOALS HET IS" ("AS IS")** en **"ZOALS BESCHIKBAAR" ("AS AVAILABLE")** basis, zonder enige expliciete of impliciete garanties of voorwaarden van welke aard dan ook.
> 
> Onder geen enkele omstandigheid zijn de auteur(s), auteursrechthebbenden of bijdragers aansprakelijk voor enige directe, indirecte, incidentele, bijzondere, voorbeeldige of gevolgschade (met inbegrip van, maar niet beperkt tot, verlies van gegevens, bestandsbeschadiging, hardwarestoringen, winstderving of bedrijfsonderbreking) die op enigerlei wijze voortvloeit uit het gebruik van of het onvermogen tot het gebruik van deze software, zelfs indien zij op de hoogte zijn gesteld van de mogelijkheid van dergelijke schade. U aanvaardt alle verantwoordelijkheid en risico's voor het gebruik van dit programma.

<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pikselgrafikk- og løkbladsanimasjonsstudio (Onion-Skinning) bygget for spillklare teksturatlaser**

[![Bygg og utgivelse](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Nedlastinger](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Språk](https://img.shields.io/badge/Språk-37_språk-purple)](#10-35-ergonomiske-temaer-og-fullstendig-lokalisering)
[![Plattform](https://img.shields.io/badge/Plattform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#kom-i-gang)
[![Webdemo](https://img.shields.io/badge/Online-Prøv_i_nettleseren_(Eksperimentell)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=no)
[![Lisens: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Prøv OniAnvil i nettleseren (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=no)** *(⚠️ Eksperimentell versjon: kan inneholde WebGL/WASM-begrensninger og feil)* • **[🌐 Offisielt dokumentasjonsnettsted](https://rbm13s.github.io/OniAnvil/no.html)**

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
**Norsk** •
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

## Innholdsfortegnelse

- [Introduksjon](#introduksjon)
- [Skjermbilder og visuell omvisning](#skjermbilder-og-visuell-omvisning)
- [Omfattende verktøyreferanse](#omfattende-verktøyreferanse)
- [Funksjonsdokumentasjon](#funksjonsdokumentasjon)
  - [1. Dobbelt atlas-sentrert arkitektur](#1-dobbelt-atlas-sentrert-arkitektur)
  - [2. Flerspors lagtidslinje](#2-flerspors-lagtidslinje)
  - [3. Sanntids direkte forhåndsvisning (Live Preview)](#3-sanntids-direkte-forhåndsvisning-live-preview)
  - [4. Avansert løkbladsmotor (Onion Skinning)](#4-avansert-løkbladsmotor-onion-skinning)
  - [5. Smart referanse- og skissetegningslag](#5-smart-referanse--og-skissetegningslag)
  - [6. Presisjonsmarkeringsverktøy og marsjerende maur (Marching Ants)](#6-presisjonsmarkeringsverktøy-og-marsjerende-maur-marching-ants)
  - [7. Sanntidseffekter, filtre og RotSprite](#7-sanntidseffekter-filtre-og-rotsprite)
  - [8. Palettstyring, fargebytte og karakterskins](#8-palettstyring-fargebytte-og-karakterskins)
  - [9. Spritearkoppdeler, 9-Grid UI og eksport til spillmotorer](#9-spritearkoppdeler-9-grid-ui-og-eksport-til-spillmotorer)
  - [10. 35 ergonomiske temaer og fullstendig lokalisering](#10-35-ergonomiske-temaer-og-fullstendig-lokalisering)
- [Tastatursnarveier og kontroller](#tastatursnarveier-og-kontroller)
- [Kom i gang](#kom-i-gang)
  - [Frittstående binærfiler](#frittstående-binærfiler)
  - [Bygging fra kildekode](#bygging-fra-kildekode)
- [Filformat og ledsagerarkitektur](#filformat-og-ledsagerarkitektur)
- [Bidrag og anerkjennelser](#bidrag-og-anerkjennelser)
- [Lisens og ansvarsfraskrivelse](#lisens-og-ansvarsfraskrivelse)

---

## Introduksjon

**OniAnvil** (*fra **Oni**on-Skinning [løkbladteknikk] + **Anvil** [ambolt for smiing av sprites]*) er et åpen kildekode, høyytelses pikselgrafikk- og animasjonsstudio skrevet i Rust. Programmet kjører integrert med maskinvareakselerert grafikk på Windows, Linux og macOS, og som en installasjonsfri WebAssembly-applikasjon direkte i moderne nettlesere.

De fleste eksisterende sprite-editorer behandler animasjoner som en stabel separate bildefiler, noe som tvinger spillutviklere inn i tidkrevende runder med eksport, oppdeling og pakking. **OniAnvil snur denne modellen på hodet:** spillets teksturatlas (`atlas_rgba`) er den primære kilden til sannhet. Bilderammer, lagspor, animasjonstagger og kollisjonspunkter holdes kontinuerlig synkronisert med atlasset på en ikke-destruktiv måte.

### Hvorfor OniAnvil?
- 🚀 **Lynrask ytelse i ren Rust**: Øyeblikkelig oppstart på under millisekundet, knivskarp grafikk drevet av `wgpu` og `egui 0.31`, ingen forsinkelse ved navigering i lange tidslinjer.
- 🎬 **Flerspors lagtidslinje**: NLE-inspirerte lagspor (`V1, V2, V3...`) med lagkomposisjon ovenfra og ned, sammenslåing, koblede bilderammer og variable varigheter.
- 🧅 **Førsteklasses løkblad (Onion Skinning)**: Gjennomsiktighet over flere rammer med uavhengig fargetoning fremover og bakover, tilpassbare opasitetskurver og sømløs loop-støtte.
- 🎨 **Øyeblikkelige karakterskins**: Lag dag-/nattvarianter, lagfarger eller elementskins med direkte remapping i atlasset, samt import og eksport av GPL-, PAL- og HEX-filer.
- 📦 **Direkte eksport til spillmotorer**: Eksporter skalerte PNG-filer med ekstruderte kanter for å eliminere teksturblødning i 2D-motorer (Godot, Bevy, Raylib, Defold), sammen med standardisert Sprite Sheet JSON-metadata og animerte GIF-filer.

---

## Skjermbilder og visuell omvisning

<div align="center">

### 1. Hovedlerret og studioarbeidsområde
*Høyytelses pikselarbeidsområde med maskinvareakselerert visningsområde, flerspors tidslinje, sanntids direkte forhåndsvisning, palettruter og presisjonsverktøylinjer.*
<img src="screenshots/main.png" alt="OniAnvil Hovedarbeidsområde" width="950" />

<br/><br/>

### 2. Flerspors animasjonstidslinje
*Ikke-lineær flerspors tidslinje med stablede lagrader (`V1, V2...`), koblede rammeinstanser (`🔗`), hold-multiplikatorer og navngitte fargekodede animasjonstagger.*
<img src="screenshots/timeline.png" alt="OniAnvil Flerspors tidslinje" width="950" />

<br/><br/>

### 3. Presisjonsverktøy og hurtighandlingslinje
*Komplett tegneverktøysett med 15 verktøy i standard- og kompaktdokking-modus, sammen med den øvre verktøylinjen for penselstørrelse, symmetri, kantomslag og Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Handlingslinje" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Verktøy standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Verktøy kompakt" width="300" />
</p>

<br/><br/>

### 4. Sanntids direkte animasjonsforhåndsvisning (Live Preview)
*Dedikert avspillingsdokk med tagg-filtrering, variable FPS-hastighetskontroller, glidebryter for zoom og bakgrunnsbytte for lerretet (sjakkbrett, mørk ensfarget, lys ensfarget).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Forhåndsvisning" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Forhåndsvisning zoom" width="460" />
</p>

<br/><br/>

### 5. Palettstyring, fargebytte og fargeramper
*Omfattende fargemotor med ruter for forgrunns-/bakgrunnsfarger, automatisk generering av topunkts fargeramper og navngitte palettskins.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palett og skins" width="950" />

<br/><br/>

### 6. Skissetegning og konfigurerbart løkblad (Onion Skinning)
*Last inn konseptskisser eller fotografier som underlag/overlag med transformeringskontroller, smart fargeprøvetaking med palettilpasning og flerrammers løkbladskygger.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Skisse og løkblad" width="950" />

<br/><br/>

### 7. Dreiepunkter (Pivots) og gameplay-handlingspunkter (Action Points)
*Definer spritens sentrum/origo og tilpassede navngitte gameplay-punkter (våpenfester, prosjektilpunkter, hode, fottrinn) med interaktive lerretsmarkører.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Dreiepunkter og handlingspunkter" width="950" />

<br/><br/>

### 8. Spritearkoppdeler (Sheet Slicer) og 9-Grid (9-Slice) skalering
*Automatisk grensegjenkjenning via autokorrelasjon (f.eks. kattens gangsyklus) samt skalerbar 9-slice UI-boksmarginkonfigurasjon.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Spritearkoppdeler" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Slice skalering" width="460" />
</p>

<br/><br/>

### 9. Sanntids bilde-FX og filterdialoger (med direkte forhåndsvisning)
*Alle effektdialoger har direkte forhåndsvisning på lerretet, interaktive pipetter og ikke-destruktiv avbryting.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Konturgenerator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Erstatt farge" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Fargetone Metning Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrisefiltre" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Tilordne til palett" width="460" />
</p>

<br/><br/>

### 10. Lerretsgitter og bakgrunnsmoduser
*Bytt sømløst mellom pikselgitter, flisegitter, mørkt sjakkbrett, lyst sjakkbrett og tilpasset ensfarget bakgrunn.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Gitter og mørkt sjakkbrett" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Lyst sjakkbrett uten gitter" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Tilpasset ensfarget bakgrunn" width="300" />
</p>

<br/><br/>

### 11. Veiviser for opprettelse av nytt prosjekt
*Konfigurer lerretsdimensjoner, rammer, kolonnegitter og velg blant over 20 autentiske retropaletter ved oppstart.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Veiviser for nytt prosjekt" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletter for nytt prosjekt" width="460" />
</p>

<br/><br/>

### 12. 35 autentiske retro- og moderne temaer
*Inkluderer Studio Dark, Studio Light, CRT Amber/Green/Cyan-fosfor, retrokonsoller (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) og designertemaer.*
<p align="center">
  <img src="screenshots/themes.png" alt="Temameny" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber-tema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX-tema" width="300" />
</p>

<br/><br/>

### 13. Menysystem og innebygd hurtigtasthjelp
<p align="center">
  <img src="screenshots/file_menu.png" alt="Filmeny" width="180" />
  <img src="screenshots/edit_menu.png" alt="Redigeringsmeny" width="180" />
  <img src="screenshots/select_menu.png" alt="Markeringsmeny" width="180" />
  <img src="screenshots/view_menu.png" alt="Visningsmeny" width="180" />
  <img src="screenshots/project_menu.png" alt="Prosjektmeny" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Hjelpevindu for hurtigtaster" width="700" />
</p>

</div>

---

## Omfattende verktøyreferanse

OniAnvil inneholder 15 presisjonsverktøy for tegning, geometri, markering og metadata. Ikonene vises ved hjelp av åpen kildekode-pakken **Font Awesome Free** *(Ikoner: CC BY 4.0, Fonter: SIL OFL 1.1)*.

| Verktøy | Ikon | Snarvei | Penselstørrelse | Pixel Perfect | Beskrivelse |
|:---|:---:|:---:|:---:|:---:|:---|
| **Blyant** | ✏️ | <kbd>B</kbd> | 1..32 px | Ja | Primært tegneverktøy. 1px-modus støtter **Pixel-Perfect**-tegning (eliminerer automatisk stygge trappetrinns-L-hjørner). Støtter tilpassede stempelpensler fra markeringer (`Ctrl/Cmd+B`). |
| **Viskelær** | 🧹 | <kbd>E</kbd> | 1..32 px | Nei | Sletter piksler tilbake til full gjennomsiktighet (`[0, 0, 0, 0]`). Begrenses av markeringsmasken hvis den er aktiv. |
| **Pipette** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nei | Venstreklikk henter farge til Forgrunn (Primær); Høyreklikk henter til Bakgrunn (Sekundær). Kan hente farge fra lerretets piksler, lag eller referanseskisse. |
| **Malingspann** | 🪣 | <kbd>G</kbd> | Ikke relevant | Nei | Rask købasert BFS-fylling med justerbar fargetoleranse (`0..=255`). Støtter sammenhengende vs. global fylling samt markeringsavgrensning. |
| **Fargeovergang** | 🌈 | <kbd>G</kbd> *(bytt)* | Ikke relevant | Nei | Lineære og radielle overganger mellom forgrunns- og bakgrunnsfarger. Støtter Bayer- (2x2, 4x4, 8x8) og Blue Noise-dithering med styrkekontroll. |
| **Linje** | 📏 | <kbd>L</kbd> | 1..32 px | Ja | Bresenham-linjealgoritme med direkte forhåndsvisning under trekking. Hold <kbd>Shift</kbd> for å låse vinkler til trinn på 45°/90°. |
| **Kurve** | 〰️ | <kbd>C</kbd> | 1..32 px | Ja | Kvadratisk Bézier-pikselkurve med 3 klikk. Klikk 1 setter startpunkt, Klikk 2 setter sluttpunkt, musebevegelse justerer krumning, Klikk 3 fester kurven. |
| **Rektangel** | ⬜ | <kbd>U</kbd> | 1..32 px | Nei | Geometriske rektangler med omriss eller fyll. Hold <kbd>Shift</kbd> for å låse til et 1:1 kvadrat. |
| **Ellipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Nei | Ellipser og sirkler med omriss eller fyll via midtpunktsrasterisering. Hold <kbd>Shift</kbd> for å tvinge frem en perfekt sirkel. |
| **Dithering-pensel** | 🏁 | <kbd>D</kbd> | 1..32 px | Nei | Maler vekslende sjakkbrettmønstret dithering mellom forgrunns- og bakgrunnsfarger uten å introdusere ekstra palettfarger. |
| **Skyggeverktøy** | 🌓 | <kbd>S</kbd> | 1..32 px | Nei | Trinnvis lysning (Dodge) og mørkning (Burn) basert på fargerekkefølgen i den aktive paletten. |
| **Rektangulær markering** | ⬚ | <kbd>M</kbd> | Ikke relevant | Nei | Rektangulær markeringsramme med animert kantlinje av marsjerende maur. Støtter legg til (<kbd>Shift</kbd>) og trekk fra (<kbd>Alt</kbd>). |
| **Lassomarkering** | ➰ | <kbd>Q</kbd> | Ikke relevant | Nei | Frihåndspolygonlasso. Bruker 8-tilkoblet kantrasterisering med 4-tilkoblet ytre flomfylling, noe som garanterer 100 % massive markeringer uten hull selv ved overlappende løkker. |
| **Tryllestav** | 🪄 | <kbd>W</kbd> | Ikke relevant | Nei | Merker sammenhengende (eller globale) områder med matchende farge og justerbar toleranseterskel. |
| **Dreiepunkt & handlingspunkt** | 🎯 | <kbd>P</kbd> | Ikke relevant | Nei | Interaktiv plassering på lerretet av spritens origo (dreiepunkt) og tilpassede gameplay-handlingspunkter (`weapon`, `head`, `feet`, `hitbox`) som eksporteres til JSON-metadata. |

---

## Funksjonsdokumentasjon

### 1. Dobbelt atlas-sentrert arkitektur

I OniAnvil utgjør hovedteksturen (`atlas_rgba`) den faktiske spritearkstrukturen slik den leses inn av spillmotoren din:
- **Bilderammemodus (Frame Mode)**: Fokuserer på én enkelt aktiv animasjonsramme, noe som gir et distraksjonsfritt pikselarbeidsområde. Endringer oppdaterer umiddelbart den tilhørende cellen i atlasstrukturen.
- **Atlasmodus (Atlas Mode)**: Viser hele spritearkets rutenett for oversikt, justering på tvers av rammer og batch-redigering.
- **Ledsagerfil (Companion File)**: Prosjekter lagres som rene `.png`-spriteark sammen med en lett `.onianvil.json`-metadatafil som inneholder rammeoppsett, laghierarkier, tagger, varigheter og palettskins.

### 2. Flerspors lagtidslinje

Den nederste tidslinjen er utformet etter prinsipper fra profesjonell videoredigeringsprogramvare:
- **Lagspor (`V1, V2, V3...`)**: Lagene stables visuelt ovenfra og ned. Sporet som ligger øverst i tidslinjen (`V1`), tegnes øverst på lerretet.
- **Lagkontroller**:
  - Klikk på `V1` / `V2`-merkene for raskt å bytte aktivt tegnelag.
  - Slå lagsynlighet (øyeikon) og redigeringslås (hengelåsikon) av og på.
  - Juster individuelt lags opasitet (`0%..=100%`).
  - Flytt lag opp/ned i hierarkiet, dupliser lag og slå sammen lag nedover i underliggende spor.
  - Slett lag med en sikkerhetsbekreftelsesdialog.
- **Bilderammer og klipp**:
  - Hold-multiplikator: tildel tilpassede varigheter per ramme (f.eks. `2×`, `3×` for pauser eller effektrammer).
  - Koblede rammer: koble repeterende rammer sammen gjennom animasjonen, slik at en endring i én automatisk oppdaterer alle tilknyttede instanser.
  - Tagg-looping: organiser animasjoner i navngitte tagger (`idle`, `walk`, `attack`) og loop spesifikke tagger under avspilling.
  - Avspillingskontroller: Fremover, Bakover, Ping-pong og Loop med justerbar bildefrekvens (`1..=120` FPS).

### 3. Sanntids direkte forhåndsvisning (Live Preview)

Det høyre forhåndsvisningspanelet spiller av animasjonene dine i sanntid, fullstendig uavhengig av lerretets zoomnivå og panoreringsposisjon:
- **Tagg-filtrering**: Isoler avspillingen til bestemte tagger (`walk`, `jump`) eller forhåndsvis hele animasjonssekvensen.
- **Avspillingsstyring**: Spill av, Pause, Forrige ramme, Neste ramme, Loop-bryter og tilpassede hastigheter (`0.25x..=4.0x`).
- **Visningsmoduser**: Bytt mellom mørkt sjakkbrett, lyst sjakkbrett, ren hvit, ren svart eller tilpassede bakgrunnsfarger.

### 4. Avansert løkbladsmotor (Onion Skinning)

Skap flytende og levende animasjoner med det dedikerte løkbladsystemet:
- **Tilpasset fargetoning**: Uavhengige fargetoner for forrige rammer (standard: rød) og neste rammer (standard: blå).
- **Opasitetsavtagende**: Justerbar basisopasitet og trinnvis dempingskurve for tilstøtende faser.
- **Loop-understøttelse**: Aktiver løkbladsvisning på tvers av første og siste ramme i sykliske animasjoner.
- **Hurtigtitt (Quick-Peek)**: Hold <kbd>Mellomrom</kbd> nede uten å dra med musen for øyeblikkelig å inspisere tilstøtende bilderammer.

### 5. Smart referanse- og skissetegningslag

Gjør arbeidsflyten raskere ved å tegne etter håndtegnede skisser eller referansefotografier:
- **Formater**: Last inn PNG-, JPEG- og WebP-referansebilder.
- **Blandingsmoduser**: Underlag (under piksellagene dine) eller Overlag (over piksellagene dine).
- **Transformeringsstyring**: Uavhengig zoom, posisjonsforskyvning, rotasjon og opasitetsglidebryter.
- **Smart fargeprøvetaking**: Bruk pipetten på skisselaget med valgfri automatisk tilpasning til nærmeste farge i den aktive prosjektpaletten.
- **1-klikks baking (Bake)**: Flat ut det skalerte referansebildet direkte til pikselgrafikk på den aktive rammen med et enkelt klikk.

### 6. Presisjonsmarkeringsverktøy og marsjerende maur (Marching Ants)

- **Animerte marsjerende maur**: Aktive markeringer vises med en animert tofarget stiplet omkrets med høy kontrast.
- **Massiv lasso med flomfylling**: I motsetning til enkle strålesøkingsalgoritmer som skaper uønskede hull ved overlappende løkker, rasteriserer OniAnvils lasso en lukket 8-tilkoblet grense og fyller det ytre rommet, noe som sikrer 100 % massive markeringer uten indre hull.
- **Markeringshandlinger**: Utvid, Forminsk, Ramme (med sirkulær, kvadratisk og rombisk kjerneform), Flytt, Inverter, Slett, Fri rotasjon samt Utklippstavle (Kopier/Klipp ut/Lim inn).
- **Kopier og lim inn på tvers av rammer**: Velg områder på én ramme, trykk <kbd>Ctrl/Cmd+C</kbd>, bytt ramme, og trykk <kbd>Ctrl/Cmd+V</kbd> for å lime inn på nøyaktig samme relative koordinater uavhengig av tastaturoppsett.

### 7. Sanntidseffekter, filtre og RotSprite

Alle bildeeffekter inneholder en avmerkingsboks for **Direkte forhåndsvisning (Live Preview)**, som viser endringer direkte på lerretet og muliggjør sømløs avbryting uten å forsøple angrehistorikken:
- **Erstatt farge (`Shift+R`)**:
  - Interaktiv pipette: klikk på pipetteikonet og velg farger direkte fra lerretets piksler eller palettpanelet.
  - Hurtigvalgstripe: venstreklikk for "Fra"-farge, høyreklikk for "Til"-farge.
  - Toleranseglidebryter og batch-erstatning på tvers av alle animasjonsrammer.
- **Fargejustering (`Ctrl/Cmd+U`)**: Justering av HSV-fargetone, metning, lysstyrke og kontrast i sanntid.
- **Automatisk konturgenerator**: Skap skarpe pikselkonturer rundt figurer (Utvendig, Innvendig, Sentrert) med firkantede eller rombiske hjørnesammenføyninger.
- **3×3-konvolusjonsfiltre**: Uskarphet, Skarphet, Kantgjenkjenning og Relieff (Emboss) med bevaring av alfakanalen.
- **Fri rotasjon (`Ctrl/Cmd+Shift+R`)**: Roter markerte områder eller hele rammer i vilkårlige vinkler ved hjelp av den klassiske **RotSprite**-algoritmen eller Nearest Neighbor.
- **Tilordne til palett**: Dither lerretets farger inn i den aktive paletten med Bayer 2x2, 4x4, 8x8 eller Blue Noise.

### 8. Palettstyring, fargebytte og karakterskins

- **Innebygde paletter**: Over 20 autentiske retropaletter, inkludert Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized og Sweetie 16.
- **Import og eksport**: Støtte for GIMP Palette (`.gpl`), PAL (`.pal`), Hex-paletter (`.hex`) og bildestriper (`.png`).
- **Palettskins**: Definer flere fargevarianter for en enkelt figur (f.eks. Standard, Gullrustning, Skygge, Is). Ved å bytte skin oppdateres alle farger øyeblikkelig på tvers av hele atlasset.
- **Fargeramper (Ramp Generator)**: Beregn automatisk jevne fargeoverganger mellom to valgte nyanser med et justerbart antall mellomtrinn.

### 9. Spritearkoppdeler, 9-Grid UI og eksport til spillmotorer

- **Spritearkoppdeler (Sheet Slicer)**: Autokorrelasjonsbasert grensegjenkjenning deler automatisk rå spriteark i ensartede eller tett avgrensede animasjonsrammer.
- **9-Grid (9-Slice) skalering**: Still inn tilpassede marger (topp, bunn, venstre, høyre) for å forhåndsvise skalerbare UI-bokser og knappepaneler.
- **Ekstrudert PNG-kanteksport**: Ekstruderer kantpikslers farger inn i gjennomsiktige marger, noe som forhindrer teksturfiltreringsstriper og blødning i OpenGL, DirectX, Vulkan og 2D-spillmotorer (som Godot).
- **Animert GIF-eksport**: Generer optimaliserte animerte GIF-filer med egendefinert repetisjonsantall, rammevarigheter og gjennomsiktig bakgrunn.
- **Standard Sprite Sheet JSON**: Eksporterer rammeutsnitt, tagger, varigheter og dreiepunkter som samsvarer med standardformatet for spriteark-JSON i moderne 2D-spillmotorer.

### 10. 35 ergonomiske temaer og fullstendig lokalisering

- **Utvalgte temaer**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT Amber, CRT Green, CRT Cyan, Retrokonsoller (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) og Retromaskiner (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Flerspråklig grensesnitt**: Bytt sømløst mellom språkene i grensesnittet med 100 % paritet over alle tekststrenger og verktøytips.

---

## Tastatursnarveier og kontroller

| Snarvei | Verktøy / Handling | Beskrivelse |
|:---|:---|:---|
| <kbd>B</kbd> | **Blyant** | 1px presisjonstegning med valgfri Pixel-Perfect-modus |
| <kbd>E</kbd> | **Viskelær** | Slett piksler tilbake til gjennomsiktighet |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipette** | Hent farge fra lerretets piksler eller referanseskisse |
| <kbd>G</kbd> | **Malingspann / Fargeovergang** | Bytt mellom malingspann og dithering-fargeovergang |
| <kbd>L</kbd> | **Linje** | Tegn rette linjer med direkte forhåndsvisning og Shift-vinkellås |
| <kbd>C</kbd> | **Kurve** | Kvadratisk Bézier-pikselkurve med 3 klikk |
| <kbd>U</kbd> | **Rektangel** | Geometrisk rektangel og kvadrat med omriss eller fyll |
| <kbd>O</kbd> | **Ellipse** | Geometrisk ellipse og sirkel med omriss eller fyll |
| <kbd>D</kbd> | **Dithering-pensel** | Mal med sjakkbrettmønstrede matriser |
| <kbd>S</kbd> | **Skyggeverktøy** | Trinnvis lysning (Dodge) og mørkning (Burn) langs palettrampen |
| <kbd>M</kbd> | **Rektangulær markering** | Rektangulær områdemarkering med marsjerende maur |
| <kbd>Q</kbd> | **Lassomarkering** | Frihånds massiv polygonmarkering |
| <kbd>W</kbd> | **Tryllestav** | Velg sammenhengende fargeregioner |
| <kbd>P</kbd> | **Dreiepunkter** | Plasser spritens origo og gameplay-handlingspunkter |
| <kbd>X</kbd> | **Bytt farger** | Bytt om på primær- og sekundærfarge |
| <kbd>[</kbd> / <kbd>]</kbd> | **Penselstørrelse** | Forminsk / øk penselstørrelse (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopier** | Kopier merkede piksler til systemets utklippstavle |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Klipp ut** | Klipp ut merkede piksler til utklippstavlen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Lim inn** | Lim inn piksler fra utklippstavlen inn i aktiv ramme |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Slett** | Tøm piksler innenfor den aktive markeringen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Merk alt** | Merk alle piksler i den aktive rammen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Avmerk** | Tøm aktiv markeringsmaske |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Inverter markering** | Inverter aktiv markeringsmaske |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dupliser ramme** | Opprett en kopi av gjeldende ramme i tidslinjen |
| <kbd>Mellomrom</kbd> | **Spill av / Pause** | Start eller sett animasjonsavspilling på pause |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Erstatt farge** | Åpne dialogboksen for fargeerstatning med forhåndsvisning |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Fargejustering** | Åpne dialogboksen for justering av HSV og kontrast |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Fri rotasjon** | Åpne dialogboksen for fri vinkelrotasjon (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Angre** | Angre siste penselstrøk eller verktøyhandling |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Gjør om** | Gjenta tidligere angret handling |
| <kbd>Mellomrom</kbd> + Dra / MMB | **Panorer lerret** | Jevn og uendelig panorering over lerretets koordinater |
| <kbd>Ctrl/Cmd</kbd> + Hjul / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Pikselskarp zoom sentrert rundt musepekeren |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Visningsmodus** | Bytt mellom rammemodus (<kbd>1</kbd>) og atlasmodus (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Tilpass lerret** | Sentrer og tilpass aktiv ramme til arbeidsområdet |
| <kbd>F1</kbd> | **Hjelp** | Åpne hjelpevinduet for hurtigtaster |

---

## Kom i gang

### Frittstående binærfiler

Last ned ferdigkompilerte, frittstående kjørbare filer fra **[GitHub Releases-siden](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Last ned `OniAnvil-windows-x86_64.zip`, pakk ut, og dobbeltklikk på `OniAnvil.exe`.
- **Linux**: Last ned `OniAnvil-linux-x86_64.tar.gz` (portabel) eller installer pakken `onianvil_*_amd64.deb` for Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Last ned `OniAnvil-macos-universal.zip`, pakk ut, og dra `OniAnvil.app` til Programmer-mappen. Den universelle binærfilen kjører med direkte innebygd ytelse på både Apple Silicon (M1/M2/M3/M4) og Intel-Macer.
- **Web**: Kjør direkte i nettleseren uten installasjon: **[Start WebAssembly-appen](https://rbm13s.github.io/OniAnvil/app/?lang=no)**.
  > [!WARNING]
  > WebAssembly-versjonen (nettleser) har status som **Eksperimentell**. På grunn av nettleserens sandkassebegrensninger, WebGL/WebGPU-forskjeller og begrensninger i utklippstavlen kan det oppstå mindre grafiske feil eller ytelsesforskjeller sammenlignet med skrivebordsversjonen. For produksjonsarbeid anbefales de opprinnelige skrivebordsversjonene på det sterkeste.

---

### Bygging fra kildekode

#### Forutsetninger
- **Rust-verktøykjede**: Installer Rust (versjon 1.80+ anbefales) via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux-byggeavhengigheter** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompiler og kjør den opprinnelige skrivebordsversjonen
```bash
# Klon arkivet
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Kjør testpakken
cargo test

# Bygg og kjør den optimaliserte release-binærfilen
cargo run --release
```

#### 2. Kompiler WebAssembly-versjonen (via Trunk)
```bash
# Installer kompileringsmålet wasm32 og byggeverktøyet Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Kjør lokal utviklingsserver med hurtigoppdatering (hot-reload)
trunk serve

# Bygg optimaliserte utgivelsesfiler til docs/app/
./build_wasm.sh        # På Linux/macOS
./build_wasm.ps1       # På Windows PowerShell
```

---

## Filformat og ledsagerarkitektur

OniAnvil unngår proprietære binære formater. Hvert prosjekt består av to rene filer under versjonskontroll:

```
my_character/
├── my_character.png            # Standard PNG-atlas (kan åpnes i enhver spillmotor eller bildefremviser)
└── my_character.onianvil.json   # Lettvekts JSON-metadatafil
```

Filen `.onianvil.json` lagrer:
- Lerretsrutenettoppsett: rammedimensjoner, kolonneantall, totalt antall rammer.
- Laghierarkier: spornavn, opasitetsnivåer, synlighetsflagg, låsestatus.
- Animasjonstagger: navn, start- og sluttrammeindeks, avspillingsretning (Fremover, Bakover, Ping-pong), rammevarigheter.
- Palettdefinisjoner: aktive fargeruter, navngitte skin-varianter, indeks for aktivt skin.
- Referanseskissetilstand: bildesti, skalering, panoreringsforskyvning, rotasjon, opasitet, lagmodus.
- Dreiepunkter: tilpassede handlingspunkter, våpenfester og kollisjonssentre.

---

## Bidrag og anerkjennelser

Bidrag, feilrapporter og forslag til nye funksjoner er hjertelig velkomne!
1. Fork arkivet på GitHub.
2. Opprett en funksjonsgren: `git checkout -b feature/min-nye-funksjon`.
3. Lagre endringene dine: `git commit -am 'Legg til nytt pikselverktøy'`.
4. Skyv til grenen: `git push origin feature/min-nye-funksjon`.
5. Åpne en Pull Request.

### Anerkjennelser
- Startdemospriten **"Cat 2D Pixel Art"** er skapt av **xzany** ([prosjektside på itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Bygget med det fantastiske GUI-biblioteket **[egui](https://github.com/emilk/egui)** og grafikkøkosystemet **[wgpu](https://wgpu.rs/)**.
- Brukergrensesnittikoner fra **[Font Awesome Free](https://fontawesome.com)** av @fontawesome (Ikoner: CC BY 4.0, Fonter: SIL OFL 1.1, Kode: MIT-lisens).

---

## Lisens og ansvarsfraskrivelse

Dette prosjektet er lisensiert under **[MIT-lisensen](../LICENSE)**. Du står fritt til å bruke, endre, distribuere og integrere OniAnvil i både kommersielle og private spillutviklingsprosjekter.

### Fraskrivelse av garanti og ansvarsbegrensning
> [!IMPORTANT]
> **OniAnvil** distribueres **gratis** på **"SOM DEN ER" ("AS IS")** og **"SOM TILGJENGELIG"**-basis, uten garantier eller betingelser av noe slag, verken uttrykte eller underforståtte.
> 
> Ikke under noen omstendigheter skal opphavsrettshaverne, forfatterne eller bidragsyterne holdes ansvarlige for noen direkte, indirekte, tilfeldige, spesielle, eksemplariske eller følgeskader (inkludert, men ikke begrenset til, tap av data, filkorrupsjon, maskinvarefeil, tapt fortjeneste eller driftsavbrudd) som på noen måte skyldes bruken av eller manglende evne til å bruke denne programvaren, selv om det er informert om muligheten for slik skade. Du påtar deg alt ansvar og all risiko ved bruk av dette programmet.

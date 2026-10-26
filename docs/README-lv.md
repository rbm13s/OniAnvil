<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pikseļmākslas un sīpolu mizas animācijas (Onion-Skinning) studija spēļu gataviem tekstūru atlantiem**

[![Būvēšana un laidiens](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Lejupielādes](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderētājs](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Valodas](https://img.shields.io/badge/Valodas-37_Valodas-purple)](#10-35-ergonomiskas-motīvu-tēmas-un-daudzvalodu-lokalizācija)
[![Platforma](https://img.shields.io/badge/Platforma-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#darba-sākšana)
[![Tīmekļa lietotne](https://img.shields.io/badge/Tiešsaistes_lietotne-Izmēģināt_pārlūkā_(Eksperimentāli)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=lv)
[![Licence: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Izmēģiniet OniAnvil pārlūkprogrammā (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=lv)** *(⚠️ Eksperimentāla tīmekļa versija: var saturēt WebGL/WASM ierobežojumus un kļūdas)* • **[🌐 Oficiālā dokumentācijas vietne](https://rbm13s.github.io/OniAnvil/lv.html)**

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
[Eesti](README-et.md) •
**Latviešu** •
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

## Satura rādītājs

- [Ievads](#ievads)
- [Ekrānuzņēmumi un vizuālā ekskursija](#ekrānuzņēmumi-un-vizuālā-ekskursija)
- [Visaptveroša rīku rokasgrāmata](#visaptveroša-rīku-rokasgrāmata)
- [Funkcionālā dokumentācija](#funkcionālā-dokumentācija)
  - [1. Dubultā atlanta centrētā arhitektūra](#1-dubultā-atlanta-centrētā-arhitektūra)
  - [2. Vairākceliņu slāņu laika josla](#2-vairākceliņu-slāņu-laika-josla)
  - [3. Reāllaika animācijas tiešais priekšskatījums](#3-reāllaika-animācijas-tiešais-priekšskatījums)
  - [4. Uzlabots sīpola mizas dzinējs (Onion Skinning)](#4-uzlabots-sīpola-mizas-dzinējs-onion-skinning)
  - [5. Viedais skiču un atsauces attēlu slānis](#5-viedais-skiču-un-atsauces-attēlu-slānis)
  - [6. Precīza atlases kopa un skrejošās skudras](#6-precīza-atlases-kopa-un-skrejošās-skudras)
  - [7. Tiešie efekti, filtri un RotSprite](#7-tiešie-efekti-filtri-un-rotsprite)
  - [8. Paletes pārvaldība, maiņa un tēlu apvalki (Skins)](#8-paletes-pārvaldība-maiņa-un-tēlu-apvalki-skins)
  - [9. Lokšņu griezējs, 9-Grid UI un eksports spēļu dzinējiem](#9-lokšņu-griezējs-9-grid-ui-un-eksports-spēļu-dzinējiem)
  - [10. 35 ergonomiskas motīvu tēmas un daudzvalodu lokalizācija](#10-35-ergonomiskas-motīvu-tēmas-un-daudzvalodu-lokalizācija)
- [Vadība un tastatūras saīsnes](#vadība-un-tastatūras-saīsnes)
- [Darba sākšana](#darba-sākšana)
  - [Gatavie izpildāmie faili](#gatavie-izpildāmie-faili)
  - [Kompilēšana no pirmkoda](#kompilēšana-no-pirmkoda)
- [Failu formāts un pavadošā arhitektūra](#failu-formāts-un-pavadošā-arhitektūra)
- [Līdzdalība un pateicības](#līdzdalība-un-pateicības)
- [Licence un atruna](#licence-un-atruna)

---

## Ievads

**OniAnvil** (*nosaukums veidots no **Oni**on-Skinning (sīpola mizas tehnika) + **Anvil** (lakts spraitu kalšanai)*) ir atvērtā koda augstas veiktspējas pikseļmākslas redaktors un animācijas studija, kas izstrādāta Rust valodā. Tā darbojas ar aparatūras paātrinājumu operētājsistēmās Windows, Linux un macOS, kā arī bez uzstādīšanas tieši mūsdienu tīmekļa pārlūkprogrammās kā WebAssembly lietotne.

Lielākā daļa esošo spraitu redaktoru animāciju uztver kā atsevišķu attēlu kaudzi, spiežot izstrādātājus veikt apnicīgas eksportēšanas, sagriešanas un pārpakošanas darbības. **OniAnvil apgriež šo pieeju kājām gaisā:** spēles spraitu loksnes tekstūru atlass (`atlas_rgba`) ir primārais patiesības avots. Kadru šūnas, slāņu celiņi, animāciju birkas un sadursmju enkura punkti nepārtraukti un bez zudumiem tiek sinhronizēti ar tekstūru atlasu.

### Kāpēc OniAnvil?
- 🚀 **Zibensātra vietējā Rust veiktspēja**: Tūlītēja palaišana zem milisekundes, dzidrs renderējums ar `wgpu` un `egui 0.31` un pilnīga aiztures neesamība, ritinot garas laika joslas.
- 🎬 **Vairākceliņu slāņu laika josla**: Nelineāro video redaktoru iedvesmoti slāņu celiņi (`V1, V2, V3...`) ar kompozīciju no augšas uz leju, slāņu sapludināšanu uz leju, sasaistītiem kadriem un mainīgu kadra aiztures ilgumu.
- 🧅 **Pirmās klases sīpola mizas tehnika (Onion Skinning)**: Daudzkadru caurspīdīgums ar neatkarīgu priekšējā un aizmugurējā virziena tonēšanu, pielāgojamām necaurredzamības līknēm un bezšuvju ciklu atkārtošanos.
- 🎨 **Tūlītēji tēlu palešu apvalki (Skins)**: Viegli izveidojiet dienas/nakts variantus, komandu krāsas vai elementu stilus ar reāllaika atlanta pārkrāsošanu un GPL, PAL un HEX atbalstu.
- 📦 **Tiešais eksports spēļu dzinējiem**: Eksportējiet mērogotus PNG attēlus ar paplašinātiem malu pikseļiem (extrude), lai novērstu tekstūru šuvju artefaktus 2D dzinējos (Godot, Bevy, Raylib, Defold), kopā ar standarta JSON metadatiem un animētiem GIF.

---

## Ekrānuzņēmumi un vizuālā ekskursija

<div align="center">

### 1. Galvenais audekls un studijas darbvieta
*Augstas veiktspējas pikseļu darbvieta ar aparatūras paātrinātu skatu, vairākceliņu laika joslu, reāllaika priekšskatījumu, palešu krāsu lodziņiem un precīzām rīkjoslām.*
<img src="screenshots/main.png" alt="OniAnvil galvenā darbvieta" width="950" />

<br/><br/>

### 2. Vairākceliņu animācijas laika josla
*Nelineāra laika josla ar slāņu rindu kaudzi (`V1, V2...`), sasaistītām kadru instancēm (`🔗`), aiztures reizinātājiem un krāsu kodētām animācijas birkām.*
<img src="screenshots/timeline.png" alt="OniAnvil vairākceliņu laika josla" width="950" />

<br/><br/>

### 3. Precīzie rīki un augšējā darbību josla
*Pilns 15 rīku zīmēšanas komplekts ar standarta un kompakto dokošanu, kā arī augšējā ātro iestatījumu josla otiņas izmēram, simetrijai, ciklam un Pixel-Perfect režīmam.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Darbību rīkjosla" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Standarta rīku josla" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Kompaktā rīku josla" width="300" />
</p>

<br/><br/>

### 4. Reāllaika animācijas tiešais priekšskatījums
*Atsevišķs priekšskatījuma doks ar birku filtrēšanu, mainīgu FPS ātruma kontroli, tālummaiņas slīdni un fona pārslēgšanu (šaha galdiņš, tumšs, gaišs).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Tiešais priekšskatījums" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Priekšskatījuma tālummaiņa" width="460" />
</p>

<br/><br/>

### 5. Palešu pārvaldība, maiņa un krāsu pārejas
*Visaptverošs krāsu dzinējs ar priekšplāna/fona lodziņiem, automātisku divu punktu krāsu pāreju veidošanu un nosauktiem tēlu palešu apvalkiem.*
<img src="screenshots/palette_and_swaps.png" alt="Palete un apvalku maiņa" width="950" />

<br/><br/>

### 6. Skiču atsauces zīmēšana un konfigurējama sīpola miza
*Ielādējiet koncepciju skices vai fotoattēlus kā pamata vai virsējo slāni ar transformācijām, viedo pipeti ar piesaisti paletei un daudzkadru caurspīdīgumu.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Skice un sīpola miza" width="950" />

<br/><br/>

### 7. Enkura punkti (Pivot) un darbības punkti (Action Points)
*Nosakiet spraita centru un pielāgotus spēles darbības punktus (ieroču enkuri, šāviena vietas, galva, soļi) ar interaktīvām audekla vadīklām.*
<img src="screenshots/pivot_and_action_points.png" alt="Enkura un darbības punkti" width="950" />

<br/><br/>

### 8. Spraitu loksnes griezējs un 9-Grid (9-Slice) mērogotājs
*Automātiska spraitu robežu autokorelācijas noteikšana (piem., ejošs kaķis) un 9 daļu mērogojamu lietotāja saskarnes kārbu apmaļu konfigurēšana.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Loksnes griezējs" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="9-Grid mērogotājs" width="460" />
</p>

<br/><br/>

### 9. Reāllaika attēlu efekti un filtri (Live Preview)
*Visos efektu dialogos ir pieejams reāllaika audekla priekšskatījums, interaktīvas pipetes un droša atcelšana, nesabojājot atcelšanas vēsturi.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Kontūru ģenerators" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Krāsas aizstāšana" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Tonis, piesātinājums un kontrasts" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matricas filtrs" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Piesaiste paletei ar pustoņiem" width="460" />
</p>

<br/><br/>

### 10. Audekla režģis un fona režīmi
*Pārslēdzieties starp pikseļu režģi, kadru robežām, tumšu šaha galdiņu, gaišu šaha galdiņu un pielāgotu vienkrāsainu fonu.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Režģis un tumšs fons" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Gaišs fons bez režģa" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Pielāgots vienkrāsains fons" width="300" />
</p>

<br/><br/>

### 11. Jauna projekta izveides vednis
*Iestatiet audekla izmērus, kadru skaitu, kolonnu izkārtojumu un izvēlieties no vairāk nekā 20 autentiskām retro paletēm projekta sākumā.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Jauna projekta vednis" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Projekta paletes" width="460" />
</p>

<br/><br/>

### 12. 35 autentiskas retro un modernas tēmas
*Ietver Studio Dark, Studio Light, CRT fosforus (Amber, Green, Cyan), retro konsoles (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) un dizaina tēmas.*
<p align="center">
  <img src="screenshots/themes.png" alt="Tēmu izvēlne" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber tēma" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX tēma" width="300" />
</p>

<br/><br/>

### 13. Izvēļņu sistēma un iebūvētā tastatūras palīdzība
<p align="center">
  <img src="screenshots/file_menu.png" alt="Izvēlne Fails" width="180" />
  <img src="screenshots/edit_menu.png" alt="Izvēlne Labot" width="180" />
  <img src="screenshots/select_menu.png" alt="Izvēlne Atlasīt" width="180" />
  <img src="screenshots/view_menu.png" alt="Izvēlne Skats" width="180" />
  <img src="screenshots/project_menu.png" alt="Izvēlne Projekts" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Tastatūras saīšņu palīdzības logs" width="700" />
</p>

</div>

---

## Visaptveroša rīku rokasgrāmata

OniAnvil ietver 15 profesionālus zīmēšanas, ģeometrijas, atlases un metadatu rīkus. Ikonas izmanto atvērtā koda **Font Awesome Free** ikonu bibliotēku *(Ikonas: CC BY 4.0, Fonti: SIL OFL 1.1)*.

| Rīks | Ikona | Īsinājumtaustiņš | Otiņas izmērs | Pixel Perfect | Apraksts |
|:---|:---:|:---:|:---:|:---:|:---|
| **Zīmulis** | ✏️ | <kbd>B</kbd> | 1..32 px | Jā | Pamata zīmēšanas rīks. 1 px režīmā atbalsta **Pixel-Perfect** zīmēšanu (automātiski novērš kāpņveida L veida stūrus). Atbalsta pielāgotas zīmogu otiņas no atlases (`Ctrl/Cmd+B`). |
| **Dzēšgumija** | 🧹 | <kbd>E</kbd> | 1..32 px | Nē | Dzēš pikseļus līdz pilnīgam caurspīdīgumam (`[0, 0, 0, 0]`). Aktīvas atlases gadījumā ierobežota ar atlases masku. |
| **Pipete** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nē | Kreisais klikšķis paņem krāsu priekšplānā (Primary); labais klikšķis fonā (Secondary). Paņem krāsu no audekla, slāņiem vai skices. |
| **Aizpildījums** | 🪣 | <kbd>G</kbd> | — | Nē | Ātrs rindu rindas (BFS) aizpildījums ar regulējamu krāsu toleranci (`0..=255`). Atbalsta blakus esošo un globālo aizpildījumu un atlases ierobežojumu. |
| **Gradients** | 🌈 | <kbd>G</kbd> *(pārsl.)* | — | Nē | Lineārie un radiālie gradienti starp priekšplāna un fona krāsām. Atbalsta Baiera (2x2, 4x4, 8x8) un Blue Noise pustoņus ar stipruma kontroli. |
| **Līnija** | 📏 | <kbd>L</kbd> | 1..32 px | Jā | Brēzenhema taisno līniju algoritms ar reāllaika priekšskatījumu. <kbd>Shift</kbd> fiksē leņķus ar 45°/90° soli. |
| **Līkne** | 〰️ | <kbd>C</kbd> | 1..32 px | Jā | 3 klikšķu kvadrātiskā Bezjē līkne. 1. klikšķis iestata sākuma punktu, 2. klikšķis beigu punktu, peles kustība izliec līkumu, 3. klikšķis apstiprina līkni. |
| **Taisnstūris** | ⬜ | <kbd>U</kbd> | 1..32 px | Nē | Kontūras vai aizpildīti ģeometriskie taisnstūri. <kbd>Shift</kbd> fiksē proporcijas 1:1 kvadrātā. |
| **Elipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Nē | Kontūras vai aizpildītas elipses ar viduspunkta rasterizāciju. <kbd>Shift</kbd> fiksē perfektu apli. |
| **Pustoņu otiņa** | 🏁 | <kbd>D</kbd> | 1..32 px | Nē | Krāso ar šaha galdiņa pustoņu rakstiem starp priekšplāna un fona krāsām, nepievienojot liekas krāsas paletē. |
| **Ēnošanas rīks** | 🌓 | <kbd>S</kbd> | 1..32 px | Nē | Pakāpeniska gaišināšana (Dodge) un tumšināšana (Burn) pēc aktīvās paletes krāsu secības. |
| **Taisnstūra atlase** | ⬚ | <kbd>M</kbd> | — | Nē | Taisnstūra laukuma atlase ar animētu skrejošo skudru robežu. Pievienošana (<kbd>Shift</kbd>) un atņemšana (<kbd>Alt</kbd>). |
| **Lazo atlase** | ➰ | <kbd>Q</kbd> | — | Nē | Brīvroku daudzstūra lazo. Izmanto 8 savienojumu robežas rasterizāciju un 4 savienojumu ārējo aizpildījumu, garantējot 100% blīvu, bezcaurumu atlasi pat uz cilpām. |
| **Burvju nūjiņa** | 🪄 | <kbd>W</kbd> | — | Nē | Atlasa blakus esošos (vai visus) vienādas krāsas apgabalus ar regulējamu tolerances slieksni. |
| **Enkura un darbības punkti** | 🎯 | <kbd>P</kbd> | — | Nē | Spraita centra un pielāgotu spēles darbības punktu (`weapon`, `head`, `feet`, `hitbox`) interaktīva novietošana uz audekla un eksports JSON metadatos. |

---

## Funkcionālā dokumentācija

### 1. Dubultā atlanta centrētā arhitektūra

OniAnvil galvenais tekstūru atlass (`atlas_rgba`) reprezentē spraitu loksnes patieso tekstūru tieši tā, kā to ielādē spēles dzinējs:
- **Kadra režīms (Frame Mode)**: Koncentrējas uz vienu aktīvo animācijas kadru, nodrošinot ērtu pikseļu audeklu. Labojumi uzreiz atjaunina atbilstošo šūnu tekstūru atlasā.
- **Atlanta režīms (Atlas Mode)**: Parāda visu spraitu loksnes režģi, ļaujot veikt makro līmeņa korekcijas, kadru izlīdzināšanas pārbaudes un pakešapstrādi.
- **Pavadošais fails (Companion File)**: Projekti tiek saglabāti kā tīrs `.png` spraitu loksnes attēls kopā ar vieglu `.onianvil.json` pavadošo failu, kurā ir kadru izkārtojums, slāņu hierarhija, birkas, aiztures reizinātāji un palešu apvalki.

### 2. Vairākceliņu slāņu laika josla

Apakšējā laika josla veidota pēc profesionālu nelineāro video redaktoru principiem:
- **Slāņu celiņi (`V1, V2, V3...`)**: Slāņi izkārtoti vizuāli no augšas uz leju. Celiņš, kas atrodas augstāk laika joslā (`V1`), kadra audeklā zīmējas virs zemāk esošajiem.
- **Celiņu vadīklas**:
  - Noklikšķiniet uz `V1` / `V2` nozīmītēm, lai ātri pārslēgtu aktīvo zīmēšanas slāni.
  - Pārslēdziet slāņa redzamību (acs ikona) un bloķēšanu pret labojumiem (piekaramā slēdzene).
  - Pielāgojiet atsevišķu slāņu necaurredzamību (`0%..=100%`).
  - Pārvietojiet slāni augšup/lejup hierarhijā, dublējiet un sapludiniet slāni uz leju ar zemāko celiņu (Merge Down).
  - Droša slāņa dzēšana ar apstiprinājuma dialogu.
- **Kadru klipi**:
  - Aiztures reizinātājs: piešķiriet pielāgotu kadru ilgumu (piem., `2×`, `3×` aizture pauzēm vai sitiena kadriem).
  - Sasaistītie kadri (`🔗`): sasaistiet atkārtotus kadrus visā animācijā, lai viena kadra labošana automātiski atjauninātu visus pārējos.
  - Birku cikli: sakārtojiet animācijas nosauktās birkās (`idle`, `walk`, `attack`) un atskaņojiet konkrētas birkas ciklā.
  - Atskaņošanas vadīklas: Uz priekšu, Atpakaļ, Ping-Pong un Nepārtraukts cikls ar regulējamu ātrumu no `1` līdz `120` FPS.

### 3. Reāllaika animācijas tiešais priekšskatījums

Labās puses priekšskatījuma doks atskaņo animācijas reāllaikā, pilnīgi neatkarīgi no audekla tālummaiņas vai pārbīdes stāvokļa:
- **Birku filtrēšana**: Izolējiet atskaņošanu konkrētām birkām (`walk`, `jump`) vai skatiet visu animācijas secību.
- **Atskaņošanas vadīklas**: Atskaņot, Pauze, Iepriekšējais kadrs, Nākamais kadrs, Cikla poga un pielāgots atskaņošanas ātrums (`0.25x..=4.0x`).
- **Rādīšanas režīmi**: Pārslēdzieties starp tumšu šaha galdiņu, gaišu šaha galdiņu, tīri baltu, tīri melnu vai pielāgotu audekla fonu.

### 4. Uzlabots sīpola mizas dzinējs (Onion Skinning)

Veidojiet plūstošas animācijas ar īpašu sīpola mizas sistēmu:
- **Pielāgota tonēšana**: Neatkarīgas toņu krāsas iepriekšējiem kadriem (noklusējums: sarkans) un nākamajiem kadriem (noklusējums: zils).
- **Necaurredzamības rimšana**: Konfigurējama bāzes necaurredzamība un pakāpeniska rimšanas līkne katram nākamajam solim.
- **Cikla savienošana**: Iespējojiet caurspīdīgumu starp pirmo un pēdējo kadru bezšuvju cilpu radīšanai.
- **Ātrais ieskats (Quick-Peek)**: Turiet nospiestu <kbd>Atstarpes taustiņu</kbd> bez vilkšanas, lai tūlītēji redzētu blakus esošos kadrus.

### 5. Viedais skiču un atsauces attēlu slānis

Paātriniet darbu, zīmējot pāri ar roku zīmētām skicēm vai atsauces fotogrāfijām:
- **Formāti**: Ielādējiet PNG, JPEG un WebP atsauces attēlus.
- **Sajaukšanas režīmi**: Apakšējais slānis (zem pikseļu slāņiem) vai Virsējais slānis (virs pikseļu slāņiem).
- **Transformācijas**: Neatkarīgs mērogs, pozīcijas nobīde, pagriešana un necaurredzamības slīdnis.
- **Viedā krāsu ņemšana**: Izmantojiet pipeti uz skices slāņa ar automātisku piesaisti tuvākajai krāsai aktīvā projekta paletē.
- **1 klikšķa iekaldināšana (Bake)**: Pārveidojiet mērogoto atsauces attēlu tīrā pikseļmākslā aktīvajā kadrā ar vienu klikšķi.

### 6. Precīza atlases kopa un skrejošās skudras

- **Animētas skrejošās skudras**: Aktīvās atlases rāda animētu, skaidri pamanāmu divtoņu pārtrauktās līnijas perimetru.
- **Blīvs daudzstūru lazo**: Atšķirībā no vienkāršotiem stariem, kas rada caurumus uz pārklājošām cilpām, OniAnvil lazo rasterizē slēgtu 8 savienojumu robežu un aizpilda ārpusi, garantējot 100% blīvu, bezcaurumu atlasi.
- **Atlases operācijas**: Paplašināt, Sašaurināt, Apmale (ar apaļu, kvadrātveida un rombveida kodolu), Pārvietot, Invertēt, Dzēst, Brīvā pagriešana un sistēmas starpliktuves Kopēt/Izgriezt/Ielīmēt.
- **Starpkadru kopēšana un ielīmēšana**: Atlasiet apgabalu vienā kadrā, nospiediet <kbd>Ctrl/Cmd+C</kbd>, pārslēdziet kadrus un nospiediet <kbd>Ctrl/Cmd+V</kbd>, lai ielīmētu tieši tajās pašās koordinātās jebkurā tastatūras izkārtojumā.

### 7. Tiešie efekti, filtri un RotSprite

Visiem attēlu efektiem ir reāllaika **Live Preview** izvēles rūtiņa, kas parāda rezultātu tieši uz audekla un ļauj atcelt darbību bez vēstures piesārņošanas:
- **Krāsas aizstāšana (`Shift+R`)**:
  - Interaktīvā pipete: noklikšķiniet uz pipetes ikonas un izvēlieties krāsas tieši no audekla pikseļiem vai paletes joslas.
  - Ātrā paletes josla: kreisais klikšķis izvēlas krāsu "No", labais klikšķis krāsu "Uz".
  - Tolerances slīdnis un pakešveida aizstāšana vienlaikus visos animācijas kadros.
- **Krāsu regulēšana (`Ctrl/Cmd+U`)**: Reāllaika HSV toņa maiņa, piesātinājums, spilgtums un kontrasts.
- **Automātiskais kontūru ģenerators**: Izveidojiet asu tēla pikseļu kontūru (Ārpusē, Iekšpusē, Centrēti) ar kvadrātveida vai rombveida stūriem.
- **3×3 matricas filtri**: Izplūdināšana, Asināšana, Malu noteikšana un Reljefs ar alfa kanāla saglabāšanu.
- **Brīvā pagriešana (`Ctrl/Cmd+Shift+R`)**: Pagrieziet apgabalus vai pilnus kadrus jebkurā leņķī ar klasisko **RotSprite** vai tuvākā kaimiņa algoritmu.
- **Piesaiste paletei**: Kvantizējiet audekla krāsas aktīvajā paletē ar Baiera 2x2, 4x4, 8x8 vai Blue Noise pustoņiem.

### 8. Paletes pārvaldība, maiņa un tēlu apvalki (Skins)

- **Iebūvētās paletes**: 20+ autentiskas retro paletes, tostarp Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized un Sweetie 16.
- **Imports un eksports**: GIMP palešu (`.gpl`), PAL (`.pal`), HEX (`.hex`) un attēlu joslu (`.png`) atbalsts.
- **Palešu maiņa (Apvalki)**: Definējiet vairākus tēla krāsu apvalkus (piemēram, Noklusējums, Zelta bruņas, Ēna, Ledus). Apvalka maiņa acumirklī pārkrāso visu kadru pikseļus visā atlasā.
- **Krāsu pāreju ģenerators**: Automātiski interpolējiet vienmērīgas krāsu pārejas starp diviem toņiem ar pielāgojamu soļu skaitu.

### 9. Lokšņu griezējs, 9-Grid UI un eksports spēļu dzinējiem

- **Lokšņu griezējs (Sheet Slicer)**: Autokorelācijas robežu noteikšana automātiski sagriež neapstrādātas spraitu loksnes vienmērīgos vai precīzi ierobežotos animācijas kadros.
- **9-Grid (9-Slice) mērogošana**: Iestatiet pielāgojamas apmales (augšā, apakšā, pa kreisi, pa labi), lai priekšskatītu mērogojamus saskarnes lodziņus un pogas bez stūru deformācijas.
- **PNG eksports ar malu pikseļu ekstrūziju (Extruded Border)**: Pavairo malu pikseļus caurspīdīgajās malās, novēršot tekstūru filtrēšanas šuves un krāsu noplūdi OpenGL, DirectX, Vulkan un 2D spēļu dzinējos (piemēram, Godot).
- **Animēto GIF eksports**: Veidojiet optimizētus animētus GIF failus ar pielāgotu ciklu skaitu, kadru aizturēm un caurspīdīgu fonu.
- **Standarta Sprite Sheet JSON**: Eksportē kadru koordinātas, birkas, ilgumus un enkura punktus atbilstoši mūsdienu 2D spēļu dzinējos izmantotajam standartam.

### 10. 35 ergonomiskas motīvu tēmas un daudzvalodu lokalizācija

- **Izvēlētās tēmas**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT fosfori (Amber, Green, Cyan), retro konsoles (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) un retro datori (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Daudzvalodu saskarne**: Ērti pārslēdziet valodu lidojumā ar 100% vārdnīcas atbilstību 37 valodās.

---

## Vadība un tastatūras saīsnes

| Saīsne | Rīks / Darbība | Apraksts |
|:---|:---|:---|
| <kbd>B</kbd> | **Zīmulis** | 1 px precīza zīmēšana ar izvēles Pixel-Perfect režīmu |
| <kbd>E</kbd> | **Dzēšgumija** | Dzēš pikseļus līdz caurspīdīgumam |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipete** | Paņem krāsu no audekla pikseļiem vai skices |
| <kbd>G</kbd> | **Aizpildījums / Gradients** | Pārslēdz starp aizpildījumu un pustoņu gradientu |
| <kbd>L</kbd> | **Līnija** | Zīmē taisnas līnijas ar priekšskatījumu un Shift leņķa piesaisti |
| <kbd>C</kbd> | **Līkne** | 3 klikšķu kvadrātiskās Bezjē līknes rīks |
| <kbd>U</kbd> | **Taisnstūris** | Ģeometrisks taisnstūris un kvadrāts (kontūra vai aizpildīts) |
| <kbd>O</kbd> | **Elipse** | Ģeometriska elipse un aplis (kontūra vai aizpildīts) |
| <kbd>D</kbd> | **Pustoņu otiņa** | Krāso ar šaha galdiņa pustoņu matricas rakstiem |
| <kbd>S</kbd> | **Ēnošanas rīks** | Pakāpeniska gaišināšana / tumšināšana pa paleti |
| <kbd>M</kbd> | **Taisnstūra atlase** | Taisnstūra laukuma atlase ar skrejošām skudrām |
| <kbd>Q</kbd> | **Lazo atlase** | Brīvroku blīva daudzstūra atlase |
| <kbd>W</kbd> | **Burvju nūjiņa** | Atlasa blakus esošos vienādas krāsas apgabalus |
| <kbd>P</kbd> | **Enkura punkti** | Novieto spraita centru un darbības punktus |
| <kbd>X</kbd> | **Apmainīt krāsas** | Apmaina priekšplāna un fona krāsas vietām |
| <kbd>[</kbd> / <kbd>]</kbd> | **Otiņas izmērs** | Samazināt / Palielināt otiņas izmēru (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopēt** | Kopēt atlasītos pikseļus starpliktuvē |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Izgriezt** | Izgriezt atlasītos pikseļus starpliktuvē |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Ielīmēt** | Ielīmēt starpliktuves pikseļus aktīvajā kadrā |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Dzēst** | Notīrīt pikseļus aktīvajā atlasē |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Atlasīt visu** | Atlasīt visus aktīvā kadra pikseļus |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Noņemt atlasi** | Notīrīt aktīvās atlases masku |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invertēt atlasi** | Invertēt aktīvās atlases masku |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dublēt kadru** | Izveidot aktīvā kadra kopiju laika joslā |
| <kbd>Atstarpe</kbd> | **Atskaņot / Pauze** | Palaist vai apturēt animāciju laika joslā |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Aizstāt krāsu** | Atvērt krāsas aizstāšanas logu ar priekšskatījumu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Regulēt krāsu** | Atvērt HSV toņa, piesātinājuma un kontrasta logu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Brīvā pagriešana** | Atvērt patvaļīga leņķa pagriešanas logu (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Atsaukt** | Atsaukt pēdējo triepienu vai rīka darbību |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Atkārtot** | Atkārtot iepriekš atsaukto darbību |
| <kbd>Atstarpe</kbd> + Vilkšana / Peles vidējā poga | **Pārbīdīt audeklu** | Vienmērīga bezgalīga audekla koordinātu pārvietošana |
| <kbd>Ctrl/Cmd</kbd> + Ritenītis / <kbd>+</kbd> / <kbd>-</kbd> | **Tālummaiņa** | Pikseļu tālummaiņa, centrējot ap peles kursoru |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Skata režīms** | Pārslēgt starp kadra skatu (<kbd>1</kbd>) un atlanta skatu (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Ietilpināt audeklu** | Automātiski centrēt un ietilpināt aktīvo kadru |
| <kbd>F1</kbd> | **Palīdzība** | Atvērt tastatūras saīšņu palīdzības logu |

---

## Darba sākšana

### Gatavie izpildāmie faili

Lejupielādējiet iepriekš kompilētus, autonamus izpildāmos failus no **[GitHub laidienu lapas](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Lejupielādējiet `OniAnvil-windows-x86_64.zip`, atpakojiet un veiciet dubultklikšķi uz `OniAnvil.exe`.
- **Linux**: Lejupielādējiet `OniAnvil-linux-x86_64.tar.gz` (portatīvs) vai uzstādiet `onianvil_*_amd64.deb` Debian/Ubuntu pakotni:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Lejupielādējiet `OniAnvil-macos-universal.zip`, atpakojiet un velciet `OniAnvil.app` uz mapi Applications. Universālais binārais fails darbojas gan Apple Silicon (M1/M2/M3/M4), gan Intel Mac datoros.
- **Tīmekļa versija**: Palaidiet tieši savā pārlūkā bez uzstādīšanas: **[Palaist WebAssembly lietotni](https://rbm13s.github.io/OniAnvil/app/?lang=lv)**.
  > [!WARNING]
  > WebAssembly (pārlūka) laidienam ir **eksperimentāls (Experimental)** statuss. Pārlūkprogrammas smilškastes ierobežojumu, WebGL/WebGPU atšķirību un starpliktuves ierobežojumu dēļ var rasties nelieli grafiski defekti vai veiktspējas atšķirības, salīdzinot ar darbvirsmas versiju. Ražošanas darbam ļoti ieteicams izmantot vietējās darbvirsmas versijas.

---

### Kompilēšana no pirmkoda

#### Priekšnosacījumi
- **Rust rīku kopa**: Uzstādiet Rust (ieteicama versija 1.80+) no [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux būvēšanas atkarības** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Vietējās darbvirsmas versijas kompilēšana un palaišana
```bash
# Klonējiet krātuvi
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Palaidiet testus
cargo test

# Būvējiet un palaidiet optimizēto laidienu
cargo run --release
```

#### 2. WebAssembly versijas kompilēšana (ar Trunk)
```bash
# Pievienojiet wasm32 mērķi un uzstādiet Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Palaidiet vietējo izstrādes serveri ar karsto pārlādi
trunk serve

# Izveidojiet optimizētos laidiena failus mapē docs/app/
./build_wasm.sh        # Linux / macOS
./build_wasm.ps1       # Windows PowerShell
```

---

## Failu formāts un pavadošā arhitektūra

OniAnvil izvairās no slēgtiem bināriem formātiem. Katrs projekts sastāv no diviem tīriem, versiju kontrolei ideāli piemērotiem failiem:

```
my_character/
├── my_character.png            # Standarta PNG spraitu loksnes atlass (atverams jebkurā dzinējā vai skatītājā)
└── my_character.onianvil.json   # Pavadošais projekta metadatu fails
```

Fails `.onianvil.json` glabā:
- Audekla režģa iestatījumus: kadra izmērus, kolonnu skaitu, kadru kopskaitu.
- Slāņu hierarhijas: celiņu nosaukumus, necaurredzamības līmeņus, redzamības karodziņus, bloķētos stāvokļus.
- Animācijas birkas: nosaukumu, sākuma/beigu kadru indeksus, atskaņošanas virzienu (Uz priekšu, Atpakaļ, Ping-Pong), kadru ilgumus.
- Palešu definīcijas: aktīvos krāsu lodziņus, nosauktos apvalku variantus (skins), aktīvā apvalka indeksu.
- Skices slāņa stāvokli: attēla ceļu, mērogu, nobīdi, pagriešanu, necaurredzamību, slāņa režīmu.
- Enkura punktus: pielāgotos darbības punktus, ieroču enkurus un sadursmju centrus.

---

## Līdzdalība un pateicības

Visi ieteikumi, kļūdu ziņojumi un jaunu iespēju priekšlikumi ir laipni gaidīti!
1. Izveidojiet krātuves atzarojumu (Fork) vietnē GitHub.
2. Izveidojiet jaunu funkcijas zaru: `git checkout -b feature/mans-jaunais-riks`.
3. Saglabājiet izmaiņas: `git commit -am 'Pievienots jauns pikseļu rīks'`.
4. Nosūtiet zaru uz serveri: `git push origin feature/mans-jaunais-riks`.
5. Atveriet Pull Request.

### Pateicības
- Sākotnējais demonstrācijas spraits **"Cat 2D Pixel Art"** — autors **xzany** ([itch.io projekta lapa](https://xzany.itch.io/cat-2d-pixel-art)).
- Būvēts, izmantojot lielisko **[egui](https://github.com/emilk/egui)** grafiskās saskarnes bibliotēku un **[wgpu](https://wgpu.rs/)** grafikas ekosistēmu.
- Lietotāja saskarnes ikonas nodrošina **[Font Awesome Free](https://fontawesome.com)** (@fontawesome) (Ikonas: CC BY 4.0, Fonti: SIL OFL 1.1, Kods: MIT licence).

---

## Licence un atruna

Šis projekts ir licencēts saskaņā ar **[MIT licenci](LICENSE)**. Jūs varat brīvi izmantot, modificēt, izplatīt un integrēt OniAnvil komerciālos un personīgos spēļu izstrādes projektos.

### Garantijas atruna un atbildības ierobežojums
> [!IMPORTANT]
> **OniAnvil** tiek izplatīts **bez maksas** pēc principa **"KĀ IR" ("AS IS")** un **"KĀ PIEEJAMS" ("AS AVAILABLE")**, bez jebkāda veida tiešām vai netiešām garantijām vai nosacījumiem.
> 
> Autors(-i), autortiesību īpašnieki vai līdzstrādnieki nekādā gadījumā nav atbildīgi par jebkādiem tiešiem, netiešiem, nejaušiem, īpašiem, brīdinošiem vai izrietošiem zaudējumiem (tostarp, bet ne tikai, datu zudumu, failu bojājumiem, aparatūras kļūmēm, negūtu peļņu vai uzņēmējdarbības pārtraukumu), kas jebkādā veidā radušies no šīs programmatūras lietošanas vai nespējas to lietot, pat ja ir brīdināts par šāda kaitējuma iespējamību. Jūs uzņematies visu atbildību un risku par šīs programmas izmantošanu.

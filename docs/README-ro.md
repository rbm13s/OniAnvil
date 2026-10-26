<div align="center">

<img src="assets/icon.jpg" alt="Sigla OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio de animație pixel art și onion skinning bazat pe atlase gata de joc**

[![Compilare și Lansare](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Descărcări](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Randare](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Limbi](https://img.shields.io/badge/Limbi-37_Localizări-purple)](#10-35-de-teme-ergonomice-și-localizare-multilingvă)
[![Platformă](https://img.shields.io/badge/Platformă-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#ghid-introductiv)
[![Aplicație Online](https://img.shields.io/badge/Aplicație_Online-Încearcă_în_Browser_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ro)
[![Licență: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Încearcă OniAnvil în browser (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ro)** *(⚠️ Versiune experimentală: poate conține limitări grafice și erori WebGL/WASM)* • **[🌐 Site-ul oficial de documentație](https://rbm13s.github.io/OniAnvil/ro.html)**

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
[Latviešu](README-lv.md) •
[Lietuvių](README-lt.md) •
[Magyar](README-hu.md) •
**Română** •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Cuprins

- [Introducere](#introducere)
- [Capturi de ecran și tur vizual](#capturi-de-ecran-și-tur-vizual)
- [Ghid complet al instrumentelor](#ghid-complet-al-instrumentelor)
- [Documentație funcțională](#documentație-funcțională)
  - [1. Arhitectură unificată centrată pe atlas](#1-arhitectură-unificată-centrată-pe-atlas)
  - [2. Cronologie (Timeline) de straturi multi-pistă](#2-cronologie-timeline-de-straturi-multi-pistă)
  - [3. Fereastră de previzualizare a animației în timp real](#3-fereastră-de-previzualizare-a-animației-în-timp-real)
  - [4. Motor avansat de Onion Skinning](#4-motor-avansat-de-onion-skinning)
  - [5. Strat de schiță de referință și trasare inteligentă](#5-strat-de-schiță-de-referință-și-trasare-inteligentă)
  - [6. Suită de selecție de precizie și furnici în marș](#6-suită-de-selecție-de-precizie-și-furnici-în-marș)
  - [7. Efecte în timp real, filtre și RotSprite](#7-efecte-în-timp-real-filtre-și-rotsprite)
  - [8. Gestionarea paletelor, schimbarea culorilor și variante de skin-uri](#8-gestionarea-paletelor-schimbarea-culorilor-și-variante-de-skin-uri)
  - [9. Tăietor de foi sprite (Sheet Slicer), interfață 9-Grid și export pentru motoare de jocuri](#9-tăietor-de-foi-sprite-sheet-slicer-interfață-9-grid-și-export-pentru-motoare-de-jocuri)
  - [10. 35 de teme ergonomice și localizare multilingvă](#10-35-de-teme-ergonomice-și-localizare-multilingvă)
  - [Comenzi și comenzi rapide de la tastatură](#comenzi-și-comenzi-rapide-de-la-tastatură)
- [Ghid introductiv](#ghid-introductiv)
  - [Fișiere binare de sine stătătoare](#fișiere-binare-de-sine-stătătoare)
  - [Compilare din codul sursă](#compilare-din-codul-sursă)
- [Formatul fișierelor și arhitectura de companie](#formatul-fișierelor-și-arhitectura-de-companie)
- [Contribuții și mulțumiri](#contribuții-și-mulțumiri)
- [Licență și declinare a responsabilității](#licență-și-declinare-a-responsabilității)

---

## Introducere

**OniAnvil** (*nume format prin alăturarea termenilor **Oni**on-Skinning (animație cu foi de calc/ceapă) + **Anvil** (nicovală pentru făurirea sprite-urilor)*) este un editor de pixel art și suită de animație open-source de înaltă performanță, scris în limbajul Rust. Rulează nativ cu accelerare hardware pe Windows, Linux și macOS, precum și ca aplicație WebAssembly fără instalare, direct în browserele web moderne.

Majoritatea programelor clasice de editare tratează animația ca pe un set de imagini izolate, impunând pași obositori de export, tăiere și recombinare manuală în foi de textură. **OniAnvil inversează această paradigmă:** textura principală a atlasului de sprite-uri (`atlas_rgba`) reprezintă sursa primară și unică de adevăr. Celulele cadrelor, pistele de straturi, etichetele de animație și punctele de coliziune sunt permanent sincronizate cu atlasul într-un mod complet nedistructiv.

### De ce OniAnvil?
- 🚀 **Viteză nativă spectaculoasă în Rust**: pornire instantanee sub o milisecundă, randare precisă susținută de `wgpu` și `egui 0.31`, navigare fluidă pe cronologii lungi fără nicio întârziere.
- 🎬 **Cronologie de straturi multi-pistă**: piste de straturi inspirate de aplicațiile profesionale de montaj video (`V1, V2, V3...`) cu compunere de sus în jos, îmbinare în jos, cadre asociate și multiplicatori de durată (hold).
- 🧅 **Onion Skinning de primă clasă**: suprapunere fantomatică a mai multor cadre cu nuanțare cromatică independentă pentru cadrele anterioare/posterioare, curbe de atenuare a opacității și bucle continue fără cusur.
- 🎨 **Variante de culoare (Skin-uri) instantanee pentru personaje**: creați versiuni zi/noapte, culori de echipă sau efecte elementale prin remapare instantanee direct în atlas, cu import și export pentru fișiere GPL, PAL și HEX.
- 📦 **Export direct pentru motoare de jocuri**: exportați imagini PNG scalate cu extrudarea marginilor pentru a elimina artefactele de filtrare a texturilor în motoarele 2D (Godot, Bevy, Raylib, Defold), alături de metadate standard Sprite Sheet JSON și GIF-uri animate.

---

## Capturi de ecran și tur vizual

<div align="center">

### 1. Spațiul principal de lucru și pânza studioului
*Spațiu de lucru dedicat pixel art-ului de înaltă performanță cu viewport accelerat hardware, cronologie multi-pistă, previzualizare live, mostre de paletă și bare de instrumente de precizie.*
<img src="screenshots/main.png" alt="Spațiul principal de lucru OniAnvil" width="950" />

<br/><br/>

### 2. Cronologia de animație multi-pistă
*Cronologie neliniară cu straturi suprapuse (`V1, V2...`), instanțe de cadre conectate (`🔗`), multiplicatori de menținere a duratei și etichete colorate pentru animații.*
<img src="screenshots/timeline.png" alt="Cronologie multi-pistă OniAnvil" width="950" />

<br/><br/>

### 3. Instrumente de precizie și bara de acțiuni rapide
*Suită completă de 15 instrumente de desen în moduri standard și compact, însoțită de bara superioară pentru mărimea pensulei, simetrie, continuitate pe margini și Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Bara de acțiuni rapide OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Instrumente OniAnvil Standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Instrumente OniAnvil Compact" width="300" />
</p>

<br/><br/>

### 4. Previzualizare animată în timp real
*Panou de redare dedicat cu filtrare după etichete, control precis al vitezei FPS, glisor de zoom și fundal comutabil (tablă de șah întunecată, deschisă sau culoare solidă).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Previzualizare în timp real OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zoom previzualizare în timp real OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Gestionarea paletelor, schimbarea culorilor și rampe de degrade
*Motor cromatic complet cu casete de selecție Prim-plan/Fundal, generare automată de degradeuri interpolate între două nuanțe și schimb rapid de variante de skin-uri.*
<img src="screenshots/palette_and_swaps.png" alt="Paletă și variante de culoare OniAnvil" width="950" />

<br/><br/>

### 6. Schiță de referință pentru trasare și Onion Skinning configurabil
*Încărcați schițe conceptuale sau fotografii ca strat inferior/superior cu ajustări de scalare și poziție, pipetă inteligentă cu aliniere la paletă și transparență pe mai multe cadre.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Schiță și Onion Skinning în OniAnvil" width="950" />

<br/><br/>

### 7. Puncte de pivot și puncte de acțiune pentru gameplay
*Configurați originea centrală a sprite-ului și ancore specifice de gameplay (poziția armei, lansatoare de proiectile, cap, pași) prin markere interactive direct pe pânză.*
<img src="screenshots/pivot_and_action_points.png" alt="Puncte de pivot și acțiune OniAnvil" width="950" />

<br/><br/>

### 8. Tăietor de foi sprite (Sheet Slicer) și scalare 9-Grid (9-Slice)
*Tăiere automată a cadrelor pe baza autocorelației contururilor (ex. ciclul de mers al unei pisici) și configurarea marginilor pentru elemente scalabile de interfață în 9 secțiuni.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Tăietor foi sprite OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Scalare 9-Slice OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Efecte de imagine și ferestre modale de filtrare în timp real (cu previzualizare live)
*Toate casetele de dialog pentru efecte oferă previzualizare în timp real direct pe pânză, pipete interactive și anulare sigură fără a altera istoricul operațiilor.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generator contur" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Înlocuire culoare" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Nuanță, saturație și contrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtre matriceale" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Remapare la paletă" width="460" />
</p>

<br/><br/>

### 10. Grilă de pânză și moduri de fundal
*Comutați fără efort între grila de pixeli, grila de dale (tiles), tabla de șah întunecată, tabla de șah luminoasă și fundalul personalizat de culoare solidă.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Grilă și șah întunecat" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Șah luminos fără grilă" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Fundal solid personalizat" width="300" />
</p>

<br/><br/>

### 11. Asistent de creare proiect nou
*Setați dimensiunile pânzei, numărul de cadre, distribuția pe coloane a grilei atlasului și alegeți din peste 20 de palete retro veritabile încă de la deschiderea proiectului.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Asistent proiect nou" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palete proiect nou" width="460" />
</p>

<br/><br/>

### 12. 35 de teme autentice retro și moderne
*Include Studio Dark, Studio Light, ecrane CRT cu fosfor Chihlimbar/Verde/Cian, console clasice (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) și teme estetice moderne.*
<p align="center">
  <img src="screenshots/themes.png" alt="Meniu teme" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Chihlimbar" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PlayStation 1" width="300" />
</p>

<br/><br/>

### 13. Sistemul de meniuri și ajutorul integrat pentru taste rapide
<p align="center">
  <img src="screenshots/file_menu.png" alt="Meniul Fișier" width="180" />
  <img src="screenshots/edit_menu.png" alt="Meniul Editare" width="180" />
  <img src="screenshots/select_menu.png" alt="Meniul Selecție" width="180" />
  <img src="screenshots/view_menu.png" alt="Meniul Vizualizare" width="180" />
  <img src="screenshots/project_menu.png" alt="Meniul Proiect" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Fereastra de ajutor pentru comenzi rapide" width="700" />
</p>

</div>

---

## Ghid complet al instrumentelor

OniAnvil integrează 15 instrumente precise pentru desen, geometrie, selecție și manipulare de metadate. Pictogramele provin din pachetul liber **Font Awesome Free** *(Pictograme: CC BY 4.0, Fonturi: SIL OFL 1.1)*.

| Instrument | Pictogramă | Tastă rapidă | Dimensiune | Pixel Perfect | Descriere |
|:---|:---:|:---:|:---:|:---:|:---|
| **Creion** | ✏️ | <kbd>B</kbd> | 1..32 px | Da | Instrumentul primar de desenare liberă. La grosimea de 1px oferă suport **Pixel-Perfect** (elimină automat treptele și colțurile în L). Permite ștampile din selecții (`Ctrl/Cmd+B`). |
| **Radieră** | 🧹 | <kbd>E</kbd> | 1..32 px | Nu | Șterge pixelii readucându-i la transparență completă (`[0, 0, 0, 0]`). Funcționează strict în interiorul zonei dacă selecția este activă. |
| **Pipetă** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nu | Clic stânga preia culoarea în Prim-plan (Foreground); Clic dreapta în Fundal (Background). Poate prelua din pânză, din straturi sau din schița de referință. |
| **Umplere cu vopsea** | 🪣 | <kbd>G</kbd> | N/A | Nu | Umplere de mare viteză bazată pe coadă (BFS) cu toleranță de culoare reglabilă (`0..=255`). Suportă mod contiguu sau global și limitare la selecție. |
| **Gradient** | 🌈 | <kbd>G</kbd> *(comutare)* | N/A | Nu | Degradeuri liniare și radiale între culorile de prim-plan și fundal. Suportă dithering matricial Bayer (2x2, 4x4, 8x8) și Blue Noise cu reglarea intensității. |
| **Linie** | 📏 | <kbd>L</kbd> | 1..32 px | Da | Algoritmul de linii Bresenham cu previzualizare la tragere. Țineți apăsat <kbd>Shift</kbd> pentru fixarea unghiurilor la multipli de 45°/90°. |
| **Curbă** | 〰️ | <kbd>C</kbd> | 1..32 px | Da | Curbă quadratică Bézier definită prin 3 clicuri. Clic 1 fixează începutul, Clic 2 sfârșitul, mișcarea mouse-ului ajustează arcuirea, iar Clic 3 definitivează curba. |
| **Dreptunghi** | ⬜ | <kbd>U</kbd> | 1..32 px | Nu | Dreptunghiuri geometrice conturate sau pline. Țineți apăsat <kbd>Shift</kbd> pentru a păstra raportul 1:1 (pătrat). |
| **Elipsă** | ⭕ | <kbd>O</kbd> | 1..32 px | Nu | Elipse și cercuri conturate sau pline prin rasterizare cu punct median. Țineți apăsat <kbd>Shift</kbd> pentru a forța un cerc perfect. |
| **Pensulă dither** | 🏁 | <kbd>D</kbd> | 1..32 px | Nu | Pictează tipare regulate de tip tablă de șah alternând culorile primară și secundară fără a adăuga nuanțe străine în paletă. |
| **Instrument de umbrire** | 🌓 | <kbd>S</kbd> | 1..32 px | Nu | Luminează (Dodge) sau întunecă (Burn) treptat pixelii pe baza succesiunii culorilor din paleta activă. |
| **Selecție dreptunghiulară** | ⬚ | <kbd>M</kbd> | N/A | Nu | Selecție dreptunghiulară marcată prin contur animat cu furnici în marș. Suportă Adăugare (<kbd>Shift</kbd>) și Scădere (<kbd>Alt</kbd>). |
| **Selecție laț** | ➰ | <kbd>Q</kbd> | N/A | Nu | Laț poligonal liber. Folosește contur 8-conectat cu umplere 4-conectată a exteriorului, garantând selecții 100% pline, fără goluri chiar și la bucle suprapuse. |
| **Baghetă magică** | 🪄 | <kbd>W</kbd> | N/A | Nu | Selectează zone continue (sau globale) de culoare similară cu prag de toleranță ajustabil. |
| **Punct de pivot și acțiune** | 🎯 | <kbd>P</kbd> | N/A | Nu | Plasare interactivă a originii sprite-ului și a punctelor cheie de acțiune pentru gameplay (`weapon`, `head`, `feet`, `hitbox`) exportate în JSON. |

---

## Documentație funcțională

### 1. Arhitectură unificată centrată pe atlas

În cadrul OniAnvil, atlasul principal de textură (`atlas_rgba`) reprezintă imaginea autentică a foii de sprite-uri încărcată direct de motorul de joc:
- **Mod Cadru**: izolează cadrul de animație activ, oferind o pânză clară și concentrată. Orice modificare adusă actualizează instantaneu celula aferentă din atlas.
- **Mod Atlas**: afișează întreaga grilă a foii de textură, facilitând revizuiri de ansamblu, verificări de aliniere între cadre și editări în masă.
- **Fișier de companie**: proiectele salvează o imagine `.png` clară însoțită de un fișier metadata `.onianvil.json` ce include organizarea cadrelor, straturile, etichetele, multiplicatorii de durată și skin-urile de paletă.

### 2. Cronologie (Timeline) de straturi multi-pistă

Cronologia inferioară este concepută pe modelul editoarelor video neliniare (NLE):
- **Piste de straturi (`V1, V2, V3...`)**: straturile sunt suprapuse vizual de sus în jos. Pista situată mai sus în cronologie (`V1`) este desenată peste straturile inferioare pe pânză.
- **Controlul pistelor**:
  - Faceți clic pe insignele `V1` / `V2` pentru a schimba rapid stratul activ de desenare.
  - Comutați vizibilitatea (pictograma ochi) și blocarea la editare (pictograma lacăt).
  - Ajustați nivelul individual de opacitate al fiecărui strat (`0%..=100%`).
  - Mutați stratul mai sus/jos, duplicați-l sau îmbinați-l în jos (Merge Down) cu pista subiacentă.
  - Ștergeți stratul în siguranță printr-o casetă de confirmare.
- **Cadre și fragmente de animație**:
  - Multiplicator de durată (Hold): alocați durate specifice fiecărui cadru (de ex. menținere `2×` sau `3×` pentru suspans sau impact).
  - Cadre asociate: conectați cadre repetitive pe parcursul animației; editarea oricărei instanțe actualizează automat toate celelalte instanțe corelate.
  - Repetare pe etichete: grupați secvențele în etichete logice (`idle`, `walk`, `attack`) și redați bucle exclusive pentru o anumită etichetă.
  - Comenzi de redare: Înainte, Înapoi, Ping-Pong și Buclă continuă la o rată reglabilă între `1` și `120` FPS.

### 3. Fereastră de previzualizare a animației în timp real

Panoul docat Preview din partea dreaptă rulează animațiile în timp real, indiferent de nivelul de mărire sau deplasarea pânzei principale:
- **Filtrare după etichetă**: redați strict eticheta selectată (`walk`, `jump`) sau parcurgeți întreaga secvență.
- **Comenzi de redare**: Redare, Pauză, Pas înapoi, Pas înainte, Comutare buclă și multiplicatori de viteză (`0.25x..=4.0x`).
- **Moduri de afișare**: comutați instantaneu între tablă de șah întunecată, deschisă, alb pur, negru pur sau fundal colorat personalizat.

### 4. Motor avansat de Onion Skinning

Creați animații organice și precise cu ajutorul sistemului specializat de calc (foi de ceapă):
- **Nuanțare independentă**: culori distincte de evidențiere pentru cadrele precedente (implicit: roșu) și cele următoare (implicit: albastru).
- **Atenuarea transparenței**: opacitate inițială reglabilă cu reducere progresivă la fiecare pas succesiv.
- **Buclă continuă (Wrap-Around)**: permite vizualizarea primului cadru prin ultimul și invers, asigurând cicluri perfect închise.
- **Privire rapidă (Quick-Peek)**: țineți apăsată tasta <kbd>Spațiu</kbd> fără a trage pentru a verifica instantaneu cadrele vecine sau schița.

### 5. Strat de schiță de referință și trasare inteligentă

Accelerați procesul de lucru trasând direct peste schițe manuale sau imagini fotografice:
- **Formate suportate**: importați fișiere de referință PNG, JPEG și WebP.
- **Moduri de compunere**: Strat inferior (sub straturile de pixeli) sau Suprapunere (peste straturile de desen).
- **Ajustări de geometrie**: zoom independent, deplasare, rotire și glisor fin de opacitate.
- **Pipetă inteligentă**: preluați culori direct din schiță cu opțiune automată de potrivire la cea mai apropiată nuanță din paleta activă a proiectului.
- **Conversie directă (1-Click Bake)**: transferați imaginea scalată de referință direct în pixeli artizanali pe cadrul activ printr-un singur clic.

### 6. Suită de selecție de precizie și furnici în marș

- **Contur animat de furnici în marș**: selecțiile active sunt delimitate de linii întrerupte animate în două tonuri de contrast ridicat.
- **Selecție laț plină cu umplere exterioară**: spre deosebire de algoritmii clasici de tip ray-casting care generează spații goale în interiorul buclelor intersectate, lațul din OniAnvil rasterizează o frontieră închisă 8-conectată și umple exteriorul, asigurând selecții 100% pline.
- **Operații de selecție**: Mărire, Micșorare, Bordură (cu nucleu rotund, pătrat sau romb), Translație, Inversare, Ștergere, Rotire liberă și Copiere/Tăiere/Lipire în clipboard-ul sistemului.
- **Transfer precis între cadre**: selectați o zonă, apăsați <kbd>Ctrl/Cmd+C</kbd>, schimbați cadrul și apăsați <kbd>Ctrl/Cmd+V</kbd> pentru a lipi la aceleași coordonate relative pe orice format de tastatură.

### 7. Efecte în timp real, filtre și RotSprite

Fiecare efect beneficiază de opțiunea **Previzualizare live**, permițând vizualizarea transformărilor direct pe pânză și anularea fără afectarea istoricului Undo:
- **Înlocuire culoare (`Shift+R`)**:
  - Pipetă interactivă: alegeți culorile sursă și destinație direct din pixelii cadrului sau din paletă.
  - Bandă rapidă de selecție: clic stânga pentru culoarea sursă, clic dreapta pentru culoarea destinație.
  - Glisor de toleranță și înlocuire în masă pe toate cadrele animației.
- **Ajustare culoare (`Ctrl/Cmd+U`)**: reglaj în timp real pentru nuanță HSV (Hue), saturație, luminozitate și contrast.
- **Generator automat de contur**: adaugă contururi clare la nivel de pixel în jurul personajului (Exterior, Interior, Centrat) cu îmbinări drepte sau în romb.
- **Filtre de convoluție 3×3**: estompare (blur), claritate (sharpen), detectare de margini și relief (emboss), cu păstrarea transparenței canalului alfa.
- **Rotire liberă (`Ctrl/Cmd+Shift+R`)**: rotiți porțiuni selectate sau cadre complete la orice unghi folosind algoritmii **RotSprite** sau **Cel mai apropiat vecin**.
- **Remapare la paletă**: aproximați culorile din pânză la paleta activă prin dithering Bayer 2x2, 4x4, 8x8 sau Blue Noise.

### 8. Gestionarea paletelor, schimbarea culorilor și variante de skin-uri

- **Palete integrate**: peste 20 de palete retro originale, incluzând Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized și Sweetie 16.
- **Import și export**: suport complet pentru fișiere GIMP Palette (`.gpl`), PAL (`.pal`), liste hexazecimale (`.hex`) și eșantioane grafice (`.png`).
- **Schimb de palete (Skin-uri)**: creați multiple variante cromatice pentru un personaj (de ex. Implicit, Armură de Aur, Umbră, Înghețat). Schimbarea skin-ului remapează instantaneu culorile în întregul atlas.
- **Generator de rampe cromatice**: calculează degradeuri netede între două culori alese, cu număr configurabil de trepte intermediare.

### 9. Tăietor de foi sprite (Sheet Slicer), interfață 9-Grid și export pentru motoare de jocuri

- **Tăietor de foi (Sheet Slicer)**: detecția automată a limitelor prin autocorelație separă colajele de sprite-uri în cadre omogene de animație.
- **Scalare în 9 secțiuni (9-Slice)**: setați margini de secționare (sus, jos, stânga, dreapta) pentru a ajusta cutii de interfață scalabile, ferestre și butoane.
- **Export PNG cu extrudarea marginilor**: copiază culorile de pe margini în spațiul transparent, prevenind apariția liniilor de delimitare și a defectelor de filtrare în OpenGL, DirectX, Vulkan sau Godot Engine.
- **Export de GIF-uri animate**: convertor integrat optimizat cu număr de cicluri personalizabil, pauză reglabilă între cadre și fundal transparent.
- **Metadate standard Sprite Sheet JSON**: exportă coordonatele cadrelor, etichetele, timpii și ancorele într-un format JSON universal compatibil cu motoarele moderne 2D.

### 10. 35 de teme ergonomice și localizare multilingvă

- **Colecție vastă de teme**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, ecrane CRT (Chihlimbar, Verde, Cian), console istorice (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) și calculatoare de epocă (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Interfață multilingvă**: suport extins pentru 37 de limbi cu paritate completă a termenilor și schimbare instantanee a limbii.

---

## Comenzi și comenzi rapide de la tastatură

| Comandă rapidă | Instrument / Acțiune | Descriere |
|:---|:---|:---|
| <kbd>B</kbd> | **Creion** | Desen fin la 1px cu opțiune Pixel-Perfect |
| <kbd>E</kbd> | **Radieră** | Șterge pixelii readucându-i la transparență |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipetă** | Preia culoarea de pe pânză sau din schiță |
| <kbd>G</kbd> | **Umplere / Gradient** | Comută între umplere cu vopsea și degrade dither |
| <kbd>L</kbd> | **Linie** | Trasează linii drepte cu ghidaj la unghiuri prin Shift |
| <kbd>C</kbd> | **Curbă** | Instrument curbă Bézier quadratică în 3 clicuri |
| <kbd>U</kbd> | **Dreptunghi** | Dreptunghiuri și pătrate conturate sau pline |
| <kbd>O</kbd> | **Elipsă** | Elipse și cercuri conturate sau pline |
| <kbd>D</kbd> | **Pensulă dither** | Pictează texturi în tablă de șah prin dithering |
| <kbd>S</kbd> | **Instrument de umbrire** | Luminează (Dodge) sau întunecă (Burn) pe baza paletei |
| <kbd>M</kbd> | **Selecție dreptunghiulară** | Selecție dreptunghiulară cu furnici în marș |
| <kbd>Q</kbd> | **Selecție laț** | Selecție liberă poligonală cu umplere solidă |
| <kbd>W</kbd> | **Baghetă magică** | Selectează zone continue de culoare similară |
| <kbd>P</kbd> | **Puncte de pivot** | Poziționează originea sprite-ului și ancorele de acțiune |
| <kbd>X</kbd> | **Inversare culori** | Schimbă culoarea de prim-plan cu cea de fundal |
| <kbd>[</kbd> / <kbd>]</kbd> | **Mărime pensulă** | Reduce / Crește grosimea pensulei (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copiază** | Copiază pixelii selectați în clipboard |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Decupează** | Decupează pixelii selectați în clipboard |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Lipește** | Lipește pixelii din clipboard în cadrul activ |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Șterge** | Curăță pixelii din interiorul selecției active |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Selectează tot** | Selectează întregul conținut al cadrului activ |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Deselectează** | Elimină masca activă de selecție |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Inversează selecția** | Inversează masca activă de selecție |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplică cadru** | Duplică cadrul selectat în cronologie |
| <kbd>Spațiu</kbd> | **Redare / Pauză** | Pornește sau oprește redarea în cronologie |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Înlocuire culoare** | Deschide fereastra de înlocuire cu previzualizare live |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Ajustare culoare** | Deschide fereastra pentru nuanță HSV și contrast |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Rotire liberă** | Deschide caseta de rotire la orice unghi (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Anulează (Undo)** | Anulează ultima acțiune de pe pânză |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Refă (Redo)** | Reface acțiunea anterior anulată |
| <kbd>Spațiu</kbd> + Tragere / Clic mijloc | **Panoramare pânză** | Deplasare liberă și lină pe pânză |
| <kbd>Ctrl/Cmd</kbd> + Rotiță / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Mărire clară centrată pe cursorul mouse-ului |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Mod vizualizare** | Comută între Modul Cadru (<kbd>1</kbd>) și Modul Atlas (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Potrivire pe ecran** | Centrează și ajustează cadrul la spațiul de lucru |
| <kbd>F1</kbd> | **Ajutor** | Deschide sumarul comenzilor rapide de tastatură |

---

## Ghid introductiv

### Fișiere binare de sine stătătoare

Descărcați fișierele binare autonome precompilate direct de pe **[pagina de versiuni GitHub](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: descărcați `OniAnvil-windows-x86_64.zip`, dezarhivați și lansați `OniAnvil.exe`.
- **Linux**: descărcați arhiva portabilă `OniAnvil-linux-x86_64.tar.gz` sau instalați pachetul Debian/Ubuntu `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: descărcați `OniAnvil-macos-universal.zip`, extrageți și mutați aplicația `OniAnvil.app` în dosarul Aplicații (Applications). Binarul universal rulează nativ atât pe cipurile Apple Silicon (M1/M2/M3/M4), cât și pe procesoare Intel.
- **Web**: deschideți direct în browser fără descărcare sau instalare: **[Lansează versiunea WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=ro)**.
  > [!WARNING]
  > Versiunea pentru browser WebAssembly are statut **Experimental**. Din pricina restricțiilor de securitate din browser, a diferențelor dintre driverele grafice WebGL/WebGPU și a gestionării clipboard-ului, pot apărea mici erori vizuale sau scăderi de fluență comparativ cu versiunea nativă. Pentru producție îndelungată se recomandă utilizarea versiunilor desktop native.

---

### Compilare din codul sursă

#### Cerințe prealabile
- **Toolchain Rust**: instalați Rust (se recomandă versiunea 1.80+) accesând [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dependințe de compilare pentru Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compilarea și rularea versiunii desktop native
```bash
# Clonarea depozitului
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Rularea testelor
cargo test

# Compilare și lansare versiune optimizată de lansare
cargo run --release
```

#### 2. Compilarea versiunii WebAssembly (prin Trunk)
```bash
# Instalarea țintei wasm32 și a utilitarului Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Pornirea serverului local de dezvoltare cu reîncărcare la cald
trunk serve

# Compilarea pachetului final optimizat în folderul docs/app/
./build_wasm.sh        # Pe Linux/macOS
./build_wasm.ps1       # În Windows PowerShell
```

---

## Formatul fișierelor și arhitectura de companie

OniAnvil evită utilizarea formatelor binare închise sau opace. Fiecare proiect este structurat sub forma a două fișiere compatibile cu sistemele de control al versiunilor:

```
my_character/
├── my_character.png            # Imagine standard PNG a atlasului sprite (se deschide în orice editor grafic sau motor de joc)
└── my_character.onianvil.json   # Fișier metadata de companie
```

Fișierul sidecar `.onianvil.json` stochează:
- Setările grilei de pânză: dimensiunile cadrelor, numărul de coloane și numărul total de cadre.
- Ierarhia straturilor: denumirea pistelor, nivelul de opacitate, indicatorii de vizibilitate și starea de blocare.
- Etichetele de animație: nume, indicii cadrelor de start/sfârșit, sensul de redare (Înainte, Înapoi, Ping-Pong), timpi individuali.
- Definirea paletelor: culorile active, variantele nominale de skin-uri și indexul skin-ului curent.
- Starea schiței de referință: calea fișierului, scara, deplasarea pe axe, unghiul de rotație, opacitatea și modul de strat.
- Puncte de pivot: ancore personalizate de acțiune, prinderi pentru arme și centre de coliziune.

---

## Contribuții și mulțumiri

Orice sugestie, raportare de problemă sau propunere de îmbunătățire este binevenită!
1. Creați un Fork al depozitului pe GitHub.
2. Creați o ramură nouă pentru funcționalitatea propusă: `git checkout -b feature/nou-instrument-pixel`.
3. Comiteți modificările: `git commit -am 'Adaugă un nou instrument de pixel'`.
4. Trimiteți ramura pe GitHub: `git push origin feature/nou-instrument-pixel`.
5. Deschideți un Pull Request.

### Mulțumiri
- Sprite-ul demonstrativ animat cu pisica **"Cat 2D Pixel Art"** este creat de către **xzany** ([pagina proiectului pe itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Dezvoltat cu ajutorul remarcabilei biblioteci grafice **[egui](https://github.com/emilk/egui)** și a infrastructurii **[wgpu](https://wgpu.rs/)**.
- Pictogramele de interfață sunt oferite prin intermediul suitei **[Font Awesome Free](https://fontawesome.com)** de la @fontawesome (Pictograme: CC BY 4.0, Fonturi: SIL OFL 1.1, Cod: Licența MIT).

---

## Licență și declinare a responsabilității

Acest proiect este licențiat sub termenii **[Licenței MIT](../LICENSE)**. Aveți libertate completă de a utiliza, modifica, distribui și integra OniAnvil în fluxurile comerciale sau personale de dezvoltare a jocurilor.

### Garanții și limitarea răspunderii (Disclaimer)
> [!IMPORTANT]
> **OniAnvil** este distribuit **în mod gratuit**, pe principiul **"AȘA CUM ESTE" ("AS IS")** și **"CONFORM DISPONIBILITĂȚII"**, fără niciun fel de garanții sau condiții, fie explicite sau implicite (incluzând, dar fără a se limita la, garanții de vandabilitate, adecvare la un anumit scop sau neîncălcare a drepturilor terților).
> 
> În nicio circumstanță autorul (autorii), deținătorii drepturilor de autor sau colaboratorii nu vor putea fi trași la răspundere pentru daune directe, indirecte, incidentale, speciale, exemplare sau subsecvente (incluzând, dar fără a se limita la, pierderi de date, coruperea fișierelor de proiect, defecțiuni hardware, pierderi de profit sau întreruperea activității comerciale) apărute în orice mod din utilizarea sau incapacitatea de utilizare a acestui program, chiar dacă au fost avertizați cu privire la posibilitatea unor asemenea daune. Utilizatorul își asumă în totalitate responsabilitatea și riscul decurgând din folosirea acestei aplicații.

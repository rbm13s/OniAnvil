<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio pixel artu a pookénkové animace (Onion Skinning) postavené na herních atlasech**

[![Sestavení a vydání](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Stažení](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Jazyky](https://img.shields.io/badge/Jazyky-37_Jazyků-purple)](#10-35-ergonomických-motivů-a-úplná-lokalizace)
[![Platformy](https://img.shields.io/badge/Platformy-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#začínáme)
[![Online aplikace](https://img.shields.io/badge/Online_aplikace-Vyzkoušet_v_prohlížeči_(Experimentální)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=cs)
[![Licence: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Vyzkoušejte OniAnvil v prohlížeči (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=cs)** *(⚠️ Experimentální sestavení: může obsahovat omezení a chyby WebGL/WASM)* • **[🌐 Oficiální web dokumentace](https://rbm13s.github.io/OniAnvil/cs.html)**

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
**Čeština** •
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
[Română](README-ro.md) •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Obsah

- [Úvod](#úvod)
- [Snímky obrazovky a vizuální prohlídka](#snímky-obrazovky-a-vizuální-prohlídka)
- [Kompletní přehled nástrojů](#kompletní-přehled-nástrojů)
- [Funkční dokumentace](#funkční-dokumentace)
  - [1. Architektura založená na atlasu jako jediném zdroji pravdy](#1-architektura-založená-na-atlasu-jako-jediném-zdroji-pravdy)
  - [2. Vícestopá časová osa vrstev](#2-vícestopá-časová-osa-vrstev)
  - [3. Živý náhled animace v reálném čase](#3-živý-náhled-animace-v-reálném-čase)
  - [4. Pokročilý systém pookénkového pauzování (Onion Skinning)](#4-pokročilý-systém-pookénkového-pauzování-onion-skinning)
  - [5. Inteligentní vrstva referenčního náčrtu](#5-inteligentní-vrstva-referenčního-náčrtu)
  - [6. Přesné nástroje výběru a pochodující mravenci](#6-přesné-nástroje-výběru-a-pochodující-mravenci)
  - [7. Živé efekty, filtry a RotSprite](#7-živé-efekty-filtry-a-rotsprite)
  - [8. Správa palet, záměna barev a skiny postav](#8-správa-palet-záměna-barev-a-skiny-postav)
  - [9. Dělič listů (Sheet Slicer), 9-Grid UI a export pro herní enginy](#9-dělič-listů-sheet-slicer-9-grid-ui-a-export-pro-herní-enginy)
  - [10. 35 ergonomických motivů a úplná lokalizace](#10-35-ergonomických-motivů-a-úplná-lokalizace)
- [Ovládání a klávesové zkratky](#ovládání-a-klávesové-zkratky)
- [Začínáme](#začínáme)
  - [Samostatné binární soubory](#samostatné-binární-soubory)
  - [Sestavení ze zdrojových kódů](#sestavení-ze-zdrojových-kódů)
- [Formát souborů a doprovodná architektura](#formát-souborů-a-doprovodná-architektura)
- [Jak přispět a poděkování](#jak-přispět-a-poděkování)
- [Licence a zřeknutí se odpovědnosti](#licence-a-zřeknutí-se-odpovědnosti)

---

## Úvod

**OniAnvil** (*název vychází z **Oni**on-Skinning [pookénková pauzovací animace] + **Anvil** [kovadlina pro kování spritů]* ) je open-source, vysoce výkonný editor pixel artu a animační studio napsané v jazyce Rust. Běží nativně s hardwarově akcelerovaným vykreslováním na systémech Windows, Linux a macOS a rovněž jako bezinstalační aplikace WebAssembly přímo v moderních webových prohlížečích.

Většina dosavadních sprite editorů vnímá animaci jako hromadu oddělených obrázkových souborů, což vývojáře nutí k únavnému exportu, řezání a opakovanému balení do atlasů. **OniAnvil toto paradigma převrací:** textura atlasu sprite sheetu (`atlas_rgba`) je primárním a jediným zdrojem pravdy. Buňky snímků, stopy vrstev, animační tagy i kolizní body ukotvení zůstávají za všech okolností nedestruktivně synchronizovány s atlasem.

### Proč OniAnvil?
- 🚀 **Bleskový nativní výkon v Rustu**: Okamžitý start v řádu zlomků milisekundy, krystalicky ostré vykreslování poháněné `wgpu` a `egui 0.31`, nulová latence při posouvání dlouhých časových os.
- 🎬 **Vícestopá časová osa vrstev (NLE)**: Stopy vrstev (`V1, V2, V3...`) s překrýváním shora dolů, slučováním dolů, propojenými instancemi snímků a nastavitelnou délkou zdržení (hold).
- 🧅 **Prvotřídní Onion Skinning**: Vícesnímkový průsvit sousedních fází s nezávislým tónováním dopředu/dozadu, nastavitelnou křivkou úbytku průhlednosti a zacyklením přes okraj animace.
- 🎨 **Okamžité paletové skiny postav**: Vytvářejte varianty den/noc, týmové barvy nebo elementární skiny díky okamžitému přemapování atlasu a podpoře importu/exportu formátů GPL, PAL i HEX.
- 📦 **Přímý export do herních enginů**: Exportujte škálované PNG s extruzí okrajových pixelů (zabraňuje nevzhledným švům při lineární filtraci v 2D enginech: Godot, Bevy, Raylib, Defold), standardní metadata Sprite Sheet JSON i animované soubory GIF.

---

## Snímky obrazovky a vizuální prohlídka

<div align="center">

### 1. Hlavní plátno a pracovní prostor studia
*Vysoce výkonné pracoviště pro pixel art s hardwarově akcelerovaným plátnem, vícestopou časovou osou, živým náhledem v reálném čase, vzorníkem palety a přesnými panely nástrojů.*
<img src="screenshots/main.png" alt="Hlavní pracovní prostor OniAnvil" width="950" />

<br/><br/>

### 2. Vícestopá časová osa animace
*Nelineární časová osa se zásobníkem vrstev (`V1, V2...`), propojenými instancemi snímků (`🔗`), násobiči délky trvání (hold) a barevně odlišenými animačními tagy.*
<img src="screenshots/timeline.png" alt="Vícestopá časová osa OniAnvil" width="950" />

<br/><br/>

### 3. Přesné nástroje a lišta rychlých akcí
*Kompletní sada 15 kreslicích nástrojů ve standardním i kompaktním dokovacím režimu, doplněná horním panelem pro rychlé nastavení velikosti štětce, symetrie, zacyklení a režimu Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Lišta akcí OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Nástroje OniAnvil - standardní" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Nástroje OniAnvil - kompaktní" width="300" />
</p>

<br/><br/>

### 4. Živý náhled animace v reálném čase
*Samostatný přehrávací dok s filtrováním podle tagů, nastavením rychlosti FPS, posuvníkem zvětšení a přepínači pozadí (šachovnice, jednolité tmavé, jednolité světlé).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Živý náhled OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zvětšení v živém náhledu" width="460" />
</p>

<br/><br/>

### 5. Správa palet, záměna barev a barevné rampy
*Komplexní barevný engine se zobrazením barvy popředí a pozadí, automatickým generováním dvoubodových gradientních přechodů a pojmenovanými paletovými skiny.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta a skiny v OniAnvil" width="950" />

<br/><br/>

### 6. Referenční náčrt a nastavitelný Onion Skinning
*Načtěte konceptuální skici nebo fotografie jako podklad/překryv s ovládáním transformace, inteligentním nabíráním barev s přichycením k paletě a vícesnímkovým pauzováním.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Náčrt a Onion Skinning v OniAnvil" width="950" />

<br/><br/>

### 7. Body počátku (Pivot) a herní akční body
*Definujte střed spritu (Origin) a vlastní pojmenované herní body (úchyty zbraní, emitory střel, hlava, kroky) pomocí interaktivních manipulátorů na plátně.*
<img src="screenshots/pivot_and_action_points.png" alt="Body počátku a akční body v OniAnvil" width="950" />

<br/><br/>

### 8. Dělič listů (Sheet Slicer) a škálování 9-Grid (9-Slice)
*Automatické rozpoznávání hranic spritů autokorelací (např. chůze kočky) společně s nastavením okrajů pro škálovatelné prvky uživatelského rozhraní 9-slice.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Dělič listů v OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Škálování 9-Slice v OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Živé obrazové efekty a filtry (s náhledem v reálném čase)
*Všechna dialogová okna efektů nabízejí živý náhled na plátně, interaktivní kapátka a možnost nedestruktivního zrušení bez znečištění historie kroků zpět.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generátor obrysu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Nahrazení barvy" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Odstín, sytost, kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Maticové filtry" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Přemapování do palety" width="460" />
</p>

<br/><br/>

### 10. Režimy mřížky a pozadí plátna
*Plynule přepínejte mezi pixelovou mřížkou, mřížkou snímků, tmavou šachovnicí, světlou šachovnicí a vlastními jednolitými barvami pozadí.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Mřížka a tmavá šachovnice" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Světlá šachovnice bez mřížky" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Vlastní jednolité pozadí" width="300" />
</p>

<br/><br/>

### 11. Průvodce vytvořením nového projektu
*Při spuštění projektu snadno nastavte rozměry snímku, počet snímků, sloupcovou mřížku atlasu a vyberte si z více než 20 autentických retro palet.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Průvodce novým projektem" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palety nového projektu" width="460" />
</p>

<br/><br/>

### 12. 35 autentických retro a moderních motivů
*Zahrnuje Studio Dark, Studio Light, fosfory CRT (jantarový, zelený, azurový), retro konzole (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) i designové motivy.*
<p align="center">
  <img src="screenshots/themes.png" alt="Nabídka motivů" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Motiv CRT jantarový" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Motiv PSX" width="300" />
</p>

<br/><br/>

### 13. Systém nabídek a integrovaná nápověda klávesových zkratek
<p align="center">
  <img src="screenshots/file_menu.png" alt="Nabídka Soubor" width="180" />
  <img src="screenshots/edit_menu.png" alt="Nabídka Úpravy" width="180" />
  <img src="screenshots/select_menu.png" alt="Nabídka Výběr" width="180" />
  <img src="screenshots/view_menu.png" alt="Nabídka Zobrazení" width="180" />
  <img src="screenshots/project_menu.png" alt="Nabídka Projekt" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Okno nápovědy klávesových zkratek" width="700" />
</p>

</div>

---

## Kompletní přehled nástrojů

OniAnvil nabízí 15 přesných nástrojů pro kreslení, geometrii, výběr a správu metadat. Ikony využívají otevřenou sadu **Font Awesome Free** *(ikony: CC BY 4.0, písma: SIL OFL 1.1)*.

| Nástroj | Ikona | Zkratka | Velikost štětce | Pixel Perfect | Popis |
|:---|:---:|:---:|:---:|:---:|:---|
| **Tužka** | ✏️ | <kbd>B</kbd> | 1..32 px | Ano | Základní nástroj pro kreslení. V režimu 1px podporuje kreslení **Pixel-Perfect** (automaticky odstraňuje nežádoucí rohové L-schodky). Podporuje vlastní razítkové štětce z výběru (`Ctrl/Cmd+B`). |
| **Guma** | 🧹 | <kbd>E</kbd> | 1..32 px | Ne | Maže pixely zpět do úplné průhlednosti (`[0, 0, 0, 0]`). Je ohraničena maskou výběru, pokud je aktivní. |
| **Kapátko** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ne | Kliknutí LTM nabere barvu do Popředí (hlavní); PTM nabere do Pozadí (vedlejší). Může snímat barvu z plátna, vrstev nebo referenčního náčrtu. |
| **Plechovka barvy** | 🪣 | <kbd>G</kbd> | N/A | Ne | Bleskurychlá frontová výplň (BFS) s nastavitelnou tolerancí barev (`0..=255`). Podporuje souvislý i globální režim a ohraničení výběrem. |
| **Přechod** | 🌈 | <kbd>G</kbd> *(přepínač)* | N/A | Ne | Lineární a radiální přechody mezi barvou popředí a pozadí. Podporuje maticový dithering Bayer (2x2, 4x4, 8x8) i Blue Noise s regulací síly. |
| **Čára** | 📏 | <kbd>L</kbd> | 1..32 px | Ano | Bresenhamův algoritmus přímek s živým náhledem při tažení. Přidržením klávesy <kbd>Shift</kbd> uzamknete úhly v krocích po 45°/90°. |
| **Křivka** | 〰️ | <kbd>C</kbd> | 1..32 px | Ano | 3bodová kvadratická Bézierova křivka. Kliknutí 1 nastaví počátek, kliknutí 2 konec, pohyb myši nastaví prohnutí a kliknutí 3 křivku potvrdí. |
| **Obdélník** | ⬜ | <kbd>U</kbd> | 1..32 px | Ne | Geometrické obdélníky s obrysem nebo výplní. Přidržením <kbd>Shift</kbd> zachováte poměr stran 1:1 (čtverec). |
| **Elipsa** | ⭕ | <kbd>O</kbd> | 1..32 px | Ne | Obrysové nebo vyplněné elipsy pomocí středové rasterizace. Přidržením <kbd>Shift</kbd> vytvoříte dokonalý kruh. |
| **Štětec ditheringu** | 🏁 | <kbd>D</kbd> | 1..32 px | Ne | Kreslí pravidelné šachovnicové vzory ditheringu střídáním barvy popředí a pozadí bez zavádění nových odstínů do palety. |
| **Nástroj stínování** | 🌓 | <kbd>S</kbd> | 1..32 px | Ne | Postupné zesvětlování (Dodge) a ztmavování (Burn) pixelů podle pořadí barev v aktivní paletě. |
| **Obdélníkový výběr** | ⬚ | <kbd>M</kbd> | N/A | Ne | Obdélníkový výběr oblasti s animovaným obrysem pochodujících mravenců. Podporuje přidávání (<kbd>Shift</kbd>) i odebírání (<kbd>Alt</kbd>). |
| **Laso** | ➰ | <kbd>Q</kbd> | N/A | Ne | Výběr mnohoúhelníku od ruky. Využívá 8-spojitou rasterizaci obrysu se 4-spojitou vnější výplní: 100% plný výběr bez mezer i při překřížení smyček. |
| **Kouzelná hůlka** | 🪄 | <kbd>W</kbd> | N/A | Ne | Vybere souvislé (nebo globální) oblasti odpovídající barvy s nastavitelným prahem tolerance. |
| **Bod počátku a akce** | 🎯 | <kbd>P</kbd> | N/A | Ne | Interaktivní umístění středu spritu (Origin) a vlastních herních akčních bodů (`weapon`, `head`, `feet`, `hitbox`) exportovaných do metadat JSON. |

---

## Funkční dokumentace

### 1. Architektura založená na atlasu jako jediném zdroji pravdy

V OniAnvil představuje hlavní textura atlasu (`atlas_rgba`) skutečnou texturu sprite sheetu, jak ji načítá herní engine:
- **Režim snímku (Frame Mode)**: Soustředí se na jediný aktivní snímek animace a poskytuje nerušené pixelové plátno. Veškeré úpravy okamžitě aktualizují odpovídající buňku v textuře atlasu.
- **Režim atlasu (Atlas Mode)**: Zobrazuje celou mřížku sprite sheetu, což umožňuje makro-úpravy, kontrolu zarovnání mezi snímky i dávkové úpravy.
- **Doprovodný soubor (Companion File)**: Projekty ukládají čisté PNG atlasy doplněné lehkým metadatovým souborem `.onianvil.json` s rozvržením snímků, hierarchií vrstev, tagy, délkami trvání a paletovými skiny.

### 2. Vícestopá časová osa vrstev

Spodní časová osa vychází z principů profesionálního nelineárního střihu videa:
- **Stopy vrstev (`V1, V2, V3...`)**: Vrstvy jsou vizuálně skládány shora dolů. Stopa umístěná výše v časové ose (`V1`) se vykresluje nad stopami ležícími níže.
- **Ovládání stop**:
  - Kliknutím na odznaky `V1` / `V2` rychle přepnete aktivní kreslicí vrstvu.
  - Přepínejte viditelnost vrstvy (ikona Oka) a zámek úprav (ikona Zámku).
  - Nastavujte průhlednost jednotlivých vrstev (`0%..=100%`).
  - Posouvejte vrstvy nahoru/dolů v hierarchii, duplikujte je a slučujte se stopou níže (Merge Down).
  - Bezpečně odstraňujte vrstvy s potvrzovacím dialogem.
- **Klip snímku**:
  - Násobič délky (Hold): přiřazujte vlastní délky trvání snímků (např. `2×`, `3×` pro pauzy nebo rázové snímky).
  - Propojené snímky (`🔗`): propojte opakující se snímky napříč animací, takže úprava jednoho automaticky aktualizuje všechny provázané instance.
  - Zacyklení podle tagů: organizujte animace do pojmenovaných tagů (`idle`, `walk`, `attack`) a přehrávejte ve smyčce vybrané sekvence.
  - Ovládání přehrávání: Přehrávání vpřed, vzad, ping-pong a smyčka s nastavitelnou rychlostí FPS (`1..=120`).

### 3. Živý náhled animace v reálném čase

Pravý dok náhledu přehrává vaše animace v reálném čase zcela nezávisle na měřítku nebo posunu hlavního plátna:
- **Filtrování tagů**: Izolujte přehrávání na konkrétní tagy (`walk`, `jump`) nebo sledujte celou sekvenci animace.
- **Ovládání přehrávání**: Tlačítka Play, Pause, krok zpět, krok vpřed, přepínač smyčky a vlastní násobiče rychlosti (`0.25x..=4.0x`).
- **Režimy zobrazení**: Přepínejte mezi tmavou šachovnicí, světlou šachovnicí, čistě bílým, čistě černým nebo vlastním pozadím.

### 4. Pokročilý systém pookénkového pauzování (Onion Skinning)

Vytvářejte plynulé animace s využitím specializovaného systému pauzování:
- **Nezávislé tónování**: Samostatné barvy zabarvení pro předchozí snímky (výchozí: červená) a následující snímky (výchozí: modrá).
- **Úbytek průhlednosti**: Nastavitelná výchozí průhlednost i křivky postupného zeslabování pro další kroky pauzování.
- **Zacyklení přes okraj (Loop Wrap-Around)**: Umožňuje pauzování mezi prvním a posledním snímkem zacyklených animací.
- **Rychlý náhled (Quick-Peek)**: Podržením klávesy <kbd>Mezerník</kbd> bez tažení myši okamžitě zobrazíte sousední snímky.

### 5. Inteligentní vrstva referenčního náčrtu

Urychlete si práci obkreslováním ručních skic nebo referenčních fotografií:
- **Formáty**: Načítejte obrázky ve formátech PNG, JPEG a WebP.
- **Režimy prolínání**: Podklad (pod pixelovými vrstvami) nebo Překryv (nad pixelovými vrstvami).
- **Ovládání transformace**: Nezávislé přiblížení, posun pozice, otočení a posuvník průhlednosti.
- **Inteligentní vzorkování barev**: Používejte kapátko na vrstvě náčrtu s volitelným automatickým přichycením k nejbližší barvě v aktivní paletě projektu.
- **Otisk na jedno kliknutí (1-Click Bake)**: Převod zvětšeného referenčního obrázku přímo na čistý pixel art v aktivním snímku.

### 6. Přesné nástroje výběru a pochodující mravenci

- **Animovaní pochodující mravenci**: Aktivní výběr se zobrazuje pomocí zřetelného dvoubarevného animovaného přerušovaného obvodu.
- **Plný výběr lasem na bázi výplně**: Na rozdíl od primitivních paprskových algoritmů, které vytvářejí nevybrané díry při překrývání smyček, laso v OniAnvil rasterizuje uzavřený 8-spojitý obvod a vyplňuje vnější prostor, což zaručuje 100% plný výběr bez nechtěných děr.
- **Operace s výběrem**: Rozšíření, Zúžení, Obrys (s kruhovým, čtvercovým a kosočtvercovým tvarem jádra), Posun, Invertovat, Smazat, Volné otočení a Schránka systému (Kopírovat/Vyjmout/Vložit).
- **Kopírování a vkládání mezi snímky**: Vyberte oblast na jednom snímku, stiskněte <kbd>Ctrl/Cmd+C</kbd>, přepněte snímek a stiskněte <kbd>Ctrl/Cmd+V</kbd> pro vložení na identické relativní souřadnice při jakémkoli rozložení klávesnice.

### 7. Živé efekty, filtry a RotSprite

Všechny obrazové efekty obsahují zaškrtávací pole **Živý náhled (Live Preview)**, které zobrazuje úpravy na plátně a umožňuje čisté zrušení bez znečištění historie kroků zpět:
- **Nahrazení barvy (`Shift+R`)**:
  - Interaktivní kapátko: klikněte na ikonu kapátka a vyberte barvy přímo z plátna nebo doku palety.
  - Lišta rychlé palety: kliknutím LTM vyberete barvu „Z“, kliknutím PTM barvu „Do“.
  - Posuvník tolerance a hromadné nahrazení napříč všemi snímky animace.
- **Barevné úpravy (`Ctrl/Cmd+U`)**: Nastavení posunu odstínu HSV (Hue), sytosti (Saturation), jasu (Value) a kontrastu v reálném čase.
- **Generátor automatického obrysu**: Vytvářejte ostré pixelové obrysy postav (Vnější, Vnitřní, Na střed) se čtvercovými nebo kosočtvercovými rohy.
- **Konvoluční maticové filtry 3×3**: Rozostření, Zaostření, Detekce hran a Reliéf s možností zachování alfa kanálu.
- **Volné otočení (`Ctrl/Cmd+Shift+R`)**: Otáčejte vybrané oblasti nebo celé snímky o libovolný úhel pomocí klasických algoritmů **RotSprite** nebo **Nearest Neighbor**.
- **Přemapování do palety**: Kvantování barev plátna do aktivní palety s ditheringem Bayer (2x2, 4x4, 8x8) nebo Blue Noise.

### 8. Správa palet, záměna barev a skiny postav

- **Vestavěné palety**: Více než 20 autentických retro palet včetně Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized a Sweetie 16.
- **Import a export**: Podpora formátů GIMP Palette (`.gpl`), PAL (`.pal`), Hex (`.hex`) a obrázkových vzorníků (`.png`).
- **Záměna palet (Skiny postav)**: Definujte několik variant skinů pro jednu postavu (např. Výchozí, Zlaté brnění, Stín, Zmrzlý). Záměna skinu okamžitě přemapuje všechny barvy snímků v celém atlasu.
- **Generátor barevných ramp**: Automatická interpolace plynulých gradientů mezi dvěma zvolenými odstíny s nastavitelným počtem mezikroků.

### 9. Dělič listů (Sheet Slicer), 9-Grid UI a export pro herní enginy

- **Dělič listů (Sheet Slicer)**: Autokorelační detekce hranic automaticky rozřeže surové sprite sheety na rovnoměrné nebo přesně ohraničené animační snímky.
- **Škálování 9-Grid (9-Slice)**: Nastavte přizpůsobitelné okraje řezů (nahoře, dole, vlevo, vpravo) pro náhled škálovatelných oken a tlačítek uživatelského rozhraní.
- **Export PNG s extruzí okrajů**: Rozšiřuje barvy okrajových pixelů do průhledných okrajů, což odstraňuje švy při filtrování textur v OpenGL, DirectX, Vulkanu i 2D herních enginech (jako je Godot).
- **Export animovaného GIF**: Vytvářejte optimalizované soubory GIF s nastavitelným počtem smyček, zpožděním snímků a průhledným pozadím.
- **Standardní formát Sprite Sheet JSON**: Exportuje souřadnice snímků, tagy, doby trvání a body ukotvení odpovídající standardnímu schématu JSON v moderních herních enginech.

### 10. 35 ergonomických motivů a úplná lokalizace

- **Vybrané motivy**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT fosfory (Jantar, Zelená, Azurová), retro konzole (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) a retro počítače (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Podpora 37 jazyků**: Plynulé přepínání jazyků v reálném čase se 100% paritou všech řetězců a nápověd.

---

## Ovládání a klávesové zkratky

| Zkratka | Nástroj / Akce | Popis |
|:---|:---|:---|
| <kbd>B</kbd> | **Tužka** | Přesné kreslení 1px s volitelným režimem Pixel-Perfect |
| <kbd>E</kbd> | **Guma** | Mazání pixelů zpět do průhlednosti |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Kapátko** | Nabírání barvy z plátna nebo referenčního náčrtu |
| <kbd>G</kbd> | **Výplň / Přechod** | Přepínání mezi plechovkou barvy a přechodem |
| <kbd>L</kbd> | **Čára** | Kreslení rovných čar s živým náhledem a přichycením přes Shift |
| <kbd>C</kbd> | **Křivka** | 3bodová kvadratická Bézierova křivka |
| <kbd>U</kbd> | **Obdélník** | Kreslení obdélníků a čtverců (obrys nebo výplň) |
| <kbd>O</kbd> | **Elipsa** | Kreslení elips a kruhů (obrys nebo výplň) |
| <kbd>D</kbd> | **Štětec ditheringu** | Kreslení maticovým šachovnicovým vzorem ditheringu |
| <kbd>S</kbd> | **Nástroj stínování** | Postupné zesvětlování / ztmavování v rámci palety |
| <kbd>M</kbd> | **Obdélníkový výběr** | Výběr obdélníkové oblasti s pochodujícími mravenci |
| <kbd>Q</kbd> | **Laso** | Plný odruční polygonální výběr |
| <kbd>W</kbd> | **Kouzelná hůlka** | Výběr souvislých oblastí stejné barvy |
| <kbd>P</kbd> | **Body počátku a akce** | Umístění středu spritu a akčních bodů |
| <kbd>X</kbd> | **Prohodit barvy** | Prohození barvy popředí a pozadí |
| <kbd>[</kbd> / <kbd>]</kbd> | **Velikost štětce** | Zmenšení / zvětšení velikosti štětce (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopírovat** | Zkopírování vybraných pixelů do schránky |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Vyjmout** | Vyjmutí vybraných pixelů do schránky |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Vložit** | Vložení pixelů ze schránky do aktivního snímku |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Smazat** | Vymazání pixelů uvnitř aktivního výběru |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Vybrat vše** | Výběr všech pixelů v aktivním snímku |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Zrušit výběr** | Odstranění aktivní masky výběru |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invertovat výběr** | Obrácení aktivní masky výběru |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplikovat snímek** | Duplikace aktivního snímku v časové ose |
| <kbd>Mezerník</kbd> | **Přehrát / Pozastavit** | Přepínání přehrávání animace v časové ose |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Nahradit barvu** | Otevření dialogu nahrazení barvy s živým náhledem |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Barevné úpravy** | Otevření dialogu úpravy HSV odstínu, sytosti a kontrastu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Volné otočení** | Otevření dialogu otočení o libovolný úhel (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Zpět** | Vrácení posledního tahu nebo akce nástroje |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Znovu** | Opakování dříve vrácené akce |
| <kbd>Mezerník</kbd> + Tažení / Prostřední tlačítko | **Posun plátna** | Plynulý nekonečný posun po souřadnicích plátna |
| <kbd>Ctrl/Cmd</kbd> + Kolečko / <kbd>+</kbd> / <kbd>-</kbd> | **Přiblížení** | Ostré přiblížení se středem na kurzoru myši |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Režim zobrazení** | Přepínání mezi zobrazením snímku (<kbd>1</kbd>) a atlasu (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Přizpůsobit plátno** | Automatické přizpůsobení a vycentrování aktivního snímku |
| <kbd>F1</kbd> | **Nápověda** | Otevření okna nápovědy klávesových zkratek |

---

## Začínáme

### Samostatné binární soubory

Stáhněte si předkompilované, samostatné spustitelné soubory ze **[stránky vydání na GitHubu](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Stáhněte soubor `OniAnvil-windows-x86_64.zip`, rozbalte jej a spusťte `OniAnvil.exe`.
- **Linux**: Stáhněte archiv `OniAnvil-linux-x86_64.tar.gz` (přenosná verze) nebo nainstalujte balíček `onianvil_*_amd64.deb` pro Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Stáhněte soubor `OniAnvil-macos-universal.zip`, rozbalte jej a přetáhněte aplikaci `OniAnvil.app` do složky Aplikace. Univerzální binární soubor běží nativně na procesorech Apple Silicon (M1/M2/M3/M4) i na počítačích Mac s procesory Intel.
- **Web**: Spusťte přímo v prohlížeči bez instalace: **[Spustit aplikaci WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=cs)**.
  > [!WARNING]
  > Verze pro webové prohlížeče (WebAssembly) má status **Experimentální (Experimental)**. Vzhledem k omezením sandboxu prohlížeče, odlišnostem ovladačů WebGL/WebGPU a limitacím systémové schránky se mohou vyskytnout drobné vizuální chyby či omezení výkonu ve srovnání s nativní desktopovou verzí. Pro plnohodnotnou produkční práci důrazně doporučujeme nativní desktopové sestavení.

---

### Sestavení ze zdrojových kódů

#### Požadavky
- **Sada nástrojů Rust**: Nainstalujte Rust (doporučena verze 1.80+) prostřednictvím [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Závislosti sestavení pro Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompilace a spuštění nativní verze pro počítače
```bash
# Naklonujte repozitář
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Spusťte sadu testů
cargo test

# Sestavte a spusťte optimalizovanou verzi vydání
cargo run --release
```

#### 2. Kompilace verze WebAssembly (pomocí Trunk)
```bash
# Přidejte cíl kompilace wasm32 a nainstalujte nástroj Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Spusťte lokální vývojový server s automatickým obnovováním
trunk serve

# Sestavte optimalizované produkční soubory do složky docs/app/
./build_wasm.sh        # Na systémech Linux/macOS
./build_wasm.ps1       # V prostředí Windows PowerShell
```

---

## Formát souborů a doprovodná architektura

OniAnvil nepoužívá proprietární binární formáty. Každý projekt tvoří dva čisté soubory připravené pro verzování:

```
muj_hrdina/
├── muj_hrdina.png            # Standardní atlas sprite sheetu v PNG (otevřete v jakémkoli enginu i prohlížeči)
└── muj_hrdina.onianvil.json   # Doprovodný soubor metadat JSON
```

Doprovodný soubor `.onianvil.json` ukládá:
- Konfiguraci mřížky plátna: rozměry snímků, počet sloupců, celkový počet snímků.
- Hierarchie vrstev: názvy stop, úrovně průhlednosti, příznaky viditelnosti a zámku.
- Animační tagy: název, indexy počátečního a koncového snímku, směr přehrávání (Vpřed, Vzad, Ping-Pong), doby trvání.
- Definice palet: aktivní barevné vzorky, pojmenované varianty skinů, index aktivního skinu.
- Stav referenčního náčrtu: cestu k obrázku, měřítko, posun, úhel otočení, průhlednost, režim vrstvy.
- Body počátku a akční body: střed spritu (Origin) a vlastní herní akční body (úchyty zbraní, body zásahu).

---

## Jak přispět a poděkování

Vřele vítáme jakékoli příspěvky, hlášení chyb i návrhy nových funkcí!
1. Vytvořte fork repozitáře na GitHubu.
2. Vytvořte větev funkce: `git checkout -b feature/muj-novy-nastroj`.
3. Potvrďte změny do gitu: `git commit -am 'Přidání nového pixelového nástroje'`.
4. Odešlete větev do svého forku: `git push origin feature/muj-novy-nastroj`.
5. Otevřete Pull Request.

### Poděkování
- Ukázkový demonstrační sprite animace kočky **"Cat 2D Pixel Art"** vytvořil autor **xzany** ([stránka na itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Vytvořeno pomocí skvělé knihovny GUI **[egui](https://github.com/emilk/egui)** a grafického ekosystému **[wgpu](https://wgpu.rs/)**.
- Ikony rozhraní využívají sadu **[Font Awesome Free](https://fontawesome.com)** od @fontawesome (ikony: CC BY 4.0, písma: SIL OFL 1.1, kód: licence MIT).

---

## Licence a zřeknutí se odpovědnosti

Tento projekt je licencován pod licencí **[MIT License](../LICENSE)**. OniAnvil můžete svobodně používat, upravovat, šířit a integrovat do komerčních i nekomerčních herních projektů.

### Zřeknutí se záruk a omezení odpovědnosti
> [!IMPORTANT]
> Software **OniAnvil** je šířen **zcela zdarma** na bázi **„TAK, JAK JE“ („AS IS“)** a **„DOKUD JE K DISPOZICI“ („AS AVAILABLE“)**, bez jakýchkoli záruk nebo podmínek, ať už výslovných nebo předpokládaných.
> 
> Autoři, držitelé autorských práv ani přispěvatelé **v žádném případě nenesou odpovědnost** za jakékoli přímé, nepřímé, náhodné, zvláštní, exemplární nebo následné škody (včetně, ale bez omezení na ztrátu dat, poškození souborů, selhání hardwaru, ušlý zisk nebo přerušení podnikání) vzniklé jakýmkoli způsobem v důsledku používání nebo nemožnosti používat tento software, a to i v případě, že byli na možnost takové škody upozorněni. Veškerou odpovědnost a rizika spojená s používáním tohoto programu přebírá uživatel.

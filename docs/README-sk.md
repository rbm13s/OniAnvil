<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Štúdio pixel artu a animácie po okienkach (Onion Skinning) postavené na herných atlasoch**

[![Zostavenie a vydanie](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Stiahnutia](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Jazyky](https://img.shields.io/badge/Jazyky-37_Jazykov-purple)](#10-35-ergonomických-motívov-a-úplná-lokalizácia)
[![Platformy](https://img.shields.io/badge/Platformy-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#začíname)
[![Online aplikácia](https://img.shields.io/badge/Online_aplikácia-Vyskúšať_v_prehliadači_(Experimentálna)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sk)
[![Licencia: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Vyskúšajte OniAnvil v prehliadači (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=sk)** *(⚠️ Experimentálne zostavenie: môže obsahovať obmedzenia a chyby WebGL/WASM)* • **[🌐 Oficiálna webová stránka dokumentácie](https://rbm13s.github.io/OniAnvil/sk.html)**

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
**Slovenčina** •
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
- [Snímky obrazovky a vizuálna prehliadka](#snímky-obrazovky-a-vizuálna-prehliadka)
- [Kompletný prehľad nástrojov](#kompletný-prehľad-nástrojov)
- [Funkčná dokumentácia](#funkčná-dokumentácia)
  - [1. Architektúra založená na atlase ako jedinom zdroji pravdy](#1-architektúra-založená-na-atlase-ako-jedinom-zdroji-pravdy)
  - [2. Viacstopová časová os vrstiev](#2-viacstopová-časová-os-vrstiev)
  - [3. Živý náhľad animácie v reálnom čase](#3-živý-náhľad-animácie-v-reálnom-čase)
  - [4. Pokročilý systém pauzovania po okienkach (Onion Skinning)](#4-pokročilý-systém-pauzovania-po-okienkach-onion-skinning)
  - [5. Inteligentná vrstva referenčného náčrtu](#5-inteligentná-vrstva-referenčného-náčrtu)
  - [6. Presné nástroje výberu a pochodujúce mravce](#6-presné-nástroje-výberu-a-pochodujúce-mravce)
  - [7. Živé efekty, filtre a RotSprite](#7-živé-efekty-filtre-a-rotsprite)
  - [8. Správa paliet, zámena farieb a skiny postáv](#8-správa-paliet-zámena-farieb-a-skiny-postáv)
  - [9. Delenie hárkov (Sheet Slicer), 9-Grid UI a export pre herné enginy](#9-delenie-hárkov-sheet-slicer-9-grid-ui-a-export-pre-herné-enginy)
  - [10. 35 ergonomických motívov a úplná lokalizácia](#10-35-ergonomických-motívov-a-úplná-lokalizácia)
- [Ovládanie a klávesové skratky](#ovládanie-a-klávesové-skratky)
- [Začíname](#začíname)
  - [Samostatné binárne súbory](#samostatné-binárne-súbory)
  - [Zostavenie zo zdrojových kódov](#zostavenie-zo-zdrojových-kódov)
- [Formát súborov a sprievodná architektúra](#formát-súborov-a-sprievodná-architektúra)
- [Ako prispieť a poďakovanie](#ako-prispieť-a-poďakovanie)
- [Licencia a zrieknutie sa zodpovednosti](#licencia-a-zrieknutie-sa-zodpovednosti)

---

## Úvod

**OniAnvil** (*názov pochádza zo spojenia **Oni**on-Skinning [pauzovanie po okienkach] + **Anvil** [kováčska nákova na kovanie spritov]* ) je open-source, vysoko výkonný editor pixel artu a animačné štúdio napísané v jazyku Rust. Beží natívne s hardvérovo akcelerovaným vykresľovaním na systémoch Windows, Linux a macOS a rovnako ako bezinštalačná aplikácia WebAssembly priamo v moderných webových prehliadačoch.

Väčšina doterajších sprite editorov vníma animáciu ako zhluk oddelených obrázkových súborov, čo vývojárov núti k únavnému exportu, deleniu a opätovnému baleniu do atlasov. **OniAnvil tento prístup obracia:** textúra atlasu sprite sheetu (`atlas_rgba`) je primárnym a jediným zdrojom pravdy. Bunky snímok, stopy vrstiev, animačné tagy aj kolízne body ukotvenia zostávajú za každých okolností nedeštruktívne synchronizované s atlasom.

### Prečo OniAnvil?
- 🚀 **Bleskový natívny výkon v Ruste**: Okamžitý štart v zlomku milisekundy, krištáľovo čisté vykresľovanie poháňané `wgpu` a `egui 0.31`, nulové oneskorenie pri posúvaní dlhých časových osí.
- 🎬 **Viacstopová časová os vrstiev (NLE)**: Stopy vrstiev (`V1, V2, V3...`) s vrstvením zhora nadol, zlučovaním nadol, prepojenými inštanciami snímok a nastaviteľnou dĺžkou zdržania (hold).
- 🧅 **Prvotriedny Onion Skinning**: Viacsnímkový priesvit susedných fáz s nezávislým tónovaním dopredu/dozadu, nastaviteľnými krivkami úbytku priehľadnosti a zacyklením cez okraj animácie.
- 🎨 **Okamžité paletové skiny postáv**: Vytvárajte varianty deň/noc, tímové farby alebo elementárne skiny vďaka okamžitému premapovaniu atlasu a podpore importu/exportu formátov GPL, PAL aj HEX.
- 📦 **Priamy export do herných enginov**: Exportujte škálované PNG s extruziou okrajových pixelov (zabraňuje nežiaducim švom pri lineárnej filtrácii v 2D enginoch: Godot, Bevy, Raylib, Defold), štandardné metadáta Sprite Sheet JSON aj animované súbory GIF.

---

## Snímky obrazovky a vizuálna prehliadka

<div align="center">

### 1. Hlavné plátno a pracovný priestor štúdia
*Vysoko výkonné pracovisko pre pixel art s hardvérovo akcelerovaným plátnom, viacstopovou časovou osou, živým náhľadom v reálnom čase, vzorkovníkom palety a precíznymi panelmi nástrojov.*
<img src="screenshots/main.png" alt="Hlavný pracovný priestor OniAnvil" width="950" />

<br/><br/>

### 2. Viacstopová časová os animácie
*Nelineárna časová os so zásobníkom vrstiev (`V1, V2...`), prepojenými inštanciami snímok (`🔗`), násobičmi dĺžky trvania (hold) a farebne odlíšenými animačnými tagmi.*
<img src="screenshots/timeline.png" alt="Viacstopová časová os OniAnvil" width="950" />

<br/><br/>

### 3. Presné nástroje a lišta rýchlych akcií
*Kompletná sada 15 kresliacich nástrojov v štandardnom aj kompaktnom dokovacom režime, doplnená horným panelom pre rýchle nastavenie veľkosti štetca, symetrie, zacyklenia a režimu Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Lišta akcií OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Nástroje OniAnvil - štandardné" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Nástroje OniAnvil - kompaktné" width="300" />
</p>

<br/><br/>

### 4. Živý náhľad animácie v reálnom čase
*Samostatný prehrávací dok s filtrovaním podľa tagov, nastavením rýchlosti FPS, posuvníkom zväčšenia a prepínačmi pozadia (šachovnica, jednoliate tmavé, jednoliate svetlé).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Živý náhľad OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zväčšenie v živom náhľade" width="460" />
</p>

<br/><br/>

### 5. Správa paliet, zámena farieb a farebné rampy
*Komplexný farebný engine so zobrazením farby popredia a pozadia, automatickým generovaním dvojbodových gradientových prechodov a pomenovanými paletovými skinmi.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta a skiny v OniAnvil" width="950" />

<br/><br/>

### 6. Referenčný náčrt a nastaviteľný Onion Skinning
*Načítajte konceptuálne skice alebo fotografie ako podklad/prekrytie s ovládaním transformácie, inteligentným naberaním farieb s prichytením k palete a viacsnímkovým priesvitom.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Náčrt a Onion Skinning v OniAnvil" width="950" />

<br/><br/>

### 7. Body ukotvenia (Pivot) a herné akčné body
*Definujte stred spritu (Origin) a vlastné pomenované herné body (úchyty zbraní, emitory projektilov, hlava, kroky) pomocou interaktívnych manipulátorov na plátne.*
<img src="screenshots/pivot_and_action_points.png" alt="Body ukotvenia a akčné body v OniAnvil" width="950" />

<br/><br/>

### 8. Delenie hárkov (Sheet Slicer) a škálovanie 9-Grid (9-Slice)
*Automatické rozpoznávanie hraníc spritov autokoreláciou (napr. chôdza mačky) spoločne s nastavením okrajov pre škálovateľné prvky používateľského rozhrania 9-slice.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Delenie hárkov v OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Škálovanie 9-Slice v OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Živé obrazové efekty a filtre (s náhľadom v reálnom čase)
*Všetky dialógové okná efektov ponúkajú živý náhľad na plátne, interaktívne kvapkadlá a možnosť nedeštruktívneho zrušenia bez znečistenia histórie krokov späť.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generátor obrysu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Nahradenie farby" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Odtieň, sýtosť, kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Maticové filtre" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Premapovanie do palety" width="460" />
</p>

<br/><br/>

### 10. Režimy mriežky a pozadia plátna
*Plynule prepínajte medzi pixelovou mriežkou, mriežkou snímok, tmavou šachovnicou, svetlou šachovnicou a vlastnými jednoliatymi farbami pozadia.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Mriežka a tmavá šachovnica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Svetlá šachovnica bez mriežky" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Vlastné jednoliate pozadie" width="300" />
</p>

<br/><br/>

### 11. Sprievodca vytvorením nového projektu
*Pri spustení projektu ľahko nastavte rozmery snímky, počet snímok, stĺpcovú mriežku atlasu a vyberte si z viac ako 20 autentických retro paliet.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Sprievodca novým projektom" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palety nového projektu" width="460" />
</p>

<br/><br/>

### 12. 35 autentických retro a moderných motívov
*Zahŕňa Studio Dark, Studio Light, fosfory CRT (jantárový, zelený, azúrový), retro konzoly (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) aj dizajnové motívy.*
<p align="center">
  <img src="screenshots/themes.png" alt="Ponuka motívov" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Motív CRT jantárový" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Motív PSX" width="300" />
</p>

<br/><br/>

### 13. Systém ponúk a integrovaný pomocník klávesových skratiek
<p align="center">
  <img src="screenshots/file_menu.png" alt="Ponuka Súbor" width="180" />
  <img src="screenshots/edit_menu.png" alt="Ponuka Úpravy" width="180" />
  <img src="screenshots/select_menu.png" alt="Ponuka Výber" width="180" />
  <img src="screenshots/view_menu.png" alt="Ponuka Zobrazenie" width="180" />
  <img src="screenshots/project_menu.png" alt="Ponuka Projekt" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Okno pomocníka klávesových skratiek" width="700" />
</p>

</div>

---

## Kompletný prehľad nástrojov

OniAnvil ponúka 15 precíznych nástrojov na kreslenie, geometriu, výber a správu metadát. Ikony využívajú otvorenú sadu **Font Awesome Free** *(ikony: CC BY 4.0, písma: SIL OFL 1.1)*.

| Nástroj | Ikona | Skratka | Veľkosť štetca | Pixel Perfect | Popis |
|:---|:---:|:---:|:---:|:---:|:---|
| **Ceruzka** | ✏️ | <kbd>B</kbd> | 1..32 px | Áno | Základný nástroj na kreslenie. V režime 1px podporuje kreslenie **Pixel-Perfect** (automaticky odstraňuje nežiaduce rohové L-schodíky). Podporuje vlastné pečiatkové štetce z výberu (`Ctrl/Cmd+B`). |
| **Guma** | 🧹 | <kbd>E</kbd> | 1..32 px | Nie | Maže pixely späť do úplnej priehľadnosti (`[0, 0, 0, 0]`). Je ohraničená maskou výberu, ak je aktívna. |
| **Kvapkadlo** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nie | Kliknutie ĽTM naberie farbu do Popredia (hlavná); PTM naberie do Pozadia (vedľajšia). Môže snímať farbu z plátna, vrstiev alebo referenčného náčrtu. |
| **Plechovka farby** | 🪣 | <kbd>G</kbd> | N/A | Nie | Bleskurýchla frontová výplň (BFS) s nastaviteľnou toleranciou farieb (`0..=255`). Podporuje súvislý aj globálny režim a ohraničenie výberom. |
| **Prechod** | 🌈 | <kbd>G</kbd> *(prepínač)* | N/A | Nie | Lineárne a radiálne prechody medzi farbou popredia a pozadia. Podporuje maticový dithering Bayer (2x2, 4x4, 8x8) aj Blue Noise s reguláciou sily. |
| **Čiara** | 📏 | <kbd>L</kbd> | 1..32 px | Áno | Bresenhamov algoritmus priamok so živým náhľadom pri ťahaní. Podržaním klávesu <kbd>Shift</kbd> uzamknete uhly v krokoch po 45°/90°. |
| **Krivka** | 〰️ | <kbd>C</kbd> | 1..32 px | Áno | 3-bodová kvadratická Bézierova krivka. Kliknutie 1 nastaví začiatok, kliknutie 2 koniec, pohyb myši nastaví prehnutie a kliknutie 3 krivku potvrdí. |
| **Obdĺžnik** | ⬜ | <kbd>U</kbd> | 1..32 px | Nie | Geometrické obdĺžniky s obrysom alebo výplňou. Podržaním <kbd>Shift</kbd> zachováte pomer strán 1:1 (štvorec). |
| **Elipsa** | ⭕ | <kbd>O</kbd> | 1..32 px | Nie | Obrysové alebo vyplnené elipsy pomocou stredovej rasterizácie. Podržaním <kbd>Shift</kbd> vytvoríte dokonalý kruh. |
| **Štetec ditheringu** | 🏁 | <kbd>D</kbd> | 1..32 px | Nie | Kreslí pravidelné šachovnicové vzory ditheringu striedaním farby popredia a pozadia bez zavádzania nových odtieňov do palety. |
| **Nástroj tieňovania** | 🌓 | <kbd>S</kbd> | 1..32 px | Nie | Postupné zosvetľovanie (Dodge) a stmavovanie (Burn) pixelov podľa poradia farieb v aktívnej palete. |
| **Obdĺžnikový výber** | ⬚ | <kbd>M</kbd> | N/A | Nie | Obdĺžnikový výber oblasti s animovaným obrysom pochodujúcich mravcov. Podporuje pridávanie (<kbd>Shift</kbd>) aj odoberanie (<kbd>Alt</kbd>). |
| **Laso výber** | ➰ | <kbd>Q</kbd> | N/A | Nie | Voľný mnohouholníkový výber od ruky. Využíva 8-spojitú rasterizáciu obrysu so 4-spojitou vonkajšou výplňou: 100% plný výber bez medzier aj pri prekrížení slučiek. |
| **Kúzelná palička** | 🪄 | <kbd>W</kbd> | N/A | Nie | Vyberie súvislé (alebo globálne) oblasti zodpovedajúcej farby s nastaviteľným prahom tolerancie. |
| **Bod ukotvenia a akcie** | 🎯 | <kbd>P</kbd> | N/A | Nie | Interaktívne umiestnenie stredu spritu (Origin) a vlastných herných akčných bodov (`weapon`, `head`, `feet`, `hitbox`) exportovaných do metadát JSON. |

---

## Funkčná dokumentácia

### 1. Architektúra založená na atlase ako jedinom zdroji pravdy

V OniAnvil predstavuje hlavná textúra atlasu (`atlas_rgba`) skutočnú textúru sprite sheetu, ako ju načítava herný engine:
- **Režim snímky (Frame Mode)**: Sústreďuje sa na jedinú aktívnu snímku animácie a poskytuje nerušené pixelové plátno. Všetky úpravy okamžite aktualizujú zodpovedajúcu bunku v textúre atlasu.
- **Režim atlasu (Atlas Mode)**: Zobrazuje celú mriežku sprite sheetu, čo umožňuje makro-úpravy, kontrolu zarovnania medzi snímkami aj dávkové úpravy.
- **Sprievodný súbor (Companion File)**: Projekty ukladajú čisté PNG atlasy doplnené ľahkým metadátovým súborom `.onianvil.json` s rozvrhnutím snímok, hierarchiou vrstiev, tagmi, dĺžkami trvania a paletovými skinmi.

### 2. Viacstopová časová os vrstiev

Spodná časová os vychádza z princípov profesionálneho nelineárneho strihu videa:
- **Stopy vrstiev (`V1, V2, V3...`)**: Vrstvy sú vizuálne ukladané zhora nadol. Stopa umiestnená vyššie v časovej osi (`V1`) sa vykresľuje nad stopami ležiacimi nižšie.
- **Ovládanie stôp**:
  - Kliknutím na odznaky `V1` / `V2` rýchlo prepnete aktívnu kresliacu vrstvu.
  - Prepínajte viditeľnosť vrstvy (ikona Oka) a zámok úprav (ikona Zámku).
  - Nastavujte priehľadnosť jednotlivých vrstiev (`0%..=100%`).
  - Posúvajte vrstvy nahor/nadol v hierarchii, duplikujte ich a zlučujte so stopou nižšie (Merge Down).
  - Bezpečne odstraňujte vrstvy s potvrdzovacím dialógom.
- **Klip snímky**:
  - Násobič dĺžky (Hold): priraďujte vlastné dĺžky trvania snímok (napr. `2×`, `3×` pre pauzy alebo rázové snímky).
  - Prepojené snímky (`🔗`): prepojte opakujúce sa snímky naprieč animáciou, takže úprava jednej automaticky aktualizuje všetky previazané inštancie.
  - Zacyklenie podľa tagov: organizujte animácie do pomenovaných tagov (`idle`, `walk`, `attack`) a prehrávajte v slučke vybrané sekvencie.
  - Ovládanie prehrávania: Prehrávanie vpred, vzad, ping-pong a slučka s nastaviteľnou rýchlosťou FPS (`1..=120`).

### 3. Živý náhľad animácie v reálnom čase

Pravý dok náhľadu prehráva vaše animácie v reálnom čase úplne nezávisle od mierky alebo posunu hlavného plátna:
- **Filtrovanie tagov**: Izolujte prehrávanie na konkrétne tagy (`walk`, `jump`) alebo sledujte celú sekvenciu animácie.
- **Ovládanie prehrávania**: Tlačidlá Play, Pause, krok späť, krok vpred, prepínač slučky a vlastné násobiče rýchlosti (`0.25x..=4.0x`).
- **Režimy zobrazenia**: Prepínajte medzi tmavou šachovnicou, svetlou šachovnicou, čisto bielym, čisto čiernym alebo vlastným pozadím.

### 4. Pokročilý systém pauzovania po okienkach (Onion Skinning)

Vytvárajte plynulé animácie s využitím špecializovaného systému pauzovania:
- **Nezávislé tónovanie**: Samostatné farby zafarbenia pre predchádzajúce snímky (predvolené: červená) a nasledujúce snímky (predvolené: modrá).
- **Úbytok priehľadnosti**: Nastaviteľná východisková priehľadnosť aj krivky postupného zoslabovania pre ďalšie kroky pauzovania.
- **Zacyklenie cez okraj (Loop Wrap-Around)**: Umožňuje pauzovanie medzi prvou a poslednou snímkou zacyklených animácií.
- **Rýchly náhľad (Quick-Peek)**: Podržaním klávesu <kbd>Medzerník</kbd> bez ťahania myši okamžite zobrazíte susedné snímky.

### 5. Inteligentná vrstva referenčného náčrtu

Urýchlite si prácu obkresľovaním ručných skíc alebo referenčných fotografií:
- **Formáty**: Načítajte obrázky vo formátoch PNG, JPEG a WebP.
- **Režimy prelínania**: Podklad (pod pixelovými vrstvami) alebo Prekrytie (nad pixelovými vrstvami).
- **Ovládanie transformácie**: Nezávislé priblíženie, posun pozície, otočenie a posuvník priehľadnosti.
- **Inteligentné vzorkovanie farieb**: Používajte kvapkadlo na vrstve náčrtu s voliteľným automatickým prichytením k najbližšej farbe v aktívnej palete projektu.
- **Odtlačok na jedno kliknutie (1-Click Bake)**: Prevod zväčšeného referenčného obrázka priamo na čistý pixel art v aktívnej snímke.

### 6. Presné nástroje výberu a pochodujúce mravce

- **Animované pochodujúce mravce**: Aktívny výber sa zobrazuje pomocou zreteľného dvojfarebného animovaného prerušovaného obvodu.
- **Plný výber lasom na báze výplne**: Na rozdiel od primitívnych lúčových algoritmov, ktoré vytvárajú nevybraté diery pri prekrývaní slučiek, laso v OniAnvil rasterizuje uzavretý 8-spojitý obvod a vypĺňa vonkajší priestor, čo zaručuje 100% plný výber bez nechcených dier.
- **Operácie s výberom**: Rozšírenie, Zúženie, Obrys (s kruhovým, štvorcovým a kosoštvorcovým tvarom jadra), Posun, Invertovať, Zmazať, Voľné otočenie a Schránka systému (Kopírovať/Vystrihnúť/Prilepiť).
- **Kopírovanie a vkladanie medzi snímkami**: Vyberte oblasť na jednej snímke, stlačte <kbd>Ctrl/Cmd+C</kbd>, prepnite snímku a stlačte <kbd>Ctrl/Cmd+V</kbd> pre vloženie na identické relatívne súradnice pri akomkoľvek rozložení klávesnice.

### 7. Živé efekty, filtre a RotSprite

Všetky obrazové efekty obsahujú začiarkavacie pole **Živý náhľad (Live Preview)**, ktoré zobrazuje úpravy na plátne a umožňuje čisté zrušenie bez znečistenia histórie krokov späť:
- **Nahradenie farby (`Shift+R`)**:
  - Interaktívne kvapkadlo: kliknite na ikonu kvapkadla a vyberte farby priamo z plátna alebo doku palety.
  - Lišta rýchlej palety: kliknutím ĽTM vyberiete farbu „Z“, kliknutím PTM farbu „Do“.
  - Posuvník tolerancie a hromadné nahradenie naprieč všetkými snímkami animácie.
- **Farebné úpravy (`Ctrl/Cmd+U`)**: Nastavenie posunu odtieňa HSV (Hue), sýtosti (Saturation), jasu (Value) a kontrastu v reálnom čase.
- **Generátor automatického obrysu**: Vytvárajte ostré pixelové obrysy postáv (Vonkajší, Vnútorný, Na stred) so štvorcovými alebo kosoštvorcovými rohmi.
- **Konvolučné maticové filtre 3×3**: Rozostrenie, Zaostrenie, Detekcia hrán a Reliéf s možnosťou zachovania alfa kanála.
- **Voľné otočenie (`Ctrl/Cmd+Shift+R`)**: Otáčajte vybrané oblasti alebo celé snímky o ľubovoľný uhol pomocou klasických algoritmov **RotSprite** alebo **Nearest Neighbor**.
- **Premapovanie do palety**: Kvantovanie farieb plátna do aktívnej palety s ditheringom Bayer (2x2, 4x4, 8x8) alebo Blue Noise.

### 8. Správa paliet, zámena farieb a skiny postáv

- **Vstavané palety**: Viac ako 20 autentických retro paliet vrátane Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized a Sweetie 16.
- **Import a export**: Podpora formátov GIMP Palette (`.gpl`), PAL (`.pal`), Hex (`.hex`) a obrázkových vzorkovníkov (`.png`).
- **Zámena paliet (Skiny postáv)**: Definujte niekoľko variantov skinov pre jednu postavu (napr. Predvolený, Zlaté brnenie, Tieň, Zmrznutý). Zámena skinu okamžite premapuje všetky farby snímok v celom atlase.
- **Generátor farebných rámp**: Automatická interpolácia plynulých gradientov medzi dvoma zvolenými odtieňmi s nastaviteľným počtom medzikrokov.

### 9. Delenie hárkov (Sheet Slicer), 9-Grid UI a export pre herné enginy

- **Delenie hárkov (Sheet Slicer)**: Autokorelačná detekcia hraníc automaticky rozreže surové sprite sheety na rovnomerné alebo presne ohraničené animačné snímky.
- **Škálovanie 9-Grid (9-Slice)**: Nastavte prispôsobiteľné okraje rezov (hore, dole, vľavo, vpravo) pre náhľad škálovateľných okien a tlačidiel používateľského rozhrania.
- **Export PNG s extruziou okrajov**: Rozširuje farby okrajových pixelov do priehľadných okrajov, čo odstraňuje švy pri filtrovaní textúr v OpenGL, DirectX, Vulkane aj 2D herných enginoch (ako je Godot).
- **Export animovaného GIF**: Vytvárajte optimalizované súbory GIF s nastaviteľným počtom slučiek, oneskorením snímok a priehľadným pozadím.
- **Štandardný formát Sprite Sheet JSON**: Exportuje súradnice snímok, tagy, doby trvania a body ukotvenia zodpovedajúce štandardnej schéme JSON v moderných herných enginoch.

### 10. 35 ergonomických motívov a úplná lokalizácia

- **Vybrané motívy**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT fosfory (Jantár, Zelená, Azúrová), retro konzoly (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) a retro počítače (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Podpora 37 jazykov**: Plynulé prepínanie jazykov v reálnom čase so 100% paritou všetkých reťazcov a pomocníkov.

---

## Ovládanie a klávesové skratky

| Skratka | Nástroj / Akcia | Popis |
|:---|:---|:---|
| <kbd>B</kbd> | **Ceruzka** | Presné kreslenie 1px s voliteľným režimom Pixel-Perfect |
| <kbd>E</kbd> | **Guma** | Mazanie pixelov späť do priehľadnosti |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Kvapkadlo** | Naberanie farby z plátna alebo referenčného náčrtu |
| <kbd>G</kbd> | **Výplň / Prechod** | Prepínanie medzi plechovkou farby a prechodom |
| <kbd>L</kbd> | **Čiara** | Kreslenie rovných čiar so živým náhľadom a prichytením cez Shift |
| <kbd>C</kbd> | **Krivka** | 3-bodová kvadratická Bézierova krivka |
| <kbd>U</kbd> | **Obdĺžnik** | Kreslenie obdĺžnikov a štvorcov (obrys alebo výplň) |
| <kbd>O</kbd> | **Elipsa** | Kreslenie elíps a kruhov (obrys alebo výplň) |
| <kbd>D</kbd> | **Štetec ditheringu** | Kreslenie maticovým šachovnicovým vzorom ditheringu |
| <kbd>S</kbd> | **Nástroj tieňovania** | Postupné zosvetľovanie / stmavovanie v rámci palety |
| <kbd>M</kbd> | **Obdĺžnikový výber** | Výber obdĺžnikovej oblasti s pochodujúcimi mravcami |
| <kbd>Q</kbd> | **Laso výber** | Plný voľný polygonálny výber od ruky |
| <kbd>W</kbd> | **Kúzelná palička** | Výber súvislých oblastí rovnakej farby |
| <kbd>P</kbd> | **Body ukotvenia a akcie** | Umiestnenie stredu spritu a akčných bodov |
| <kbd>X</kbd> | **Prehodiť farby** | Prehodenie farby popredia a pozadia |
| <kbd>[</kbd> / <kbd>]</kbd> | **Veľkosť štetca** | Zmenšenie / zväčšenie veľkosti štetca (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopírovať** | Skopírovanie vybraných pixelov do schránky |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Vystrihnúť** | Vybratie vybraných pixelov do schránky |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Prilepiť** | Vloženie pixelov zo schránky do aktívnej snímky |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Zmazať** | Vymazanie pixelov vo vnútri aktívneho výberu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Vybrať všetko** | Výber všetkých pixelov v aktívnej snímke |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Zrušiť výber** | Odstránenie aktívnej masky výberu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invertovať výber** | Obrátenie aktívnej masky výberu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplikovať snímku** | Duplikácia aktívnej snímky v časovej osi |
| <kbd>Medzerník</kbd> | **Prehrať / Pozastaviť** | Prepínanie prehrávania animácie v časovej osi |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Nahradiť farbu** | Otvorenie dialógu nahradenia farby so živým náhľadom |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Farebné úpravy** | Otvorenie dialógu úpravy HSV odtieňa, sýtosti a kontrastu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Voľné otočenie** | Otvorenie dialógu otočenia o ľubovoľný uhol (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Späť** | Vrátenie posledného ťahu alebo akcie nástroja |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Znova** | Opakovanie predtým vrátenej akcie |
| <kbd>Medzerník</kbd> + Ťahanie / Stredné tlačidlo | **Posun plátna** | Plynulý nekonečný posun po súradniciach plátna |
| <kbd>Ctrl/Cmd</kbd> + Koliesko / <kbd>+</kbd> / <kbd>-</kbd> | **Priblíženie** | Ostré priblíženie so stredom na kurzore myši |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Režim zobrazenia** | Prepínanie medzi zobrazením snímky (<kbd>1</kbd>) a atlasu (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Prispôsobiť plátno** | Automatické prispôsobenie a vycentrovanie aktívnej snímky |
| <kbd>F1</kbd> | **Pomocník** | Otvorenie okna pomocníka klávesových skratiek |

---

## Začíname

### Samostatné binárne súbory

Stiahnite si predkompilované, samostatné spustiteľné súbory zo **[stránky vydaní na GitHube](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Stiahnite súbor `OniAnvil-windows-x86_64.zip`, rozbaľte ho a spustite `OniAnvil.exe`.
- **Linux**: Stiahnite archív `OniAnvil-linux-x86_64.tar.gz` (prenosná verzia) alebo nainštalujte balík `onianvil_*_amd64.deb` pre Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Stiahnite súbor `OniAnvil-macos-universal.zip`, rozbaľte ho a pretiahnite aplikáciu `OniAnvil.app` do priečinka Aplikácie. Univerzálny binárny súbor beží natívne na procesoroch Apple Silicon (M1/M2/M3/M4) aj na počítačoch Mac s procesormi Intel.
- **Web**: Spustite priamo v prehliadači bez inštalácie: **[Spustiť aplikáciu WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=sk)**.
  > [!WARNING]
  > Verzia pre webové prehliadače (WebAssembly) má status **Experimentálna (Experimental)**. Vzhľadom na obmedzenia sandboxu prehliadača, odlišnosti ovládačov WebGL/WebGPU a limitácie systémovej schránky sa môžu vyskytnúť drobné vizuálne chyby či obmedzenia výkonu v porovnaní s natívnou desktopovou verziou. Pre plnohodnotnú produkčnú prácu dôrazne odporúčame natívne desktopové zostavenia.

---

### Zostavenie zo zdrojových kódov

#### Požiadavky
- **Sada nástrojov Rust**: Nainštalujte Rust (odporúčaná verzia 1.80+) prostredníctvom [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Závislosti zostavenia pre Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompilácia a spustenie natívnej verzie pre počítače
```bash
# Naklonujte repozitár
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Spustite sadu testov
cargo test

# Zostavte a spustite optimalizovanú verziu vydania
cargo run --release
```

#### 2. Kompilácia verzie WebAssembly (pomocou Trunk)
```bash
# Pridajte cieľ kompilácie wasm32 a nainštalujte nástroj Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Spustite lokálny vývojový server s automatickým obnovovaním
trunk serve

# Zostavte optimalizované produkčné súbory do priečinka docs/app/
./build_wasm.sh        # Na systémoch Linux/macOS
./build_wasm.ps1       # V prostredí Windows PowerShell
```

---

## Formát súborov a sprievodná architektúra

OniAnvil nepoužíva proprietárne binárne formáty. Každý projekt tvoria dva čisté súbory pripravené na verzovanie:

```
moj_hrdina/
├── moj_hrdina.png            # Štandardný atlas sprite sheetu v PNG (otvoríte v akomkoľvek engine aj prehliadači)
└── moj_hrdina.onianvil.json   # Sprievodný súbor metadát JSON
```

Sprievodný súbor `.onianvil.json` ukladá:
- Konfiguráciu mriežky plátna: rozmery snímok, počet stĺpcov, celkový počet snímok.
- Hierarchie vrstiev: názvy stôp, úrovne priehľadnosti, príznaky viditeľnosti a zámku.
- Animačné tagy: názov, indexy počiatočnej a koncovej snímky, smer prehrávania (Vpred, Vzad, Ping-Pong), doby trvania.
- Definície paliet: aktívne farebné vzorky, pomenované varianty skinov, index aktívneho skinu.
- Stav referenčného náčrtu: cestu k obrázku, mierku, posun, uhol otočenia, priehľadnosť, režim vrstvy.
- Body ukotvenia a akčné body: stred spritu (Origin) a vlastné herné akčné body (úchyty zbraní, body zásahu).

---

## Ako prispieť a poďakovanie

Vrelo vítame akékoľvek príspevky, hlásenia chýb aj návrhy nových funkcií!
1. Vytvorte fork repozitára na GitHube.
2. Vytvorte vetvu funkcie: `git checkout -b feature/moj-novy-nastroj`.
3. Potvrďte zmeny do gitu: `git commit -am 'Pridanie nového pixelového nástroja'`.
4. Odošlite vetvu do svojho forku: `git push origin feature/moj-novy-nastroj`.
5. Otvorte Pull Request.

### Poďakovanie
- Ukážkový demonštračný sprite animácie mačky **"Cat 2D Pixel Art"** vytvoril autor **xzany** ([stránka na itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Vytvorené pomocou skvelej knižnice GUI **[egui](https://github.com/emilk/egui)** a grafického ekosystému **[wgpu](https://wgpu.rs/)**.
- Ikony rozhrania využívajú sadu **[Font Awesome Free](https://fontawesome.com)** od @fontawesome (ikony: CC BY 4.0, písma: SIL OFL 1.1, kód: licencia MIT).

---

## Licencia a zrieknutie sa zodpovednosti

Tento projekt je licencovaný pod licenciou **[MIT License](../LICENSE)**. OniAnvil môžete slobodne používať, upravovať, šíriť a integrovať do komerčných aj nekomerčných herných projektov.

### Zrieknutie sa záruk a obmedzenie zodpovednosti
> [!IMPORTANT]
> Softvér **OniAnvil** sa šíri **úplne zadarmo** na báze **„TAK, AKO JE“ („AS IS“)** a **„POKÝM JE K DISPOZÍCII“ („AS AVAILABLE“)**, bez akýchkoľvek záruk alebo podmienok, či už výslovných alebo predpokladaných.
> 
> Autori, držitelia autorských práv ani prispievatelia **v žiadnom prípade nenesú zodpovednosť** za akékoľvek priame, nepriame, náhodné, osobitné, exemplárne alebo následné škody (vrátane, ale bez obmedzenia na stratu dát, poškodenie súborov, zlyhanie hardvéru, ušlý zisk alebo prerušenie podnikania) vzniknuté akýmkoľvek spôsobom v dôsledku používania alebo nemožnosti používať tento softvér, a to aj v prípade, že boli na možnosť takejto škody upozornení. Všetku zodpovednosť a riziká spojené s používaním tohto programu preberá používateľ.

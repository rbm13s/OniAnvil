<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil logó" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixel art és hagymahéj (onion-skinning) animációs stúdió játékra kész atlaszokhoz**

[![Építés és kiadás](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Letöltések](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderelő](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Nyelvek](https://img.shields.io/badge/Nyelvek-37_nyelv-purple)](#10-35-ergonomikus-téma-és-teljes-honosítás)
[![Platformok](https://img.shields.io/badge/Platformok-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#első-lépések)
[![Webes demó](https://img.shields.io/badge/Online_alkalmazás-Kipróbálás_böngészőben_(kísérleti)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=hu)
[![Licenc: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Próbálja ki az OniAnvil-t a böngészőben (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=hu)** *(⚠️ Kísérleti webes verzió: WebGL/WASM korlátozások és hibák előfordulhatnak)* • **[🌐 Hivatalos dokumentációs weboldal](https://rbm13s.github.io/OniAnvil/hu.html)**

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
**Magyar** •
[Română](README-ro.md) •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Tartalomjegyzék

- [Bevezetés](#bevezetés)
- [Képernyőképek és vizuális bemutató](#képernyőképek-és-vizuális-bemutató)
- [Részletes eszközreferencia](#részletes-eszközreferencia)
- [Funkcionális dokumentáció](#funkcionális-dokumentáció)
  - [1. Duális atlaszközpontú architektúra](#1-duális-atlaszközpontú-architektúra)
  - [2. Többsávos réteg idővonal](#2-többsávos-réteg-idővonal)
  - [3. Valós idejű élő animációs előnézet](#3-valós-idejű-élő-animációs-előnézet)
  - [4. Fejlett hagymahéj (Onion Skinning) motor](#4-fejlett-hagymahéj-onion-skinning-motor)
  - [5. Okos referencia és vázlatkövető réteg](#5-okos-referencia-és-vázlatkövető-réteg)
  - [6. Precíziós kijelölőkészlet és masírozó hangyák](#6-precíziós-kijelölőkészlet-és-masírozó-hangyák)
  - [7. Élő effektek, szűrők és RotSprite](#7-élő-effektek-szűrők-és-rotsprite)
  - [8. Palettakezelés, csere és karakterskinek](#8-palettakezelés-csere-és-karakterskinek)
  - [9. Sprite lap szeletelő, 9-Grid felület és játékmotor export](#9-sprite-lap-szeletelő-9-grid-felület-és-játékmotor-export)
  - [10. 35 ergonomikus téma és teljes honosítás](#10-35-ergonomikus-téma-és-teljes-honosítás)
- [Irányítás és gyorsbillentyűk](#irányítás-és-gyorsbillentyűk)
- [Első lépések](#első-lépések)
  - [Önálló futtatható állományok](#önálló-futtatható-állományok)
  - [Fordítás forráskódból](#fordítás-forráskódból)
- [Fájlformátum és kísérő architektúra](#fájlformátum-és-kísérő-architektúra)
- [Közreműködés és köszönetnyilvánítás](#közreműködés-és-köszönetnyilvánítás)
- [Licenc és felelősségkizárás](#licenc-és-felelősségkizárás)

---

## Bevezetés

Az **OniAnvil** (*a hagymahéj animációt jelentő **Oni**on-skinning és a sprite-kovácsoló **Anvil** (üllő) szavak összetételéből*) egy nyílt forráskódú, kimagasló teljesítményű pixel art szerkesztő és animációs szoftvercsomag, amely Rust nyelven íródott. Natívan fut hardveresen gyorsított rendereléssel Windows, Linux és macOS rendszereken, valamint telepítés nélkül, közvetlenül a modern webböngészőkben WebAssembly segítségével.

A legtöbb hagyományos képszerkesztő az animációkra különálló képfájlok halmazaként tekint, ami a fejlesztőket unalmas exportálási, szeletelési és újrarendezési lépésekre kényszeríti. **Az OniAnvil megfordítja ezt a paradigmát:** a játékmotor által betöltött sprite lap atlasz textúrája (`atlas_rgba`) jelenti az egyedüli hiteles forrást (source of truth). A képkockák cellái, a rétegsávok, az animációs címkék és az ütközési forgáspontok mindvégig roncsolásmentesen és folyamatosan szinkronban maradnak az atlasszal.

### Miért az OniAnvil?
- 🚀 **Villámgyors natív Rust teljesítmény**: Azonnali, ezredmásodperc alatti indulás, a `wgpu` és az `egui 0.31` által biztosított kristálytiszta renderelés, és nulla akadás a hosszú idővonalak görgetésekor.
- 🎬 **Többsávos réteg idővonal**: Professzionális vágóprogramok által ihletett rétegsávok (`V1, V2, V3...`) fentről lefelé történő kompozitálással, rétegek lefelé egyesítésével, összekapcsolt képkockákkal és egyéni tartási szorzókkal.
- 🧅 **Első osztályú hagymahéj (Onion Skinning)**: Többkockás szellemkép megjelenítés független előre/hátra színezéssel, testreszabható átlátszósági görbékkel és hurok átfordulással.
- 🎨 **Azonnali palettavariációk és karakterskinek**: Hozzon létre nappali/éjszakai változatokat, csapat- vagy elemi skineket az atlasz valós idejű újraképezésével, valamint GPL, PAL és HEX paletták importálásával és exportálásával.
- 📦 **Közvetlen játékmotor export**: Exportáljon méretezett PNG képeket extrudált (kihúzott) szegélyekkel a textúraszűrési hibák (texture bleeding) megelőzésére 2D motorokban (Godot, Bevy, Raylib, Defold), a standard sprite sheet JSON metaadatokkal és animált GIF-ekkel együtt.

---

## Képernyőképek és vizuális bemutató

<div align="center">

### 1. Fő vászon és munkaterület
*Nagy teljesítményű pixel art munkaterület hardveresen gyorsított nézőkével, többsávos idővonallal, valós idejű élő előnézettel, palettamintákkal és precíziós eszköztárakkal.*
<img src="screenshots/main.png" alt="OniAnvil Fő munkaterület" width="950" />

<br/><br/>

### 2. Többsávos animációs idővonal
*Nemlineáris többsávos idővonal egymásra helyezett rétegsorokkal (`V1, V2...`), összekapcsolt képkocka példányokkal (`🔗`), kitartási szorzókkal és elnevezett, színkódolt animációs címkékkel.*
<img src="screenshots/timeline.png" alt="OniAnvil Többsávos idővonal" width="950" />

<br/><br/>

### 3. Precíziós eszközök és gyorsműveleti eszköztár
*Teljes 15 eszközös rajzkészlet normál és kompakt dokkolási módokkal, valamint felső gyorselérési sávval az ecsetmérethez, szimmetriához, átforduláshoz és Pixel-Perfect kapcsolóhoz.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Gyorsműveleti eszköztár" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Normál eszköztár" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Kompakt eszköztár" width="300" />
</p>

<br/><br/>

### 4. Valós idejű élő animációs előnézet
*Különálló élő lejátszási dokk címkeszűréssel, változtatható FPS sebességszabályzóval, nagyítási csúszkával és nézőke-háttér kapcsolókkal (sakktábla, egyszínű sötét, egyszínű világos).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Élő előnézet" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Élő előnézet nagyítás" width="460" />
</p>

<br/><br/>

### 5. Palettakezelés, csere és színskálák
*Átfogó színkezelő motor előtér- és háttérszín dobozokkal, automatikus kétpontos színátmenet generálással és elnevezett palettavariációkkal (skinek).*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Paletta és cserék" width="950" />

<br/><br/>

### 6. Referenciavázlat átrajzolás és konfigurálható hagymahéj
*Töltsön be koncepcióvázlatokat vagy fotókat alsó vagy felső rétegként transzformációs vezérlőkkel, intelligens színmintavételezéssel palettaillesztéssel és többkockás hagymahéjjal.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Vázlat és hagymahéj" width="950" />

<br/><br/>

### 7. Forgáspontok és akciópontok
*Határozza meg a sprite kezdőpontját (Origin) és az egyéni, elnevezett játékbeli pontokat (fegyverrögzítés, lövedékindítás, fej, láblépés) interaktív vászonsegédekkel.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Forgáspontok és akciópontok" width="950" />

<br/><br/>

### 8. Sprite lap szeletelő és 9-Grid (9-Slice) méretező
*Automatikus sprite határvonal-felismerő autokorrelációs szeletelés (pl. sétáló macska ciklus), valamint skálázható 9-szeletes felhasználói felület margóbeállítása.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Lap szeletelő" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Grid méretező" width="460" />
</p>

<br/><br/>

### 9. Valós idejű képeffektek és szűrőablakok (élő előnézettel)
*Minden effektus párbeszédablaka élő vászon-előnézetet, interaktív pipettát és roncsolásmentes elvetési lehetőséget biztosít.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Körvonal generátor" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Színcsere" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Árnyalat, telítettség és kontraszt" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Mátrix szűrő" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Palettához igazítás" width="460" />
</p>

<br/><br/>

### 10. Vászonrács és háttérmódok
*Váltson zökkenőmentesen a pixelrácsok, csemperácsok, sötét sakktábla, világos sakktábla és az egyéni egyszínű háttérszínek között.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Rács és sötét sakktábla" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Világos sakktábla rács nélkül" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Egyéni egyszínű háttér" width="300" />
</p>

<br/><br/>

### 11. Új projekt varázsló
*Állítsa be a projekt vászonméreteit, a képkockák számát, az atlasz oszlopelrendezését, és válasszon több mint 20 autentikus retro paletta közül indításkor.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Új projekt varázsló" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Új projekt paletták" width="460" />
</p>

<br/><br/>

### 12. 35 autentikus retro és modern téma
*Tartalmazza a Studio Dark, Studio Light témákat, a borostyán/zöld/cián CRT foszforokat, retro konzolokat (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) és tervezői témákat.*
<p align="center">
  <img src="screenshots/themes.png" alt="Témák menü" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Borostyán téma" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX téma" width="300" />
</p>

<br/><br/>

### 13. Menürendszer és beépített billentyűzetes súgó
<p align="center">
  <img src="screenshots/file_menu.png" alt="Fájl menü" width="180" />
  <img src="screenshots/edit_menu.png" alt="Szerkesztés menü" width="180" />
  <img src="screenshots/select_menu.png" alt="Kijelölés menü" width="180" />
  <img src="screenshots/view_menu.png" alt="Nézet menü" width="180" />
  <img src="screenshots/project_menu.png" alt="Projekt menü" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Gyorsbillentyűk súgóablak" width="700" />
</p>

</div>

---

## Részletes eszközreferencia

Az OniAnvil 15 precíziós rajz-, alakzat-, kijelölő- és metaadat-eszközt tartalmaz. Az ikonok megjelenítését a nyílt forráskódú **Font Awesome Free** ikoncsomag biztosítja *(Ikonok: CC BY 4.0, Betűkészletek: SIL OFL 1.1)*.

| Eszköz | Ikon | Gyorsbillentyű | Ecsetméret | Pixel Perfect | Leírás |
|:---|:---:|:---:|:---:|:---:|:---|
| **Ceruza** | ✏️ | <kbd>B</kbd> | 1..32 px | Igen | Elsődleges rajzeszköz. Az 1px-es mód támogatja a **Pixel-Perfect** rajzolást (automatikusan megszünteti a csúnya, lépcsőzetes L-alakú sarkokat). Támogatja a kijelölésekből létrehozott egyéni pecsét ecseteket (`Ctrl/Cmd+B`). |
| **Radír** | 🧹 | <kbd>E</kbd> | 1..32 px | Nem | Törli a képpontokat teljes átlátszóságra (`[0, 0, 0, 0]`). Aktív kijelölés esetén szigorúan azon belül működik. |
| **Pipetta** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nem | A bal kattintás az Előtérszínbe (Foreground), a jobb kattintás a Háttérszínbe (Background) vesz mintát. Mintát vehet vászonpixelekből, rétegekből vagy vázlatból. |
| **Festékes vödör** | 🪣 | <kbd>G</kbd> | N/A | Nem | Nagy sebességű soralapú BFS kitöltés konfigurálható színtoleranciával (`0..=255`). Támogatja az összefüggő és globális kitöltést, valamint a kijelölési határokat. |
| **Színátmenet** | 🌈 | <kbd>G</kbd> *(váltás)* | N/A | Nem | Lineáris és sugaras színátmenetek az előtér- és háttérszínek között. Támogatja a Bayer (2x2, 4x4, 8x8) és a Blue Noise ditheringet állítható erősséggel. |
| **Vonal** | 📏 | <kbd>L</kbd> | 1..32 px | Igen | Bresenham vonalalgoritmus élő húzási előnézettel. Tartsa lenyomva a <kbd>Shift</kbd> billentyűt a szögek 45°/90°-os rögzítéséhez. |
| **Görbe** | 〰️ | <kbd>C</kbd> | 1..32 px | Igen | 3 kattintásos másodfokú Bézier-görbe. 1. kattintás: Kezdőpont, 2. kattintás: Végpont, egérmozgatás: görbület beállítása, 3. kattintás: görbe rögzítése. |
| **Téglalap** | ⬜ | <kbd>U</kbd> | 1..32 px | Nem | Körvonalas vagy kitöltött geometriai téglalapok. Tartsa nyomva a <kbd>Shift</kbd> billentyűt 1:1 arányú négyzet rajzolásához. |
| **Ellipszis** | ⭕ | <kbd>O</kbd> | 1..32 px | Nem | Körvonalas vagy kitöltött ellipszisek felezőpontos raszterezéssel. Tartsa nyomva a <kbd>Shift</kbd> billentyűt tökéletes kör rajzolásához. |
| **Dither ecset** | 🏁 | <kbd>D</kbd> | 1..32 px | Nem | Váltakozó sakktáblamintás szórási (dither) textúrát fest az előtér- és háttérszín között, anélkül, hogy felesleges színeket adna a palettához. |
| **Árnyékoló eszköz** | 🌓 | <kbd>S</kbd> | 1..32 px | Nem | Lépésenkénti világosítás (Dodge) és sötétítés (Burn) az aktív paletta színrendezése alapján. |
| **Téglalap kijelölés** | ⬚ | <kbd>M</kbd> | N/A | Nem | Téglalap alakú területkijelölés animált masírozó hangyák kerettel. Hozzáadás (<kbd>Shift</kbd>) és kivonás (<kbd>Alt</kbd>) támogatása. |
| **Lasszó kijelölés** | ➰ | <kbd>Q</kbd> | N/A | Nem | Szabadkézi sokszög lasszó. 8-szorosan összefüggő határvonal-raszterezést és 4-szeresen összefüggő külső elárasztást használ, garantálva a 100%-ban tömör, lyukmentes kijelölést még hurkok esetén is. |
| **Varázspálca** | 🪄 | <kbd>W</kbd> | N/A | Nem | Összefüggő (vagy globális) azonos színű területek kijelölése állítható toleranciaküszöbbel. |
| **Forgáspont és akció** | 🎯 | <kbd>P</kbd> | N/A | Nem | A sprite kezdőpontjának (Origin) és egyéni játékmeneti akciópontjainak (`weapon`, `head`, `feet`, `hitbox`) interaktív elhelyezése a vásznon, JSON exportálással. |

---

## Funkcionális dokumentáció

### 1. Duális atlaszközpontú architektúra

Az OniAnvilban a mester textúraatlasz (`atlas_rgba`) a valódi sprite lap textúrát képviseli, pontosan úgy, ahogyan azt a játékmotor betölti.
- **Képkocka mód (Frame Mode)**: Egyetlen aktív animációs képkockára fókuszál, zavaró tényezőktől mentes pixelvásznat biztosítva. A módosítások azonnal frissítik a megfelelő cellát az atlasz textúrájában.
- **Atlasz mód (Atlas Mode)**: Megjeleníti a teljes sprite lap rácsot, lehetővé téve a makroszintű beállításokat, a képkockák közötti igazítás ellenőrzését és a kötegelt szerkesztést.
- **Kísérőfájl (Companion File)**: A projektek tiszta `.png` sprite lapokat mentenek egy könnyű `.onianvil.json` kísérőfájl mellett, amely tartalmazza a képkockák elrendezését, réteghierarchiáit, címkéit, kitartási szorzóit és palettavariációit.

### 2. Többsávos réteg idővonal

Az alsó idővonal a professzionális nemlineáris videovágó szoftverek mintájára készült:
- **Rétegsávok (`V1, V2, V3...`)**: A rétegek vizuálisan fentről lefelé helyezkednek el. Az idővonalon feljebb lévő sáv (`V1`) a képkocka vásznán felül jelenik meg.
- **Sávvezérlők**:
  - Kattintson a `V1` / `V2` jelvényekre az aktív rajzréteg gyors váltásához.
  - Váltsa a réteg láthatóságát (szem ikon) és zárolását (lakat ikon).
  - Állítsa be az egyéni réteg átlátszatlanságát (`0%..=100%`).
  - Mozgassa a réteget fel/le a hierarchiában, duplikálja a réteget, és olvassza be az alatta lévő sávba (Merge Down).
  - Törölje a réteget megerősítő párbeszédablakkal.
- **Képkocka klipek**:
  - Kitartási szorzó (Hold): egyéni képkocka-időtartamok hozzárendelése (pl. `2×`, `3×` tartás szünetekhez vagy becsapódási fázisokhoz).
  - Összekapcsolt képkockák: ismétlődő fázisok összekapcsolása az animációban, így az egyik szerkesztése automatikusan frissíti az összes kapcsolt példányt.
  - Címkeismétlés: rendezze az animációkat elnevezett címkékbe (`idle`, `walk`, `attack`), és ismételje meg a kiválasztott címkéket lejátszáskor.
  - Lejátszásvezérlők: Előre, Hátra, Ping-Pong és Hurkolt lejátszás állítható képkockasebességgel (`1..=120` FPS).

### 3. Valós idejű élő animációs előnézet

A jobb oldali Előnézet dokkolópanel valós időben játssza le az animációkat, teljesen függetlenül a vászon nagyításától vagy görgetési pozíciójától:
- **Címkeszűrés**: Korlátozza a lejátszást adott címkékre (`walk`, `jump`), vagy tekintse meg a teljes animációs sorozatot.
- **Lejátszásvezérlők**: Lejátszás, Szünet, Előző lépés, Következő lépés, Ismétlés kapcsoló és egyéni lejátszási sebességek (`0.25x..=4.0x`).
- **Megjelenítési módok**: Váltson sötét sakktábla, világos sakktábla, tiszta fehér, tiszta fekete vagy egyéni vászonhátterek között.

### 4. Fejlett hagymahéj (Onion Skinning) motor

Készítsen folyékony animációkat a dedikált hagymahéj-rendszer segítségével:
- **Egyéni színezés**: Független árnyalati színek az előző képkockákhoz (alapértelmezett: vörös) és a következő képkockákhoz (alapértelmezett: kék).
- **Átlátszósági lecsengés**: Konfigurálható alapértelmezett átlátszóság és lépésenkénti elhalványulási görbék.
- **Hurok átfordulás**: Engedélyezi a szellemképet a ciklikus animációk első és utolsó képkockája között.
- **Gyors betekintés (Quick-Peek)**: Tartsa lenyomva a <kbd>Szóköz</kbd> billentyűt egérhúzás nélkül a szomszédos képkockák azonnali előnézetéhez.

### 5. Okos referencia és vázlatkövető réteg

Gyorsítsa fel munkafolyamatát kézzel rajzolt vázlatok vagy referenciafotók átrajzolásával:
- **Támogatott formátumok**: Töltsön be PNG, JPEG és WebP referenciaképeket.
- **Keverési módok**: Alátétként (a pixelrétegek alatt) vagy Rátétként (a pixelrétegek felett).
- **Transzformációs vezérlők**: Független zoom, pozícióeltolás, elforgatás és átlátszósági csúszka.
- **Intelligens színmintavétel**: Használja a Pipettát a vázlatrétegen, opcionális automatikus igazítással az aktív projektpaletta legközelebbi színéhez.
- **1-kattintásos beégetés**: Olvassza be a méretezett referenciaképet közvetlenül tiszta pixel artként az aktív képkockára.

### 6. Precíziós kijelölőkészlet és masírozó hangyák

- **Animált masírozó hangyák**: Az aktív kijelölések animált, nagy kontrasztú, kéttónusú szaggatott kerettel jelennek meg.
- **Tömör elárasztásos lasszó kijelölés**: A felületes sugárkövető algoritmusokkal ellentétben, amelyek lyukakat hagynak az átfedő hurkok rajzolásakor, az OniAnvil lasszója zárt, 8-szorosan összefüggő határvonalat raszterez és elárasztja a külső teret, így 100%-ban tömör, hibátlan kijelölést garantál.
- **Kijelölési műveletek**: Bővítés, Szűkítés, Szegély (kör, négyzet és rombusz magformákkal), Eltolás, Megfordítás, Törlés, Szabad forgatás és Rendszer-vágólap Másolás/Kivágás/Beillesztés.
- **Képkockák közötti másolás és beillesztés**: Jelöljön ki egy területet az egyik kockán, nyomja meg a <kbd>Ctrl/Cmd+C</kbd> billentyűkombinációt, váltson kockát, és illessze be a <kbd>Ctrl/Cmd+V</kbd> gombbal azonos koordinátákra, bármilyen billentyűzetkiosztás mellett.

### 7. Élő effektek, szűrők és RotSprite

Minden képeffektus tartalmaz egy valós idejű **Élő előnézet** jelölőnégyzetet, amely közvetlenül a vásznon mutatja a módosításokat, és zökkenőmentes elvetést tesz lehetővé anélkül, hogy teleírná a visszavonási előzményeket:
- **Színcsere (`Shift+R`)**:
  - Interaktív pipetta: kattintson a pipetta ikonra, és válasszon színeket közvetlenül a képpontokból vagy a palettapanelről.
  - Gyorsválasztó palettasáv: bal kattintással kiválasztja a „Honnan” színt, jobb kattintással a „Hová” színt.
  - Tolerancia csúszka és kötegelt csere az animáció összes képkockáján egyszerre.
- **Színbeállítás (`Ctrl/Cmd+U`)**: Valós idejű HSV színárnyalat (Hue), telítettség (Saturation), érték/fényerő (Value) és kontraszt finomhangolás.
- **Automatikus körvonal-generátor**: Hozzon létre tűéles pixel körvonalakat (Külső, Belső, Középre igazított) négyzet vagy rombusz sarokillesztésekkel.
- **3×3 konvolúciós mátrixszűrők**: Életlenítés, Élesítés, Élkeresés és Domborítás alfa-csatorna megőrzésével.
- **Tetszőleges forgatás (`Ctrl/Cmd+R`)**: Forgassa el a kijelölt területeket vagy a teljes képkockát tetszőleges szögben a klasszikus **RotSprite** vagy a legközelebbi szomszéd (Nearest Neighbor) algoritmusokkal.
- **Palettához igazítás (Remap)**: Képezze le a vászon színeit az aktív palettára Bayer 2x2, 4x4, 8x8 vagy Blue Noise dithering alkalmazásával.

### 8. Palettakezelés, csere és karakterskinek

- **Beépített paletták**: Több mint 20 autentikus retro paletta, beleértve: Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized és Sweetie 16.
- **Importálás és exportálás**: GIMP paletta (`.gpl`), PAL (`.pal`), Hex paletta (`.hex`) és képi színcsíkok (`.png`) támogatása.
- **Palettacsere (Skinek)**: Hozzon létre több színvariációt egyetlen karakterhez (pl. Alapértelmezett, Arany páncél, Árnyék, Fagyos). A skinek cseréje azonnal újraképezi az összes képkocka színét az egész atlaszon.
- **Színskála-generátor**: Automatikusan sima színátmeneteket interpolál két kiválasztott színárnyalat között állítható köztes lépésszámmal.

### 9. Sprite lap szeletelő, 9-Grid felület és játékmotor export

- **Lap szeletelő (Sheet Slicer)**: Az autokorrelációs határfelismerés automatikusan egyenletes vagy szorosan illeszkedő animációs képkockákra szeleteli a nyers sprite lapokat.
- **9-Grid (9-Slice) méretezés**: Állítson be testreszabható szeletelési margókat (felső, alsó, bal, jobb) a skálázható felhasználói felületi panelek és gombok torzulásmentes előnézetéhez.
- **Extrudált szegélyű PNG export**: Kihúzza a szélső képpontok színeit az átlátszó margókba, megszüntetve a textúraszűrési szegélyhibákat és a villódzást OpenGL, DirectX, Vulkan és 2D motorokban (mint a Godot).
- **Animált GIF export**: Optimalizált animált GIF generálás egyéni ciklusszámmal, képkockakésleltetéssel és átlátszó háttérrel.
- **Standard Sprite Sheet JSON**: Exportálja a képkockák koordinátáit, címkéit, időtartamait és forgáspontjait a modern 2D játékmotorok által támogatott szabványos JSON formátumban.

### 10. 35 ergonomikus téma és teljes honosítás

- **Válogatott témák**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Borostyán CRT, Zöld CRT, Cián CRT, Retro konzolok (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) és Retro számítógépek (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Teljes körű honosítás**: Zökkenőmentes váltás 37 támogatott nyelv között, teljes szótári kulcsparitással és anyanyelvi fordításokkal.

---

## Irányítás és gyorsbillentyűk

| Gyorsbillentyű | Eszköz / Művelet | Leírás |
|:---|:---|:---|
| <kbd>B</kbd> | **Ceruza** | 1px precíziós rajzolás választható Pixel-Perfect móddal |
| <kbd>E</kbd> | **Radír** | Képpontok törlése teljes átlátszóságra |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipetta** | Színmintavételezés a vászonról vagy a vázlatról |
| <kbd>G</kbd> | **Festékes vödör / Átmenet** | Váltás a kitöltés és a dither színátmenet között |
| <kbd>L</kbd> | **Vonal** | Egyenes vonalak rajzolása élő előnézettel és Shift rögzítéssel |
| <kbd>C</kbd> | **Görbe** | 3 kattintásos másodfokú Bézier-görbe rajzolása |
| <kbd>U</kbd> | **Téglalap** | Geometriai téglalap és négyzet körvonal/kitöltés |
| <kbd>O</kbd> | **Ellipszis** | Geometriai ellipszis és kör körvonal/kitöltés |
| <kbd>D</kbd> | **Dither ecset** | Sakktáblamintás szórási mátrix festése |
| <kbd>S</kbd> | **Árnyékoló eszköz** | Lépésenkénti világosítás (Dodge) és sötétítés (Burn) a palettán |
| <kbd>M</kbd> | **Téglalap kijelölés** | Területkijelölés masírozó hangyákkal |
| <kbd>Q</kbd> | **Lasszó kijelölés** | Szabadkézi tömör sokszög kijelölés |
| <kbd>W</kbd> | **Varázspálca** | Egybefüggő színterületek kijelölése |
| <kbd>P</kbd> | **Forgáspontok** | Sprite kezdőpont (Origin) és akciópontok elhelyezése |
| <kbd>X</kbd> | **Színek felcserélése** | Előtér és háttér színek felcserélése |
| <kbd>[</kbd> / <kbd>]</kbd> | **Ecsetméret** | Ecsetméret csökkentése / növelése (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Másolás** | Kijelölt képpontok másolása a rendszer vágólapjára |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Kivágás** | Kijelölt képpontok kivágása a vágólapra |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Beillesztés** | Vágólapon lévő képpontok beillesztése az aktív képkockára |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Törlés** | Képpontok törlése az aktív kijelölésen belül |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Összes kijelölése** | Az aktív képkocka összes képpontjának kijelölése |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Kijelölés megszüntetése** | Aktív kijelölési maszk törlése |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Kijelölés megfordítása** | Kijelölési maszk invertálása |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Képkocka duplikálása** | Aktív képkocka megkettőzése az idővonalon |
| <kbd>Szóköz</kbd> | **Lejátszás / Szünet** | Animáció lejátszásának indítása és megállítása |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Színcsere** | Színcsere ablak megnyitása élő előnézettel |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Színbeállítás** | HSV árnyalat / telítettség / kontraszt ablak megnyitása |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Szabad forgatás** | Tetszőleges szögű forgatás megnyitása (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Visszavonás** | Utolsó vászonművelet vagy ecsetvonás visszavonása |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Újra** | Visszavont művelet ismételt végrehajtása |
| <kbd>Szóköz</kbd> + Húzás / Egér középső | **Vászon mozgatása** | Sima, végtelen pásztázás a vászon koordinátáin |
| <kbd>Ctrl/Cmd</kbd> + Görgő / <kbd>+</kbd> / <kbd>-</kbd> | **Nagyítás** | Pixel-éles nagyítás az egérkurzor körül |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Nézetmód** | Váltás Képkocka nézet (<kbd>1</kbd>) és Atlasz nézet (<kbd>2</kbd>) között |
| <kbd>F</kbd> | **Vászon igazítása** | Aktív képkocka automatikus igazítása a munkaterülethez |
| <kbd>F1</kbd> | **Súgó** | Gyorsbillentyűk súgóablakának megnyitása |

---

## Első lépések

### Önálló futtatható állományok

Töltse le az előre lefordított, önálló futtatható binárisokat a **[GitHub Kiadások oldaláról](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Töltse le az `OniAnvil-windows-x86_64.zip` fájlt, csomagolja ki, majd kattintson duplán az `OniAnvil.exe` fájlra.
- **Linux**: Töltse le az `OniAnvil-linux-x86_64.tar.gz` (hordozható) fájlt, vagy telepítse az `onianvil_*_amd64.deb` Debian/Ubuntu csomagot:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Töltse le az `OniAnvil-macos-universal.zip` fájlt, csomagolja ki, és húzza az `OniAnvil.app` fájlt az Alkalmazások mappába. Az univerzális bináris natívan fut Apple Silicon (M1/M2/M3/M4) és Intel Mac gépeken is.
- **Web**: Futtassa közvetlenül a böngészőben telepítés nélkül: **[WebAssembly alkalmazás indítása](https://rbm13s.github.io/OniAnvil/app/?lang=hu)**.
  > [!WARNING]
  > A WebAssembly (böngészős) verzió **Kísérleti (Experimental)** státuszú. A böngésző biztonsági korlátai, a WebGL/WebGPU eltérések és a vágólap-korlátozások miatt kisebb grafikai hibák vagy teljesítménybeli lassulások léphetnek fel a natív asztali verzióhoz képest. Éles és komoly munkához a natív asztali verziók használatát javasoljuk.

---

### Fordítás forráskódból

#### Előfeltételek
- **Rust Eszközkészlet**: Telepítse a Rust-ot (1.80+ ajánlott) a [rustup.rs](https://rustup.rs/) weboldalon keresztül:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux fordítási függőségek** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Natív asztali verzió fordítása és futtatása
```bash
# Tároló klónozása
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Tesztkészlet futtatása
cargo test

# Optimalizált kiadási változat fordítása és futtatása
cargo run --release
```

#### 2. WebAssembly verzió fordítása (Trunk használatával)
```bash
# wasm32 fordítási cél és Trunk fordítóeszköz telepítése
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Helyi fejlesztői szerver futtatása automatikus újratöltéssel
trunk serve

# Optimalizált kiadási fájlok építése a docs/app/ könyvtárba
./build_wasm.sh        # Linux / macOS alatt
./build_wasm.ps1       # Windows PowerShell alatt
```

---

## Fájlformátum és kísérő architektúra

Az OniAnvil kerüli a zárt, védett bináris formátumokat. Minden projekt két tiszta, verziókezelhető fájlból áll:

```
my_character/
├── my_character.png            # Standard PNG sprite lap atlasz (megnyitható bármely motorban vagy nézegetőben)
└── my_character.onianvil.json   # Kísérő metaadatfájl (sidecar)
```

A `.onianvil.json` kísérőfájl a következőket tárolja:
- Vászonrács beállításai: képkockaméretek, oszlopok száma, összes képkockaszám.
- Réteghierarchiák: sávok nevei, átlátszósági szintek, láthatósági jelzők, zárolt állapotok.
- Animációs címkék: név, kezdő- és végkocka indexek, lejátszási irány (Előre, Hátra, Ping-Pong), képkocka-időtartamok.
- Palettadefiníciók: aktív színminták, elnevezett skinek listája, aktív skin indexe.
- Referenciavázlat állapota: képfájl útvonala, méretezés, eltolás, elforgatás, átlátszóság, rétegmód.
- Forgáspontok: egyéni akciópontok, fegyverrögzítési pozíciók és ütközési középpontok.

---

## Közreműködés és köszönetnyilvánítás

A hozzájárulásokat, hibajelentéseket és funkciójavaslatokat örömmel fogadjuk!
1. Készítsen egy Forkot a tárolóból a GitHubon.
2. Hozzon létre egy funkcióágat: `git checkout -b feature/uj-pixel-eszkoz`.
3. Végezze el a módosításokat és rögzítse: `git commit -am 'Új pixel eszköz hozzáadása'`.
4. Töltse fel a módosításokat: `git push origin feature/uj-pixel-eszkoz`.
5. Nyisson egy Pull Requestet.

### Köszönetnyilvánítás
- A **"Cat 2D Pixel Art"** kezdő animációs demó sprite készítője: **xzany** ([itch.io projektoldal](https://xzany.itch.io/cat-2d-pixel-art)).
- Készült a fantasztikus **[egui](https://github.com/emilk/egui)** grafikus könyvtár és a **[wgpu](https://wgpu.rs/)** grafikus ökoszisztéma segítségével.
- A felhasználói felület ikonjait a **[Font Awesome Free](https://fontawesome.com)** biztosítja (@fontawesome — Ikonok: CC BY 4.0, Betűkészletek: SIL OFL 1.1, Kód: MIT Licenc).

---

## Licenc és felelősségkizárás

Ez a projekt a **[MIT Licenc](LICENSE)** feltételei szerint van licencelve. Szabadon használhatja, módosíthatja, terjesztheti és beépítheti az OniAnvil-t kereskedelmi és személyes játékfejlesztési munkafolyamataiba.

### Garancia kizárása és a felelősség korlátozása
> [!IMPORTANT]
> Az **OniAnvil** szoftver terjesztése **díjmentesen**, **„ÚGY, AHOGY VAN” („AS IS”)** és **„MEGLÉVŐ ÁLLAPOTÁBAN” („AS AVAILABLE”)** történik, bármiféle kifejezett vagy hallgatólagos garancia vagy feltétel nélkül.
> 
> A szerző(k), a szerzői jog tulajdonosai vagy a közreműködők semmilyen körülmények között nem tehetők felelőssé semmilyen közvetlen, közvetett, véletlen, különleges, példaértékű vagy következményes kárért (beleértve, de nem kizárólagosan az adatok elvesztését, fájlok sérülését, hardverhibát, elmaradt hasznot vagy az üzletmenet megszakadását), amely a szoftver használatából vagy használatának ellehetetlenüléséből bármilyen módon származik, még akkor sem, ha figyelmeztettek az ilyen károk bekövetkeztének lehetőségére. A szoftver használatával járó minden felelősséget és kockázatot a felhasználó visel.

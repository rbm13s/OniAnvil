<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pikselių meno ir svogūno sluoksniavimo animacijos („Onion-Skinning“) studija, sukurta žaidimams paruoštiems atlasams**

[![Kompiliavimas ir laida](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Atsisiuntimai](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Atvaizdavimas](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Kalbos](https://img.shields.io/badge/Kalbos-37_Kalbų-purple)](#10-35-ergonomiškos-temos-ir-daugiakalbė-lokalizacija)
[![Platforma](https://img.shields.io/badge/Platforma-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#darbo-pradžia)
[![Interneto programa](https://img.shields.io/badge/Internetinė_programa-Išbandyti_naršyklėje_(Eksperimentinė)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=lt)
[![Licencija: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Išbandykite „OniAnvil“ naršyklėje („WebAssembly“)](https://rbm13s.github.io/OniAnvil/app/?lang=lt)** *(⚠️ Eksperimentinė naršyklės versija: gali turėti „WebGL“ / „WASM“ apribojimų ir klaidų)* • **[🌐 Oficiali dokumentacijos svetainė](https://rbm13s.github.io/OniAnvil/lt.html)**

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
**Lietuvių** •
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

## Turinys

- [Įvadas](#įvadas)
- [Ekrano nuotraukos ir vaizdinė apžvalga](#ekrano-nuotraukos-ir-vaizdinė-apžvalga)
- [Išsamus įrankių vadovas](#išsamus-įrankių-vadovas)
- [Funkcinė dokumentacija](#funkcinė-dokumentacija)
  - [1. Dvigubo atlaso architektūra](#1-dvigubo-atlaso-architektūra)
  - [2. Daugiaspalvė sluoksnių laiko juosta](#2-daugiaspalvė-sluoksnių-laiko-juosta)
  - [3. Tiesioginė animacijos peržiūra realiuoju laiku](#3-tiesioginė-animacijos-peržiūra-realiuoju-laiku)
  - [4. Pažangus svogūno sluoksniavimo variklis („Onion Skinning“)](#4-pažangus-svogūno-sluoksniavimo-variklis-onion-skinning)
  - [5. Išmanusis eskizo ir pavyzdinio atvaizdo sluoksnis](#5-išmanusis-eskizo-ir-pavyzdinio-atvaizdo-sluoksnis)
  - [6. Tiksli atranka ir bėgiojančios skruzdėlės](#6-tiksli-atranka-ir-bėgiojančios-skruzdėlės)
  - [7. Tiesioginiai efektai, filtrai ir „RotSprite“](#7-tiesioginiai-efektai-filtrai-ir-rotsprite)
  - [8. Paletės valdymas, keitimas ir personažų apvalkalai („Skins“)](#8-paletės-valdymas-keitimas-ir-personažų-apvalkalai-skins)
  - [9. Lapo pjaustyklė, 9-Grid vartotojo sąsaja ir eksportas žaidimų varikliams](#9-lapo-pjaustyklė-9-grid-vartotojo-sąsaja-ir-eksportas-žaidimų-varikliams)
  - [10. 35 ergonomiškos temos ir daugiakalbė lokalizacija](#10-35-ergonomiškos-temos-ir-daugiakalbė-lokalizacija)
- [Valdymas ir spartieji klavišai](#valdymas-ir-spartieji-klavišai)
- [Darbo pradžia](#darbo-pradžia)
  - [Paruošti paleidžiamieji failai](#paruošti-paleidžiamieji-failai)
  - [Kompiliavimas iš pirminio kodo](#kompiliavimas-iš-pirminio-kodo)
- [Failų formatas ir lydinčioji architektūra](#failų-formatas-ir-lydinčioji-architektūra)
- [Prisidėjimas ir padėkos](#prisidėjimas-ir-padėkos)
- [Licencija ir atsakomybės atsisakymas](#licencija-ir-atsakomybės-atsisakymas)

---

## Įvadas

**„OniAnvil“** (*pavadinimas kilęs iš **Oni**on-Skinning (svogūno sluoksniavimo technika) + **Anvil** (priekalas spraitams kalti)*) yra atvirojo kodo, itin našus pikselių meno redaktorius ir animacijos studija, sukurta „Rust“ kalba. Ji veikia natyviai su aparatiškai spartintu grafikos atvaizdavimu „Windows“, „Linux“ ir „macOS“ sistemose, taip pat be diegimo tiesiogiai šiuolaikinėse naršyklėse kaip „WebAssembly“ programa.

Dauguma esamų spraitų redaktorių animaciją laiko atskirų vaizdo failų rinkiniu, priversdami kūrėjus atlikti varginančius eksportavimo, pjaustymo ir perpakaavimo veiksmus. **„OniAnvil“ apverčia šią paradigmą:** žaidimo spraitų lapo tekstūros atlasas (`atlas_rgba`) yra pagrindinis tiesos šaltinis. Kadrų elementai, sluoksnių takeliai, animacijos žymos ir susidūrimo atramos taškai yra nuolat ir be nuostolių sinchronizuojami su tekstūros atlasu.

### Kodėl „OniAnvil“?
- 🚀 **Žaibiškas natyvus „Rust“ našumas**: Momentinis paleidimas per mažiau nei milisekundę, aiškus atvaizdavimas naudojant `wgpu` ir `egui 0.31` bei sklandus darbas net ir peržiūrint ilgas laiko juostas be jokio vėlavimo.
- 🎬 **Daugiaspalvė sluoksnių laiko juosta**: Nelijinių vaizdo redaktorių įkvėpti sluoksnių takeliai (`V1, V2, V3...`) su kompozicija iš viršaus į apačią, sluoksnių suliejimu žemyn, susietais kadrais ir kintama kadro išlaikymo trukme.
- 🧅 **Pirmos klasės svogūno sluoksniavimas („Onion Skinning“)**: Kelių kadrų pusiau skaidrus atvaizdavimas su nepriklausomu priekinės ir atgalinės krypties tonavimu, pritaikomomis nepermatomumo kreivėmis ir sklandžiu ciklo perėjimu.
- 🎨 **Momentiniai personažų paletės apvalkalai („Skins“)**: Lengvai kurkite dienos / nakties variantus, komandų spalvas ar elementinius stilius realiuoju laiku perbraižydami atlasą su GPL, PAL ir HEX palaikymu.
- 📦 **Tiesioginis eksportas į žaidimų variklius**: Eksportuokite masteliuotus PNG failus su išplėstais kraštiniais pikseliais („extrude“), kad išvengtumėte tekstūrų siūlių defektų 2D varikliuose („Godot“, „Bevy“, „Raylib“, „Defold“), kartu su standartiniais JSON metadatenimis ir animuotais GIF.

---

## Ekrano nuotraukos ir vaizdinė apžvalga

<div align="center">

### 1. Pagrindinė drobė ir studijos darbo sritis
*Didelio našumo pikselių darbo erdvė su aparatiškai spartintu vaizdu, kelių takelių laiko juosta, tiesiogine peržiūra, paletės langeliais ir tiksliomis įrankių juostomis.*
<img src="screenshots/main.png" alt="„OniAnvil“ pagrindinė darbo erdvė" width="950" />

<br/><br/>

### 2. Daugiaspalvė animacijos laiko juosta
*Netiesinė kelių takelių laiko juosta su sluoksnių eilių krūva (`V1, V2...`), susietų kadrų egzemplioriais (`🔗`), išlaikymo daugikliais ir spalvomis pažymėtomis animacijos žymomis.*
<img src="screenshots/timeline.png" alt="„OniAnvil“ kelių takelių laiko juosta" width="950" />

<br/><br/>

### 3. Tikslieji įrankiai ir viršutinė veiksmų juosta
*Pilnas 15 įrankių piešimo rinkinys su standartiniu ir kompaktišku fiksavimu bei viršutine sparčiųjų nustatymų juosta teptuko dydžiui, simetrijai, ciklui ir „Pixel-Perfect“ režimui.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Veiksmų įrankių juosta" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Standartinė įrankių juosta" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Kompaktiška įrankių juosta" width="300" />
</p>

<br/><br/>

### 4. Tiesioginė animacijos peržiūra realiuoju laiku
*Atskira peržiūros doko sritis su žymų filtravimu, kintamu kadrų per sekundę (FPS) greičiu, priartinimo slankikliu ir drobės fono perjungikliais (šachmatai, tamsus, šviesus).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Tiesioginė peržiūra" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Peržiūros priartinimas" width="460" />
</p>

<br/><br/>

### 5. Paletės valdymas, keitimas ir spalvų perėjimai
*Išsami spalvų sistema su priekinės ir fono spalvų langeliais, automatiniu dviejų taškų spalvų perėjimų generavimu ir personažo apvalkalų keitimu.*
<img src="screenshots/palette_and_swaps.png" alt="Paletė ir apvalkalų keitimas" width="950" />

<br/><br/>

### 6. Eskizo pavyzdinis sluoksnis ir konfigūruojamas svogūno sluoksniavimas
*Įkelkite eskizus ar nuotraukas kaip apatinį arba viršutinį sluoksnį su transformavimo valdikliais, išmaniąja pipete su fiksavimu prie paletės ir kelių kadrų svogūno sluoksniavimu.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Eskizas ir svogūno sluoksniavimas" width="950" />

<br/><br/>

### 7. Atramos taškai („Pivot“) ir veiksmo taškai („Action Points“)
*Nustatykite spraito koordinatinį centrą ir pritaikytus žaidimo veiksmo taškus (ginklų tvirtinimas, šūvio pradžia, galva, pėdos) naudodami interaktyvius drobės valdiklius.*
<img src="screenshots/pivot_and_action_points.png" alt="Atramos ir veiksmo taškai" width="950" />

<br/><br/>

### 8. Spraitų lapo pjaustyklė ir 9-Grid (9-Slice) mastelio keitiklis
*Automatinis spraitų ribų atpažinimas pagal autokoreliaciją (pvz., einančios katės ciklas) ir 9 sričių keičiamo dydžio vartotojo sąsajos rėmelių paraščių konfigūravimas.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Lapo pjaustyklė" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="9-Grid mastelio keitiklis" width="460" />
</p>

<br/><br/>

### 9. Realaus laiko vaizdo efektai ir filtrai („Live Preview“)
*Visi efektų dialogo langai turi tiesioginę peržiūrą drobėje, interaktyvias pipetes ir saugų atšaukimą be istorijos teršimo.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Kontūro generatorius" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Spalvos pakeitimas" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Atspalvis, sodrumas ir kontrastas" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matricos filtras" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Pririšimas prie paletės" width="460" />
</p>

<br/><br/>

### 10. Drobės tinklelis ir fono režimai
*Sklandžiai perjunkite pikselių tinklelį, kadrų ribas, tamsią šachmatų lentą, šviesią šachmatų lentą ir pasirinktinę vientisą fono spalvą.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Tinklelis ir tamsus fonas" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Šviesus fonas be tinklelio" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Pasirinktinis vientisas fonas" width="300" />
</p>

<br/><br/>

### 11. Naujo projekto kūrimo vedlys
*Nustatykite drobės matmenis, kadrų skaičių, stulpelių išdėstymą ir pasirinkite iš daugiau nei 20 autentiškų retro palečių projekto pradžioje.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Naujo projekto vedlys" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Projekto paletės" width="460" />
</p>

<br/><br/>

### 12. 35 autentiškos retro ir šiuolaikinės temos
*Apima „Studio Dark“, „Studio Light“, CRT fosforus („Amber“, „Green“, „Cyan“), retro konsoles („PlayStation 1“, „Game Boy“, „SNES“, „Dreamcast“, „Genesis“) ir dizainerių temas.*
<p align="center">
  <img src="screenshots/themes.png" alt="Temų meniu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="„CRT Amber“ tema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="„PSX“ tema" width="300" />
</p>

<br/><br/>

### 13. Meniu sistema ir integruota sparčiųjų klavišų pagalba
<p align="center">
  <img src="screenshots/file_menu.png" alt="Failo meniu" width="180" />
  <img src="screenshots/edit_menu.png" alt="Redagavimo meniu" width="180" />
  <img src="screenshots/select_menu.png" alt="Atrankos meniu" width="180" />
  <img src="screenshots/view_menu.png" alt="Rodymo meniu" width="180" />
  <img src="screenshots/project_menu.png" alt="Projekto meniu" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Sparčiųjų klavišų pagalbos langas" width="700" />
</p>

</div>

---

## Išsamus įrankių vadovas

„OniAnvil“ apima 15 profesionalių piešimo, geometrijos, atrankos ir metaduomenų įrankių. Piktogramoms naudojamas atvirojo kodo **Font Awesome Free** piktogramų rinkinys *(Piktogramos: CC BY 4.0, Šriftai: SIL OFL 1.1)*.

| Įrankis | Piktograma | Spartusis klavišas | Teptuko dydis | Pixel Perfect | Aprašymas |
|:---|:---:|:---:|:---:|:---:|:---|
| **Pieštukas** | ✏️ | <kbd>B</kbd> | 1..32 px | Taip | Pagrindinis piešimo įrankis. 1 px režime palaiko **Pixel-Perfect** piešimą (automatiškai pašalina laiptuotus L formos kampus). Palaiko antspaudų teptukus iš atrankos (`Ctrl/Cmd+B`). |
| **Trintukas** | 🧹 | <kbd>E</kbd> | 1..32 px | Ne | Išvalo pikselius iki visiško skaidrumo (`[0, 0, 0, 0]`). Esant aktyviai atrankai, veikia tik jos ribose. |
| **Pipetė** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Ne | Kairysis paspaudimas paima spalvą į priekinę (Primary); dešinysis – į fono (Secondary). Paima iš drobės, sluoksnių arba eskizo. |
| **Užpildas** | 🪣 | <kbd>G</kbd> | — | Ne | Greitas eilių pagrindu (BFS) veikiantis užpildymas su reguliuojamu tolerancijos slenksčiu (`0..=255`). Palaiko gretimą ir visuotinį užpildymą bei atrankos ribojimą. |
| **Gradientas** | 🌈 | <kbd>G</kbd> *(perjungti)* | — | Ne | Tiesiniai ir radialiniai gradientai tarp priekinės ir fono spalvų. Palaiko „Bayer“ (2x2, 4x4, 8x8) ir „Blue Noise“ taškavimą su stiprumo valdymu. |
| **Linija** | 📏 | <kbd>L</kbd> | 1..32 px | Taip | Bresenhamo tiesių linijų algoritmas su tiesiogine peržiūra. Laikant <kbd>Shift</kbd>, kampai fiksuojami 45°/90° žingsniais. |
| **Kreivė** | 〰️ | <kbd>C</kbd> | 1..32 px | Taip | 3 paspaudimų kvadratinė Bezjė kreivė. 1 paspaudimas nustato pradžią, 2 – pabaigą, pelės judesys išlenkia kreivę, 3 paspaudimas ją patvirtina. |
| **Stačiakampis** | ⬜ | <kbd>U</kbd> | 1..32 px | Ne | Kontūriniai arba užpildyti geometriniai stačiakampiai. Laikant <kbd>Shift</kbd>, išlaikomos 1:1 kvadrato proporcijos. |
| **Elipsė** | ⭕ | <kbd>O</kbd> | 1..32 px | Ne | Kontūrinės arba užpildytos elipsės su vidurio taško rasterizavimu. Laikant <kbd>Shift</kbd>, fiksuojamas tobulas apskritimas. |
| **Rastravimo teptukas** | 🏁 | <kbd>D</kbd> | 1..32 px | Ne | Tapo šachmatų lentos rastravimo raštais tarp priekinės ir fono spalvų neįvedant papildomų spalvų į paletę. |
| **Šešėliavimo įrankis** | 🌓 | <kbd>S</kbd> | 1..32 px | Ne | Laipsniškas šviesinimas („Dodge“) ir tamsinimas („Burn“) pagal aktyvios paletės spalvų tvarką. |
| **Stačiakampė atranka** | ⬚ | <kbd>M</kbd> | — | Ne | Stačiakampės srities atranka su animuota bėgiojančių skruzdėlių riba. Pridėjimas (<kbd>Shift</kbd>) ir atėmimas (<kbd>Alt</kbd>). |
| **Laso atranka** | ➰ | <kbd>Q</kbd> | — | Ne | Laisva daugiakampė atranka. Naudoja 8 jungčių ribos rasterizavimą ir 4 jungčių išorinį užpildymą, garantuodama 100% vientisą atranką be skylių. |
| **Burtų lazdelė** | 🪄 | <kbd>W</kbd> | — | Ne | Pasirenka gretimas (arba visas) sutampančios spalvos sritis su reguliuojamu tolerancijos slenksčiu. |
| **Atramos ir veiksmo taškai** | 🎯 | <kbd>P</kbd> | — | Ne | Spraito centro ir pasirinktų žaidimo veiksmo taškų (`weapon`, `head`, `feet`, `hitbox`) interaktyvus išdėstymas drobėje ir eksportas į JSON metaduomenis. |

---

## Funkcinė dokumentacija

### 1. Dvigubo atlaso architektūra

„OniAnvil“ sistemoje pagrindinis tekstūros atlasas (`atlas_rgba`) reprezentuoja tikrąją spraitų lapo tekstūrą, kuri yra tiesiogiai įkeliama į žaidimų variklį:
- **Kadro režimas („Frame Mode“)**: Sutelkia dėmesį į vieną aktyvų animacijos kadrą, suteikdamas patogią pikselių drobę. Pakeitimai nedelsiant atnaujina atitinkamą atlaso tekstūros langelį.
- **Atlaso režimas („Atlas Mode“)**: Rodo visą spraitų lapo tinklelio vaizdą, leidžiantį atlikti makro lygio korekcijas, kadrų lygiavimo patikrą ir masinį redagavimą.
- **Lydintysis failas („Companion File“)**: Projektai išsaugomi kaip švarus `.png` spraitų lapas kartu su lengvu `.onianvil.json` lydinčiuoju failu, kuriame saugomas kadrų išdėstymas, sluoksnių hierarchija, žymos, išlaikymo daugikliai ir palečių apvalkalai.

### 2. Daugiaspalvė sluoksnių laiko juosta

Apatinė laiko juosta sukurta pagal profesionalių nelijinių vaizdo redagavimo programų principus:
- **Sluoksnių takeliai (`V1, V2, V3...`)**: Sluoksniai išdėstyti vizualiai iš viršaus į apačią. Takelis, esantis aukščiau laiko juostoje (`V1`), kadro drobėje atvaizduojamas ant žemiau esančiųjų.
- **Takelių valdikliai**:
  - Spustelėkite `V1` / `V2` ženkliukus, kad greitai perjungtumėte aktyvų piešimo sluoksnį.
  - Perjunkite sluoksnio matomumą (akies piktograma) ir redagavimo užraktą (spynos piktograma).
  - Reguliuokite atskiro sluoksnio nepermatomumą (`0%..=100%`).
  - Perkelkite sluoksnį aukštyn / žemyn hierarchijoje, dubliuokite ir suliekite sluoksnį žemyn su žemesniuoju takeliu („Merge Down“).
  - Saugus sluoksnio trynimas su patvirtinimo langu.
- **Kadrų fragmentai**:
  - Išlaikymo daugiklis: priskirkite pasirinktinę kadrų trukmę (pvz., `2×`, `3×` pauzėms ar smūgio kadrams).
  - Susieti kadrai (`🔗`): susiekite pasikartojančius kadrus visoje animacijoje taip, kad vieno redagavimas automatiškai atnaujintų visus susietus egzempliorius.
  - Žymų ciklas: suskirstykite animacijas į pavadintas žymas (`idle`, `walk`, `attack`) ir leiskite konkrečias žymas ratu.
  - Atkūrimo valdikliai: Pirmyn, Atgal, „Ping-Pong“ ir Nuolatinis ciklas su reguliuojamu kadrų per sekundę greičiu (`1..=120` FPS).

### 3. Tiesioginė animacijos peržiūra realiuoju laiku

Dešinėje esantis peržiūros dokas leidžia peržiūrėti animaciją realiuoju laiku, nepriklausomai nuo drobės mastelio ar padėties:
- **Žymų filtravimas**: Paleiskite tik pasirinktą žymą (`walk`, `jump`) arba peržiūrėkite visą animacijos seką.
- **Atkūrimo valdikliai**: Paleisti, Pristabdyti, Ankstesnis kadras, Kitas kadras, Ciklo jungiklis ir pasirinktinis greitis (`0.25x..=4.0x`).
- **Rodymo režimai**: Perjunkite tamsią šachmatų lentą, šviesią šachmatų lentą, grynai baltą, grynai juodą ar pasirinktinį drobės foną.

### 4. Pažangus svogūno sluoksniavimo variklis („Onion Skinning“)

Kurkite sklandžias animacijas naudodami pažangią svogūno sluoksniavimo sistemą:
- **Pritaikytas tonavimas**: Nepriklausomos atspalvių spalvos ankstesniems kadrams (numatytoji: raudona) ir būsimiems kadrams (numatytoji: mėlyna).
- **Nepermatomumo slopinimas**: Konfigūruojamas bazinis nepermatomumas ir tolygaus slopinimo kreivė tolimesniems žingsniams.
- **Ciklinis apgaubimas**: Įjunkite pusiau skaidrų persidengimą tarp pirmojo ir paskutinio kadrų, kad sukurtumėte vientisus animacijos ciklus.
- **Greita peržiūra („Quick-Peek“)**: Laikykite nuspaudę <kbd>Tarpo klavišą</kbd> nevilkdami pelės, kad akimirksniu pamatytumėte gretimus kadrus.

### 5. Išmanusis eskizo ir pavyzdinio atvaizdo sluoksnis

Paspartinkite savo darbą piešdami ant ranka pieštų eskizų ar pavyzdinių nuotraukų:
- **Formatai**: Įkelkite PNG, JPEG ir WebP pavyzdinius vaizdus.
- **Suliejimo režimai**: Pagrindas (po pikselių sluoksniais) arba Viršutinis sluoksnis (ant pikselių sluoksnių).
- **Transformacijos**: Nepriklausomas mastelis, padėties poslinkis, pasukimas ir nepermatomumo slankiklis.
- **Išmanusis spalvų parinkimas**: Naudokite pipetę ant eskizo sluoksnio su automatiniu pritraukimu prie artimiausios aktyvaus projekto paletės spalvos.
- **Kepimas 1 paspaudimu („Bake“)**: Vienu paspaudimu paverskite masteliuotą pavyzdinį vaizdą švariu pikselių menu aktyviame kadre.

### 6. Tiksli atranka ir bėgiojančios skruzdėlės

- **Animuotos bėgiojančios skruzdėlės**: Aktyvios atrankos rodo animuotą, gerai matomą dviejų tonų punktyrinį kontūrą.
- **Vientisas daugiakampis lasas**: Priešingai nei paprasti spindulių algoritmai, sukuriantys tuštumas persidengiančiose kilpose, „OniAnvil“ lasas rasterizuoja uždarą 8 jungčių ribą ir užpildo išorę, garantuodamas 100% vientisą atranką be jokių skylių.
- **Atrankos veiksmai**: Išplėsti, Sutraukti, Apvadas (su apskritimo, kvadrato ir rombo formos branduoliais), Perkelti, Invertuoti, Ištrinti, Laisvas pasukimas ir sistemos iškarpinės Kopijuoti / Iškirpti / Įklijuoti.
- **Kopijavimas ir įklijavimas tarp kadrų**: Pasirinkite sritį viename kadre, paspauskite <kbd>Ctrl/Cmd+C</kbd>, perjunkite kadrus ir paspauskite <kbd>Ctrl/Cmd+V</kbd>, kad įklijuotumėte į tas pačias santykines koordinates bet kurioje klaviatūros kalboje.

### 7. Tiesioginiai efektai, filtrai ir „RotSprite“

Visi vaizdo efektai turi tiesioginės peržiūros žymimąjį langelį **Live Preview**, kuris rodo rezultatą tiesiai drobėje ir leidžia atšaukti veiksmą neteršiant atšaukimų istorijos:
- **Spalvos pakeitimas (`Shift+R`)**:
  - Interaktyvi pipetė: spustelėkite pipetės piktogramą ir paimkite spalvas tiesiai iš drobės pikselių arba paletės juostos.
  - Greitosios paletės juosta: kairysis paspaudimas parenka spalvą „Iš“, dešinysis – „Į“.
  - Tolerancijos slankiklis ir masinis pakeitimas iškart per visus animacijos kadrus.
- **Spalvų korekcija (`Ctrl/Cmd+U`)**: Realaus laiko HSV atspalvio postūmis, sodrumas, šviesumas ir kontrasto reguliavimas.
- **Automatinis kontūro generatorius**: Sukurkite ryškų personažo pikselių kontūrą (Išorėje, Viduje, Centre) su kvadratiniais arba rombiniais kampais.
- **3×3 matriciniai filtrai**: Suliejimas, Ryškinimas, Kraštų aptikimas ir Reljefas išsaugant alfa kanalą.
- **Laisvas pasukimas (`Ctrl/Cmd+Shift+R`)**: Pasukite pasirinktą sritį arba visą kadrą bet kokiu kampu naudodami klasikinį **RotSprite** arba artimiausio kaimyno algoritmą.
- **Pririšimas prie paletės**: Kvantizuokite drobės spalvas į aktyvią paletę naudodami „Bayer“ 2x2, 4x4, 8x8 arba „Blue Noise“ taškavimą.

### 8. Paletės valdymas, keitimas ir personažų apvalkalai („Skins“)

- **Integruotos paletės**: Daugiau nei 20 autentiškų retro palečių, įskaitant „Game Boy“, „PICO-8“, „Commodore 64“, „NES“, „Endesga 32“, „Solarized“ ir „Sweetie 16“.
- **Importas ir eksportas**: GIMP paletės (`.gpl`), PAL (`.pal`), HEX (`.hex`) ir paveikslėlių juostų (`.png`) palaikymas.
- **Palečių keitimas (Apvalkalai)**: Nustatykite kelis vieno personažo spalvų apvalkalus (pvz., Numatytoji, Auksiniai šarvai, Šešėlis, Ledas). Apvalkalo pakeitimas akimirksniu perbraižo visų kadrų spalvas visame atlase.
- **Spalvų perėjimo generatorius**: Automatiškai interpoliuokite tolygų gradientą tarp dviejų pasirinktų atspalvių su konfigūruojamu žingsnių skaičiumi.

### 9. Lapo pjaustyklė, 9-Grid vartotojo sąsaja ir eksportas žaidimų varikliams

- **Lapo pjaustyklė („Sheet Slicer“)**: Autokoreliacijos kraštų aptikimas automatiškai supjausto neapdorotus spraitų lapus į tolygius arba tiksliai apribotus animacijos kadrus.
- **9-Grid (9-Slice) mastelio keitimas**: Nustatykite keičiamas paraštes (viršuje, apačioje, kairėje, dešinėje), kad peržiūrėtumėte pritaikomus sąsajos langelius ir mygtukus be kampų iškraipymo.
- **PNG eksportas su išplėstais kraštiniais pikseliais („Extruded Border“)**: Padaugina kraštinius pikselius į skaidrias paraštes, užkirsdamas kelią tekstūrų filtravimo siūlėms „OpenGL“, „DirectX“, „Vulkan“ ir 2D žaidimų varikliuose (pvz., „Godot“).
- **Animuotų GIF eksportas**: Kurkite optimizuotus animuotus GIF failus su pasirinktiniu ciklų skaičiumi, kadrų delsa ir skaidriu fonu.
- **Standartinis spraitų lapo JSON**: Eksportuoja kadrų koordinates, žymas, trukmes ir atramos taškus pagal šiuolaikiniuose 2D varikliuose taikomą standartą.

### 10. 35 ergonomiškos temos ir daugiakalbė lokalizacija

- **Atrinktos temos**: „Studio Dark“, „Studio Light“, „Pitch Black“, „Cyberpunk Neon“, „Synthwave“, „Nord Frost“, „Dracula“, „Tokyo Night“, „Catppuccin Mocha“, „Gruvbox Dark“, „Monokai Pro“, „Solarized Dark“, „Solarized Light“, CRT fosforai („Amber“, „Green“, „Cyan“), retro konsolės („Game Boy“, „Game Boy Light“, „Super Game Boy“, „NES/Famicom“, „SNES“, „Genesis/Mega Drive“, „PlayStation 1“, „PlayStation 2“, „Dreamcast“) ir retro kompiuteriai („C64“, „ZX Spectrum“, „Amiga Workbench“, „Atari ST“, „MSX“, „Apple II“, „DOS CGA“, „Mac Classic“, „Windows 98“).
- **Daugiakalbė sąsaja**: Sklandžiai perjunkite kalbą tiesioginio darbo metu su 100% žodyno atitikimu 37 kalbose.

---

## Valdymas ir spartieji klavišai

| Spartusis klavišas | Įrankis / Veiksmas | Aprašymas |
|:---|:---|:---|
| <kbd>B</kbd> | **Pieštukas** | 1 px tikslus piešimas su pasirinktiniu „Pixel-Perfect“ režimu |
| <kbd>E</kbd> | **Trintukas** | Išvalo pikselius iki skaidrumo |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipetė** | Paima spalvą iš drobės pikselių arba eskizo |
| <kbd>G</kbd> | **Užpildas / Gradientas** | Perjungia tarp užpildo ir taškuoto gradiento |
| <kbd>L</kbd> | **Linija** | Brėžia tiesias linijas su peržiūra ir „Shift“ kampo fiksavimu |
| <kbd>C</kbd> | **Kreivė** | 3 paspaudimų kvadratinės Bezjė kreivės įrankis |
| <kbd>U</kbd> | **Stačiakampis** | Geometrinis stačiakampis ir kvadratas (kontūras arba užpildytas) |
| <kbd>O</kbd> | **Elipsė** | Geometrinė elipsė ir apskritimas (kontūras arba užpildytas) |
| <kbd>D</kbd> | **Rastravimo teptukas** | Tapo šachmatų lentos rastravimo matricos raštais |
| <kbd>S</kbd> | **Šešėliavimo įrankis** | Laipsniškas šviesinimas / tamsinimas pagal paletę |
| <kbd>M</kbd> | **Stačiakampė atranka** | Stačiakampės srities atranka su bėgiojančiomis skruzdėlėmis |
| <kbd>Q</kbd> | **Laso atranka** | Laisva vientisa daugiakampė atranka |
| <kbd>W</kbd> | **Burtų lazdelė** | Pasirenka gretimas vienodos spalvos sritis |
| <kbd>P</kbd> | **Atramos taškai** | Išdėsto spraito centrą ir veiksmo taškus |
| <kbd>X</kbd> | **Sukeisti spalvas** | Sukeičia priekinę ir fono spalvas vietomis |
| <kbd>[</kbd> / <kbd>]</kbd> | **Teptuko dydis** | Sumažinti / Padidinti teptuko dydį (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopijuoti** | Kopijuoti pasirinktus pikselius į iškarpinę |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Iškirpti** | Iškirpti pasirinktus pikselius į iškarpinę |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Įklijuoti** | Įklijuoti iškarpinės pikselius į aktyvų kadrą |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Ištrinti** | Išvalyti pikselius aktyvios atrankos viduje |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Pasirinkti viską** | Pasirinkti visus aktyvaus kadro pikselius |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Panaikinti atranką** | Išvalyti aktyvią atrankos kaukę |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invertuoti atranką** | Invertuoti aktyvią atrankos kaukę |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Dubliuoti kadrą** | Sukurti aktyvaus kadro kopiją laiko juostoje |
| <kbd>Tarpas</kbd> | **Paleisti / Pauzė** | Paleisti arba sustabdyti animaciją laiko juostoje |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Pakeisti spalvą** | Atverti spalvos pakeitimo langą su tiesiogine peržiūra |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Koreguoti spalvą** | Atverti HSV atspalvio, sodrumo ir kontrasto langą |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Laisvas pasukimas** | Atverti laisvo pasukimo langą („RotSprite“) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Atšaukti** | Atšaukti paskutinį potėpį ar įrankio veiksmą |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Pakartoti** | Pakartoti anksčiau atšauktą veiksmą |
| <kbd>Tarpas</kbd> + Vilkimas / Vidurinis pelės mygtukas | **Slinkti drobę** | Sklandus begalinis drobės koordinačių stumdymas |
| <kbd>Ctrl/Cmd</kbd> + Ratukas / <kbd>+</kbd> / <kbd>-</kbd> | **Mastelis** | Tikslus pikselių priartinimas pelės žymeklio vietoje |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Rodymo režimas** | Perjungti tarp kadro rodymo (<kbd>1</kbd>) ir atlaso rodymo (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Talpinti drobę** | Automatiškai centruoti ir sutalpinti kadrą |
| <kbd>F1</kbd> | **Pagalba** | Atverti sparčiųjų klavišų pagalbos langą |

---

## Darbo pradžia

### Paruošti paleidžiamieji failai

Atsisiųskite iš anksto sukompiliuotus, autonominius failus iš **[„GitHub“ laidų puslapio](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Atsisiųskite `OniAnvil-windows-x86_64.zip`, išpakuokite ir du kartus spustelėkite `OniAnvil.exe`.
- **Linux**: Atsisiųskite `OniAnvil-linux-x86_64.tar.gz` (nešiojamas) arba įdiekite `onianvil_*_amd64.deb` „Debian“ / „Ubuntu“ paketą:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Atsisiųskite `OniAnvil-macos-universal.zip`, išpakuokite ir nuvilkite `OniAnvil.app` į aplanką „Applications“. Universalus dvejetainis failas veikia tiek „Apple Silicon“ (M1/M2/M3/M4), tiek „Intel“ pagrindu veikiančiuose „Mac“ kompiuteriuose.
- **Interneto versija**: Paleiskite tiesiogiai naršyklėje be diegimo: **[Paleisti „WebAssembly“ programą](https://rbm13s.github.io/OniAnvil/app/?lang=lt)**.
  > [!WARNING]
  > „WebAssembly“ (naršyklės) versija turi **eksperimentinės („Experimental“)** statusą. Dėl naršyklės saugumo apribojimų, „WebGL“ / „WebGPU“ skirtumų ir iškarpinės ypatumų gali pasitaikyti nedidelių grafinių klaidų ar našumo netolygumų, palyginti su darbalaukio versija. Gamybiniam darbui primygtinai rekomenduojama naudoti natyvias darbalaukio versijas.

---

### Kompiliavimas iš pirminio kodo

#### Būtinosios sąlygos
- **„Rust“ įrankių rinkinys**: Įdiekite „Rust“ (rekomenduojama 1.80+ versija) per [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **„Linux“ kompiliavimo priklausomybės** („Debian“ / „Ubuntu“):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Natyvios darbalaukio versijos kompiliavimas ir paleidimas
```bash
# Klonuokite saugyklą
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Paleiskite testus
cargo test

# Sukurkite ir paleiskite optimizuotą laidą
cargo run --release
```

#### 2. „WebAssembly“ versijos kompiliavimas (naudojant „Trunk“)
```bash
# Pridėkite wasm32 taikinį ir įdiekite „Trunk“
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Paleiskite vietinį kūrimo serverį su karštuoju atnaujinimu
trunk serve

# Sukurkite optimizuotus failus kataloge docs/app/
./build_wasm.sh        # „Linux“ / „macOS“ aplinkoje
./build_wasm.ps1       # „Windows PowerShell“ aplinkoje
```

---

## Failų formatas ir lydinčioji architektūra

„OniAnvil“ vengia uždarų dvejetainių formatų. Kiekvienas projektas susideda iš dviejų švarių, versijų kontrolei idealiai tinkančių failų:

```
my_character/
├── my_character.png            # Standartinis PNG spraitų tekstūros atlasas (atveriamas bet kuriame variklyje)
└── my_character.onianvil.json   # Lydintysis projekto metaduomenų failas
```

`.onianvil.json` faile saugoma:
- Drobės tinklelio konfigūracija: kadro matmenys, stulpelių skaičius, bendras kadrų skaičius.
- Sluoksnių hierarchija: takelių pavadinimai, nepermatomumo lygiai, matomumo vėliavėlės, užrakinimo būsenos.
- Animacijos žymos: pavadinimas, pradžios / pabaigos kadrų indeksai, atkūrimo kryptis (Pirmyn, Atgal, „Ping-Pong“), kadrų trukmės.
- Palečių apibrėžimai: aktyvūs spalvų langeliai, pavadinti apvalkalų variantai („skins“), aktyvaus apvalkalo indeksas.
- Eskizo sluoksnio būsena: vaizdo kelias, mastelis, poslinkis, pasukimas, nepermatomumas, sluoksnio režimas.
- Atramos taškai: pasirinktiniai veiksmo taškai, ginklų tvirtinimo vietos ir susidūrimo centrai.

---

## Prisidėjimas ir padėkos

Visi pasiūlymai, pranešimai apie klaidas ir naujų funkcijų idėjos yra labai laukiami!
1. Sukurkite projekto atšaką („Fork“) „GitHub“.
2. Sukurkite naują funkcionalumo šaką: `git checkout -b feature/mano-naujas-irankis`.
3. Įrašykite pakeitimus: `git commit -am 'Pridėtas naujas pikselių įrankis'`.
4. Nusiųskite šaką į serverį: `git push origin feature/mano-naujas-irankis`.
5. Atverkite „Pull Request“.

### Padėkos
- Pradinis demonstracinis spraitas **„Cat 2D Pixel Art“** sukurtas autoriaus **xzany** ([„itch.io“ projekto puslapis](https://xzany.itch.io/cat-2d-pixel-art)).
- Sukurta naudojant puikią **[egui](https://github.com/emilk/egui)** vartotojo sąsajos biblioteką ir **[wgpu](https://wgpu.rs/)** grafikos ekosistemą.
- Vartotojo sąsajos piktogramas teikia **[Font Awesome Free](https://fontawesome.com)** (@fontawesome) (Piktogramos: CC BY 4.0, Šriftai: SIL OFL 1.1, Kodas: MIT licencija).

---

## Licencija ir atsakomybės atsisakymas

Šis projektas licencijuotas pagal **[MIT licenciją](LICENSE)**. Galite laisvai naudoti, modifikuoti, platinti ir integruoti „OniAnvil“ į komercinius bei asmeninius žaidimų kūrimo projektus.

### Garantijos atsisakymas ir atsakomybės apribojimas
> [!IMPORTANT]
> **„OniAnvil“** platinama **nemokamai** pagal principą **„KAIP YRA“ („AS IS“)** ir **„KIEK PRIEINAMA“ („AS AVAILABLE“)**, be jokių tiesioginių ar numanomų garantijų ar sąlygų.
> 
> Jokiomis aplinkybėmis autorius (-iai), autorių teisių turėtojai ar bendraautoriai neatsako už jokią tiesioginę, netiesioginę, atsitiktinę, specialiąją, pavyzdinę ar pasekminę žalą (įskaitant, bet neapsiribojant, duomenų praradimą, failų sugadinimą, aparatinės įrangos gedimus, negautą pelną ar verslo nutraukimą), atsiradusią bet kokiu būdu dėl šios programinės įrangos naudojimo ar negalėjimo ja naudotis, net jei buvo pranešta apie tokios žalos galimybę. Jūs prisiimate visą atsakomybę ir riziką, susijusią su šios programos naudojimu.

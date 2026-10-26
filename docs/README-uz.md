<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logotipi" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**O'yinga Tayyor Atlaslar Negizida Yaratilgan Piksel Grafikasi va Piyoz Po'sti (Onion-Skinning) Animatsiya Studiyasi**

[![Yig'ish va Chiqarish](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Yuklamalar](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderlovchi](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Tillar](https://img.shields.io/badge/Languages-37_Til-purple)](#10-35-ergonomik-mavzu-va-toliq-mahalliylashtirish)
[![Platforma](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#ishni-boshlash)
[![Onlayn Ilova](https://img.shields.io/badge/Veb_Ilova-Brauzerda_Sinash_(Eksperimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=uz)
[![Litsenziya: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 OniAnvil-ni Brauzerda Ishga Tushiring (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=uz)** *(⚠️ Eksperimental nashr: WebGL/WASM cheklovlari va xatoliklar uchrashi mumkin)* • **[🌐 Rasmiy Hujjatlar Veb-Sayti](https://rbm13s.github.io/OniAnvil/uz.html)**

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
**Oʻzbekcha** •
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
[Română](README-ro.md) •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Mundarija

- [Kirish](#kirish)
- [Skrinshotlar va vizual sayohat](#skrinshotlar-va-vizual-sayohat)
- [Asboblarning to'liq qo'llanmasi](#asboblarning-toliq-qollanmasi)
- [Funksional hujjatlar](#funksional-hujjatlar)
  - [1. Qo'shaloq atlasga asoslangan arxitektura](#1-qoshaloq-atlasga-asoslangan-arxitektura)
  - [2. Ko'p yo'lakli qatlam vaqt shkalasi](#2-kop-yolakli-qatlam-vaqt-shkalasi)
  - [3. Haqiqiy vaqt rejimida jonli animatsiya ko'rinishi](#3-haqiqiy-vaqt-rejimida-jonli-animatsiya-korinishi)
  - [4. Mukammallashtirilgan piyoz po'sti (Onion Skinning) dvigateli](#4-mukammallashtirilgan-piyoz-posti-onion-skinning-dvigateli)
  - [5. Aqlli namunaviy eskiz qatlami](#5-aqlli-namunaviy-eskiz-qatlami)
  - [6. Aniq tanlov asboblari va yuguruvchi chumolilar](#6-aniq-tanlov-asboblari-va-yuguruvchi-chumolilar)
  - [7. Jonli effektlar, filtrlar va RotSprite](#7-jonli-effektlar-filtrlar-va-rotsprite)
  - [8. Palitralarni boshqarish, almashtirish va qahramon qiyofalari](#8-palitralarni-boshqarish-almashtirish-va-qahramon-qiyofalari)
  - [9. Varaq kesgich, 9-Grid UI va o'yin dvigateliga eksport](#9-varaq-kesgich-9-grid-ui-va-oyin-dvigateliga-eksport)
  - [10. 35 ergonomik mavzu va to'liq mahalliylashtirish](#10-35-ergonomik-mavzu-va-toliq-mahalliylashtirish)
- [Boshqaruv va klaviatura yorliqlari](#boshqaruv-va-klaviatura-yorliqlari)
- [Ishni boshlash](#ishni-boshlash)
  - [Tayyor binar fayllar](#tayyor-binar-fayllar)
  - [Manba kodidan yig'ish](#manba-kodidan-yigish)
- [Fayl formati va hamroh arxitektura](#fayl-formati-va-hamroh-arxitektura)
- [Loyiha rivojiga hissa qo'shish va minnatdorchilik](#loyiha-rivojiga-hissa-qoshish-va-minnatdorchilik)
- [Litsenziya va kafolatdan voz kechish](#litsenziya-va-kafolatdan-voz-kechish)

---

## Kirish

**OniAnvil** (*nomi **Oni**on-Skinning (qatlamli piyoz po'sti animatsiyasi) + **Anvil** (spraytlar bolg'alanadigan bosqon) so'zlarining birikmasidan olingan*), Rust dasturlash tilida yaratilgan ochiq manbali, yuqori unumdorlikka ega piksel grafikasi muharriri va animatsiya studiyasidir. Windows, Linux va macOS operatsion tizimlarida apparat tezlatkichidan foydalangan holda mahalliy tezlikda ishlaydi; shuningdek zamonaviy veb-brauzerlarda o'rnatishni talab qilmaydigan tezkor WebAssembly ilovasi sifatida bevosita foydalanish mumkin.

Mavjud sprayt muharrirlarining aksariyati animatsiyalarni bir-biriga bog'lanmagan alohida tasvirlar to'plami deb hisoblaydi va yaratuvchilarni eksport qilish, qismlarga ajratish hamda qayta paketlash kabi mashaqqatli bosqichlarga majbur etadi. **OniAnvil bu yondashuvni butunlay o'zgartiradi:** o'yin sprayt varag'ining atlas teksturasi (`atlas_rgba`) barcha ma'lumotlarning bosh manbaidir. Kadr katakchalari, qatlam yo'laklari, animatsiya teglari va to'qnashuv tayanch nuqtalari (pivot) doimo atlas bilan to'liq sinxron holatda saqlanadi.

### Nima uchun OniAnvil?
- 🚀 **Rust Tufayli Chaqmoqdek Tez Mahalliy Unumdorlik**: Millisekunddan kam vaqtda ochilish, `wgpu` va `egui 0.31` yordamida tiniq grafika va uzun vaqt shkalalarini varaqlashda hech qanday kechikishning yo'qligi.
- 🎬 **Chiziqli Bo'lmagan Ko'p Yo'lakli Qatlam Vaqt Shkalasi**: Video montaj dasturlaridan ilhomlangan qatlam yo'laklari (`V1, V2, V3...`), qatlamlarni yuqoridan pastga vizual qorishtirish, qatlamlarni birlashtirish, bog'langan kadrlar va o'zgaruvchan ushlab turish muddatlari.
- 🧅 **Birinchi Darajali Piyoz Po'sti (Onion Skinning)**: Oldingi va keyingi kadrlar uchun alohida tus berish (standart: Qizil/Moviy), moslashtiriladigan shaffoflik egri chiziqlari va halqasimon silliq o'tish.
- 🎨 **Bir Lahzada Qahramon Qiyofalari (Skins)**: Atlasni jonli qayta xaritalash orqali tun/kun variantlari, jamoa ranglari yoki sehrli elementlar qiyofasini yarating; GPL, PAL va HEX formatlarini import/eksport qiling.
- 📦 **O'yin Dvigatellariga To'g'ridan-To'g'ri Eksport**: 2D o'yin dvigatellarida (Godot, Bevy, Raylib, Defold) tekstura sizishi va chetki chok nuqsonlarini bartaraf etuvchi chetlari cho'zilgan (extruded) PNG hoshiyalari, standart sprayt varag'i JSON meta-ma'lumotlari va optimallashgan harakatlanuvchi GIF fayllari.

---

## Skrinshotlar va vizual sayohat

<div align="center">

### 1. Asosiy Xolst va Studiya Ish Maydoni
*Apparat tezlatkichli ko'rish oynasi, ko'p yo'lakli vaqt shkalasi, real vaqt rejimida jonli ko'rinish, palitra ranglari va aniq asboblar panellarini o'z ichiga olgan yuqori unumdor ish muhiti.*
<img src="screenshots/main.png" alt="OniAnvil Asosiy Ish Maydoni" width="950" />

<br/><br/>

### 2. Ko'p Yo'lakli Animatsiya Vaqt Shkalasi
*Qatlam qatorlari (`V1, V2...`), bog'langan kadr namunalari (`🔗`), ushlab turish ko'paytiruvchilari va ranglar bilan ajratilgan animatsiya teglariga ega qulay vaqt shkalasi.*
<img src="screenshots/timeline.png" alt="OniAnvil Ko'p Yo'lakli Vaqt Shkalasi" width="950" />

<br/><br/>

### 3. Aniq Asboblar va Harakatlar Paneli
*Standart va ixcham birikish rejimlarini qo'llab-quvvatlaydigan 15 ta rasm chizish asbobi hamda mo'yqalam o'lchami, simmetriya, aylanma o'tish va Pixel-Perfect sozlamalarini beruvchi yuqori panel.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Tezkor Harakatlar Paneli" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Asboblar Standart" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Asboblar Ixcham" width="300" />
</p>

<br/><br/>

### 4. Haqiqiy Vaqt Rejimida Jonli Animatsiya Ko'rinishi
*Teglar bo'yicha saralash, o'zgaruvchan FPS tezligi boshqaruvi, masshtab slayderi va fon rejimlari (shaxmat doskasi, to'q tekis rang, och tekis rang) bilan jihozlangan maxsus ko'rish paneli.*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Jonli Ko'rinish" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Jonli Ko'rinish Masshtabi" width="460" />
</p>

<br/><br/>

### 5. Palitralarni Boshqarish, Almashtirish va Rang Gradiyentlari
*Asosiy/Fon rang qutilari, ikki nuqta o'rtasida avtomatlashtirilgan rang shkalasi (ramp) yaratish va nomlangan palitra qiyofasi (skin) almashtirishlarini ta'minlovchi mukammal tizim.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palitra va Almashtirishlar" width="950" />

<br/><br/>

### 6. Namunaviy Eskiz Chizish va Moslanuvchan Onion Skinning
*Qoralama eskizlar yoki fotosuratlarni ostki/ustki qatlam sifatida yuklash, transformatsiya boshqaruvi, palitraga bog'lanuvchi aqlli pipetka va ko'p kadrli piyoz po'sti soyalanishi.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Eskiz va Piyoz Po'sti" width="950" />

<br/><br/>

### 7. Tayanch Nuqtalari (Pivot) va To'qnashuv Harakat Nuqtalari
*Interaktiv xolst yo'riqnomalari orqali sprayt markazini va maxsus nomlangan o'yin nuqtalarini (qurol o'rni, o'q otish nuqtasi, bosh, qadam tovushi) belgilash.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Tayanch va Harakat Nuqtalari" width="950" />

<br/><br/>

### 8. Sprayt Varag'i Kesgich va 9-Grid (9-Slice) Masshtablagich
*Avtokorrelatsiya orqali avtomatik chegara aniqlash bilan varaqni kesish (masalan, yurayotgan mushuk sikli) va masshtablanuvchi 9-bo'lakli interfeys qutisi hoshiyalarini sozlash.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Varaq Kesgich" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Bo'lakli Masshtablagich" width="460" />
</p>

<br/><br/>

### 9. Real Vaqt Tasvir Effektlari va Filtr Oynalari (Jonli Ko'rinish Bilan)
*Xolstda jonli ko'rsatish, interaktiv pipetka va bekor qilishlar tarixini ifloslantirmasdan rad etish imkonini beruvchi effektlar muloqot oynalari.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Kontur Yaratgich" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Rang Almashtirish" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Rang Tusi To'yinganlik Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matritsa Filtri" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Palitraga Qayta Xaritalash" width="460" />
</p>

<br/><br/>

### 10. Xolst To'ri va Fon Rejimlari
*Piksel to'ri, koshin chegarasi to'ri, to'q shaxmat doskasi, och shaxmat doskasi va moslashtiriladigan yaxlit fon ranglari o'rtasida oson almashing.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="To'r va To'q Shaxmat Foni" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Och Shaxmat Foni" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Maxsus Yaxlit Fon Rangi" width="300" />
</p>

<br/><br/>

### 11. Yangi Loyiha Yaratish Sehrgari
*Loyiha boshlanishida kadr o'lchamlarini, kadrlar sonini, ustunlar to'ri tartibini belgilang va 20 dan ortiq original retro palitralardan birini tanlang.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Yangi Loyiha Sehrgari" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Yangi Loyiha Palitralari" width="460" />
</p>

<br/><br/>

### 12. 35 Ta Haqiqiy Retro va Zamonaviy Mavzular
*Studio Dark, Studio Light, CRT Amber/Yashil/Moviy fosforlari, retro konsollar (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) va maxsus dizaynerlik mavzulari.*
<p align="center">
  <img src="screenshots/themes.png" alt="Mavzular Menyusi" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber Mavzusi" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX Mavzusi" width="300" />
</p>

<br/><br/>

### 13. Menyu Tizimi va Ichki Klaviatura Yordami
<p align="center">
  <img src="screenshots/file_menu.png" alt="Fayl Menyusi" width="180" />
  <img src="screenshots/edit_menu.png" alt="Tahrirlash Menyusi" width="180" />
  <img src="screenshots/select_menu.png" alt="Tanlash Menyusi" width="180" />
  <img src="screenshots/view_menu.png" alt="Ko'rinish Menyusi" width="180" />
  <img src="screenshots/project_menu.png" alt="Loyiha Menyusi" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Klaviatura Yorliqlari Yordam Oynasi" width="700" />
</p>

</div>

---

## Asboblarning to'liq qo'llanmasi

OniAnvil yuqori aniqlikdagi chizish, geometriya, tanlov va meta-ma'lumotlarni joylashtirish uchun 15 ta professional vositani taqdim etadi. Belgilar ochiq kodli **Font Awesome Free** to'plamidan olingan *(Belgilar: CC BY 4.0, Shriftlar: SIL OFL 1.1)*.

| Asbob | Belgi | Yorliq | Mo'yqalam | Pixel Perfect | Tavsifi |
|:---|:---:|:---:|:---:|:---:|:---|
| **Qalam** | ✏️ | <kbd>B</kbd> | 1..32 px | Ha | Asosiy rasm chizish vositasi. 1 px rejimida **Pixel-Perfect** chizishni qo'llab-quvvatlaydi (xunuk L-simon zinapoya burchaklarini avtomatik tozalaydi). Tanlangan sohadan maxsus shtamp mo'yqalam yasash imkoniyatiga ega (`Ctrl/Cmd+B`). |
| **O'chirg'ich** | 🧹 | <kbd>E</kbd> | 1..32 px | Yo'q | Piksellarni to'liq shaffof holatgacha o'chiradi (`[0, 0, 0, 0]`). Faol tanlov mavjud bo'lsa, faqat shu chegara ichida amal qiladi. |
| **Pipetka** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Yo'q | Chap tugma rangni Asosiy (Old) rangga; O'ng tugma Fon rangiga oladi. Xolst piksellaridan, qatlamlardan yoki namunaviy eskizdan rang olishi mumkin. |
| **Bo'yoq idishi** | 🪣 | <kbd>G</kbd> | Yo'q | Yo'q | Sozlanadigan Rang Tolerantligi (`0..=255`) bilan tezkor navbatli BFS to'ldirish. Tutash va Global to'ldirish hamda tanlov bilan cheklashni qo'llab-quvvatlaydi. |
| **Gradient** | 🌈 | <kbd>G</kbd> *(almashish)* | Yo'q | Yo'q | Asosiy va Fon ranglari o'rtasida chiziqli yoki radial gradient chizadi. Bayer (2x2, 4x4, 8x8) va Blue Noise ditering yordamini o'z ichiga oladi. |
| **Chiziq** | 📏 | <kbd>L</kbd> | 1..32 px | Ha | Jonli ko'rinishga ega Brezenxem to'g'ri chiziq algoritmi. Burchaklarni 45°/90° ga qat'iy qilish uchun <kbd>Shift</kbd> tugmasini bosib turing. |
| **Egri chiziq** | 〰️ | <kbd>C</kbd> | 1..32 px | Ha | 3 marta bosiladigan kvadratik Bezye egri chizig'i. 1-bosish Boshlanishni, 2-bosish Oxirini belgilaydi, sichqon harakati egilishni boshqaradi, 3-bosish chiziqni tasdiqlaydi. |
| **To'g'ri to'rtburchak** | ⬜ | <kbd>U</kbd> | 1..32 px | Yo'q | Konturli yoki Bo'yalgan geometrik to'rtburchaklar. 1:1 kvadrat hosil qilish uchun <kbd>Shift</kbd> tugmasini bosing. |
| **Ellips** | ⭕ | <kbd>O</kbd> | 1..32 px | Yo'q | O'rta nuqta rastrlash usuli orqali konturli yoki bo'yalgan ellipslar. Mukammal doira chizish uchun <kbd>Shift</kbd> tugmasini bosing. |
| **Ditering mo'yqalami** | 🏁 | <kbd>D</kbd> | 1..32 px | Yo'q | Palitraga ortiqcha rang qo'shmasdan Asosiy va Fon ranglari o'rtasida shaxmat doskasi ditering naqshlarini chizadi. |
| **Soyalash vositasi** | 🌓 | <kbd>S</kbd> | 1..32 px | Yo'q | Faol palitra rang tartibiga binoan piksellarni asta-sekin Ochroq qilish (Dodge) va To'qroq qilish (Burn) amalini bajaradi. |
| **To'g'ri to'rtburchak tanlov** | ⬚ | <kbd>M</kbd> | Yo'q | Yo'q | Yuguruvchi chumolilar chegarasiga ega to'rtburchak soha tanlovi. Qo'shish (<kbd>Shift</kbd>) va Ayirish (<kbd>Alt</kbd>) amallarini qo'llab-quvvatlaydi. |
| **Lasso tanlovi** | ➰ | <kbd>Q</kbd> | Yo'q | Yo'q | Erkin ko'pburchakli lasso. 8-bog'lanishli chegara rastrlashi va 4-bog'lanishli tashqi to'ldirish yordamida tutashgan halqalarda ham 100% yaxlit va teshiklarsiz tanlov yaratadi. |
| **Sehrli tayoqcha** | 🪄 | <kbd>W</kbd> | Yo'q | Yo'q | Sozlanadigan tolerantlik chegarasi bilan tutash (yoki xolstdagi barcha mos) rang sohalarini tanlaydi. |
| **Tayanch va Harakat** | 🎯 | <kbd>P</kbd> | Yo'q | Yo'q | JSON meta-ma'lumotiga chiqarish uchun Sprayt Boshlang'ich Nuqtasi (Pivot) va maxsus Harakat Nuqtalarini (`weapon`, `head`, `feet`, `hitbox`) xolstga interaktiv joylashtirish. |

---

## Funksional hujjatlar

### 1. Qo'shaloq atlasga asoslangan arxitektura

OniAnvil-da asosiy tekstura atlasi (`atlas_rgba`) o'yin dvigateli tomonidan yuklanadigan asl sprayt varag'i teksturasini ifodalaydi:
- **Kadr Rejimi**: Chalg'ituvchi omillarsiz qulay piksel xolstini ta'minlab, faqat bitta faol animatsiya kadriga e'tiborni qaratadi. Qilingan barcha o'zgarishlar darhol atlas teksturasidagi tegishli katakchaga o'tadi.
- **Atlas Rejimi**: Barcha sprayt varag'i to'rini ko'rsatadi; umumiy sozlashlar, kadrlararo moslikni tekshirish va ommaviy tahrirlash imkonini beradi.
- **Hamroh Fayl**: Loyihalar toza `.png` sprayt varag'i va kadr tuzilishlari, qatlamlar ierarxiyasi, teglar, ushlab turish ko'paytiruvchilari hamda palitra qiyofalarini saqlovchi yengil `.onianvil.json` hamroh fayli shaklida saqlanadi.

### 2. Ko'p yo'lakli qatlam vaqt shkalasi

Pastki vaqt shkalasi professional noan'anaviy video montaj dasturlari asosida yaratilgan:
- **Qatlam Yo'laklari (`V1, V2, V3...`)**: Qatlamlar vizual ravishda yuqoridan pastga joylashadi. Vaqt shkalasida yuqoriroqda joylashgan yo'lak (`V1`), kadr xolstida eng ustki qatlamda chiziladi.
- **Yo'lak Boshqaruvi**:
  - Faol rasm chizish qatlamini tezda almashtirish uchun `V1` / `V2` yorliqlariga bosing.
  - Qatlam ko'rinishini (Ko'z belgisi) va tahrirdan qulflashni (Qulf belgisi) yoqing yoki o'chiring.
  - Har bir qatlamning shaffofligini mustaqil sozlang (`0%..=100%`).
  - Qatlamni yuqoriga/pastga siljiting, nusxasini oling yoki ostidagi yo'lak bilan birlashtiring (Pastga Birlashtirish).
  - Tasdiqlash oynasi orqali qatlamni xavfsiz o'chiring.
- **Kadr Kliplari**:
  - Ushlab turish ko'paytiruvchisi (Hold): alohida kadr davomiyligini belgilang (masalan, zarba oldi pauzalari uchun `2×`, `3×` kechikish).
  - Bog'langan kadrlar: takrorlanuvchi kadrlarni animatsiya bo'ylab bog'lang; birini tahrirlash barcha bog'langan nusxalarni avtomatik yangilaydi.
  - Teg aylanishi: animatsiyalarni nomlangan guruhlarga ajrating (`idle`, `walk`, `attack`) va ijro etishda faqat kerakli tegni aylantiring.
  - Ijro boshqaruvi: Sozlanadigan FPS (`1..=120`) bilan Oldinga, Orqaga, Ping-pong va Takrorlash rejimlari.

### 3. Haqiqiy vaqt rejimida jonli animatsiya ko'rinishi

O'ng tarafdagi Ko'rish paneli xolstingizning masshtabi yoki o'rnidan qat'i nazar animatsiyalarni haqiqiy vaqt rejimida ko'rsatib turadi:
- **Teglar Saralashi**: Ijroni faqat tanlangan teg bilan cheklang (`walk`, `jump`) yoki butun animatsiya ketma-ketligini ko'ring.
- **Ijro Boshqaruvi**: O'ynatish, Pauza, Oldingi Qadam, Keyingi Qadam, Takrorlash va maxsus ijro tezliklari (`0.25x..=4.0x`).
- **Ko'rinish Rejimlari**: To'q shaxmat foni, och shaxmat foni, mutlaq oq, mutlaq qora yoki xolstning maxsus fonlari o'rtasida almashing.

### 4. Mukammallashtirilgan piyoz po'sti (Onion Skinning) dvigateli

Maxsus piyoz po'sti tizimi bilan ravon animatsiyalarni loyihalang:
- **Mustaqil Tus Berish**: Oldingi kadrlar (standart: Qizil) va keyingi kadrlar (standart: Moviy) uchun alohida rang tuslari.
- **Shaffoflik So'nishi**: Dastlabki shaffoflik va keyingi har bir qadam uchun pasayish egri chiziqlarini sozlash.
- **Halqasimon Silliq O'tish**: Cheksiz takrorlanuvchi animatsiyalarda birinchi va oxirgi kadrlar o'rtasida piyoz po'sti soyasini uzluksiz ko'rsatish.
- **Tezkor Qarash (Quick-Peek)**: Sichqonchani qimirlatmasdan <kbd>Bo'sh joy</kbd> tugmasini bosib turish orqali qo'shni kadrlarni darhol tekshirish.

### 5. Aqlli namunaviy eskiz qatlami

Qo'lda chizilgan eskizlar yoki namunaviy fotosuratlarni shablon sifatida ishlatib, ish jarayonini tezlashtiring:
- **Formatlar**: PNG, JPEG va WebP namunaviy tasvirlarini yuklang.
- **Qorishma Rejimlari**: Ostki Qatlam (piksel qatlamlaringiz ostida) yoki Ustki Qatlam (piksel qatlamlaringiz ustida).
- **Transformatsiya Boshqaruvi**: Mustaqil masshtab, joylashuv siljishi, aylantirish burchagi va shaffoflik slayderi.
- **Aqlli Rang Namunasi Olish**: Faol loyiha palitrangizdagi eng yaqin rangga avtomatik bog'lanish imkoniyati bilan eskiz qatlamidan Pipetka orqali rang oling.
- **Bir Bosishda Piksellarga Aylantirish (Bake)**: Masshtablangan namunani birgina bosish orqali faol kadrda toza piksel tasviriga aylantiring.

### 6. Aniq tanlov asboblari va yuguruvchi chumolilar

- **Harakatlanuvchi Yuguruvchi Chumolilar**: Faol tanlovlar yuqori kontrastli, ikki tusli harakatlanuvchi chiziqlar bilan ajratib ko'rsatiladi.
- **Yaxlit To'ldiruvchi Lasso Tanlovi**: Kesishuvchi halqalar chizilganda tanlanmagan teshiklar qoldiruvchi sodda nurlanish algoritmlaridan farqli o'laroq, OniAnvil-ning lasso vositasi yopiq 8-bog'lanishli chegara chizadi va tashqi hududni to'ldirib, halqalarda ham 100% yaxlit va bo'shliqlarsiz tanlov hosil qiladi.
- **Tanlov Amallari**: Kengaytirish, Qisqartirish, Chegara (Doira, Kvadrat va Romb yadrosi bilan), Siljitish, Invertlash, O'chirish, Erkin Aylantirish va Tizim Buferiga Nusxalash/Qirqish/Qo'yish.
- **Kadrlararo Nusxalash va Qo'yish**: Bir kadrdagi sohani tanlang, <kbd>Ctrl/Cmd+C</kbd> bosing, kadrni almashtiring va istalgan klaviatura tilida aynan o'sha nisbiy koordinatalarga qo'yish uchun <kbd>Ctrl/Cmd+V</kbd> bosing.

### 7. Jonli effektlar, filtrlar va RotSprite

Barcha tasvir effektlari kiritilgan o'zgarishlarni xolstda ko'rsatuvchi va bekor qilishlar tarixini ifloslantirmasdan rad etish imkonini beruvchi real vaqtdagi **Jonli Ko'rinish** parametriga ega:
- **Rang Almashtirish (`Shift+R`)**:
  - Interaktiv Pipetka: pipetka belgisini bosing va to'g'ridan-to'g'ri kadr piksellaridan yoki palitradan ranglarni tanlang.
  - Tez tanlash palitra tasmasi: "Qaysi" rangni tanlash uchun chap tugma, "Qaysiga" rangni tanlash uchun o'ng tugma.
  - Tolerantlik slayderi va butun animatsiya kadrlari bo'ylab ommaviy almashtirish yordami.
- **Rang Sozlash (`Ctrl/Cmd+U`)**: Real vaqt rejimida HSV Tusi (Hue), To'yinganlik (Saturation), Yorqinlik (Value) va Kontrastni sozlash.
- **Avtomatik Kontur Yaratgich**: Kvadrat yoki Romb burchak birlashmalari bilan piksel darajasida aniq qahramon konturlarini (Tashqi, Ichki, Markazlashtirilgan) yarating.
- **3×3 Matritsa Filtrlari**: Shaffoflikni saqlab qolish imkoniyati bilan Xiralashtirish, O'tkirlash, Chegaralarni Aniqlash va Bo'rtma filtrlari.
- **Erkin Aylantirish (`Ctrl/Cmd+R`)**: Klassik **RotSprite** yoki **Eng Yaqin Qo'shni** algoritmlari yordamida tanlangan sohalarni yoki to'liq kadrlarni istalgan burchakka burang.
- **Palitraga Qayta Xaritalash**: Bayer 2x2, 4x4, 8x8 yoki Blue Noise yordamida xolst ranglarini faol palitraga ditering orqali moslashtiring.

### 8. Palitralarni boshqarish, almashtirish va qahramon qiyofalari

- **O'rnatilgan Palitralar**: Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized va Sweetie 16 kabi 20 dan ortiq original retro palitralar.
- **Import va Eksport**: GIMP Palitrasi (`.gpl`), PAL (`.pal`), Hex palitrasi (`.hex`) va tasvir rang lentalari (`.png`) formati qo'llab-quvvatlanadi.
- **Palitralarni Almashtirish (Qiyofalar - Skins)**: Bitta qahramon uchun bir nechta rang qiyofasi variantini belgilang (masalan, Standart, Oltin Sovut, Soya, Muz). Qiyofani almashtirish atlas bo'yicha barcha kadr ranglarini darhol qayta xaritalaydi.
- **Rang Gradiyentlari Generatori (Ramp)**: Sozlanadigan oraliq qadamlar soni bilan tanlangan ikki tus o'rtasida silliq rang o'tishlarini avtomatik ravishda hosil qiling.

### 9. Varaq kesgich, 9-Grid UI va o'yin dvigateliga eksport

- **Varaq Kesgich (Sheet Slicer)**: Avtokorrelatsiyali chegaralarni aniqlash algoritmi xom sprayt varaqlarini teng yoki aniq chegaralangan animatsiya kadrlariga avtomatik bo'lib chiqadi.
- **9-Grid (9-Slice) Masshtablash**: Masshtablanadigan interfeys qutilari va tugma panellarini burchaklari buzilmagan holda ko'rish uchun moslashtiriladigan hoshiya o'lchamlarini (yuqori, pastki, chap, o'ng) belgilang.
- **Chetlari Cho'zilgan (Extruded Border) PNG Eksporti**: Chekka piksel ranglarini shaffof chetlarga qarab cho'zish orqali OpenGL, DirectX, Vulkan va 2D o'yin dvigatellarida (masalan, Godot) tekstura filtratsiyasi choklari va rang sizishini bartaraf etadi.
- **Animatsiyali GIF Eksporti**: Takrorlanishlar soni, kadrlar kechikishi va shaffof fonni qo'llab-quvvatlovchi optimallashgan harakatlanuvchi GIF fayllarini yarating.
- **Standart Sprayt Varag'i JSON**: Zamonaviy 2D o'yin dvigatellarida qabul qilingan standart sprayt varag'i JSON sxemasiga mos keluvchi kadr bo'laklari, teglar, davomiyliklar va tayanch nuqtalari koordinatalarini eksport qiladi.

### 10. 35 ergonomik mavzu va to'liq mahalliylashtirish

- **Tanlangan Mavzular**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Yashil CRT, Moviy CRT, Retro Konsollar (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) va Retro Kompyuterlar (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Keng Til Qo'llab-Quvvatlashi**: 100% lug'at kalitlari mosligi bilan tillar o'rtasida erkin almashing.

---

## Boshqaruv va klaviatura yorliqlari

| Yorliq | Asbob / Amal | Tavsifi |
|:---|:---|:---|
| <kbd>B</kbd> | **Qalam** | Pixel-Perfect rejimiga ega 1 px aniq chizish |
| <kbd>E</kbd> | **O'chirg'ich** | Piksellarni shaffof holatgacha o'chiradi |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipetka** | Xolst yoki namunaviy eskizdan rang namunasini oladi |
| <kbd>G</kbd> | **Bo'yoq idishi / Gradient** | Bo'yoq idishi va Ditering Gradienti o'rtasida almashadi |
| <kbd>L</kbd> | **Chiziq** | Jonli ko'rinish va Shift burchak qat'iyligi bilan to'g'ri chiziqlar |
| <kbd>C</kbd> | **Egri chiziq** | 3 marta bosiladigan kvadratik Bezye egri chizig'i |
| <kbd>U</kbd> | **To'g'ri to'rtburchak** | Geometrik to'rtburchak va kvadrat konturi/to'ldirilishi |
| <kbd>O</kbd> | **Ellips** | Geometrik ellips va doira konturi/to'ldirilishi |
| <kbd>D</kbd> | **Ditering mo'yqalami** | Shaxmat doskasi ditering matritsasi naqshlari bilan chizadi |
| <kbd>S</kbd> | **Soyalash vositasi** | Palitra bo'ylab Ochroq / To'qroq qilish amalini bajaradi |
| <kbd>M</kbd> | **To'g'ri to'rtburchak tanlov** | Yuguruvchi chumolilar bilan to'rtburchak sohani tanlash |
| <kbd>Q</kbd> | **Lasso tanlovi** | Erkin chiziqlar bilan butunlay yaxlit ko'pburchak tanlovi |
| <kbd>W</kbd> | **Sehrli tayoqcha** | Bir xil rangdagi tutash hududlarni tanlaydi |
| <kbd>P</kbd> | **Tayanch Nuqtalari** | Sprayt markazi va harakat nuqtalarini joylashtiradi |
| <kbd>X</kbd> | **Ranglarni Almashtirish** | Asosiy va Fon ranglarining o'rnini almashtiradi |
| <kbd>[</kbd> / <kbd>]</kbd> | **Mo'yqalam O'lchami** | Mo'yqalam o'lchamini kichraytiradi / kattalashtiradi (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Nusxalash** | Tanlangan piksellarni tizim buferiga nusxalaydi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Qirqib olish** | Tanlangan piksellarni buferga qirqib oladi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Qo'yish** | Buferdagi piksellarni faol kadrga qo'yadi |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **O'chirish** | Faol tanlov ichidagi piksellarni tozalaydi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Hammasini Tanlash** | Faol kadrdagi barcha piksellarni tanlaydi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Tanlovni Bekor Qilish** | Faol tanlov niqobini tozalaydi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Tanlovni Invertlash** | Faol tanlov niqobini teskari qiladi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Kadr Nusxasini Yaratish** | Vaqt shkalasidagi faol kadrni nusxalaydi |
| <kbd>Bo'sh joy</kbd> | **O'ynatish / Pauza** | Vaqt shkalasida animatsiya ijrosini boshlaydi/to'xtatadi |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Rang Almashtirish** | Jonli ko'rinishga ega Rang Almashtirish oynasini ochadi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Rang Sozlash** | HSV Tusi / To'yinganlik / Kontrast oynasini ochadi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Erkin Aylantirish** | Istalgan burchakka burish oynasini (RotSprite) ochadi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Bekor Qilish** | Xolstdagi oxirgi chizish yoki asbob amalini bekor qiladi |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Qaytarish** | Bekor qilingan amalni yana qaytaradi |
| <kbd>Bo'sh joy</kbd> + Surish / O'rta Tugma | **Xolstni Siljitish** | Xolst koordinatalari bo'ylab silliq harakatlanish |
| <kbd>Ctrl/Cmd</kbd> + G'ildirakcha / <kbd>+</kbd> / <kbd>-</kbd> | **Masshtab** | Kursor markazida piksellar aniqligida yaqinlashtirish/uzoqlashtirish |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Ko'rinish Rejimi** | Kadr Ko'rinishi (<kbd>1</kbd>) va Atlas Ko'rinishi (<kbd>2</kbd>) o'rtasida almashish |
| <kbd>F</kbd> | **Xolstni Sig'dirish** | Faol kadrni ish maydoni markaziga sig'diradi |
| <kbd>F1</kbd> | **Yordam** | Klaviatura Yorliqlari yordam oynasini ochadi |

---

## Ishni boshlash

### Tayyor binar fayllar

Oldindan yig'ilgan mustaqil ijro fayllarini **[GitHub Chiqarilishlar Sahifasidan](https://github.com/rbm13s/OniAnvil/releases/latest)** yuklab oling:

- **Windows**: `OniAnvil-windows-x86_64.zip` arxivini yuklab oling, arxivdan chiqaring va `OniAnvil.exe` faylini ishga tushiring.
- **Linux**: `OniAnvil-linux-x86_64.tar.gz` portativ arxivini yuklab oling yoki Debian/Ubuntu uchun `onianvil_*_amd64.deb` paketini o'rnating:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: `OniAnvil-macos-universal.zip` arxivini yuklab oling, arxivdan chiqaring va `OniAnvil.app` faylini Ilovalar (Applications) papkasiga o'tkazing. Universal binar paket Apple Silicon (M1/M2/M3/M4) hamda Intel Mac kompyuterlarida mahalliy ishlaydi.
- **Web**: Hech narsa o'rnatmasdan to'g'ridan-to'g'ri brauzeringizda ishga tushiring: **[WebAssembly Ilovasini Ochish](https://rbm13s.github.io/OniAnvil/app/?lang=uz)**.
  > [!WARNING]
  > WebAssembly (brauzer) nashri **Eksperimental (Experimental)** maqomiga ega. Brauzer xavfsizlik cheklovlari, WebGL/WebGPU farqlari va tizim buferi xususiyatlari tufayli mahalliy ish stoli versiyasiga nisbatan kichik vizual kamchiliklar yoki tezlik pasayishi kuzatilishi mumkin. Mahsulot yaratish ishlari uchun mahalliy ish stoli versiyalaridan foydalanish qat'iy tavsiya etiladi.

---

### Manba kodidan yig'ish

#### Talablar
- **Rust Asboblar To'plami**: [rustup.rs](https://rustup.rs/) orqali Rust-ni o'rnating (1.80+ tavsiya etiladi):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux Yig'ish Bog'liqliklari** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Mahalliy Ish Stoli Versiyasini Yig'ish va Ishga Tushirish
```bash
# Repozitoriyni klonlash
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Testlarni ishga tushirish
cargo test

# Optimallashgan nashrni yig'ish va ishga tushirish
cargo run --release
```

#### 2. WebAssembly Versiyasini Yig'ish (Trunk orqali)
```bash
# wasm32 maqsadli platformasini va Trunk yig'uvchisini o'rnatish
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Jonli yangilanish bilan mahalliy serverni ishga tushirish
trunk serve

# Tayyor fayllarni docs/app/ papkasiga yig'ish
./build_wasm.sh        # Linux/macOS tizimlarida
./build_wasm.ps1       # Windows PowerShell tizimida
```

---

## Fayl formati va hamroh arxitektura

OniAnvil yopiq yoki mulkiy binar formatlardan foydalanmaydi. Har bir loyiha versiyalarni boshqarish tizimlari bilan to'liq mos keluvchi ikkita toza fayldan iborat:

```
my_character/
├── my_character.png            # Standart PNG sprayt varag'i atlasi (har qanday dvigatel yoki ko'rish dasturida ochiladi)
└── my_character.onianvil.json   # Hamroh meta-ma'lumot fayli
```

`.onianvil.json` hamroh faylida quyidagilar saqlanadi:
- Xolst to'ri sozlamalari: kadr o'lchamlari, ustunlar soni, jami kadrlar soni.
- Qatlamlar ierarxiyasi: yo'lak nomlari, shaffoflik darajalari, ko'rinish bayroqlari, qulflanish holatlari.
- Animatsiya teglari: nomi, boshlang'ich/oxirgi kadr indekslari, ijro yo'nalishi (Oldinga, Orqaga, Ping-pong), kadr davomiyliklari.
- Palitra ta'riflari: faol ranglar, nomlangan qiyofa variantlari, faol qiyofa indeksi.
- Namunaviy eskiz holati: tasvir fayl yo'li, masshtab, siljish o'rni, aylanish burchagi, shaffoflik, qatlam rejimi.
- Tayanch nuqtalari: maxsus harakat nuqtalari, qurol mahkamlash o'rni va to'qnashuv markazlari.

---

## Loyiha rivojiga hissa qo'shish va minnatdorchilik

Sizning hissangiz, xatoliklar haqidagi xabarlaringiz va takliflaringiz har doim mamnuniyat bilan qabul qilinadi!
1. GitHub-da repozitoriyni fork qiling.
2. Yangi funksiya tarmog'ini oching: `git checkout -b feature/yangi-asbob`.
3. O'zgarishlaringizni saqlang: `git commit -am 'Yangi piksel asbobi qo‘shildi'`.
4. Tarmoqni serverga yuboring: `git push origin feature/yangi-asbob`.
5. Pull Request (Birlashtirish So'rovi) oching.

### Minnatdorchilik
- Boshlang'ich namoyish animatsiyasi sprayti **"Cat 2D Pixel Art"**, muallif **xzany** tomonidan yaratilgan ([itch.io loyiha sahifasi](https://xzany.itch.io/cat-2d-pixel-art)).
- Ajoyib **[egui](https://github.com/emilk/egui)** grafik interfeys kutubxonasi va **[wgpu](https://wgpu.rs/)** grafika ekotizimi asosida qurilgan.
- Foydalanuvchi interfeysi belgilari @fontawesome tomonidan taqdim etilgan **[Font Awesome Free](https://fontawesome.com)** to'plami bilan ta'minlangan (Belgilar: CC BY 4.0, Shriftlar: SIL OFL 1.1, Kod: MIT Litsenziyasi).

---

## Litsenziya va kafolatdan voz kechish

Ushbu loyiha **[MIT Litsenziyasi](LICENSE)** asosida litsenziyalangan. Siz OniAnvil-dan tijoriy va shaxsiy o'yinlar yaratish jarayonida erkin foydalanishingiz, uni o'zgartirishingiz, tarqatishingiz va loyihalaringizga qo'shishingiz mumkin.

### Kafolatdan Voz Kechish va Javobgarlikni Cheklash
> [!IMPORTANT]
> **OniAnvil** dasturi ochiq yoki nazarda tutilgan hech qanday kafolatlarsiz, **bepul**, **"XUDDI SHUNDAY" ("AS IS")** va **"MAVJUD BO'LGANIDEK" ("AS AVAILABLE")** tamoyili asosida tarqatiladi.
> 
> Mualliflik huquqi egalari, mualliflar yoki hissa qo'shuvchilar; ushbu dasturiy ta'minotdan foydalanish yoki foydalana olmaslik oqibatida yuzaga keladigan har qanday to'g'ridan-to'g'ri, bilvosita, tasodifiy, maxsus, jazo tariqasidagi yoki keltirib chiqarilgan zararlar (jumladan, ma'lumotlarning yo'qolishi, loyiha fayllarining buzilishi, apparat nosozliklari, boy berilgan foyda yoki ish to'xtab qolishi kabilarni o'z ichiga olgan holda) uchun hech qanday sharoitda javobgarlikni o'z zimmasiga olmaydi. Dasturdan foydalanish bilan bog'liq barcha xavf-xatarlar foydalanuvchining zimmasidadir.

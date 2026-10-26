<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Loqosu" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Oyun Atlasları Ətrafında Qurulmuş Piksel Qrafikası və Soğan Qabığı (Onion-Skinning) Animasiya Studiyası**

[![Quruluş və Buraxılış](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Yükləmələr](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderləşdirici](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Dillər](https://img.shields.io/badge/Languages-37_Dil-purple)](#10-35-erqonomik-tema-ve-tam-lokallasdirma)
[![Platforma](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#baslangic)
[![Onlayn Tətbiq](https://img.shields.io/badge/Veb_Tətbiq-Brauzerdə_Sına_(Eksperimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=az)
[![Lisenziya: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 OniAnvil-i Brauzerdə Sınaqdan Keçirin (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=az)** *(⚠️ Eksperimental buraxılış: WebGL/WASM məhdudiyyətləri və xətalar ola bilər)* • **[🌐 Rəsmi Sənədləşmə Veb-Saytı](https://rbm13s.github.io/OniAnvil/az.html)**

<br/>

**Languages / Языки / Мови / Sprachen / 语言:**<br/>
[English](../README.md) •
[Русский](README-ru.md) •
[Українська](README-uk.md) •
[Беларуская](README-be.md) •
[Қазақша](README-kk.md) •
[ქართული](README-ka.md) •
[Հայերեն](README-hy.md) •
**Azərbaycan** •
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
[Română](README-ro.md) •
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## Mündəricat

- [Giriş](#giris)
- [Ekran görüntüləri və vizual tur](#ekran-goruntuleri-ve-vizual-tur)
- [Hərtərəfli alətlər bələdçisi](#herterefli-aletler-beledcisi)
- [Funksional sənədləşmə](#funksional-senedlesme)
  - [1. İkili atlas mərkəzli arxitektura](#1-ikili-atlas-merkezli-arxitektura)
  - [2. Çoxzolaqlı qat zaman xətti](#2-coxzolaqli-qat-zaman-xetti)
  - [3. Real vaxtda canlı animasiya önbaxışı](#3-real-vaxtda-canli-animasiya-onbaxisi)
  - [4. Təkmilləşdirilmiş soğan qabığı (Onion Skinning) mühərriki](#4-tekmillesdirilmis-sogan-qabigi-onion-skinning-muherriki)
  - [5. Ağıllı istinad və eskiz qatı](#5-agilli-istinad-ve-eskiz-qati)
  - [6. Dəqiq seçim dəsti və qaçan qarışqalar](#6-deqiq-secim-desti-ve-qacan-qarisqalar)
  - [7. Canlı effektlər, filtrlər və RotSprite](#7-canli-effektler-filtrler-ve-rotsprite)
  - [8. Palitraların idarə edilməsi, rəng dəyişimi və personaj cildləri](#8-palitralarin-idare-edilmesi-reng-deyisimi-ve-personaj-cildleri)
  - [9. Vərəq kəsici, 9-Grid UI və oyun mühərrikinə ixrac](#9-vereq-kesici-9-grid-ui-ve-oyun-muherrikine-ixrac)
  - [10. 35 erqonomik tema və tam lokallaşdırma](#10-35-erqonomik-tema-ve-tam-lokallasdirma)
- [İdarəetmə və klaviatura qısayolları](#idareetme-ve-klaviatura-qisayollari)
- [Başlanğıc](#baslangic)
  - [Hazır binar fayllar](#hazir-binar-fayllar)
  - [Mənbə kodundan yığma](#menbe-kodundan-yigma)
- [Fayl formatı və müşayiətçi arxitektura](#fayl-formati-ve-musayietci-arxitektura)
- [Töhfə vermə və təşəkkürlər](#tohfe-verme-ve-tesekkurler)
- [Lisenziya və zəmanətdən imtina](#lisenziya-ve-zemanetden-imtina)

---

## Giriş

**OniAnvil** (*adı **Oni**on-Skinning (soğan qabığı animasiyası) + **Anvil** (sprayt döymə zindanı) sözlərindən yaranıb*), Rust proqramlaşdırma dilində yaradılmış açıq mənbəli, yüksək məhsuldarlıqlı piksel qrafikası redaktoru və animasiya studiyasıdır. Windows, Linux və macOS əməliyyat sistemlərində aparat sürətləndirməsi ilə yerli sürətdə icra olunur; həmçinin quraşdırma tələb etmədən müasir veb-brauzerlərdə birbaşa WebAssembly tətbiqi kimi fəaliyyət göstərir.

Mövcud sprayt redaktorlarının böyük əksəriyyəti animasiyaya ayrı-ayrı şəkil faylları toplusu kimi yanaşır və tərtibatçıları yorucu ixrac, kəsmə və yenidən qablaşdırma addımlarına məcbur edir. **OniAnvil bu yanaşmanı kökündən dəyişir:** oyun sprayt vərəqinin atlas teksturası (`atlas_rgba`) əsas həqiqət mənbəyidir. Kadr xanaları, qat zolaqları, animasiya teqləri və toqquşma dayaq nöqtələri (pivot) hər an atlasla sinxron vəziyyətdə saxlanılır.

### Niyə OniAnvil?
- 🚀 **İldırım Sürətli Yerli Rust Məhsuldarlığı**: Milisaniyədən az açılış müddəti, `wgpu` və `egui 0.31` tərəfindən təmin edilən dəqiq qrafika və uzun zaman xətlərində naviqasiya zamanı sıfır gecikmə.
- 🎬 **Qeyri-Xətti Çoxzolaqlı Qat Zaman Xətti**: Video montaj proqramlarından ilhamlanmış qat zolaqları (`V1, V2, V3...`), yuxarıdan aşağıya qatların vizual qarışdırılması, qatların birləşdirilməsi, əlaqəli kadrlar və dəyişən saxlama müddətləri.
- 🧅 **Birinci Dərəcəli Soğan Qabığı (Onion Skinning)**: Əvvəlki və sonrakı kadrlar üçün müstəqil rəngləmə (standart: Qırmızı/Mavi), tənzimlənən şəffaflıq əyriləri və dövri keçid.
- 🎨 **Dərhal Personaj Palitra Cildləri (Skins)**: Atlasın canlı yenidən xəritələnməsi ilə gecə/gündüz variantları, komanda rəngləri və ya elementar cildlər yaradın; GPL, PAL və HEX formatlarını idxal/ixrac edin.
- 📦 **Oyun Mühərriklərinə Birbaşa İxrac**: 2D oyun mühərriklərində (Godot, Bevy, Raylib, Defold) tekstura sızmasını və kənar tikiş qüsurlarını aradan qaldıran genişləndirilmiş (extruded) PNG haşiyələri, standart sprayt vərəqi JSON meta-məlumatları və optimallaşdırılmış animasiyalı GIF faylları.

---

## Ekran görüntüləri və vizual tur

<div align="center">

### 1. Əsas Kətan və Studiya İş Sahəsi
*Aparat sürətləndirməli baxış pəncərəsi, çoxzolaqlı zaman xətti, real vaxtda canlı önbaxış, palitra rəngləri və dəqiq alət panellərini ehtiva edən yüksək məhsuldarlıqlı iş mühiti.*
<img src="screenshots/main.png" alt="OniAnvil Əsas İş Sahəsi" width="950" />

<br/><br/>

### 2. Çoxzolaqlı Animasiya Zaman Xətti
*Qat sətirləri (`V1, V2...`), əlaqəli kadr nümunələri (`🔗`), saxlama vurucuları və rəng kodlu animasiya teqləri olan qeyri-xətti çoxzolaqlı zaman xətti.*
<img src="screenshots/timeline.png" alt="OniAnvil Çoxzolaqlı Zaman Xətti" width="950" />

<br/><br/>

### 3. Dəqiq Alətlər və Əməliyyat Paneli
*Standart və yığcam birləşmə rejimlərini dəstəkləyən 15 alətlik rəsm dəsti və fırça ölçüsü, simmetriya, dövretmə və Pixel-Perfect seçimlərini təqdim edən yuxarı cəld əməliyyat paneli.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Cəld Əməliyyat Paneli" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Alətlər Standart" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Alətlər Yığcam" width="300" />
</p>

<br/><br/>

### 4. Real Vaxtda Canlı Animasiya Önbaxışı
*Teq süzgəci, dəyişən FPS sürət tənzimləyiciləri, miqyas sürüşdürücüsü və fon rejimləri (şahmat lövhəsi, tünd düz rəng, açıq düz rəng) olan xüsusi interaktiv baxış paneli.*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Canlı Önbaxış" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Canlı Önbaxış Miqyası" width="460" />
</p>

<br/><br/>

### 5. Palitraların İdarə Edilməsi, Rəng Dəyişimi və Rəng Keçidləri
*Əsas/Fon rəng qutuları, iki nöqtə arası avtomatlaşdırılmış rəng şkalası (ramp) istehsalı və adlandırılmış palitra cildi (skin) dəyişikliklərini təmin edən hərtərəfli rəng sistemi.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palitra və Dəyişikliklər" width="950" />

<br/><br/>

### 6. İstinad Eskizinin Çəkilməsi və Tənzimlənən Onion Skinning
*Konsept eskizlərini və ya fotoşəkilləri alt/üst qat kimi yükləmə, transformasiya idarəetməsi, palitraya bağlanan ağıllı pipet və çoxkadrllı soğan qabığı kölgələnməsi.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Eskiz və Soğan Qabığı" width="950" />

<br/><br/>

### 7. Dayaq Nöqtələri (Pivot) və Toqquşma Fəaliyyət Nöqtələri
*İnteraktiv kətan bələdçiləri ilə spraytın mərkəzini və xüsusi adlandırılmış oyun nöqtələrini (silah yuvaları, mərmi çıxışları, baş, addım səsləri) təyin etmə.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Dayaq və Fəaliyyət Nöqtələri" width="950" />

<br/><br/>

### 8. Sprayt Vərəqi Kəsici və 9-Grid (9-Slice) Miqyaslayıcı
*Avtokorrelyasiya sərhəd aşkarlaması ilə avtomatik vərəq kəsmə (məsələn, pişik yerişi dövrü) və miqyaslanan 9-hissəli interfeys qutusu kənar boşluğu tənzimləməsi.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Vərəq Kəsici" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Hissəli Miqyaslayıcı" width="460" />
</p>

<br/><br/>

### 9. Real Vaxt Təsvir Effektləri və Filtr Dialoqları (Canlı Önbaxış ilə)
*Canlı kətan önbaxışı, interaktiv pipet və ləğv etmə tarixçəsini pozmadan imtina imkanı təqdim edən effekt dialoqları.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Kontur Generatoru" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Rəng Əvəzləmə" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Rəng Çaları Doyma Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matris Filtri" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Palitraya Yenidən Xəritələmə" width="460" />
</p>

<br/><br/>

### 10. Kətan Toru və Fon Rejimləri
*Piksel toru, kafel sərhəd toru, tünd şahmat lövhəsi, açıq şahmat lövhəsi və fərdi bütöv fon rəngləri arasında rahatlıqla keçid edin.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Tor və Tünd Şahmat Fonu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Açıq Şahmat Fonu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Xüsusi Bütöv Fon Rəngi" width="300" />
</p>

<br/><br/>

### 11. Yeni Layihə Yaratma Sehrbazı
*Layihə başladıqda kətan ölçülərini, kadr sayını, sütun toru nizamını təyin edin və 20-dən çox orijinal retro palitra arasından seçim edin.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Yeni Layihə Sehrbazı" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Yeni Layihə Palitraları" width="460" />
</p>

<br/><br/>

### 12. 35 Orijinal Retro və Müasir Mövzu
*Studio Dark, Studio Light, CRT Amber/Yaşıl/Mavi fosforları, retro konsollar (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) və dizayner mövzuları.*
<p align="center">
  <img src="screenshots/themes.png" alt="Mövzular Menyusu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber Mövzusu" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX Mövzusu" width="300" />
</p>

<br/><br/>

### 13. Menyu Sistemi və Daxili Klaviatura Yardımı
<p align="center">
  <img src="screenshots/file_menu.png" alt="Fayl Menyusu" width="180" />
  <img src="screenshots/edit_menu.png" alt="Redaktə Menyusu" width="180" />
  <img src="screenshots/select_menu.png" alt="Seçim Menyusu" width="180" />
  <img src="screenshots/view_menu.png" alt="Görünüş Menyusu" width="180" />
  <img src="screenshots/project_menu.png" alt="Layihə Menyusu" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Klaviatura Qısayolları Yardım Pəncərəsi" width="700" />
</p>

</div>

---

## Hərtərəfli alətlər bələdçisi

OniAnvil dəqiq rəsm, həndəsə, seçim və meta-məlumat yerləşdirmə üçün 15 peşəkar aləti ehtiva edir. Nişanlar açıq mənbəli **Font Awesome Free** ikon dəsti ilə təqdim olunur *(Nişanlar: CC BY 4.0, Şriftlər: SIL OFL 1.1)*.

| Alət | Nişan | Qısayol | Fırça Ölçüsü | Pixel Perfect | Təsvir |
|:---|:---:|:---:|:---:|:---:|:---|
| **Qələm** | ✏️ | <kbd>B</kbd> | 1..32 px | Bəli | Əsas rəsm aləti. 1 px rejimində **Pixel-Perfect** çəkməni dəstəkləyir (arzuolunmaz L-şəkilli pillə künclərini avtomatik təmizləyir). Seçimdən xüsusi möhür fırçası yaratmağı dəstəkləyir (`Ctrl/Cmd+B`). |
| **Pozan** | 🧹 | <kbd>E</kbd> | 1..32 px | Xeyr | Pikselləri tam şəffaflığa qədər silir (`[0, 0, 0, 0]`). Aktiv seçim varsa yalnız seçim sahəsi daxilində işləyir. |
| **Pipet** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Xeyr | Sol klik rəngi Əsas (Ön plan) rəngə; Sağ klik Fon rənginə götürür. Kətan piksellərindən, qatlardan və ya istinad eskizindən rəng götürə bilir. |
| **Boya qabı** | 🪣 | <kbd>G</kbd> | Yoxdur | Xeyr | Tənzimlənən Rəng Tolerantlığı (`0..=255`) ilə yüksək sürətli növbə əsaslı BFS doldurma. Qonşu və Qlobal doldurma ilə seçim məhdudiyyətini dəstəkləyir. |
| **Qradiyent** | 🌈 | <kbd>G</kbd> *(keçid)* | Yoxdur | Xeyr | Əsas və Fon rəngləri arasında xətti və ya radial qradiyent çəkir. Bayer (2x2, 4x4, 8x8) və Blue Noise dizerinq dəstəyinə malikdir. |
| **Xətt** | 📏 | <kbd>L</kbd> | 1..32 px | Bəli | Canlı sürükləmə önbaxışlı Brezenhem xətt alqoritmi. Bucaqları 45°/90° addımlarına bərkitmək üçün <kbd>Shift</kbd> düyməsini basıb saxlayın. |
| **Əyri** | 〰️ | <kbd>C</kbd> | 1..32 px | Bəli | 3 klikli kvadratik Bezye əyrisi. 1-ci klik Başlanğıcı, 2-ci klik Sonu təyin edir, siçan hərəkəti əyriliyi nizamlayır, 3-cü klik əyrini təsdiqləyir. |
| **Düzbucaqlı** | ⬜ | <kbd>U</kbd> | 1..32 px | Xeyr | Konturlu və ya Dolu həndəsi düzbucaqlılar. 1:1 kvadrat çəkmək üçün <kbd>Shift</kbd> saxlayın. |
| **Elips** | ⭕ | <kbd>O</kbd> | 1..32 px | Xeyr | Orta nöqtə rastrlaşdırması ilə konturlu və ya dolu elipslər. Mükəmməl bir dairə çəkmək üçün <kbd>Shift</kbd> saxlayın. |
| **Dizerinq fırçası** | 🏁 | <kbd>D</kbd> | 1..32 px | Xeyr | Palitraya əlavə rənglər qatmadan Əsas və Fon rəngləri arasında şahmat lövhəsi dizerinq naxışları ilə rəngləyir. |
| **Kölgələndirmə aləti** | 🌓 | <kbd>S</kbd> | 1..32 px | Xeyr | Aktiv palitra rəng ardıcıllığına əsasən pikselləri addım-addım Açıqlaşdırma (Dodge) və Tündləşdirmə (Burn) əməliyyatı aparır. |
| **Düzbucaqlı seçim** | ⬚ | <kbd>M</kbd> | Yoxdur | Xeyr | Qaçan qarışqalar sərhədli düzbucaqlı sahə seçimi. Əlavə etmə (<kbd>Shift</kbd>) və Çıxarma (<kbd>Alt</kbd>) əməliyyatlarını dəstəkləyir. |
| **Lasso seçimi** | ➰ | <kbd>Q</kbd> | Yoxdur | Xeyr | Sərbəst çoxbucaqlı lasso. 8-əlaqəli sərhəd rastrlaşdırması və 4-əlaqəli xarici doldurmadan istifadə edərək kəsişən ilgəklərdə belə 100% bütöv və dəliksiz seçim təmin edir. |
| **Sehrli çubuq** | 🪄 | <kbd>W</kbd> | Yoxdur | Xeyr | Tənzimlənən tolerantlıq həddi ilə qonşu (və ya bütün kətandakı uyğun) rəng zonalarını seçir. |
| **Dayaq və Fəaliyyət** | 🎯 | <kbd>P</kbd> | Yoxdur | Xeyr | JSON meta-məlumatına ixrac edilmək üçün Sprayt Başlanğıc Nöqtəsinin (Pivot) və xüsusi oyun Fəaliyyət Nöqtələrinin (`weapon`, `head`, `feet`, `hitbox`) kətana interaktiv yerləşdirilməsi. |

---

## Funksional sənədləşmə

### 1. İkili atlas mərkəzli arxitektura

OniAnvil-də əsas tekstura atlası (`atlas_rgba`) oyun mühərrikiniz tərəfindən birbaşa yüklənən həqiqi sprayt vərəqi teksturasını təmsil edir:
- **Kadr Rejimi**: Diqqəti yayındırmayan bir piksel kətanı təmin edərək yalnız tək bir aktiv animasiya kadrına fokuslanır. Çəkilən hər bir ştrix anında atlas teksturasındakı müvafiq xanaya köçürülür.
- **Atlas Rejimi**: Bütün sprayt vərəqi torunu nümayiş etdirir; makro tənzimləmələrə, kadrlararası düzülüş nəzarətinə və toplu redaktəyə imkan verir.
- **Müşayiətçi Fayl**: Layihələr təmiz `.png` sprayt vərəqi və kadr sxemlərini, qat iyerarxiyasını, teqləri, saxlama vurucularını və palitra cildlərini ehtiva edən yüngül `.onianvil.json` müşayiətçi faylı kimi saxlanılır.

### 2. Çoxzolaqlı qat zaman xətti

Aşağı zaman xətti peşəkar qeyri-xətti video montaj proqramları əsasında layihələndirilmişdir:
- **Qat Zolaqları (`V1, V2, V3...`)**: Qatlar vizual olaraq yuxarıdan aşağıya doğru düzülür. Zaman xəttində daha yuxarıda yerləşən zolaq (`V1`), kadr kətanında ən üstdə nümayiş etdirilir.
- **Zolaq İdarəetməsi**:
  - Aktiv rəsm qatını cəld dəyişmək üçün `V1` / `V2` lövhəciklərinə klikləyin.
  - Qatın görünüşünü (Göz nişanı) və qatın redaktə kilidini (Qıfıl nişanı) açıb-bağlayın.
  - Müstəqil qat şəffaflığını tənzimləyin (`0%..=100%`).
  - Qatı iyerarxiyada yuxarı/aşağı daşıyın, dublikatını çıxarın və ya altındakı zolaqla birləşdirin (Aşağı Birləşdir).
  - Təsdiq pəncərəsi vasitəsilə qatı təhlükəsiz şəkildə silin.
- **Kadr Klipləri**:
  - Saxlama vurucusu (Hold): fərdi kadr müddətləri təyin edin (məsələn, zərbə anları və ya fasilələr üçün `2×`, `3×` gecikmə).
  - Əlaqəli kadrlar: təkrarlanan kadrları animasiya boyu bir-birinə bağlayın; birində edilən dəyişiklik bütün əlaqəli nüsxələri avtomatik yeniləyir.
  - Teq dövrəsi: animasiyaları adlandırılmış teqlər üzrə qruplaşdırın (`idle`, `walk`, `attack`) və oxutma zamanı xüsusi teqləri dövrəyə salın.
  - Oxutma idarəetməsi: Tənzimlənən FPS (`1..=120`) ilə İrəli, Geri, Pinq-ponq və Dövrə rejimlərində oynatma.

### 3. Real vaxtda canlı animasiya önbaxışı

Sağ paneldə yerləşən Önbaxış bölməsi kətanınızın miqyasından və ya yerindən asılı olmayaraq animasiyaları real vaxtda nümayiş etdirir:
- **Teq Süzgəci**: Oxutmanı konkret teqlərlə məhdudlaşdırın (`walk`, `jump`) və ya bütün animasiya zəncirinə baxın.
- **Oxutma İdarəetməsi**: Oynat, Fasilə, Əvvəlki Addım, Növbəti Addım, Dövrə keçidi və xüsusi oxutma sürətləri (`0.25x..=4.0x`).
- **Görüntü Rejimləri**: Tünd şahmat fonu, açıq şahmat fonu, təmiz ağ, təmiz qara və ya fərdi kətan fonları arasında keçid edin.

### 4. Təkmilləşdirilmiş soğan qabığı (Onion Skinning) mühərriki

Xüsusi soğan qabığı sistemi ilə axıcı animasiyalar formalaşdırın:
- **Müstəqil Rəngləmə**: Əvvəlki kadrlar (standart: Qırmızı) və sonrakı kadrlar (standart: Mavi) üçün fərdi rəng çalarları.
- **Şəffaflıq Sönməsi**: Tənzimlənən ilkin şəffaflıq və hər sonrakı addım üçün tədrici zəifləmə əyriləri.
- **Dövri Keçid**: Qapalı dövri animasiyalarda birinci və sonuncu kadrlar arasında soğan qabığı xəyalını kəsintisiz nümayiş etdirin.
- **Cəld Baxış (Quick-Peek)**: Siçanı tərpətmədən <kbd>Boşluq</kbd> düyməsini basıb saxlayaraq qonşu kadrları anında nəzərdən keçirin.

### 5. Ağıllı istinad və eskiz qatı

Əllə çəkilmiş eskizləri və ya istinad fotoşəkillərini şablon kimi istifadə edərək işinizi sürətləndirin:
- **Formatlar**: PNG, JPEG və WebP formatında istinad şəkillərini yükləyin.
- **Qarışdırma Rejimləri**: Alt Qat (piksel qatlarınızın altında) və ya Üst Qat (piksel qatlarınızın üstündə).
- **Transformasiya Nəzarəti**: Müstəqil miqyas, mövqe sürüşməsi, fırlanma bucağı və şəffaflıq sürüşdürücüsü.
- **Ağıllı Rəng Nümunəsi Alma**: Aktiv layihə palitranızdakı ən yaxın rəngə avtomatik bağlanma seçimi ilə eskiz qatından Pipet vasitəsilə rəng götürün.
- **Tək Kliklə Piksellərə Çevirmə (Bake)**: Miqyaslanmış istinad təsvirini bir kliklə birbaşa aktiv kadr üzərində təmiz piksel qrafikasına çevirin.

### 6. Dəqiq seçim dəsti və qaçan qarışqalar

- **Animasiyalı Qaçan Qarışqalar**: Aktiv seçimlər yüksək kontrastlı, iki tonlu hərəkətli kəsik xətli konturla göstərilir.
- **Bütöv Doldurmalı Lasso Seçimi**: Kəsişən ilgəklər çəkildikdə seçilməmiş dəliklər yaradan sadə şüa alqoritmlərindən fərqli olaraq, OniAnvil-in lasso aləti qapalı 8-əlaqəli sərhəd qurur və xarici sahəni dolduraraq hətta ilgəklərdə belə 100% bütöv və dəliksiz seçim yaradır.
- **Seçim Əməliyyatları**: Genişləndir, Daralt, Haşiyə (Dairə, Kvadrat və Romb nüvə formaları ilə), Yerini dəyiş, İnvert et, Sil, Sərbəst Fırlat və Sistem Mübadilə Buferinə Kopyala/Kəs/Yapışdır.
- **Kadrlararası Kopyalama və Yapışdırma**: Bir kadrdakı sahəni seçin, <kbd>Ctrl/Cmd+C</kbd> basın, kadrı dəyişin və istənilən klaviatura tərtibatında eyni nisbi koordinatlara yapışdırmaq üçün <kbd>Ctrl/Cmd+V</kbd> basın.

### 7. Canlı effektlər, filtrlər və RotSprite

Bütün təsvir effektləri dəyişiklikləri kətanda nümayiş etdirən və ləğv tarixçəsini pozmadan imtina imkanı verən real vaxtlı **Canlı Önbaxış** qutusunu ehtiva edir:
- **Rəng Əvəzləmə (`Shift+R`)**:
  - İnteraktiv Pipet: pipet simgəsinə klikləyin və birbaşa kətan piksellərindən və ya palitra panelindən rəngləri seçin.
  - Cəld seçim palitra zolağı: "Mənbə" rəngi seçmək üçün sol klik, "Hədəf" rəngi seçmək üçün sağ klik.
  - Tolerantlıq sürüşdürücüsü və bütün animasiya kadrlarında toplu əvəzləmə dəstəyi.
- **Rəng Tənzimləməsi (`Ctrl/Cmd+U`)**: Real vaxtda HSV Çaları (Hue), Doyma (Saturation), Parlaqlıq (Value) və Kontrast nizamlaması.
- **Avtomatik Kontur Generatoru**: Kvadrat və ya Romb künc birləşmələri ilə piksel dəqiqliyində personaj konturları (Xarici, Daxili, Mərkəzləşdirilmiş) yaradın.
- **3×3 Matris Filtrləri**: Şəffaflığı qoruma seçimi ilə Bulanıqlıq, İtiləmə, Kənar Aşkarlama və Qabartma filtrləri.
- **Sərbəst Fırlanma (`Ctrl/Cmd+R`)**: Klassik **RotSprite** və ya **Ən Yaxın Qonşu** alqoritmlərindən istifadə edərək seçilmiş sahələri və ya tam kadrları istənilən bucaqda fırladın.
- **Palitraya Yenidən Xəritələmə**: Bayer 2x2, 4x4, 8x8 və ya Blue Noise istifadə edərək kətan rənglərini aktiv palitraya dizerinq ilə uyğunlaşdırın.

### 8. Palitraların idarə edilməsi, rəng dəyişimi və personaj cildləri

- **Daxili Palitralar**: Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized və Sweetie 16 daxil olmaqla 20-dən çox orijinal retro palitra.
- **İdxal və İxrac**: GIMP Palitrası (`.gpl`), PAL (`.pal`), Hex palitrası (`.hex`) və qrafik rəng zolaqları (`.png`) dəstəyi.
- **Palitra Dəyişimi (Cildlər - Skins)**: Tək bir personaj üçün birdən çox palitra cildi variantı müəyyən edin (məsələn, Standart, Qızıl Zireh, Kölgə, Buz). Cildləri dəyişmək atlas üzrə bütün kadr rənglərini anında yenidən xəritələyir.
- **Rəng Şkalası Generatoru (Ramp)**: Tənzimlənən addım sayı ilə seçilmiş iki çalar arasında avtomatik olaraq hamar qradiyent keçidləri qurun.

### 9. Vərəq kəsici, 9-Grid UI və oyun mühərrikinə ixrac

- **Vərəq Kəsici (Sheet Slicer)**: Avtokorrelyasiyalı sərhəd aşkarlama alqoritmi xam sprayt vərəqlərini bərabər və ya dəqiq məhdudlaşdırılmış animasiya kadrlarına avtomatik bölür.
- **9-Grid (9-Slice) Miqyaslama**: Miqyaslanan interfeys qutularını və düymə panellərini küncləri təhrif etmədən görmək üçün fərdiləşdirilə bilən kənar boşluqları (yuxarı, aşağı, sol, sağ) təyin edin.
- **Genişləndirilmiş Haşiyəli (Extruded Border) PNG İxracı**: Kənar piksel rənglərini şəffaf kənar boşluqlarına doğru uzadaraq OpenGL, DirectX, Vulkan və 2D oyun mühərriklərində (məsələn, Godot) tekstura filtrləmə tikişlərini və rəng sızmasını aradan qaldırır.
- **Animasiyalı GIF İxracı**: Fərdi dövrə sayı, kadr gecikmələri və şəffaf fon dəstəyi ilə optimallaşdırılmış hərəkətli GIF faylları yaradın.
- **Standart Sprayt Vərəqi JSON**: Müasir 2D oyun mühərriklərində istifadə olunan standart sprayt vərəqi JSON sxeminə uyğun kadr kəsikləri, teqlər, müddətlər və dayaq nöqtəsi koordinatlarını ixrac edir.

### 10. 35 erqonomik tema və tam lokallaşdırma

- **Seçilmiş Mövzular**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Yaşıl CRT, Göy CRT, Retro Konsollar (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) və Retro Kompüterlər (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Geniş Dil Dəstəyi**: 100% lüğət açarı uyğunluğu ilə dillər arasında problemsiz keçid edin.

---

## İdarəetmə və klaviatura qısayolları

| Qısayol | Alət / Fəaliyyət | Təsvir |
|:---|:---|:---|
| <kbd>B</kbd> | **Qələm** | Pixel-Perfect rejim dəstəkli 1 px dəqiq rəsm |
| <kbd>E</kbd> | **Pozan** | Pikselləri şəffaflığa qədər silir |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipet** | Kətandan və ya istinad eskizindən rəng nümunəsi götürür |
| <kbd>G</kbd> | **Boya qabı / Qradiyent** | Boya qabı ilə Dizerinq Qradiyenti arasında keçid edir |
| <kbd>L</kbd> | **Xətt** | Canlı önbaxış və Shift bucaq bərkidilməsi ilə düz xətlər |
| <kbd>C</kbd> | **Əyri** | 3 klikli kvadratik Bezye əyri aləti |
| <kbd>U</kbd> | **Düzbucaqlı** | Həndəsi düzbucaqlı və kvadrat kontur/doldurma |
| <kbd>O</kbd> | **Elips** | Həndəsi elips və dairə kontur/doldurma |
| <kbd>D</kbd> | **Dizerinq fırçası** | Şahmat lövhəsi dizerinq matris naxışları ilə çəkir |
| <kbd>S</kbd> | **Kölgələndirmə aləti** | Palitra şkalası üzrə Açıqlaşdırma / Tündləşdirmə edir |
| <kbd>M</kbd> | **Düzbucaqlı seçim** | Qaçan qarışqalar ilə düzbucaqlı sahə seçimi |
| <kbd>Q</kbd> | **Lasso seçimi** | Sərbəst xətlərlə tam bütöv çoxbucaqlı seçimi |
| <kbd>W</kbd> | **Sehrli çubuq** | Eyni rəngli qonşu sahələri seçir |
| <kbd>P</kbd> | **Dayaq Nöqtələri** | Sprayt mərkəzini və fəaliyyət nöqtələrini yerləşdirir |
| <kbd>X</kbd> | **Rəngləri Dəyiş** | Əsas və Fon rənglərinin yerini dəyişir |
| <kbd>[</kbd> / <kbd>]</kbd> | **Fırça Ölçüsü** | Fırça ölçüsünü azaldır / artırır (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopyala** | Seçilmiş pikselləri sistem mübadilə buferinə kopyalayır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Kəs** | Seçilmiş pikselləri buferə kəsir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Yapışdır** | Buferdəki pikselləri aktiv kadra yapışdırır |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Sil** | Aktiv seçim daxilindəki pikselləri təmizləyir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Hamısını Seç** | Aktiv kadrdakı bütün pikselləri seçir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Seçimi Ləğv Et** | Aktiv seçim maskasını təmizləyir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Seçimi İnvert Et** | Aktiv seçim maskasını tərs çevirir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Kadrdan Dublikat Çıxar** | Zaman xəttindəki aktiv kadrı nüsxələyir |
| <kbd>Boşluq</kbd> | **Oynat / Fasilə** | Zaman xəttində animasiya oxutmanı başladır/dayandırır |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Rəng Əvəzləmə** | Canlı önbaxışlı Rəng Əvəzləmə pəncərəsini açır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Rəng Tənzimləməsi** | HSV Çaları / Doyma / Kontrast pəncərəsini açır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Sərbəst Fırlanma** | İstənilən bucaqda fırlanma pəncərəsini (RotSprite) açır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Geri Al** | Son kətan ştrixini və ya alət fəaliyyətini geri alır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Təkrar Et** | Geri alınmış fəaliyyəti təkrarlayır |
| <kbd>Boşluq</kbd> + Sürükləmə / Orta Klik | **Kətanı Sürüşdür** | Kətan koordinatları üzrə hamar naviqasiya təmin edir |
| <kbd>Ctrl/Cmd</kbd> + Təkər / <kbd>+</kbd> / <kbd>-</kbd> | **Miqyas** | Kursor mərkəzli piksel dəqiqliyində böyütmə/kiçiltmə |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Görünüş Rejimi** | Kadr Baxışı (<kbd>1</kbd>) ilə Atlas Baxışı (<kbd>2</kbd>) arasında keçid |
| <kbd>F</kbd> | **Kətanı Uyğunlaşdır** | Aktiv kadrı iş sahəsinin mərkəzinə yerləşdirir |
| <kbd>F1</kbd> | **Yardım** | Klaviatura Qısayolları pəncərəsini açır |

---

## Başlanğıc

### Hazır binar fayllar

Əvvəlcədən yığılmış müstəqil icra olunan faylları **[GitHub Buraxılışlar Səhifəsindən](https://github.com/rbm13s/OniAnvil/releases/latest)** endirin:

- **Windows**: `OniAnvil-windows-x86_64.zip` arxivini endirin, çıxarın və `OniAnvil.exe` faylına iki dəfə klikləyin.
- **Linux**: `OniAnvil-linux-x86_64.tar.gz` portativ arxivini endirin və ya Debian/Ubuntu üçün `onianvil_*_amd64.deb` paketini quraşdırın:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: `OniAnvil-macos-universal.zip` arxivini endirin, çıxarın və `OniAnvil.app` faylını Proqramlar (Applications) qovluğuna köçürün. Universal binar paket həm Apple Silicon (M1/M2/M3/M4), həm də Intel Mac kompüterlərində yerli olaraq icra olunur.
- **Web**: Quraşdırma aparmadan birbaşa brauzerinizdə işə salın: **[WebAssembly Tətbiqini Aç](https://rbm13s.github.io/OniAnvil/app/?lang=az)**.
  > [!WARNING]
  > WebAssembly (brauzer) buraxılışı **Eksperimental (Experimental)** statusa malikdir. Brauzerin təhlükəsizlik mühiti məhdudiyyətləri, WebGL/WebGPU fərqləri və sistem mübadilə buferi məhdudiyyətləri səbəbindən yerli masaüstü versiyası ilə müqayisədə kiçik vizual qüsurlar və ya performans enişləri ilə qarşılaşa bilərsiniz. İstehsalat işləri üçün yerli masaüstü versiyalar qətiyyətlə tövsiyə olunur.

---

### Mənbə kodundan yığma

#### Tələblər
- **Rust Alətlər Dəsti**: [rustup.rs](https://rustup.rs/) vasitəsilə Rust-ı quraşdırın (1.80+ tövsiyə olunur):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux Yığma Asılılıqları** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Yerli Masaüstü Versiyasını Yığma və İcra Etmə
```bash
# Repozitoriyanı klonlaşdırın
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Testləri işə salın
cargo test

# Optimallaşdırılmış buraxılış faylını yığın və işə salın
cargo run --release
```

#### 2. WebAssembly Versiyasını Yığma (Trunk ilə)
```bash
# wasm32 hədəf platformasını və Trunk yığıcısını quraşdırın
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Avtomatik yenilənmə ilə yerli inkişaf serverini işə salın
trunk serve

# Hazır paylama fayllarını docs/app/ qovluğuna yığın
./build_wasm.sh        # Linux/macOS mühitində
./build_wasm.ps1       # Windows PowerShell mühitində
```

---

## Fayl formatı və müşayiətçi arxitektura

OniAnvil qapalı və ya mülkiyyətçi binar formatlardan qaçır. Hər bir layihə versiya nəzarəti sistemləri ilə tam uyğun olan iki təmiz fayldan ibarətdir:

```
my_character/
├── my_character.png            # Standart PNG sprayt vərəqi atlası (bütün mühərriklərdə və baxış proqramlarında açıla bilir)
└── my_character.onianvil.json   # Müşayiətçi meta-məlumat faylı
```

`.onianvil.json` müşayiətçi faylı aşağıdakıları saxlayır:
- Kətan toru konfiqurasiyası: kadr ölçüləri, sütun sayı, ümumi kadr sayı.
- Qat iyerarxiyaları: zolaq adları, şəffaflıq dərəcələri, görünüş bayraqları, kilid vəziyyətləri.
- Animasiya teqləri: ad, başlanğıc/son kadr indeksləri, oxutma istiqaməti (İrəli, Geri, Pinq-ponq), kadr müddətləri.
- Palitra tərifləri: aktiv rənglər, adlandırılmış cild variantları, aktiv cild indeksi.
- İstinad eskizinin vəziyyəti: təsvirin fayl yolu, miqyas, sürüşmə mövqeyi, fırlanma bucağı, şəffaflıq, qat rejimi.
- Dayaq nöqtələri: fərdi fəaliyyət nöqtələri, silah bağlama nöqtələri və toqquşma mərkəzləri.

---

## Töhfə vermə və təşəkkürlər

Töhfələriniz, xəta bildirişləriniz və təklifləriniz böyük məmnuniyyətlə qarşılanır!
1. GitHub-da repozitoriyanı fork edin.
2. Yeni bir funksional şaxə yaradın: `git checkout -b feature/yeni-alet`.
3. Dəyişikliklərinizi qeyd edin: `git commit -am 'Yeni piksel aləti əlavə edildi'`.
4. Şaxənizi uzaq serverə göndərin: `git push origin feature/yeni-alet`.
5. Pull Request (Birləşdirmə İstəyi) açın.

### Təşəkkürlər
- İlkin nümayiş animasiya spraytı **"Cat 2D Pixel Art"**, müəllif **xzany** tərəfindən hazırlanmışdır ([itch.io layihə səhifəsi](https://xzany.itch.io/cat-2d-pixel-art)).
- Möhtəşəm **[egui](https://github.com/emilk/egui)** qrafik interfeys kitabxanası və **[wgpu](https://wgpu.rs/)** qrafika ekosistemi üzərində qurulmuşdur.
- İstifadəçi interfeysi nişanları @fontawesome tərəfindən təqdim edilən **[Font Awesome Free](https://fontawesome.com)** paketi ilə təmin edilmişdir (Nişanlar: CC BY 4.0, Şriftlər: SIL OFL 1.1, Kod: MIT Lisenziyası).

---

## Lisenziya və zəmanətdən imtina

Bu layihə **[MIT Lisenziyası](LICENSE)** altında lisenziyalaşdırılmışdır. Siz OniAnvil proqramını kommersiya və şəxsi oyun hazırlama proseslərinizdə sərbəst şəkildə istifadə edə, dəyişdirə, yaya və layihələrinizə inteqrasiya edə bilərsiniz.

### Zəmanətdən İmtina və Məsuliyyətin Məhdudlaşdırılması
> [!IMPORTANT]
> **OniAnvil** proqramı açıq və ya gizli heç bir zəmanət yaxud şərt olmadan, **ödənişsiz olaraq**, **"OLDUĞU KİMİ" ("AS IS")** və **"MÖVCUD OLDUĞU KİMİ" ("AS AVAILABLE")** prinsipi ilə yayılır.
> 
> Müəllif hüququ sahibləri, müəlliflər və ya töhfə verənlər; bu proqram təminatının istifadəsindən və ya istifadə edilə bilməməsindən irəli gələn hər hansı bir birbaşa, dolayı, təsadüfi, xüsusi, cəza və ya nəticə etibarilə yaranan zərərlərə görə (məlumatların itirilməsi, layihə fayllarının zədələnməsi, aparat nasazlığı, qazanc itkisi və ya işin dayanması daxil olmaqla, lakin bunlarla məhdudlaşmayaraq) heç bir halda məsuliyyət daşımır. Proqram təminatının istifadəsi ilə bağlı bütün risk və məsuliyyət istifadəçinin üzərinə düşür.

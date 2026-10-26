<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logosu" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Oyuna Hazır Atlaslar Etrafında İnşa Edilmiş Piksel Sanatı ve Soğan Zarı (Onion-Skin) Animasyon Stüdyosu**

[![Derleme ve Yayınlama](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![İndirmeler](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![İşleyici](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Diller](https://img.shields.io/badge/Languages-37_Dil-purple)](#10-35-ergonomik-tema-ve-yerellestirme)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#baslarken)
[![Çevrimiçi Uygulama](https://img.shields.io/badge/Web_Uygulaması-Tarayıcıda_Dene_(Deneysel)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=tr)
[![Lisans: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 OniAnvil'i Tarayıcıda Deneyin (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=tr)** *(⚠️ Deneysel sürüm: WebGL/WASM kısıtlamaları ve hatalar içerebilir)* • **[🌐 Resmi Dokümantasyon Web Sitesi](https://rbm13s.github.io/OniAnvil/tr.html)**

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
[Română](README-ro.md) •
**Türkçe** •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## İçindekiler

- [Giriş](#giris)
- [Ekran Görüntüleri ve Görsel Tur](#ekran-goruntuleri-ve-gorsel-tur)
- [Kapsamlı Araçlar Kılavuzu](#kapsamli-araclar-kilavuzu)
- [İşlevsel Dokümantasyon](#islevsel-dokumantasyon)
  - [1. Çift Atlas Merkezli Mimari](#1-cift-atlas-merkezli-mimari)
  - [2. Çok Kanallı Katman Zaman Çizelgesi](#2-cok-kanalli-katman-zaman-cizelgesi)
  - [3. Gerçek Zamanlı Canlı Animasyon Önizleme](#3-gercek-zamanli-canli-animasyon-onizleme)
  - [4. Gelişmiş Onion Skinning (Soğan Zarı) Motoru](#4-gelismis-onion-skinning-sogan-zari-motoru)
  - [5. Akıllı Referans ve Eskiz Katmanı](#5-akilli-referans-ve-eskiz-katmani)
  - [6. Hassas Seçim Araçları ve Kayan Karıncalar](#6-hassas-secim-araclari-ve-kayan-karincalar)
  - [7. Canlı Efektler, Filtreler ve RotSprite](#7-canli-efektler-filtreler-ve-rotsprite)
  - [8. Palet Yönetimi, Değişimi ve Karakter Görünümleri](#8-palet-yonetimi-degisimi-ve-karakter-gorunumleri)
  - [9. Sayfa Dilimleyici, 9-Grid UI ve Oyun Motoruna Dışa Aktarım](#9-sayfa-dilimleyici-9-grid-ui-ve-oyun-motoruna-disa-aktarim)
  - [10. 35 Ergonomik Tema ve Yerelleştirme](#10-35-ergonomik-tema-ve-yerellestirme)
- [Kontroller ve Klavye Kısayolları](#kontroller-ve-klavye-kisayollari)
- [Başlarken](#baslarken)
  - [Hazır İkili Dosyalar](#hazir-ikili-dosyalar)
  - [Kaynak Koddan Derleme](#kaynak-koddan-derleme)
- [Dosya Formatı ve Eşlikçi Mimarisi](#dosya-formati-ve-eslikci-mimarisi)
- [Katkıda Bulunma ve Teşekkürler](#katkida-bulunma-ve-tesekkurler)
- [Lisans ve Garanti Reddi](#lisans-ve-garanti-reddi)

---

## Giriş

**OniAnvil** (*adını **Oni**on-Skinning (soğan zarı animasyonu) + **Anvil** (sprite dövme örsü) sözcüklerinden alır*), Rust dilinde geliştirilmiş açık kaynaklı, yüksek performanslı bir piksel sanatı düzenleyicisi ve animasyon stüdyosudur. Windows, Linux ve macOS üzerinde donanım hızlandırmalı olarak yerel hızda çalışır; ayrıca modern web tarayıcılarında kurulum gerektirmeyen sıfır gecikmeli bir WebAssembly uygulaması olarak doğrudan kullanılabilir.

Mevcut sprite düzenleyicilerin büyük kısmı animasyonları birbirinden bağımsız görüntü dosyaları yığını olarak ele alır ve geliştiricileri dışa aktarma, dilimleme ve yeniden paketleme gibi zahmetli adımlara zorlar. **OniAnvil bu paradigmayı tamamen tersine çevirir:** oyunun sprite sayfası atlas dokusu (`atlas_rgba`) temel hakikat kaynağıdır. Kare hücreleri, katman kanalları, animasyon etiketleri ve çarpışma dayanak noktaları (pivot) her an doğrudan atlas ile senkronize tutulur.

### Neden OniAnvil?
- 🚀 **Yıldırım Hızında Yerel Rust Performansı**: Milisaniyenin altında açılış süresi, `wgpu` ve `egui 0.31` destekli keskin çizim performansı ve uzun zaman çizelgelerinde gezinirken sıfır gecikme.
- 🎬 **Çok Kanallı Katman Zaman Çizelgesi**: NLE video düzenleyicilerinden ilham alan katman kanalları (`V1, V2, V3...`), yukarıdan aşağıya görsel harmanlama, katman birleştirme, bağlı kareler ve değişken bekleme süreleri.
- 🧅 **Birinci Sınıf Soğan Zarı (Onion Skinning)**: Önceki ve sonraki kareler için bağımsız renklendirme (varsayılan: Kırmızı/Mavi), özelleştirilebilir opaklık eğrileri ve döngü bağlama.
- 🎨 **Anında Karakter Palet Görünümleri (Skins)**: Canlı atlas eşlemesi ile gece/gündüz varyantları, takım renkleri veya element temalı görünümler oluşturun; GPL, PAL ve HEX formatlarını içe/dışa aktarın.
- 📦 **Doğrudan Oyun Motoruna Dışa Aktarım**: 2D oyun motorlarındaki (Godot, Bevy, Raylib, Defold) doku sızıntısını ve dikiş izlerini ortadan kaldıran taşmalı (extruded) PNG sınırları, standart sprite sayfası JSON üstverileri ve optimize edilmiş animasyonlu GIF dışa aktarımı.

---

## Ekran Görüntüleri ve Görsel Tur

<div align="center">

### 1. Ana Tuval ve Stüdyo Çalışma Alanı
*Donanım hızlandırmalı görünüm penceresi, çok kanallı zaman çizelgesi, gerçek zamanlı canlı önizleme, palet renkleri ve hassas araç çubuklarını içeren yüksek performanslı piksel çalışma alanı.*
<img src="screenshots/main.png" alt="OniAnvil Ana Çalışma Alanı" width="950" />

<br/><br/>

### 2. Çok Kanallı Animasyon Zaman Çizelgesi
*Katman satırları (`V1, V2...`), bağlı kare örnekleri (`🔗`), bekleme çarpanları ve renk kodlu animasyon etiketlerine sahip doğrusal olmayan çok kanallı zaman çizelgesi.*
<img src="screenshots/timeline.png" alt="OniAnvil Çok Kanallı Zaman Çizelgesi" width="950" />

<br/><br/>

### 3. Hassas Araçlar ve Eylem Çubuğu
*Standart ve kompakt kenetlenme modlarını destekleyen 15 araçlık çizim paketi ve fırça boyutu, simetri, dolaşım ve piksel kusursuz (pixel-perfect) seçeneklerini sunan üst hızlı eylem çubuğu.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Hızlı Eylem Araç Çubuğu" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Araçlar Standart" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Araçlar Kompakt" width="300" />
</p>

<br/><br/>

### 4. Gerçek Zamanlı Canlı Animasyon Önizleme
*Etiket filtreleme, değişken FPS hız kontrolleri, yakınlaştırma kaydırıcısı ve arka plan modları (dama tahtası, koyu düz renk, açık düz renk) içeren özel canlı oynatma paneli.*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Canlı Önizleme" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Canlı Önizleme Yakınlaştırma" width="460" />
</p>

<br/><br/>

### 5. Palet Yönetimi, Değişimi ve Renk Skalaları
*Ön Plan/Arka Plan renk kutuları, iki nokta arası otomatik renk skalası (ramp) üretimi ve adlandırılmış palet görünümü (skin) değişimlerini içeren kapsamlı renk motoru.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palet ve Değişimler" width="950" />

<br/><br/>

### 6. Referans Eskiz Çizimi ve Yapılandırılabilir Onion Skinning
*Konsept eskizlerini veya fotoğrafları alt/üst katman olarak yükleme, dönüştürme kontrolleri, palete hizalama özellikli akıllı damlalık ve çok kareli soğan zarı gölgelemesi.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Eskiz ve Soğan Zarı" width="950" />

<br/><br/>

### 7. Dayanak Noktaları (Pivot) ve Çarpışma Eylem Noktaları
*Etkileşimli tuval kılavuzlarıyla sprite merkezini ve özel adlandırılmış oynanış noktalarını (silah yuvaları, mermi çıkış noktaları, kafa, ayak sesleri) tanımlama.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Dayanak ve Eylem Noktaları" width="950" />

<br/><br/>

### 8. Sprite Sayfası Dilimleyici ve 9-Grid (9-Slice) Ölçekleyici
*Otomatik sınır tanıma korelasyonu ile sayfa dilimleme (örn. yürüyen kedi döngüsü) ve ölçeklenebilir 9-dilimli arayüz kutusu kenar boşluğu yapılandırması.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Sayfa Dilimleyici" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Dilim Ölçekleyici" width="460" />
</p>

<br/><br/>

### 9. Gerçek Zamanlı Görüntü Efektleri ve Filtre Pencereleri (Canlı Önizleme)
*Canlı tuval önizlemesi, etkileşimli damlalık ve geri alma geçmişini kirletmeden iptal olanağı sunan efekt diyalogları.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Dış Çizgi Oluşturucu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Renk Değiştirme" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Renk Tonu Doygunluk Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matris Filtresi" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Palete Yeniden Eşle" width="460" />
</p>

<br/><br/>

### 10. Tuval Izgarası ve Arka Plan Modları
*Piksel ızgarası, döşeme sınır ızgarası, koyu dama tahtası, açık dama tahtası ve özel düz renkli arka planlar arasında kesintisiz geçiş yapın.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Izgara ve Koyu Dama Tahtası" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Açık Dama Tahtası" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Özel Düz Renk Arka Plan" width="300" />
</p>

<br/><br/>

### 11. Yeni Proje Oluşturma Sihirbazı
*Proje başlatılırken tuval boyutlarını, kare sayısını, sütun ızgara düzenini yapılandırın ve 20'den fazla otantik retro palet arasından seçim yapın.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Yeni Proje Sihirbazı" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Yeni Proje Paletleri" width="460" />
</p>

<br/><br/>

### 12. 35 Özgün Retro ve Modern Tema
*Studio Dark, Studio Light, CRT Amber/Yeşil/Camgöbeği fosforları, retro konsollar (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) ve özel tasarımcı temaları.*
<p align="center">
  <img src="screenshots/themes.png" alt="Temalar Menüsü" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Amber Teması" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX Teması" width="300" />
</p>

<br/><br/>

### 13. Menü Sistemi ve Dahili Klavye Yardımı
<p align="center">
  <img src="screenshots/file_menu.png" alt="Dosya Menüsü" width="180" />
  <img src="screenshots/edit_menu.png" alt="Düzenle Menüsü" width="180" />
  <img src="screenshots/select_menu.png" alt="Seçim Menüsü" width="180" />
  <img src="screenshots/view_menu.png" alt="Görünüm Menüsü" width="180" />
  <img src="screenshots/project_menu.png" alt="Proje Menüsü" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Klavye Kısayolları Yardım Penceresi" width="700" />
</p>

</div>

---

## Kapsamlı Araçlar Kılavuzu

OniAnvil; hassas çizim, şekil, seçim ve üstveri yerleştirme için 15 profesyonel araç içerir. İkonlar açık kaynaklı **Font Awesome Free** ikon paketi ile oluşturulmuştur *(İkonlar: CC BY 4.0, Yazı Tipleri: SIL OFL 1.1)*.

| Araç | İkon | Kısayol | Fırça Boyutu | Piksel Kusursuz | Açıklama |
|:---|:---:|:---:|:---:|:---:|:---|
| **Kurşun Kalem** | ✏️ | <kbd>B</kbd> | 1..32 px | Evet | Temel çizim aracı. 1 px modunda **Piksel Kusursuz (Pixel-Perfect)** çizimi destekler (çirkin L şeklindeki merdiven köşelerini otomatik olarak temizler). Seçimden özel damga fırçası oluşturmayı destekler (`Ctrl/Cmd+B`). |
| **Silgi** | 🧹 | <kbd>E</kbd> | 1..32 px | Hayır | Pikselleri şeffaflığa dönüştürerek siler (`[0, 0, 0, 0]`). Aktif seçim varsa yalnızca seçim alanı içinde çalışır. |
| **Damlalık** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Hayır | Sol tık rengi Birincil (Ön Plan) renge; Sağ tık İkincil (Arka Plan) renge alır. Tuval piksellerinden, katmanlardan veya referans eskizden renk alabilir. |
| **Boya Kovası** | 🪣 | <kbd>G</kbd> | Yok | Hayır | Ayarlanabilir Renk Toleransı (`0..=255`) ile yüksek hızlı kuyruk tabanlı BFS doldurma. Bitişik ve Genel doldurma ile seçim sınırlandırmasını destekler. |
| **Gradyan** | 🌈 | <kbd>G</kbd> *(geçiş)* | Yok | Hayır | Ön Plan ve Arka Plan renkleri arasında doğrusal veya dairesel gradyan oluşturur. Bayer (2x2, 4x4, 8x8) ve Blue Noise noktalama (dithering) desteği sunar. |
| **Çizgi** | 📏 | <kbd>L</kbd> | 1..32 px | Evet | Canlı sürükleme önizlemeli Bresenham çizgi algoritması. Açıları 45°/90° adımlarına sabitlemek için <kbd>Shift</kbd> tuşunu basılı tutun. |
| **Eğri** | 〰️ | <kbd>C</kbd> | 1..32 px | Evet | 3 tıklamalı ikinci dereceden Bézier eğrisi. 1. tık Başlangıcı, 2. tık Bitişi belirler, fare hareketi kavisi ayarlar, 3. tık eğriyi işler. |
| **Dikdörtgen** | ⬜ | <kbd>U</kbd> | 1..32 px | Hayır | Konturlu veya Dolu geometrik dikdörtgenler. 1:1 kare elde etmek için <kbd>Shift</kbd> tuşunu basılı tutun. |
| **Elips** | ⭕ | <kbd>O</kbd> | 1..32 px | Hayır | Orta nokta rasterizasyonu ile konturlu veya dolu elipsler. Kusursuz bir çember çizmek için <kbd>Shift</kbd> tuşunu basılı tutun. |
| **Haleleme Fırçası** | 🏁 | <kbd>D</kbd> | 1..32 px | Hayır | Palete fazladan renk eklemeden Ön Plan ve Arka Plan renkleri arasında dama tahtası dither desenleri boyar. |
| **Gölgelendirme Aracı** | 🌓 | <kbd>S</kbd> | 1..32 px | Hayır | Aktif palet renk sırasına göre pikselleri kademeli olarak Açıklaştırma (Dodge) ve Koyulaştırma (Burn) işlemi yapar. |
| **Dikdörtgen Seçim** | ⬚ | <kbd>M</kbd> | Yok | Hayır | Kayan karınca kenarlıklı dikdörtgen alan seçimi. Ekleme (<kbd>Shift</kbd>) ve Çıkarma (<kbd>Alt</kbd>) işlemlerini destekler. |
| **Kement Seçimi** | ➰ | <kbd>Q</kbd> | Yok | Hayır | Serbest çokgen kement. 8-bağlantılı sınır rasterizasyonu ve 4-bağlantılı dış doldurma kullanarak kesişen halkalarda bile %100 deliksiz ve dolu seçim sağlar. |
| **Sihirli Değnek** | 🪄 | <kbd>W</kbd> | Yok | Hayır | Ayarlanabilir tolerans eşiği ile bitişik (veya tuval genelindeki eşleşen) renk bölgelerini seçer. |
| **Dayanak ve Eylem** | 🎯 | <kbd>P</kbd> | Yok | Hayır | JSON üstverisine aktarılmak üzere Sprite Başlangıç Noktası (Pivot) ve özel oynanış Eylem Noktalarının (`weapon`, `head`, `feet`, `hitbox`) tuvale etkileşimli yerleşimi. |

---

## İşlevsel Dokümantasyon

### 1. Çift Atlas Merkezli Mimari

OniAnvil'de ana doku atlası (`atlas_rgba`), oyun motorunuz tarafından yüklenen gerçek sprite sayfası dokusunu temsil eder:
- **Kare Modu**: Yalnızca tek bir aktif animasyon karesine odaklanarak dikkat dağıtmayan bir piksel tuvali sağlar. Yapılan tüm çizimler anında atlas dokusundaki ilgili hücreye aktarılır.
- **Atlas Modu**: Tüm sprite sayfası ızgarasını görüntüler; makro ayarlamalara, kareler arası hizalama kontrollerine ve toplu düzenlemelere olanak tanır.
- **Eşlikçi Dosya**: Projeler, temiz bir `.png` sprite sayfası ve kare düzenlerini, katman hiyerarşilerini, etiketleri, bekleme çarpanlarını ve palet görünümlerini içeren hafif bir `.onianvil.json` eşlikçi dosyası olarak kaydedilir.

### 2. Çok Kanallı Katman Zaman Çizelgesi

Alt zaman çizelgesi profesyonel doğrusal olmayan video düzenleme yazılımları model alınarak tasarlanmıştır:
- **Katman Kanalları (`V1, V2, V3...`)**: Katmanlar yukarıdan aşağıya görsel olarak istiflenir. Zaman çizelgesinde daha üstte yer alan kanal (`V1`), kare tuvalinde en üstte oluşturulur.
- **Kanal Kontrolleri**:
  - Aktif çizim katmanını hızlıca değiştirmek için `V1` / `V2` etiketlerine tıklayın.
  - Katman görünürlüğünü (Göz ikonu) ve katman düzenleme kilidini (Kilit ikonu) açıp kapatın.
  - Bağımsız katman opaklığını ayarlayın (`%0..=%100`).
  - Katmanı hiyerarşide yukarı/aşağı taşıyın, çoğaltın veya altındaki kanalla birleştirin (Aşağı Birleştir).
  - Katmanı onay penceresiyle güvenli bir şekilde silin.
- **Kare Klipleri**:
  - Bekleme çarpanı (Hold): özel kare süreleri atayın (örn. vuruş anları veya duraklamalar için `2×`, `3×` bekleme).
  - Bağlı kareler: tekrarlayan kareleri animasyon boyunca bağlayın; böylece birini düzenlemek bağlı tüm örnekleri otomatik olarak günceller.
  - Etiket döngüsü: animasyonları adlandırılmış etiketler altında düzenleyin (`idle`, `walk`, `attack`) ve oynatma sırasında belirli etiketleri döngüye alın.
  - Oynatma kontrolleri: Ayarlanabilir FPS (`1..=120`) ile İleri, Geri, Pinpon ve Döngü modunda oynatma.

### 3. Gerçek Zamanlı Canlı Animasyon Önizleme

Sağ taraftaki Önizleme paneli, tuvalinizin yakınlaştırma veya kaydırma konumundan bağımsız olarak animasyonlarınızı gerçek zamanlı olarak oynatır:
- **Etiket Filtreleme**: Oynatmayı belirli etiketlerle sınırlandırın (`walk`, `jump`) veya tüm animasyon dizisini önizleyin.
- **Oynatma Kontrolleri**: Oynat, Duraklat, Önceki Adım, Sonraki Adım, Döngü geçişi ve özel oynatma hızları (`0.25x..=4.0x`).
- **Görüntü Modları**: Koyu dama tahtası, açık dama tahtası, saf beyaz, saf siyah veya özel tuval arka planları arasında geçiş yapın.

### 4. Gelişmiş Onion Skinning (Soğan Zarı) Motoru

Özel soğan zarı sistemi ile akıcı animasyonlar oluşturun:
- **Özel Renklendirme**: Önceki kareler (varsayılan: Kırmızı) ve sonraki kareler (varsayılan: Mavi) için bağımsız renk tonları.
- **Opaklık Zayıflaması**: Ayarlanabilir temel opaklık ve adım adım opaklık zayıflama eğrileri.
- **Döngü Bağlama**: Döngülü animasyonlarda ilk ve son kareler arasında soğan zarı hayaletini kesintisiz görüntüleyin.
- **Hızlı Bakış (Quick-Peek)**: Fareyi sürüklemeden <kbd>Boşluk</kbd> tuşunu basılı tutarak komşu kareleri anında inceleyin.

### 5. Akıllı Referans ve Eskiz Katmanı

El çizimi eskizleri veya referans fotoğrafları şablon olarak kullanarak iş akışınızı hızlandırın:
- **Formatlar**: PNG, JPEG ve WebP referans görüntülerini yükleyin.
- **Harmanlama Modları**: Alt Katman (piksel katmanlarınızın altında) veya Üst Katman (piksel katmanlarınızın üstünde).
- **Dönüştürme Kontrolleri**: Bağımsız yakınlaştırma, konum kaydırma, döndürme ve opaklık kaydırıcısı.
- **Akıllı Renk Alma**: Aktif proje paletinizdeki en yakın renge otomatik hizalama seçeneğiyle eskiz katmanından Damlalık ile renk örnekleyin.
- **Tek Tıkla Piksellere Dönüştürme (Bake)**: Ölçeklendirilmiş referans görüntüyü tek bir tıklamayla aktif kare üzerinde doğrudan piksel sanatına dönüştürün.

### 6. Hassas Seçim Araçları ve Kayan Karıncalar

- **Animasyonlu Kayan Karıncalar**: Aktif seçimler, yüksek görünürlüklü iki tonlu hareketli kesik çizgili çevre ile gösterilir.
- **Dolu Doldurmalı Kement Seçimi**: Kesişen döngüler çizildiğinde seçilmemiş delikler bırakan ilkel ışın izleme algoritmalarının aksine, OniAnvil'in kement aracı kapalı bir 8-bağlantılı sınır rasterize eder ve dış kısmı doldurarak döngülerde bile %100 dolu ve eksiksiz seçim sağlar.
- **Seçim İşlemleri**: Genişlet, Daralt, Kenarlık (Daire, Kare ve Baklava çekirdek şekilleriyle), Taşı, Tersine Çevir, Sil, Serbest Döndür ve Sistem Panosuna Kopyala/Kes/Yapıştır.
- **Kareler Arası Kopyala ve Yapıştır**: Bir karedeki bölgeyi seçin, <kbd>Ctrl/Cmd+C</kbd> tuşuna basın, kareyi değiştirin ve herhangi bir klavye düzeninde aynı göreli koordinatlara yapıştırmak için <kbd>Ctrl/Cmd+V</kbd> tuşuna basın.

### 7. Canlı Efektler, Filtreler ve RotSprite

Tüm görüntü efektleri, tuval üzerindeki değişiklikleri önizleyen ve geri alma geçmişini kirletmeden sorunsuz iptal imkanı sağlayan gerçek zamanlı bir **Canlı Önizleme** onay kutusu içerir:
- **Renk Değiştirme (`Shift+R`)**:
  - Etkileşimli Damlalık: damlalık simgesine tıklayın ve doğrudan kare piksellerinden veya palet panelinden renkleri seçin.
  - Hızlı seçim palet şeridi: "Kaynak" rengi seçmek için sol tıklayın, "Hedef" rengi seçmek için sağ tıklayın.
  - Tolerans kaydırıcısı ve tüm animasyon karelerinde toplu değiştirme desteği.
- **Renk Ayarı (`Ctrl/Cmd+U`)**: Gerçek zamanlı HSV Renk Tonu kaydırma, Doygunluk, Değer (parlaklık) ve Kontrast ayarı.
- **Otomatik Dış Çizgi Oluşturucu**: Kare veya Baklava köşe birleşimleriyle piksel düzeyinde keskin karakter dış çizgileri (Dış, İç, Ortalanmış) oluşturun.
- **3×3 Konvolüsyon Filtreleri**: Alfayı koruma seçeneğiyle Bulanıklaştırma, Keskinleştirme, Kenar Algılama ve Kabartma filtreleri.
- **Serbest Döndürme (`Ctrl/Cmd+R`)**: Klasik **RotSprite** veya **En Yakın Komşu** algoritmalarını kullanarak seçili bölgeleri veya tüm kareleri istenen açıda döndürün.
- **Palete Yeniden Eşle**: Bayer 2x2, 4x4, 8x8 veya Blue Noise kullanarak tuval renklerini aktif palete dither ile indirgeyin.

### 8. Palet Yönetimi, Değişimi ve Karakter Görünümleri

- **Dahili Paletler**: Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized ve Sweetie 16 dahil 20'den fazla otantik retro palet.
- **İçe ve Dışa Aktarım**: GIMP Paleti (`.gpl`), PAL (`.pal`), Hex paleti (`.hex`) ve resim renk şeritleri (`.png`) desteği.
- **Palet Değişimi (Görünümler - Skins)**: Tek bir karakter için birden fazla palet görünümü varyantı tanımlayın (örn. Varsayılan, Altın Zırh, Gölge, Buzul). Görünümleri değiştirmek atlas genelindeki tüm kare renklerini anında yeniden eşler.
- **Renk Skalası Oluşturucu (Ramp)**: Ayarlanabilir ara adım sayısıyla seçilen iki ton arasında otomatik olarak pürüzsüz renk geçişleri oluşturun.

### 9. Sayfa Dilimleyici, 9-Grid UI ve Oyun Motoruna Dışa Aktarım

- **Sayfa Dilimleyici**: Otomatik korelasyonlu sınır algılama algoritması ham sprite sayfalarını eşit veya sıkı sınırlı animasyon karelerine otomatik olarak böler.
- **9-Grid (9-Slice) Ölçeklendirme**: Ölçeklenebilir arayüz kutularını ve buton panellerini köşeleri bozmadan önizlemek için özelleştirilebilir dilim kenar boşlukları (üst, alt, sol, sağ) ayarlayın.
- **Taşmalı Kenarlık (Extruded Border) PNG Dışa Aktarımı**: Kenar piksel renklerini şeffaf kenar boşluklarına doğru taşırarak OpenGL, DirectX, Vulkan ve 2D oyun motorlarındaki (örneğin Godot) doku filtreleme dikişlerini ve renk sızıntısını engeller.
- **Animasyonlu GIF Dışa Aktarımı**: Özel döngü sayıları, kare gecikmeleri ve şeffaf arka plan desteğiyle optimize edilmiş hareketli GIF'ler üretin.
- **Standart Sprite Sayfası JSON**: Modern 2D oyun motorlarında kullanılan standart sprite sayfası JSON şemasına uygun kare dilimleri, etiketler, süreler ve dayanak noktası koordinatlarını dışa aktarır.

### 10. 35 Ergonomik Tema ve Yerelleştirme

- **Seçkin Temalar**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Yeşil CRT, Camgöbeği CRT, Retro Konsollar (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) ve Retro Bilgisayarlar (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Geniş Dil Desteği**: %100 sözlük anahtarı uyumuyla diller arasında kesintisiz geçiş yapın.

---

## Kontroller ve Klavye Kısayolları

| Kısayol | Araç / Eylem | Açıklama |
|:---|:---|:---|
| <kbd>B</kbd> | **Kurşun Kalem** | Piksel Kusursuz mod destekli 1 px hassas çizim |
| <kbd>E</kbd> | **Silgi** | Pikselleri şeffaflığa kadar siler |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Damlalık** | Tuvalden veya referans eskizden renk örneği alır |
| <kbd>G</kbd> | **Boya Kovası / Gradyan** | Boya Kovası ile Dither Gradyanı arasında geçiş yapar |
| <kbd>L</kbd> | **Çizgi** | Canlı önizleme ve Shift açılı düz çizgiler çizer |
| <kbd>C</kbd> | **Eğri** | 3 tıklamalı ikinci dereceden Bézier eğri aracı |
| <kbd>U</kbd> | **Dikdörtgen** | Geometrik dikdörtgen ve kare kontur/doldurma |
| <kbd>O</kbd> | **Elips** | Geometrik elips ve çember kontur/doldurma |
| <kbd>D</kbd> | **Haleleme Fırçası** | Dama tahtası dither matris desenleriyle boyar |
| <kbd>S</kbd> | **Gölgelendirme Aracı** | Palet skalası boyunca Açıklaştırma / Koyulaştırma yapar |
| <kbd>M</kbd> | **Dikdörtgen Seçim** | Kayan karıncalar eşliğinde dikdörtgen alan seçimi |
| <kbd>Q</kbd> | **Kement Seçimi** | Serbest el ile tam dolu çokgen seçimi |
| <kbd>W</kbd> | **Sihirli Değnek** | Eşleşen renkli bitişik alanları seçer |
| <kbd>P</kbd> | **Dayanak Noktaları** | Sprite merkezini ve eylem noktalarını konumlandırır |
| <kbd>X</kbd> | **Renkleri Değiş** | Ön Plan ve Arka Plan renklerinin yerini değiştirir |
| <kbd>[</kbd> / <kbd>]</kbd> | **Fırça Boyutu** | Fırça boyutunu azaltır / artırır (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopyala** | Seçili pikselleri sistem panosuna kopyalar |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Kes** | Seçili pikselleri panoya keser |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Yapıştır** | Pano piksellerini aktif kareye yapıştırır |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Sil** | Aktif seçim içindeki pikselleri temizler |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Tümünü Seç** | Aktif karedeki tüm pikselleri seçer |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Seçimi Kaldır** | Aktif seçim maskesini temizler |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Seçimi Tersine Çevir** | Aktif seçim maskesini tersine çevirir |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Kareyi Çoğalt** | Zaman çizelgesindeki aktif kareyi kopyalar |
| <kbd>Boşluk</kbd> | **Oynat / Duraklat** | Zaman çizelgesinde animasyon oynatmayı başlatır/durdurur |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Renk Değiştir** | Canlı önizlemeli Renk Değiştirme penceresini açar |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Renk Ayarı** | HSV Ton / Doygunluk / Kontrast penceresini açar |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Serbest Döndürme** | İstenen açıda döndürme penceresini (RotSprite) açar |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Geri Al** | Son tuval vuruşunu veya araç eylemini geri alır |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Yinele** | Geri alınmış eylemi tekrarlar |
| <kbd>Boşluk</kbd> + Sürükle / Orta Tık | **Tuvali Kaydır** | Tuval koordinatlarında pürüzsüz kaydırma sağlar |
| <kbd>Ctrl/Cmd</kbd> + Tekerlek / <kbd>+</kbd> / <kbd>-</kbd> | **Yakınlaştırma** | İmleç odaklı piksel düzeyinde yakınlaştırma |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Görünüm Modu** | Kare Görünümü (<kbd>1</kbd>) ile Atlas Görünümü (<kbd>2</kbd>) arasında geçiş |
| <kbd>F</kbd> | **Tuvali Sığdır** | Aktif kareyi çalışma alanının ortasına sığdırır |
| <kbd>F1</kbd> | **Yardım** | Klavye Kısayolları penceresini açar |

---

## Başlarken

### Hazır İkili Dosyalar

Önceden derlenmiş, bağımsız ikili dosyaları **[GitHub Sürümler Sayfasından](https://github.com/rbm13s/OniAnvil/releases/latest)** indirin:

- **Windows**: `OniAnvil-windows-x86_64.zip` arşivini indirin, ayıklayın ve `OniAnvil.exe` dosyasına çift tıklayın.
- **Linux**: `OniAnvil-linux-x86_64.tar.gz` taşınabilir arşivini indirin veya Debian/Ubuntu için `onianvil_*_amd64.deb` paketini yükleyin:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: `OniAnvil-macos-universal.zip` arşivini indirin, ayıklayın ve `OniAnvil.app` dosyasını Uygulamalar (Applications) klasörünüze taşıyın. Evrensel ikili paket hem Apple Silicon (M1/M2/M3/M4) hem de Intel Mac'lerde yerel olarak çalışır.
- **Web**: Kurulum yapmadan doğrudan tarayıcınızda çalıştırın: **[WebAssembly Uygulamasını Başlat](https://rbm13s.github.io/OniAnvil/app/?lang=tr)**.
  > [!WARNING]
  > WebAssembly (tarayıcı) sürümü **Deneysel (Experimental)** durumdadır. Tarayıcı koruma alanı kısıtlamaları, WebGL/WebGPU farkları ve sistem panosu sınırlamaları nedeniyle yerel masaüstü sürümüne kıyasla küçük görsel aksaklıklar veya performans düşüşleri yaşayabilirsiniz. Üretim çalışmaları için yerel masaüstü sürümleri şiddetle önerilir.

---

### Kaynak Koddan Derleme

#### Önkoşullar
- **Rust Araç Zinciri**: [rustup.rs](https://rustup.rs/) aracılığıyla Rust'ı kurun (1.80+ önerilir):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux Derleme Bağımlılıkları** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Yerel Masaüstü Sürümünü Derleme ve Çalıştırma
```bash
# Depoyu klonlayın
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Test paketini çalıştırın
cargo test

# Optimize edilmiş sürüm ikilisini derleyin ve çalıştırın
cargo run --release
```

#### 2. WebAssembly Sürümünü Derleme (Trunk ile)
```bash
# wasm32 derleme hedefini ve Trunk derleyicisini kurun
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Canlı yeniden yüklemeli yerel geliştirme sunucusunu başlatın
trunk serve

# Optimize edilmiş dağıtım dosyalarını docs/app/ dizinine derleyin
./build_wasm.sh        # Linux/macOS üzerinde
./build_wasm.ps1       # Windows PowerShell üzerinde
```

---

## Dosya Formatı ve Eşlikçi Mimarisi

OniAnvil kapalı veya tescilli ikili formatlardan kaçınır. Her proje, sürüm kontrol sistemleriyle uyumlu iki temiz dosyadan oluşur:

```
my_character/
├── my_character.png            # Standart PNG sprite sayfası atlası (tüm motorlarda ve görüntüleyicilerde açılabilir)
└── my_character.onianvil.json   # Eşlikçi üstveri dosyası
```

`.onianvil.json` eşlikçi dosyası şunları saklar:
- Tuval ızgara yapılandırması: kare boyutları, sütun sayısı, toplam kare sayısı.
- Katman hiyerarşileri: kanal adları, opaklık seviyeleri, görünürlük bayrakları, kilitli durumlar.
- Animasyon etiketleri: ad, başlangıç/bitiş kare indeksleri, oynatma yönü (İleri, Geri, Pinpon), kare süreleri.
- Palet tanımları: aktif renk yuvaları, adlandırılmış görünüm varyasyonları, aktif görünüm indeksi.
- Referans eskiz durumu: görsel dosya yolu, ölçek, kaydırma konumu, döndürme açısı, opaklık, katman modu.
- Dayanak noktaları: özel eylem noktaları, silah bağlantı noktaları ve çarpışma merkezleri.

---

## Katkıda Bulunma ve Teşekkürler

Katkılarınız, hata bildirimleriniz ve özellik önerileriniz memnuniyetle karşılanır!
1. Projeyi GitHub üzerinde forklayın.
2. Yeni bir özellik dalı oluşturun: `git checkout -b feature/yeni-ozellik`.
3. Değişikliklerinizi kaydedin: `git commit -am 'Yeni piksel aracı eklendi'`.
4. Dalınızı uzak depoya gönderin: `git push origin feature/yeni-ozellik`.
5. Bir Çekme İsteği (Pull Request) açın.

### Teşekkürler
- Başlangıç demo animasyon sprite'ı **"Cat 2D Pixel Art"**, **xzany** tarafından tasarlanmıştır ([itch.io proje sayfası](https://xzany.itch.io/cat-2d-pixel-art)).
- Harika **[egui](https://github.com/emilk/egui)** grafik arayüz kütüphanesi ve **[wgpu](https://wgpu.rs/)** grafik ekosistemi ile inşa edilmiştir.
- Kullanıcı arayüzü ikonları @fontawesome tarafından sağlanan **[Font Awesome Free](https://fontawesome.com)** paketiyle güçlendirilmiştir (İkonlar: CC BY 4.0, Yazı Tipleri: SIL OFL 1.1, Kod: MIT Lisansı).

---

## Lisans ve Garanti Reddi

Bu proje **[MIT Lisansı](LICENSE)** altında lisanslanmıştır. OniAnvil'i ticari ve kişisel oyun geliştirme süreçlerinizde özgürce kullanabilir, değiştirebilir, dağıtabilir ve projelerinize entegre edebilirsiniz.

### Garanti Reddi ve Sorumluluğun Sınırlandırılması
> [!IMPORTANT]
> **OniAnvil**, açık veya zımni hiçbir garanti ya da koşul olmaksızın, **ücretsiz olarak**, **"OLDUĞU GİBİ" ("AS IS")** ve **"MEVCUT OLDUĞU ŞEKİLDE" ("AS AVAILABLE")** dağıtılmaktadır.
> 
> Telif hakkı sahipleri, yazarlar veya katkıda bulunanlar; bu yazılımın kullanımından veya kullanılamamasından doğabilecek herhangi bir doğrudan, dolaylı, arızi, özel, cezai veya netice kabilinden zarardan (veri kaybı, proje dosyalarının bozulması, donanım arızası, kar kaybı veya iş kesintisi dahil ancak bunlarla sınırlı olmamak üzere) hiçbir koşulda sorumlu tutulamaz. Yazılımın kullanımına ilişkin tüm risk ve sorumluluk kullanıcıya aittir.

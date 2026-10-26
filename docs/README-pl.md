<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio grafiki pikselowej i animacji poklatkowej (Onion Skinning) oparte na atlasach gier**

[![Kompilacja i wydanie](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Pobrania](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Języki](https://img.shields.io/badge/Języki-37_Języków-purple)](#10-35-ergonomicznych-motywów-i-pełna-lokalizacja)
[![Platformy](https://img.shields.io/badge/Platformy-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#pierwsze-kroki)
[![Aplikacja online](https://img.shields.io/badge/Aplikacja_online-Wypróbuj_w_przeglądarce_(Eksperymentalna)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=pl)
[![Licencja: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Wypróbuj OniAnvil w przeglądarce (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=pl)** *(⚠️ Wersja eksperymentalna: może zawierać ograniczenia WebGL/WASM i błędy)* • **[🌐 Oficjalna strona dokumentacji](https://rbm13s.github.io/OniAnvil/pl.html)**

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
**Polski** •
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

## Spis treści

- [Wprowadzenie](#wprowadzenie)
- [Zrzuty ekranu i prezentacja wizualna](#zrzuty-ekranu-i-prezentacja-wizualna)
- [Kompletny przewodnik po narzędziach](#kompletny-przewodnik-po-narzędziach)
- [Dokumentacja funkcjonalna](#dokumentacja-funkcjonalna)
  - [1. Architektura oparta na atlasie jako źródle prawdy](#1-architektura-oparta-na-atlasie-jako-źródle-prawdy)
  - [2. Wielościeżkowa oś czasu warstw](#2-wielościeżkowa-oś-czasu-warstw)
  - [3. Podgląd animacji na żywo w czasie rzeczywistym](#3-podgląd-animacji-na-żywo-w-czasie-rzeczywistym)
  - [4. Zaawansowany silnik kalki (Onion Skinning)](#4-zaawansowany-silnik-kalki-onion-skinning)
  - [5. Inteligentna warstwa szkicu referencyjnego](#5-inteligentna-warstwa-szkicu-referencyjnego)
  - [6. Zestaw precyzyjnego zaznaczania i maszerujące mrówki](#6-zestaw-precyzyjnego-zaznaczania-i-maszerujące-mrówki)
  - [7. Efekty na żywo, filtry i RotSprite](#7-efekty-na-żywo-filtry-i-rotsprite)
  - [8. Zarządzanie paletami, zamiana kolorów i skórki postaci](#8-zarządzanie-paletami-zamiana-kolorów-i-skórki-postaci)
  - [9. Wycinak arkuszy (Sheet Slicer), 9-Grid UI i eksport do silników gier](#9-wycinak-arkuszy-sheet-slicer-9-grid-ui-i-eksport-do-silników-gier)
  - [10. 35 ergonomicznych motywów i pełna lokalizacja](#10-35-ergonomicznych-motywów-i-pełna-lokalizacja)
- [Sterowanie i skróty klawiszowe](#sterowanie-i-skróty-klawiszowe)
- [Pierwsze kroki](#pierwsze-kroki)
  - [Gotowe pliki binarne](#gotowe-pliki-binarne)
  - [Kompilacja ze źródeł](#kompilacja-ze-źródeł)
- [Format plików i architektura towarzysząca](#format-plików-i-architektura-towarzysząca)
- [Współpraca i podziękowania](#współpraca-i-podziękowania)
- [Licencja i wyłączenie odpowiedzialności](#licencja-i-wyłączenie-odpowiedzialności)

---

## Wprowadzenie

**OniAnvil** (*nazwa pochodzi od **Oni**on-Skinning [kalka animacyjna] + **Anvil** [kowadło do wykuwania duszków]* ) to otwartoźródłowy, wysoce wydajny edytor grafiki pikselowej oraz pakiet animacji napisany w języku Rust. Działa natywnie ze sprzętowym przyspieszeniem renderowania na systemach Windows, Linux i macOS, a także jako bezinstalacyjna aplikacja WebAssembly bezpośrednio w nowoczesnych przeglądarkach internetowych.

Większość tradycyjnych edytorów traktuje animację jako stos odizolowanych plików graficznych, zmuszając twórców gier do żmudnego eksportowania, cięcia i ponownego pakowania klatek. **OniAnvil odwraca ten paradygmat:** tekstura atlasu arkusza sprite'ów (`atlas_rgba`) stanowi nadrzędne, pojedyncze źródło prawdy. Komórki klatek, ścieżki warstw, etykiety animacji i punkty zaczepienia kolizji pozostają bezstratnie zsynchronizowane z atlasem w każdej chwili pracy.

### Dlaczego OniAnvil?
- 🚀 **Błyskawiczna wydajność natywnego Rusta**: Natychmiastowy start w ułamku milisekundy, krystalicznie ostry rendering napędzany przez `wgpu` i `egui 0.31`, zerowe opóźnienia podczas przewijania rozbudowanych osi czasu.
- 🎬 **Wielościeżkowa oś czasu (NLE)**: Ścieżki warstw (`V1, V2, V3...`) z kompozycją góra-na-dół, scalaniem w dół, powiązanymi instancjami klatek i zmiennymi czasami zatrzymania (hold).
- 🧅 **Pierwszorzędna kalka (Onion Skinning)**: Wieloklatkowy podgląd sąsiednich klatek z niezależnym barwieniem w przód/w tył, w pełni konfigurowalnymi krzywymi zanikania i zapętleniem krawędzi.
- 🎨 **Błyskawiczne skórki palet postaci**: Twórz warianty dzień/noc, kolory drużyn czy skórki żywiołów dzięki natychmiastowemu przemapowaniu atlasu oraz obsłudze formatów GPL, PAL i HEX.
- 📦 **Bezpośredni eksport do silników gier**: Eksportuj przeskalowane pliki PNG z ekstruzją pikseli krawędziowych (eliminacja przenikania tekstur w silnikach 2D: Godot, Bevy, Raylib, Defold), standardowe metadane Sprite Sheet JSON oraz animowane pliki GIF.

---

## Zrzuty ekranu i prezentacja wizualna

<div align="center">

### 1. Główne płótno i przestrzeń robocza studia
*Wysokowydajny obszar roboczy pixel art z akcelerowanym sprzętowo widokiem, wielościeżkową osią czasu, podglądem na żywo w czasie rzeczywistym, próbkami palety i precyzyjnymi paskami narzędzi.*
<img src="screenshots/main.png" alt="Główna przestrzeń robocza OniAnvil" width="950" />

<br/><br/>

### 2. Wielościeżkowa oś czasu animacji
*Nieliniowa oś czasu ze stosem warstw (`V1, V2...`), połączonymi instancjami klatek (`🔗`), mnożnikami czasu trwania (hold) oraz oznaczonymi kolorami etykietami animacji.*
<img src="screenshots/timeline.png" alt="Wielościeżkowa oś czasu OniAnvil" width="950" />

<br/><br/>

### 3. Precyzyjne narzędzia i pasek szybkich akcji
*Kompletny zestaw 15 narzędzi rysunkowych w trybie standardowym i kompaktowym, wraz z górnym paskiem szybkiego dostępu do rozmiaru pędzla, symetrii, zawijania krawędzi i trybu Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Pasek akcji OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Narzędzia OniAnvil - standardowe" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Narzędzia OniAnvil - kompaktowe" width="300" />
</p>

<br/><br/>

### 4. Podgląd animacji na żywo w czasie rzeczywistym
*Dedykowany dok odtwarzania z filtrowaniem według etykiet, regulacją prędkości FPS, suwakiem powiększenia oraz przełącznikami tła (szachownica, jednolite ciemne, jednolite jasne).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Podgląd na żywo OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Powiększenie podglądu na żywo" width="460" />
</p>

<br/><br/>

### 5. Zarządzanie paletami, zamiana kolorów i rampy tonalne
*Zaawansowany silnik kolorów z polami koloru głównego i tła, automatycznym generowaniem dwupunktowych ramp gradientowych i zamianą nazwanych skórek palet.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta i zamiana kolorów w OniAnvil" width="950" />

<br/><br/>

### 6. Szkic referencyjny i konfigurowalna kalka (Onion Skinning)
*Wczytuj szkice koncepcyjne lub fotografie jako podkład/nakładkę z pełną kontrolą transformacji, inteligentnym pobieraniem kolorów z przyciąganiem do palety i wieloklatkową kalką.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Szkic i kalka Onion Skin w OniAnvil" width="950" />

<br/><br/>

### 7. Punkty obrotu (Pivot) i punkty akcji kolizji
*Definiuj środek sprite'a oraz własne nazwane punkty rozgrywki (zaczepy broni, emitory pocisków, głowa, kroki) za pomocą interaktywnych widżetów na płótnie.*
<img src="screenshots/pivot_and_action_points.png" alt="Punkty obrotu i akcji w OniAnvil" width="950" />

<br/><br/>

### 8. Wycinak arkuszy (Sheet Slicer) i skalowanie 9-Grid (9-Slice)
*Automatyczne wykrywanie granic sprite'ów metodą autokorelacji (np. cykl chodu kota) wraz z konfiguracją marginesów skalowalnych ramek interfejsu 9-slice.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Wycinak arkuszy OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Skaler 9-Slice w OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Filtry i efekty graficzne na żywo (z podglądem w czasie rzeczywistym)
*Wszystkie okna dialogowe efektów oferują podgląd na żywo na płótnie, interaktywne pipety próbkowania oraz bezstratne anulowanie bez zaśmiecania historii operacji.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generator obrysu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Zamiana koloru" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Odcień, nasycenie, kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtry macierzowe" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Przemapowanie do palety" width="460" />
</p>

<br/><br/>

### 10. Siatki płótna i tryby tła
*Przełączaj się płynnie między siatką pikseli, siatką klatek, ciemną szachownicą, jasną szachownicą oraz własnymi jednolitymi kolorami tła.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Siatka i ciemna szachownica" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Jasna szachownica bez siatki" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Własne jednolite tło" width="300" />
</p>

<br/><br/>

### 11. Kreator tworzenia nowego projektu
*Konfiguruj wymiary klatki, liczbę klatek, układ kolumnowy atlasu oraz wybieraj spośród ponad 20 autentycznych palet retro na starcie projektu.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Kreator nowego projektu" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palety nowego projektu" width="460" />
</p>

<br/><br/>

### 12. 35 autentycznych motywów retro i nowoczesnych
*Zawiera Studio Dark, Studio Light, luminofory CRT (bursztynowy, zielony, błękitny), konsole retro (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) oraz motywy autorskie.*
<p align="center">
  <img src="screenshots/themes.png" alt="Menu motywów" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Motyw CRT bursztynowy" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Motyw PSX" width="300" />
</p>

<br/><br/>

### 13. System menu i wbudowana pomoc skrótów klawiszowych
<p align="center">
  <img src="screenshots/file_menu.png" alt="Menu Plik" width="180" />
  <img src="screenshots/edit_menu.png" alt="Menu Edycja" width="180" />
  <img src="screenshots/select_menu.png" alt="Menu Zaznacz" width="180" />
  <img src="screenshots/view_menu.png" alt="Menu Widok" width="180" />
  <img src="screenshots/project_menu.png" alt="Menu Projekt" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Okno pomocy skrótów klawiszowych" width="700" />
</p>

</div>

---

## Kompletny przewodnik po narzędziach

OniAnvil zawiera 15 precyzyjnych narzędzi do rysowania, geometrii, zaznaczania i metadanych. Ikony są renderowane przy użyciu otwartoźródłowego zestawu **Font Awesome Free** *(ikony: CC BY 4.0, fonty: SIL OFL 1.1)*.

| Narzędzie | Ikona | Skrót | Rozmiar pędzla | Pixel Perfect | Opis |
|:---|:---:|:---:|:---:|:---:|:---|
| **Ołówek** | ✏️ | <kbd>B</kbd> | 1..32 px | Tak | Główne narzędzie rysowania. W trybie 1px obsługuje tryb **Pixel-Perfect** (automatycznie eliminuje nieestetyczne narożniki schodkowe L). Obsługuje stemple z zaznaczenia jako własne pędzle (`Ctrl/Cmd+B`). |
| **Gumka** | 🧹 | <kbd>E</kbd> | 1..32 px | Nie | Wymazuje piksele z powrotem do pełnej przezroczystości (`[0, 0, 0, 0]`). Ograniczona maską zaznaczenia, jeśli jest aktywna. |
| **Pipeta** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nie | Kliknięcie LPM pobiera kolor do Głównego (Poprzedniego); PPM pobiera do Tła (Drugorzędnego). Pobiera kolor z pikseli płótna, warstw lub szkicu referencyjnego. |
| **Wypełnienie** | 🪣 | <kbd>G</kbd> | Nd. | Nie | Błyskawiczne wypełnianie obszaru oparte na kolejce (BFS) z regulowaną tolerancją koloru (`0..=255`). Obsługuje tryb ciągły (Contiguous) i globalny oraz ograniczenie zaznaczeniem. |
| **Gradient** | 🌈 | <kbd>G</kbd> *(przełącznik)* | Nd. | Nie | Gradienty liniowe i radialne między kolorem głównym a tłem. Obsługuje dithering matrycowy Bayera (2x2, 4x4, 8x8) oraz Blue Noise z kontrolą intensywności. |
| **Linia** | 📏 | <kbd>L</kbd> | 1..32 px | Tak | Algorytm linii Bresenhama z podglądem na żywo podczas przeciągania. Przytrzymaj <kbd>Shift</kbd>, aby zablokować kąty w krokach co 45°/90°. |
| **Krzywa** | 〰️ | <kbd>C</kbd> | 1..32 px | Tak | 3-klikowa kwadratowa krzywa Béziera. Kliknięcie 1 ustala punkt początkowy, kliknięcie 2 punkt końcowy, ruch myszy wygina krzywiznę, a kliknięcie 3 zatwierdza krzywą. |
| **Prostokąt** | ⬜ | <kbd>U</kbd> | 1..32 px | Nie | Geometryczne prostokąty z konturem lub wypełnieniem. Przytrzymaj <kbd>Shift</kbd>, aby zachować proporcje kwadratu 1:1. |
| **Elipsa** | ⭕ | <kbd>O</kbd> | 1..32 px | Nie | Geometryczne elipsy z konturem lub wypełnieniem z rasteryzacją punktu środkowego. Przytrzymaj <kbd>Shift</kbd>, aby uzyskać idealne koło. |
| **Pędzel ditheringu** | 🏁 | <kbd>D</kbd> | 1..32 px | Nie | Maluje naprzemiennymi wzorami szachownicy ditheringu między kolorem głównym a tłem bez wprowadzania dodatkowych odcieni do palety. |
| **Pędzel cieniowania** | 🌓 | <kbd>S</kbd> | 1..32 px | Nie | Stopniowe rozjaśnianie (Dodge) i przyciemnianie (Burn) pikseli w oparciu o kolejność próbek w aktywnej palecie. |
| **Zaznaczenie prostokątne** | ⬚ | <kbd>M</kbd> | Nd. | Nie | Prostokątne zaznaczanie obszaru z animowaną obwódką maszerujących mrówek. Obsługuje dodawanie (<kbd>Shift</kbd>) i odejmowanie (<kbd>Alt</kbd>). |
| **Lasso** | ➰ | <kbd>Q</kbd> | Nd. | Nie | Swobodne wielokątne lasso odręczne. Wykorzystuje 8-spójną rasteryzację obwiedni z 4-spójnym wypełnieniem zewnętrznym: gwarancja 100% jednolitego zaznaczenia bez luk, nawet przy przecinających się pętlach. |
| **Magiczna różdżka** | 🪄 | <kbd>W</kbd> | Nd. | Nie | Zaznacza spójne (lub globalne) obszary pasującego koloru z regulowanym progiem tolerancji. |
| **Punkt obrotu i akcji** | 🎯 | <kbd>P</kbd> | Nd. | Nie | Interaktywne umieszczanie na płótnie środka sprite'a (Origin) oraz własnych punktów akcji rozgrywki (`weapon`, `head`, `feet`, `hitbox`) eksportowanych do metadanych JSON. |

---

## Dokumentacja funkcjonalna

### 1. Architektura oparta na atlasie jako źródle prawdy

W OniAnvil główny atlas tekstury (`atlas_rgba`) reprezentuje rzeczywistą teksturę arkusza sprite'ów ładowaną przez silnik gry:
- **Tryb klatki (Frame Mode)**: Koncentruje się na pojedynczej aktywnej klatce animacji, zapewniając wolne od rozproszeń płótno pikselowe. Wszelkie zmiany natychmiast aktualizują odpowiadającą komórkę w teksturze atlasu.
- **Tryb atlasu (Atlas Mode)**: Wyświetla całą siatkę arkusza sprite'ów, umożliwiając makro-korektę, wyrównywanie między klatkami oraz edycję wsadową.
- **Plik towarzyszący (Companion File)**: Projekty zapisują czyste arkusze sprite'ów `.png` wraz z lekkim plikiem towarzyszącym `.onianvil.json` zawierającym układy klatek, hierarchie warstw, etykiety, mnożniki czasu trwania i skórki palet.

### 2. Wielościeżkowa oś czasu warstw

Dolna oś czasu została zaprojektowana na wzór profesjonalnych programów do nieliniowego montażu wideo:
- **Ścieżki warstw (`V1, V2, V3...`)**: Warstwy nakładają się wizualnie od góry do dołu. Ścieżka umieszczona wyżej na osi czasu (`V1`) renderuje się na wierzchu płótna klatki.
- **Sterowanie ścieżkami**:
  - Kliknij plakietki `V1` / `V2`, aby szybko przełączać aktywną warstwę rysowania.
  - Przełączaj widoczność warstwy (ikona Oka) oraz blokadę edycji (ikona Kłódki).
  - Reguluj przezroczystość poszczególnych warstw (`0%..=100%`).
  - Przesuwaj warstwy w górę/w dół w hierarchii, duplikuj warstwy i scalaj warstwę w dół ze ścieżką poniżej.
  - Usuwaj warstwy z bezpiecznym oknem dialogowym potwierdzenia.
- **Klip klatki**:
  - Mnożnik zatrzymania (Hold): przypisuj niestandardowe czasy trwania klatek (np. `2×`, `3×` dla pauz lub klatek uderzenia).
  - Klatki powiązane (`🔗`): łącz powtarzające się klatki w animacji, dzięki czemu edycja jednej automatycznie aktualizuje wszystkie powiązane instancje.
  - Zapętlanie etykiet: organizuj animacje w nazwane sekwencje (`idle`, `walk`, `attack`) i zapętlaj określone fragmenty podczas odtwarzania.
  - Sterowanie odtwarzaniem: odtwarzanie w przód, w tył, ping-pong i pętla z regulowaną prędkością FPS (`1..=120`).

### 3. Podgląd animacji na żywo w czasie rzeczywistym

Prawy panel podglądu odtwarza Twoje animacje w czasie rzeczywistym, całkowicie niezależnie od powiększenia czy przesunięcia głównego płótna:
- **Filtrowanie etykiet**: Izoluj odtwarzanie do konkretnych etykiet (`walk`, `jump`) lub przeglądaj całą sekwencję animacji.
- **Sterowanie odtwarzaniem**: Przyciski Play, Pause, krok w tył, krok w przód, przełącznik pętli oraz niestandardowe mnożniki prędkości (`0.25x..=4.0x`).
- **Tryby wyświetlania**: Przełączaj między ciemną szachownicą, jasną szachownicą, czystą bielą, czystą czernią lub własnymi tłami płótna.

### 4. Zaawansowany silnik kalki (Onion Skinning)

Twórz płynne animacje dzięki dedykowanemu systemowi kalkowania:
- **Niezależne barwienie**: Osobne kolory zabarwienia dla klatek poprzednich (domyślnie czerwony) i następnych (domyślnie niebieski).
- **Zanikanie przezroczystości**: Konfigurowalna przezroczystość bazowa oraz krzywe tłumienia przezroczystości dla kolejnych kroków.
- **Zapętlenie krawędzi (Loop Wrap-Around)**: Włącz kalkowanie między pierwszą a ostatnią klatką zapętlonych animacji.
- **Szybki podgląd (Quick-Peek)**: Przytrzymaj <kbd>Spację</kbd> bez przeciągania myszy, aby natychmiast podejrzeć sąsiednie klatki.

### 5. Inteligentna warstwa szkicu referencyjnego

Przyspiesz pracę, obrysowując odręczne szkice lub fotografie referencyjne:
- **Formaty**: Wczytuj obrazy referencyjne PNG, JPEG i WebP.
- **Tryby mieszania**: Podkład (pod Twoimi warstwami pikselowymi) lub Nakładka (nad warstwami pikselowymi).
- **Kontrola transformacji**: Niezależny zoom, przesunięcie pozycji, obrót oraz suwak przezroczystości.
- **Inteligentne próbkowanie kolorów**: Używaj pipety na warstwie szkicu z opcjonalnym automatycznym przyciąganiem do najbliższego koloru aktywnej palety projektu.
- **Wtopienie jednym kliknięciem (1-Click Bake)**: Spłaszcz przeskalowany obraz referencyjny bezpośrednio do pikseli na aktywnej klatce.

### 6. Zestaw precyzyjnego zaznaczania i maszerujące mrówki

- **Animowane maszerujące mrówki**: Aktywne zaznaczenia wyświetlają animowany, dwutonowy kontur o wysokiej widoczności.
- **Jednolite lasso oparte na wypełnieniu**: W przeciwieństwie do naiwnych algorytmów rzucania promieni, które tworzą luki przy przecinających się pętlach, lasso w OniAnvil rasteryzuje zamkniętą obwiednię 8-spójną i wypełnia zewnętrze, gwarantując w 100% jednolite zaznaczenie bez dziur.
- **Operacje na zaznaczeniu**: Rozszerzanie, Zwężanie, Obrys (z jądrem kołowym, kwadratowym i rombowym), Przesuwanie, Odwracanie, Kasowanie, Dowolny obrót oraz Kopiuj/Wytnij/Wklej ze schowka systemowego.
- **Kopiowanie i wklejanie między klatkami**: Zaznacz obszar na jednej klatce, wciśnij <kbd>Ctrl/Cmd+C</kbd>, zmień klatkę i wciśnij <kbd>Ctrl/Cmd+V</kbd>, aby wkleić dokładnie na tych samych współrzędnych względnych na dowolnym układzie klawiatury.

### 7. Efekty na żywo, filtry i RotSprite

Wszystkie okna dialogowe efektów graficznych zawierają pole wyboru **Podgląd na żywo (Live Preview)**, które wyświetla zmiany na płótnie i pozwala na bezproblemowe anulowanie bez zanieczyszczania historii cofania:
- **Zamiana koloru (`Shift+R`)**:
  - Interaktywna pipeta: kliknij ikonę pipety i wybierz kolory bezpośrednio z płótna klatki lub z doku palety.
  - Pasek szybkiego wyboru palety: kliknij LPM, aby wybrać kolor źródłowy („Z”), PPM, aby wybrać kolor docelowy („Na”).
  - Suwak tolerancji oraz wsadowa zamiana we wszystkich klatkach animacji.
- **Dopasowanie koloru (`Ctrl/Cmd+U`)**: Regulacja odcienia HSV (Hue), nasycenia (Saturation), jasności (Value) i kontrastu w czasie rzeczywistym.
- **Generator automatycznego obrysu**: Twórz ostre pikselowe obrysy postaci (Na zewnątrz, Wewnątrz, Wycentrowane) z łączeniami kwadratowymi lub rombowymi.
- **Filtry splotowe 3×3**: Rozmycie, Wyostrzanie, Wykrywanie krawędzi i Płaskorzeźba z opcją zachowania przezroczystości (preserve-alpha).
- **Dowolny obrót (`Ctrl/Cmd+Shift+R`)**: Obracaj zaznaczone fragmenty lub całe klatki o dowolny kąt przy użyciu klasycznego algorytmu **RotSprite** lub **Nearest Neighbor**.
- **Przemapowanie do palety**: Redukuj kolory płótna do aktywnej palety z ditheringiem Bayera (2x2, 4x4, 8x8) lub Blue Noise.

### 8. Zarządzanie paletami, zamiana kolorów i skórki postaci

- **Wbudowane palety**: Ponad 20 autentycznych palet retro, w tym Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized i Sweetie 16.
- **Import i eksport**: Obsługa formatów GIMP Palette (`.gpl`), PAL (`.pal`), Hex (`.hex`) oraz próbek z obrazów (`.png`).
- **Zamiana palet (Skórki postaci)**: Zdefiniuj wiele wariantów skórek dla jednej postaci (np. Domyślna, Złota zbroja, Cień, Zmrożona). Przełączenie skórki natychmiast przemapowuje wszystkie kolory klatek w całym atlasie.
- **Generator ramp kolorów**: Automatycznie interpoluj płynne przejścia gradientowe między dwoma wybranymi odcieniami z regulowaną liczbą kroków pośrednich.

### 9. Wycinak arkuszy (Sheet Slicer), 9-Grid UI i eksport do silników gier

- **Wycinak arkuszy (Sheet Slicer)**: Wykrywanie granic metodą autokorelacji automatycznie tnie gotowe arkusze na równe lub dopasowane klatki animacji.
- **Skalowanie 9-Grid (9-Slice)**: Ustawiaj konfigurowalne marginesy podziału (góra, dół, lewo, prawo), aby testować skalowalne panele UI i przyciski.
- **Eksport PNG z ekstruzją krawędzi**: Rozciąga kolory skrajnych pikseli na przezroczyste marginesy, eliminując artefakty filtrowania tekstur i szwy w OpenGL, DirectX, Vulkan oraz silnikach gier 2D (takich jak Godot).
- **Eksport animowanych GIF-ów**: Twórz zoptymalizowane pliki GIF z konfigurowalną liczbą pętli, opóźnieniem klatek i przezroczystym tłem.
- **Standardowy format Sprite Sheet JSON**: Eksportuje współrzędne klatek, etykiety, czasy trwania i punkty obrotu zgodne ze standardowym schematem JSON powszechnie stosowanym we współczesnych silnikach gier.

### 10. 35 ergonomicznych motywów i pełna lokalizacja

- **Dopracowane motywy**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, luminofory CRT (Bursztynowy, Zielony, Błękitny), konsole retro (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) oraz komputery retro (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Wielojęzyczność**: Płynne przełączanie między 37 językami ze 100% parytetem kluczy słownika i etykiet interfejsu.

---

## Sterowanie i skróty klawiszowe

| Skrót | Narzędzie / Akcja | Opis |
|:---|:---|:---|
| <kbd>B</kbd> | **Ołówek** | Precyzyjne rysowanie 1px z opcjonalnym trybem Pixel-Perfect |
| <kbd>E</kbd> | **Gumka** | Wymazywanie pikseli do pełnej przezroczystości |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipeta** | Pobieranie próbki koloru z płótna lub szkicu |
| <kbd>G</kbd> | **Wypełnienie / Gradient** | Przełączanie między wypełnieniem a gradientem |
| <kbd>L</kbd> | **Linia** | Rysowanie linii prostej z podglądem i przyciąganiem Shift |
| <kbd>C</kbd> | **Krzywa** | 3-klikowe narzędzie kwadratowej krzywej Béziera |
| <kbd>U</kbd> | **Prostokąt** | Rysowanie prostokątów i kwadratów (kontur lub wypełnienie) |
| <kbd>O</kbd> | **Elipsa** | Rysowanie elips i kół (kontur lub wypełnienie) |
| <kbd>D</kbd> | **Pędzel ditheringu** | Malowanie wzorcem matrycy szachownicy ditheringu |
| <kbd>S</kbd> | **Pędzel cieniowania** | Stopniowe rozjaśnianie / przyciemnianie wzdłuż palety |
| <kbd>M</kbd> | **Zaznaczenie prostokątne** | Prostokątne zaznaczanie obszaru z maszerującymi mrówkami |
| <kbd>Q</kbd> | **Lasso** | Jednolite odręczne zaznaczanie wielokątne |
| <kbd>W</kbd> | **Magiczna różdżka** | Zaznaczanie spójnych obszarów o podobnym kolorze |
| <kbd>P</kbd> | **Punkty obrotu i akcji** | Pozycjonowanie punktu zaczepienia i punktów akcji |
| <kbd>X</kbd> | **Zamiana kolorów** | Zamiana koloru głównego i koloru tła miejscami |
| <kbd>[</kbd> / <kbd>]</kbd> | **Rozmiar pędzla** | Zmniejszenie / zwiększenie rozmiaru pędzla (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopiuj** | Kopiowanie zaznaczonych pikseli do schowka |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Wytnij** | Wycięcie zaznaczonych pikseli do schowka |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Wklej** | Wklejenie pikseli ze schowka do aktywnej klatki |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Usuń** | Wyczyszczenie pikseli wewnątrz aktywnego zaznaczenia |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Zaznacz wszystko** | Zaznaczenie wszystkich pikseli w aktywnej klatce |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Odznacz** | Skasowanie aktywnej maski zaznaczenia |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Odwróć zaznaczenie** | Odwrócenie aktywnej maski zaznaczenia |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplikuj klatkę** | Zduplikowanie aktywnej klatki na osi czasu |
| <kbd>Spacja</kbd> | **Odtwarzaj / Pauza** | Przełączanie odtwarzania animacji na osi czasu |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Zamiana koloru** | Otwarcie okna zamiany koloru z podglądem na żywo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Dopasowanie koloru** | Otwarcie okna regulacji odcienia, nasycenia i kontrastu |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Dowolny obrót** | Otwarcie okna obrotu o dowolny kąt (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Cofnij** | Cofnięcie ostatniego pociągnięcia lub operacji |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Ponów** | Ponowne wykonanie cofniętej operacji |
| <kbd>Spacja</kbd> + Przeciąganie / ŚKM | **Przesuwanie płótna** | Płynne, nieskończone przesuwanie widoku płótna |
| <kbd>Ctrl/Cmd</kbd> + Kółko / <kbd>+</kbd> / <kbd>-</kbd> | **Powiększenie** | Ostre powiększanie skupione wokół kursora myszy |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Tryb widoku** | Przełączanie między widokiem klatki (<kbd>1</kbd>) a atlasu (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Dopasuj płótno** | Automatyczne dopasowanie i wyśrodkowanie klatki |
| <kbd>F1</kbd> | **Pomoc** | Otwarcie okna pomocy skrótów klawiszowych |

---

## Pierwsze kroki

### Gotowe pliki binarne

Pobierz skompilowane, samodzielne pliki binarne ze **[strony wydań na GitHubie](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Pobierz `OniAnvil-windows-x86_64.zip`, rozpakuj i uruchom dwuklikiem `OniAnvil.exe`.
- **Linux**: Pobierz `OniAnvil-linux-x86_64.tar.gz` (wersja przenośna) lub zainstaluj pakiet `onianvil_*_amd64.deb` dla systemów Debian/Ubuntu:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Pobierz `OniAnvil-macos-universal.zip`, rozpakuj i przeciągnij `OniAnvil.app` do folderu Aplikacje. Uniwersalny plik binarny działa natywnie na układach Apple Silicon (M1/M2/M3/M4) oraz komputerach Mac z procesorami Intel.
- **Web**: Uruchom bezpośrednio w przeglądarce bez instalowania czegokolwiek: **[Uruchom aplikację WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=pl)**.
  > [!WARNING]
  > Wersja przeglądarkowa (WebAssembly) posiada status **Eksperymentalny (Experimental)**. Ze względu na ograniczenia piaskownicy przeglądarki, specyfikę sterowników WebGL/WebGPU oraz ograniczenia systemowego schowka, mogą wystąpić drobne anomalie graficzne lub spadki wydajności w porównaniu z wersją natywną. Do regularnej pracy produkcyjnej zdecydowanie zaleca się wersje natywne dla komputerów stacjonarnych.

---

### Kompilacja ze źródeł

#### Wymagania wstępne
- **Zestaw narzędzi Rust**: Zainstaluj kompilator Rust (zalecana wersja 1.80+) za pośrednictwem [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Zależności kompilacji dla Linuksa** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Kompilacja i uruchomienie natywnej wersji desktopowej
```bash
# Sklonuj repozytorium
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Uruchom zestaw testów
cargo test

# Zbuduj i uruchom zoptymalizowaną wersję wydania (release)
cargo run --release
```

#### 2. Kompilacja wersji WebAssembly (za pomocą Trunk)
```bash
# Dodaj cel kompilacji wasm32 oraz zainstaluj Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Uruchom lokalny serwer deweloperski z przeładowywaniem na żywo
trunk serve

# Skompiluj zoptymalizowane pliki produkcyjne do katalogu docs/app/
./build_wasm.sh        # Na systemach Linux/macOS
./build_wasm.ps1       # W systemie Windows PowerShell
```

---

## Format plików i architektura towarzysząca

OniAnvil unika zamkniętych formatów binarnych. Każdy projekt składa się z dwóch czystych plików przyjaznych dla systemów kontroli wersji:

```
moj_bohater/
├── moj_bohater.png            # Standardowy atlas arkusza sprite'ów PNG (otwieralny w każdym silniku i przeglądarce)
└── moj_bohater.onianvil.json   # Towarzyszący plik metadanych JSON
```

Plik towarzyszący `.onianvil.json` przechowuje:
- Konfigurację siatki płótna: wymiary klatki, liczbę kolumn, liczbę klatek.
- Hierarchie warstw: nazwy ścieżek, poziomy przezroczystości, flagi widoczności, stan blokady.
- Etykiety animacji: nazwę, indeksy klatek początkowych/końcowych, kierunek odtwarzania (W przód, W tył, Ping-Pong), czasy trwania klatek.
- Definicje palet: aktywne próbki kolorów, nazwane warianty skórek, indeks aktywnej skórki.
- Stan szkicu referencyjnego: ścieżkę do obrazu, skalę, przesunięcie, kąt obrotu, przezroczystość, tryb warstwy.
- Punkty obrotu i akcji: punkt środkowy sprite'a (Origin) oraz własne punkty akcji (zaczepy broni, punkty kolizji).

---

## Współpraca i podziękowania

Wszelki wkład, zgłoszenia błędów oraz propozycje ulepszeń są niezwykle mile widziane!
1. Utwórz fork repozytorium na GitHubie.
2. Utwórz gałąź nowej funkcji: `git checkout -b feature/nowe-narzedzie`.
3. Zatwierdź swoje zmiany: `git commit -am 'Dodanie nowego narzędzia pikselowego'`.
4. Wyślij zmiany na swój fork: `git push origin feature/nowe-narzedzie`.
5. Utwórz Pull Request.

### Podziękowania
- Przykładowy sprite demonstracyjny **"Cat 2D Pixel Art"** stworzony przez **xzany** ([strona projektu na itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Zbudowano z użyciem wspaniałej biblioteki GUI **[egui](https://github.com/emilk/egui)** oraz ekosystemu graficznego **[wgpu](https://wgpu.rs/)**.
- Ikony interfejsu użytkownika pochodzą z zestawu **[Font Awesome Free](https://fontawesome.com)** autorstwa @fontawesome (ikony: CC BY 4.0, fonty: SIL OFL 1.1, kod: licencja MIT).

---

## Licencja i wyłączenie odpowiedzialności

Ten projekt jest objęty licencją **[MIT License](../LICENSE)**. Możesz go swobodnie używać, modyfikować, dystrybuować i integrować w komercyjnych oraz prywatnych projektach tworzenia gier.

### Wyłączenie gwarancji i ograniczenie odpowiedzialności
> [!IMPORTANT]
> Oprogramowanie **OniAnvil** jest rozpowszechniane **bezpłatnie** na zasadzie **„TAK JAK JEST” („AS IS”)** oraz **„W MIARĘ DOSTĘPNOŚCI” („AS AVAILABLE”)**, bez jakichkolwiek gwarancji lub warunków, wyraźnych bądź dorozumianych.
> 
> W żadnym wypadku autor(zy), właściciele praw autorskich ani osoby współpracujące **nie ponoszą odpowiedzialności** za jakiekolwiek bezpośrednie, pośrednie, przypadkowe, specjalne, moralne lub wynikowe szkody (w tym między innymi utratę danych, uszkodzenie plików, awarię sprzętu, utracone zyski lub przerwy w działalności) wynikające w jakikolwiek sposób z użytkowania lub niemożności użytkowania tego oprogramowania, nawet jeśli zostali powiadomieni o możliwości wystąpienia takich szkód. Użytkownik ponosi całkowitą odpowiedzialność i ryzyko związane z korzystaniem z tego programu.

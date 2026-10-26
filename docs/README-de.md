<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixel-Art- & Onion-Skin-Animationsstudio für spielfertige Texturatlas-Dateien**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Sprachen](https://img.shields.io/badge/Sprachen-37_Sprachen-purple)](#10-35-ergonomische-designs--vollständige-lokalisierung)
[![Plattformen](https://img.shields.io/badge/Plattformen-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#erste-schritte)
[![Web-Demo](https://img.shields.io/badge/Online_App-Im_Browser_ausprobieren_(Experimentell)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=de)
[![Lizenz: MIT](https://img.shields.io/badge/Lizenz-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 OniAnvil im Browser ausprobieren (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=de)** *(⚠️ Experimenteller Web-Build: kann WebGL/WASM-Einschränkungen und Fehler enthalten)* • **[🌐 Offizielle Dokumentations-Website](https://rbm13s.github.io/OniAnvil/de.html)**

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
**Deutsch** •
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

## Inhaltsverzeichnis

- [Einführung](#einführung)
- [Screenshots & Visuelle Tour](#screenshots--visuelle-tour)
- [Vollständige Werkzeugreferenz](#vollständige-werkzeugreferenz)
- [Funktionale Dokumentation](#funktionale-dokumentation)
  - [1. Duale atlaszentrierte Architektur](#1-duale-atlaszentrierte-architektur)
  - [2. Mehrspurige Ebenen-Zeitleiste](#2-mehrspurige-ebenen-zeitleiste)
  - [3. Echtzeit-Animations-Live-Vorschau](#3-echtzeit-animations-live-vorschau)
  - [4. Fortschrittliche Onion-Skinning-Engine](#4-fortschrittliche-onion-skinning-engine)
  - [5. Intelligente Referenz- & Skizzen-Pauspause](#5-intelligente-referenz---skizzen-pauspause)
  - [6. Präzisions-Auswahlsuite & Marching Ants](#6-präzisions-auswahlsuite--marching-ants)
  - [7. Live-Effekte, Filter & RotSprite](#7-live-effekte-filter--rotsprite)
  - [8. Palettenverwaltung, Farbschnitt & Charakter-Skins](#8-palettenverwaltung-farbschnitt--charakter-skins)
  - [9. Sheet-Slicer, 9-Grid-UI & Game-Engine-Export](#9-sheet-slicer-9-grid-ui--game-engine-export)
  - [10. 35 ergonomische Designs & zweisprachige Lokalisierung](#10-35-ergonomische-designs--zweisprachige-lokalisierung)
- [Steuerung & Tastenkombinationen](#steuerung--tastenkombinationen)
- [Erste Schritte](#erste-schritte)
  - [Eigenständige Binärdateien](#eigenständige-binärdateien)
  - [Aus dem Quellcode kompilieren](#aus-dem-quellcode-kompilieren)
- [Dateiformat & Begleitdateien-Architektur](#dateiformat--begleitdateien-architektur)
- [Mitwirken & Danksagungen](#mitwirken--danksagungen)
- [Lizenz & Haftungsausschluss](#lizenz--haftungsausschluss)

---

## Einführung

**OniAnvil** (*zusammengesetzt aus **Oni**on-Skinning [Zwiebelhaut-Animation] und **Anvil** [Amboss zur Sprite-Schmiede]*) ist ein quelloffenes, hochperformantes Pixel-Art- und Animationsstudio, geschrieben in Rust. Es läuft nativ mit hardwarebeschleunigter Grafik auf Windows, Linux und macOS sowie als installationsfreie WebAssembly-Anwendung direkt in modernen Webbrowsern.

Die meisten herkömmlichen Sprite-Editoren betrachten Animationen als lose Sammlung unzusammenhängender Bilddateien, was Spieleentwickler zu mühsamen Export-, Schneide- und Atlas-Neupack-Schritten zwingt. **OniAnvil kehrt dieses Paradigma um:** Die Spritesheet-Atlas-Textur (`atlas_rgba`) des Spiels ist die primäre Quelle der Wahrheit. Einzelbildzellen, Ebenenspuren, Animations-Tags und Kollisions-Drehpunkte bleiben zu jedem Zeitpunkt zerstörungsfrei und synchron mit dem Atlas.

### Warum OniAnvil?
- 🚀 **Rasend schnelle native Rust-Performance**: Blitzschneller Start im Millisekundenbereich, gestochen scharfe Darstellung durch `wgpu` und `egui 0.31`, kein Ruckeln beim Durchblättern langer Zeitleisten.
- 🎬 **Mehrspurige NLE-Ebenen-Zeitleiste**: Von Videoschnittprogrammen inspirierte Ebenenspuren (`V1, V2, V3...`) mit Schichtung von oben nach unten, Zusammenführen nach unten, verknüpften Frames und flexiblen Haltezeiten.
- 🧅 **Erstklassiges Onion Skinning**: Mehrbild-Zwiebelhaut mit unabhängiger Vorwärts-/Rückwärts-Farbcodierung, anpassbaren Deckkraftkurven und automatischem Schleifenumlauf.
- 🎨 **Sofortige Charakter-Paletten-Skins**: Erstellen Sie Tag-/Nacht-Varianten, Teamfarben oder Elementar-Skins mit Live-Remapping des Texturatlasses und GPL/PAL/HEX-Im-/Export.
- 📦 **Direkter Game-Engine-Export**: Export skalierter PNGs mit Randstreckung (Extruded Borders) zur Eliminierung von Textur-Nahtartefakten in 2D-Engines (Godot, Bevy, Raylib, Defold) inklusive standardisierter Spritesheet-JSON-Metadaten und animierter GIFs.

---

## Screenshots & Visuelle Tour

<div align="center">

### 1. Haupt-Arbeitsbereich & Zeichenfläche
*Leistungsstarker Pixel-Arbeitsbereich mit hardwarebeschleunigtem Viewport, mehrspuriger Zeitleiste, Echtzeit-Live-Vorschau, Farbpaletten und Präzisions-Werkzeugleisten.*
<img src="screenshots/main.png" alt="OniAnvil Haupt-Arbeitsbereich" width="950" />

<br/><br/>

### 2. Mehrspurige Animations-Zeitleiste
*Nichtlineare Mehrspur-Zeitleiste mit geschichteten Ebenenzeilen (`V1, V2...`), verknüpften Bildzellen (`🔗`), Haltezeit-Multiplikatoren und farbcodierten Animations-Tags.*
<img src="screenshots/timeline.png" alt="OniAnvil Mehrspur-Zeitleiste" width="950" />

<br/><br/>

### 3. Präzisionswerkzeuge & Schnellaktionsleiste
*Komplette 15-Werkzeuge-Suite mit Standard- und Kompakt-Docking sowie oberer Schnellaktionsleiste für Pinselgröße, Symmetrieachsen, Kantenumbruch und Pixel-Perfekt-Modus.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil Schnellaktionsleiste" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil Werkzeuge Standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil Werkzeuge Kompakt" width="300" />
</p>

<br/><br/>

### 4. Echtzeit-Animations-Live-Vorschau
*Separates Wiedergabe-Dock mit Tag-Filterung, variabler FPS-Geschwindigkeitssteuerung, Zoom-Schieberegler und umschaltbaren Viewport-Hintergründen (Schachbrett, dunkel, hell).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil Live-Vorschau" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil Live-Vorschau Zoom" width="460" />
</p>

<br/><br/>

### 5. Palettenverwaltung, Farbschnitt & Farbverläufe
*Umfassende Farb-Engine mit Vordergrund-/Hintergrund-Farbfeldern, automatischer zweipunktiger Verlaufsgenerierung und benannten Paletten-Skin-Tausch-Sets.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil Palette und Farbtausch" width="950" />

<br/><br/>

### 6. Referenzskizzen-Pauspause & Konfigurierbares Onion Skinning
*Laden Sie Konzeptskizzen oder Fotografien als Unterlage/Überlagerung mit Transformationssteuerung, intelligenter Farbaufnahme mit Paletten-Einrastung und Mehrbild-Zwiebelhaut.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil Skizze und Onion Skinning" width="950" />

<br/><br/>

### 7. Drehpunkte & Kollisions-Aktionspunkte
*Definieren Sie den Sprite-Mittelpunkt (Pivot) und benutzerdefinierte Gameplay-Punkte (Waffenanker, Projektil-Austrittspunkte, Kopf, Fußschritte) über interaktive Canvas-Gizmos.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil Dreh- und Aktionspunkte" width="950" />

<br/><br/>

### 8. Spritesheet-Slicer & 9-Grid (9-Slice) Skalierer
*Automatisches Zuschneiden von Spritesheets durch Grenzerkennung per Autokorrelation (z. B. Katzen-Laufzyklus) sowie Konfiguration von skalierbaren 9-Slice-UI-Rändern.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil Sheet-Slicer" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-Slice Skalierer" width="460" />
</p>

<br/><br/>

### 9. Echtzeit-Bildeffekte & Filterdialoge (mit Live-Vorschau)
*Alle Effektdialoge bieten direkte Canvas-Live-Vorschau, interaktive Pipetten und zerstörungsfreies Abbrechen ohne Belastung des Verlaufsspeichers.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Umriss-Generator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Farbe ersetzen" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Farbton Sättigung Kontrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Matrixfilter" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Auf Palette abbilden" width="460" />
</p>

<br/><br/>

### 10. Zeichenflächen-Raster & Hintergrundmodi
*Nahtloses Umschalten zwischen Pixelraster, Kachelfeldraster, dunklem Schachbrett, hellem Schachbrett und benutzerdefinierten Volltonfarben.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Raster und dunkles Schachbrett" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Helles Schachbrett ohne Raster" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Benutzerdefinierter Volltonhintergrund" width="300" />
</p>

<br/><br/>

### 11. Assistent für neue Projekte
*Konfigurieren Sie Einzelbildabmessungen, Frame-Anzahl, Spaltenlayout und wählen Sie beim Start aus über 20 authentischen Retro-Paletten.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Neues-Projekt-Assistent" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Palettenauswahl im Assistenten" width="460" />
</p>

<br/><br/>

### 12. 35 authentische Retro- & Moderne Designs
*Enthält Studio Dark, Studio Light, CRT-Phosphor (Bernstein/Grün/Cyan), Retro-Konsolen (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) und Designer-Themes.*
<p align="center">
  <img src="screenshots/themes.png" alt="Themen-Menü" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT Bernstein Thema" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX Thema" width="300" />
</p>

<br/><br/>

### 13. Menüsystem & Integrierte Tastaturhilfe
<p align="center">
  <img src="screenshots/file_menu.png" alt="Datei-Menü" width="180" />
  <img src="screenshots/edit_menu.png" alt="Bearbeiten-Menü" width="180" />
  <img src="screenshots/select_menu.png" alt="Auswahl-Menü" width="180" />
  <img src="screenshots/view_menu.png" alt="Ansicht-Menü" width="180" />
  <img src="screenshots/project_menu.png" alt="Projekt-Menü" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Tastaturkürzel-Hilfe-Dialog" width="700" />
</p>

</div>

---

## Vollständige Werkzeugreferenz

OniAnvil bietet 15 Präzisionswerkzeuge für Zeichnen, Geometrie, Auswahl und Metadaten. Die Symbole verwenden die quelloffene Icon-Suite **Font Awesome Free** *(Icons: CC BY 4.0, Fonts: SIL OFL 1.1)*.

| Werkzeug | Icon | Hotkey | Pinselgröße | Pixel-Perfekt | Beschreibung |
|:---|:---:|:---:|:---:|:---:|:---|
| **Bleistift** | ✏️ | <kbd>B</kbd> | 1..32 px | Ja | Primäres Zeichenwerkzeug. Der 1px-Modus unterstützt **Pixel-Perfekt** (eliminiert automatisch unschöne L-förmige Treppen-Ecken). Unterstützt eigene Auswahl-Stempelpinsel (`Ctrl/Cmd+B`). |
| **Radierer** | 🧹 | <kbd>E</kbd> | 1..32 px | Nein | Löscht Pixel zurück zu voller Transparenz (`[0, 0, 0, 0]`). Bei aktiver Auswahlmaske strikt auf diese begrenzt. |
| **Pipette** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Nein | Linksklick nimmt die Farbe als Vordergrund (Primary) auf; Rechtsklick als Hintergrund (Secondary). Kann von Canvas-Pixeln, Ebenen oder der Referenzskizze aufnehmen. |
| **Füllwerkzeug** | 🪣 | <kbd>G</kbd> | N/A | Nein | Blitzschnelle warteschlangenbasierte BFS-Füllung mit einstellbarer Farbtoleranz (`0..=255`). Unterstützt zusammenhängendes oder globales Füllen sowie Auswahlbegrenzung. |
| **Farbverlauf** | 🌈 | <kbd>G</kbd> *(Umsch.)* | N/A | Nein | Lineare und radiale Verläufe zwischen Vorder- und Hintergrundfarbe. Unterstützt Bayer- (2x2, 4x4, 8x8) und Blue-Noise-Dithering mit Stärkeregelung. |
| **Linie** | 📏 | <kbd>L</kbd> | 1..32 px | Ja | Bresenham-Linienalgorithmus mit Live-Vorschau. Halten Sie <kbd>Shift</kbd>, um Winkel auf 45°/90°-Schritte einzurasten. |
| **Kurve** | 〰️ | <kbd>C</kbd> | 1..32 px | Ja | Quadratische Bézierkurve mit 3 Klicks: Klick 1 setzt den Startpunkt, Klick 2 den Endpunkt, Mausbewegung biegt die Krümmung, Klick 3 bestätigt die Kurve. |
| **Rechteck** | ⬜ | <kbd>U</kbd> | 1..32 px | Nein | Konturierte oder gefüllte geometrische Rechtecke. Halten Sie <kbd>Shift</kbd> gedrückt für ein gleichseitiges 1:1 Quadrat. |
| **Ellipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Nein | Konturierte oder gefüllte Ellipsen mittels Mittelpunkt-Rasterisierung. Halten Sie <kbd>Shift</kbd> für einen exakten Kreis. |
| **Dither-Pinsel** | 🏁 | <kbd>D</kbd> | 1..32 px | Nein | Malt abwechselnde Schachbrett-Dithering-Muster zwischen Vorder- und Hintergrundfarbe, ohne zusätzliche Palettenfarben einzufügen. |
| **Schattierung** | 🌓 | <kbd>S</kbd> | 1..32 px | Nein | Schrittweises Aufhellen (Dodge) und Abdunkeln (Burn) basierend auf der Reihenfolge der aktiven Farbpalette. |
| **Rechteckauswahl** | ⬚ | <kbd>M</kbd> | N/A | Nein | Rechteckige Bereichsauswahl mit animierter Marching-Ants-Begrenzung. Unterstützt Hinzufügen (<kbd>Shift</kbd>) und Abziehen (<kbd>Alt</kbd>). |
| **Lasso-Auswahl** | ➰ | <kbd>Q</kbd> | N/A | Nein | Freihand-Polygon-Lasso. Nutzt 8-fach zusammenhängende Grenzrasterung mit 4-facher Außenflutung: garantiert 100% geschlossene, lückenlose Auswahlen auch bei Schleifen. |
| **Zauberstab** | 🪄 | <kbd>W</kbd> | N/A | Nein | Wählt zusammenhängende (oder globale) Bereiche übereinstimmender Farbe mit anpassbarem Toleranz-Schwellenwert aus. |
| **Dreh- & Aktionspunkte** | 🎯 | <kbd>P</kbd> | N/A | Nein | Interaktive Platzierung des Sprite-Ursprungs (Pivot) und benutzerdefinierter Gameplay-Aktionspunkte (`weapon`, `head`, `feet`, `hitbox`) mit JSON-Export. |

---

## Funktionale Dokumentation

### 1. Duale atlaszentrierte Architektur

In OniAnvil repräsentiert der Master-Texturatlas (`atlas_rgba`) die tatsächliche Spritesheet-Textur, wie sie von Ihrer Spiel-Engine geladen wird:
- **Einzelbild-Modus (Frame Mode)**: Konzentriert sich auf ein einzelnes aktives Animationsframe für ablenkungsfreies Arbeiten. Bearbeitungen aktualisieren sofort die entsprechende Zelle in der Atlas-Textur.
- **Atlas-Modus (Atlas Mode)**: Zeigt das gesamte Spritesheet-Gitter an und ermöglicht Makro-Anpassungen, Frame-übergreifende Ausrichtungsprüfungen und Stapelbearbeitungen.
- **Begleitdatei (Companion File)**: Projekte speichern saubere `.png`-Spritesheets zusammen mit einer kompakten `.onianvil.json`-Begleitdatei, welche Einzelbild-Layouts, Ebenenhierarchien, Tags, Haltezeiten und Paletten-Skins enthält.

### 2. Mehrspurige Ebenen-Zeitleiste

Die untere Zeitleiste ist an professionelle nichtlineare Videoschnittprogramme (NLE) angelehnt:
- **Ebenenspuren (`V1, V2, V3...`)**: Ebenen sind visuell von oben nach unten gestapelt. Die in der Zeitleiste höher platzierte Spur (`V1`) wird im Bildcanvas zuoberst dargestellt.
- **Spur-Bedienelemente**:
  - Klicken Sie auf die Plaketten `V1` / `V2`, um schnell die aktive Zeichenebene zu wechseln.
  - Ebenensichtbarkeit (Auge-Symbol) und Bearbeitungssperre (Schloss-Symbol) umschalten.
  - Individuelle Ebenendeckkraft anpassen (`0%..=100%`).
  - Ebene in der Hierarchie nach oben/unten verschieben, duplizieren und nach unten zusammenführen (Merge Down).
  - Ebene mit Sicherheits-Bestätigungsdialog löschen.
- **Frame-Clips**:
  - Haltezeit-Multiplikator (Hold): Weisen Sie benutzerdefinierte Bilddauern zu (z. B. `2×`, `3×` Haltezeit für Pausen oder Trefferposen).
  - Verknüpfte Frames (`🔗`): Verknüpfen Sie wiederkehrende Frames in der Animation; Bearbeitungen an einem Exemplar aktualisieren automatisch alle verknüpften Instanzen.
  - Tag-Schleifen: Organisieren Sie Animationen in benannten Tags (`idle`, `walk`, `attack`) und spielen Sie bestimmte Tags in Schleife ab.
  - Wiedergabesteuerung: Vorwärts, Rückwärts, Ping-Pong und Endlosschleife mit anpassbarer Bildrate (`1..=120` FPS).

### 3. Echtzeit-Animations-Live-Vorschau

Das rechte Vorschau-Dock spielt Ihre Animationen in Echtzeit ab, völlig unabhängig vom Zoomfaktor oder der Scrollposition Ihrer Arbeitsfläche:
- **Tag-Filterung**: Isolieren Sie die Wiedergabe auf bestimmte Tags (`walk`, `jump`) oder begutachten Sie die gesamte Animationssequenz.
- **Wiedergabesteuerung**: Wiedergabe, Pause, Einzelschritt zurück, Einzelschritt vor, Schleifenumschaltung und benutzerdefinierte Wiedergabegeschwindigkeiten (`0.25x..=4.0x`).
- **Anzeigemodi**: Schalten Sie flexibel zwischen dunklem Schachbrett, hellem Schachbrett, reinem Weiß, reinem Schwarz oder benutzerdefinierten Hintergrundfarben um.

### 4. Fortschrittliche Onion-Skinning-Engine

Gestalten Sie flüssige Animationen mit dem dedizierten Zwiebelhaut-System:
- **Benutzerdefinierte Farbgebung**: Unabhängige Tönungsfarben für vorherige Frames (Standard: Rot) und nachfolgende Frames (Standard: Blau).
- **Deckkraft-Abfall**: Konfigurierbare Basis-Deckkraft und schrittweise Dämpfungskurven für aufeinanderfolgende Geisterbilder.
- **Schleifenumlauf (Wrap-Around)**: Aktiviert Geisterbilder über das erste und letzte Einzelbild bei geschlossenen Animationsschleifen hinweg.
- **Schnellansicht (Quick-Peek)**: Halten Sie die <kbd>Leertaste</kbd> gedrückt (ohne zu ziehen), um sofort angrenzende Frames in der Vorschau einzublenden.

### 5. Intelligente Referenz- & Skizzen-Pauspause

Beschleunigen Sie Ihren Arbeitsablauf, indem Sie handgezeichnete Skizzen oder Referenzfotos nachzeichnen:
- **Formate**: Laden Sie PNG-, JPEG- und WebP-Referenzbilder.
- **Überblendungsmodi**: Unterlage (unterhalb Ihrer Pixelebenen) oder Überlagerung (oberhalb Ihrer Pixelebenen).
- **Transformationssteuerung**: Unabhängiger Zoom, Positionsversatz, Drehung und Deckkraft-Schieberegler.
- **Intelligente Farbaufnahme**: Verwenden Sie die Pipette auf der Skizzen-Ebene mit optionalem automatischen Einrasten auf die nächstliegende Farbe der aktiven Projektpalette.
- **1-Klick-Einbrennen (Bake)**: Verschmelzen Sie das skalierte Referenzbild mit einem Klick direkt als Pixel-Art in das aktive Frame.

### 6. Präzisions-Auswahlsuite & Marching Ants

- **Animierte Marching Ants**: Aktive Auswahlen heben sich durch animierte, kontrastreiche, zweifarbige gestrichelte Umrandungen deutlich ab.
- **Flutfüllungs-Lasso-Auswahl**: Im Gegensatz zu einfachen Ray-Casting-Algorithmen, die bei überlappenden Schleifen Aussparungen hinterlassen, rastert OniAnvils Lasso eine geschlossene 8-fach zusammenhängende Begrenzung und flutet den Außenraum – für 100% solide, lückenlose Auswahlen.
- **Auswahloperationen**: Erweitern, Verkleinern, Rand/Rahmen (mit Kreis-, Quadrat- und Rautenformen), Verschieben, Invertieren, Löschen, Freies Drehen sowie System-Zwischenablage Kopieren/Ausschneiden/Einfügen.
- **Frame-übergreifendes Kopieren & Einfügen**: Wählen Sie Bereiche auf einem Bild aus, drücken Sie <kbd>Ctrl/Cmd+C</kbd>, wechseln Sie den Frame und drücken Sie <kbd>Ctrl/Cmd+V</kbd>, um Pixel an exakt identischen Relativkoordinaten einzufügen – unabhängig vom Tastaturlayout.

### 7. Live-Effekte, Filter & RotSprite

Alle Bildeffekte verfügen über ein Echtzeit-Kontrollkästchen **Live-Vorschau**, das Modifikationen direkt auf der Leinwand anzeigt und ein nahtloses Abbrechen ohne Belastung des Rückgängig-Verlaufs erlaubt:
- **Farbe ersetzen (`Shift+R`)**:
  - Interaktive Pipette: Klicken Sie auf das Pipettensymbol und wählen Sie Farben direkt aus Canvas-Pixeln oder der Palettenleiste.
  - Schnellauswahl-Palettenleiste: Linksklick wählt die Ausgangsfarbe („Von“), Rechtsklick die Zielfarbe („Nach“).
  - Toleranzregler und Stapelersetzung über alle Animations-Frames hinweg.
- **Farbanpassung (`Ctrl/Cmd+U`)**: Echtzeit-HSV-Farbtonverschiebung (Hue), Sättigung (Saturation), Helligkeit (Value) und Kontrastanpassung.
- **Auto-Umriss-Generator**: Erstellen Sie gestochen scharfe Umrisse (Außen, Innen, Zentriert) mit eckigen (Quadrat) oder abgeschrägten (Raute) Ecken.
- **3×3-Faltungsfilter**: Weichzeichnen (Blur), Schärfen (Sharpen), Kantenerkennung (Edge Detect) und Relief (Emboss) mit Erhaltung des Alphakanals.
- **Freie Drehung (`Ctrl/Cmd+Shift+R`)**: Drehen Sie ausgewählte Bereiche oder ganze Frames um beliebige Winkel mit den klassischen Algorithmen **RotSprite** oder **Nearest Neighbor**.
- **Auf Palette abbilden**: Quantisieren Sie Canvas-Farben in die aktive Palette mit Bayer-Dithering (2x2, 4x4, 8x8) oder Blue Noise.

### 8. Palettenverwaltung, Farbschnitt & Charakter-Skins

- **Integrierte Paletten**: Über 20 authentische Retro-Paletten, darunter Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized und Sweetie 16.
- **Import & Export**: Unterstützung für GIMP-Paletten (`.gpl`), PAL (`.pal`), Hex-Paletten (`.hex`) und Bildfarbfelder (`.png`).
- **Palettentausch (Skins)**: Definieren Sie mehrere Farbvarianten für einen einzelnen Charakter (z. B. Standard, Goldene Rüstung, Schatten, Frost). Das Umschalten von Skins ordnet alle Atlas-Farben blitzschnell neu zu.
- **Farbverlaufs-Generator (Ramps)**: Automatisches Interpolieren weicher Farbverläufe zwischen zwei Farbtönen mit frei einstellbarer Schrittanzahl.

### 9. Sheet-Slicer, 9-Grid-UI & Game-Engine-Export

- **Sheet-Slicer**: Autokorrelations-Grenzerkennung schneidet rohe Spritesheets automatisch in gleichmäßige oder passgenau umrahmte Animations-Frames.
- **9-Grid (9-Slice) Skalierung**: Legen Sie anpassbare Ränder fest (oben, unten, links, rechts), um dehnbare Benutzeroberflächen-Boxen und Schaltflächen vorschaugenau zu prüfen.
- **PNG-Export mit extrudierten Rändern**: Streckt Randpixel in transparente Pufferzonen, wodurch Texturfilter-Artefakte und Farbbluten in OpenGL, DirectX, Vulkan und 2D-Engines (wie Godot) zuverlässig vermieden werden.
- **Animierter GIF-Export**: Generieren Sie optimierte animierte GIFs mit benutzerdefinierten Schleifenwiederholungen, Frame-Verzögerungen und Transparenz.
- **Standardmäßiges Spritesheet-JSON**: Exportiert Frame-Abschnitte, Tags, Bilddauern und Drehpunktkoordinaten passend zum Standard-Spritesheet-JSON-Schema moderner 2D-Game-Engines.

### 10. 35 ergonomische Designs & zweisprachige Lokalisierung

- **Kuratierte Designs**: Studio Dark, Studio Light, Tiefschwarz, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Bernstein-CRT, Grün-CRT, Cyan-CRT, Retro-Konsolen (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) und Retro-Computer (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Zweisprachige Unterstützung**: Nahtloser Wechsel zwischen **English** und **Deutsch** (sowie weiteren Sprachen) mit 100%iger Vollständigkeit aller Übersetzungsschlüssel.

---

## Steuerung & Tastenkombinationen

| Tastenkombination | Werkzeug / Aktion | Beschreibung |
|:---|:---|:---|
| <kbd>B</kbd> | **Bleistift** | 1px Präzisionszeichnung mit optionalem Pixel-Perfekt-Modus |
| <kbd>E</kbd> | **Radierer** | Pixel zurück zur Transparenz löschen |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipette** | Farbe von Canvas-Pixeln oder Referenzskizze aufnehmen |
| <kbd>G</kbd> | **Füllwerkzeug / Farbverlauf** | Umschalten zwischen Flutfüllung und Verlaufs-Dithering |
| <kbd>L</kbd> | **Linie** | Gerade Linien mit Live-Vorschau und Shift-Winkeleinrastung |
| <kbd>C</kbd> | **Kurve** | 3-Klick quadratisches Bézierkurven-Werkzeug |
| <kbd>U</kbd> | **Rechteck** | Geometrisches Rechteck und Quadrat (Kontur/Gefüllt) |
| <kbd>O</kbd> | **Ellipse** | Geometrische Ellipse und Kreis (Kontur/Gefüllt) |
| <kbd>D</kbd> | **Dither-Pinsel** | Schachbrettartige Dithering-Muster zeichnen |
| <kbd>S</kbd> | **Schattierung** | Schrittweises Aufhellen (Dodge) / Abdunkeln (Burn) entlang der Palette |
| <kbd>M</kbd> | **Rechteckauswahl** | Rechteckige Flächenauswahl mit Marching Ants |
| <kbd>Q</kbd> | **Lasso-Auswahl** | Freihändige, geschlossene Polygon-Auswahl |
| <kbd>W</kbd> | **Zauberstab** | Farbbereiche nach Ähnlichkeit zusammenhängend auswählen |
| <kbd>P</kbd> | **Dreh- & Aktionspunkte** | Sprite-Ursprung und Gameplay-Aktionspunkte positionieren |
| <kbd>X</kbd> | **Farben tauschen** | Vorder- und Hintergrundfarbe vertauschen |
| <kbd>[</kbd> / <kbd>]</kbd> | **Pinselgröße** | Pinselgröße verringern / vergrößern (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Kopieren** | Ausgewählte Pixel in die Zwischenablage kopieren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Ausschneiden** | Ausgewählte Pixel in die Zwischenablage ausschneiden |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Einfügen** | Zwischenablage-Pixel in das aktive Frame einfügen |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Löschen** | Pixel innerhalb der aktiven Auswahl leeren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Alles auswählen** | Alle Pixel des aktiven Frames markieren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Auswahl aufheben** | Aktive Auswahlmaske verwerfen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Auswahl umkehren** | Aktive Auswahlmaske invertieren |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Frame duplizieren** | Aktives Frame in der Zeitleiste duplizieren |
| <kbd>Leertaste</kbd> | **Wiedergabe / Pause** | Animationswiedergabe in Zeitleiste starten/stoppen |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Farbe ersetzen** | Farbersetzungs-Dialog mit Live-Vorschau öffnen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Farbanpassung** | HSV-Farbton- / Sättigungs- / Kontrast-Dialog öffnen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Freies Drehen** | Dialog für freie Winkeldrehung öffnen (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Rückgängig** | Letzten Pinselstrich oder Werkzeugaktion rückgängig machen |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Wiederholen** | Zuvor rückgängig gemachte Aktion wiederherstellen |
| <kbd>Leertaste</kbd> + Ziehen / MMB | **Canvas verschieben** | Sanftes, unendliches Verschieben des Bildausschnitts |
| <kbd>Ctrl/Cmd</kbd> + Rad / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Pixelgenaues Zoomen zentriert am Mauszeiger |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Ansichtsmodus** | Umschalten zwischen Frame-Ansicht (<kbd>1</kbd>) und Atlas-Ansicht (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Canvas einpassen** | Aktives Frame automatisch im Arbeitsbereich zentrieren |
| <kbd>F1</kbd> | **Hilfe** | Dialog für Tastaturkürzel aufrufen |

---

## Erste Schritte

### Eigenständige Binärdateien

Laden Sie vorkompilierte, eigenständige Binärdateien direkt von der **[GitHub-Releases-Seite](https://github.com/rbm13s/OniAnvil/releases/latest)** herunter:

- **Windows**: Laden Sie `OniAnvil-windows-x86_64.zip` herunter, entpacken Sie das Archiv und starten Sie `OniAnvil.exe`.
- **Linux**: Laden Sie `OniAnvil-linux-x86_64.tar.gz` (portabel) herunter oder installieren Sie das Debian/Ubuntu-Paket `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Laden Sie `OniAnvil-macos-universal.zip` herunter, entpacken Sie es und ziehen Sie `OniAnvil.app` in Ihren Programme-Ordner. Die Universal-Binärdatei läuft nativ sowohl auf Apple Silicon (M1/M2/M3/M4) als auch auf Intel-Macs.
- **Web**: Direkt im Browser ausführen ohne Installation: **[WebAssembly-App starten](https://rbm13s.github.io/OniAnvil/app/?lang=de)**.
  > [!WARNING]
  > Der WebAssembly-Browser-Build besitzt einen **experimentellen** Status. Aufgrund von Browser-Sandbox-Restriktionen, WebGL/WebGPU-Unterschieden und Einschränkungen der Zwischenablage können im Vergleich zur nativen Desktop-Version kleinere Grafikfehler oder Leistungseinbußen auftreten. Für produktives Arbeiten werden native Desktop-Builds ausdrücklich empfohlen.

---

### Aus dem Quellcode kompilieren

#### Voraussetzungen
- **Rust Toolchain**: Installieren Sie Rust (Version 1.80+ empfohlen) über [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux-Build-Abhängigkeiten** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Native Desktop-Version kompilieren & ausführen
```bash
# Repository klonen
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Testsuite ausführen
cargo test

# Optimierte Release-Binärdatei erstellen und starten
cargo run --release
```

#### 2. WebAssembly-Version kompilieren (über Trunk)
```bash
# wasm32-Zielplattform und Trunk-Builder installieren
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Lokalen Entwicklungsserver mit Hot-Reload starten
trunk serve

# Optimierte Release-Dateien nach docs/app/ kompilieren
./build_wasm.sh        # Unter Linux/macOS
./build_wasm.ps1       # Unter Windows PowerShell
```

---

## Dateiformat & Begleitdateien-Architektur

OniAnvil verzichtet auf proprietäre Binärformate. Jedes Projekt besteht aus zwei sauberen, versionskontrollierbaren Dateien:

```
my_character/
├── my_character.png            # Standard-PNG-Spritesheet-Atlas (in jeder Engine oder jedem Bildbetrachter öffenbar)
└── my_character.onianvil.json   # Begleitendes Metadaten-Sidecar
```

Die Begleitdatei `.onianvil.json` speichert:
- Canvas-Rasterkonfiguration: Einzelbildabmessungen, Spaltenanzahl, Gesamtanzahl der Frames.
- Ebenenhierarchien: Spurnamen, Deckkraftwerte, Sichtbarkeits-Flags, Sperrzustände.
- Animations-Tags: Name, Start-/Endbild-Indizes, Wiedergaberichtung (Vorwärts, Rückwärts, Ping-Pong), Haltezeiten.
- Palettendefinitionen: Aktive Farbfelder, benannte Skin-Variationen, Index des aktiven Skins.
- Referenzskizzen-Status: Bildpfad, Skalierung, Positionsversatz, Drehung, Deckkraft, Ebenenmodus.
- Drehpunkte: Benutzerdefinierte Aktionspunkte, Waffenanker und Kollisionsmittelpunkte.

---

## Mitwirken & Danksagungen

Beiträge, Fehlerberichte und Funktionsvorschläge sind herzlich willkommen!
1. Forken Sie das Repository auf GitHub.
2. Erstellen Sie einen Feature-Branch: `git checkout -b feature/mein-neues-feature`.
3. Übernehmen Sie Ihre Änderungen: `git commit -am 'Neues Pixel-Werkzeug hinzufügen'`.
4. Pushen Sie den Branch: `git push origin feature/mein-neues-feature`.
5. Öffnen Sie einen Pull Request.

### Danksagungen
- Starter-Demo-Sprite **"Cat 2D Pixel Art"** erstellt von **xzany** ([itch.io-Projektseite](https://xzany.itch.io/cat-2d-pixel-art)).
- Entwickelt mit der fantastischen **[egui](https://github.com/emilk/egui)** GUI-Bibliothek und dem **[wgpu](https://wgpu.rs/)** Grafik-Ökosystem.
- UI-Iconografie bereitgestellt von **[Font Awesome Free](https://fontawesome.com)** von @fontawesome (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License).

---

## Lizenz & Haftungsausschluss

Dieses Projekt ist unter der **[MIT-Lizenz](../LICENSE)** lizenziert. Sie dürfen OniAnvil frei verwenden, modifizieren, verbreiten und in kommerzielle sowie private Spieleentwicklungsprojekte integrieren.

### Gewährleistungsausschluss & Haftungsbeschränkung
> [!IMPORTANT]
> **OniAnvil** wird **kostenlos** auf einer **„WIE BESEHEN“ („AS IS“)** und **„WIE VERFÜGBAR“ („AS AVAILABLE“)** Basis zur Verfügung gestellt, ohne jegliche ausdrückliche oder stillschweigende Gewährleistung oder Zusicherung irgendeiner Art.
> 
> Unter keinen Umständen haften die Urheber, Urheberrechtsinhaber oder Mitwirkenden für direkte, indirekte, zufällige, besondere, exemplarische Schäden oder Folgeschäden (einschließlich, aber nicht beschränkt auf Datenverlust, Dateibeschädigung, Hardwareausfälle, entgangenen Gewinn oder Betriebsunterbrechung), die auf irgendeine Weise aus der Nutzung oder der Unfähigkeit zur Nutzung dieser Software entstehen, selbst wenn auf die Möglichkeit solcher Schäden hingewiesen wurde. Sie tragen die alleinige Verantwortung und alle Risiken bei der Nutzung dieses Programms.

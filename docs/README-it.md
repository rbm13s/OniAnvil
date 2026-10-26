<div align="center">

<img src="assets/icon.jpg" alt="Logo OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio di animazione pixel art e onion skinning basato su atlanti pronti per il motore di gioco**

[![Compilazione e rilascio](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Download](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Lingue](https://img.shields.io/badge/Lingue-37_Localizzazioni-purple)](#10-35-temi-ergonomici-e-localizzazione-multilingue)
[![Piattaforma](https://img.shields.io/badge/Piattaforma-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#guida-introduttiva)
[![Web Demo](https://img.shields.io/badge/App_Online-Prova_nel_Browser_(Sperimentale)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=it)
[![Licenza: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Prova OniAnvil nel browser (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=it)** *(⚠️ Versione sperimentale: potrebbe presentare limitazioni grafiche e bug WebGL/WASM)* • **[🌐 Sito web ufficiale della documentazione](https://rbm13s.github.io/OniAnvil/it.html)**

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
**Italiano** •
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

## Indice dei contenuti

- [Introduzione](#introduzione)
- [Screenshot e tour visivo](#screenshot-e-tour-visivo)
- [Guida completa agli strumenti](#guida-completa-agli-strumenti)
- [Documentazione funzionale](#documentazione-funzionale)
  - [1. Architettura unificata incentrata sull'atlante](#1-architettura-unificata-incentrata-sullatlante)
  - [2. Sequenza temporale (Timeline) multilivello](#2-sequenza-temporale-timeline-multilivello)
  - [3. Finestra di anteprima di animazione in tempo reale](#3-finestra-di-anteprima-di-animazione-in-tempo-reale)
  - [4. Motore avanzato di Onion Skinning](#4-motore-avanzato-di-onion-skinning)
  - [5. Livello bozzetto di riferimento e ricalco intelligente](#5-livello-bozzetto-di-riferimento-e-ricalco-intelligente)
  - [6. Suite di selezione di precisione e linee tratteggiate animate](#6-suite-di-selezione-di-precisione-e-linee-tratteggiate-animate)
  - [7. Effetti in tempo reale, filtri e RotSprite](#7-effetti-in-tempo-reale-filtri-e-rotsprite)
  - [8. Gestione tavolozze, scambio colori e varianti (Skin)](#8-gestione-tavolozze-scambio-colori-e-varianti-skin)
  - [9. Ritaglio fogli sprite (Sheet Slicer), UI 9-Grid ed esportazione per motori di gioco](#9-ritaglio-fogli-sprite-sheet-slicer-ui-9-grid-ed-esportazione-per-motori-di-gioco)
  - [10. 35 temi ergonomici e localizzazione multilingue](#10-35-temi-ergonomici-e-localizzazione-multilingue)
- [Comandi e scorciatoie da tastiera](#comandi-e-scorciatoie-da-tastiera)
- [Guida introduttiva](#guida-introduttiva)
  - [Eseguibili precompilati](#eseguibili-precompilati)
  - [Compilazione dal codice sorgente](#compilazione-dal-codice-sorgente)
- [Formato dei file e architettura dei metadati](#formato-dei-file-e-architettura-dei-metadati)
- [Contribuzione e ringraziamenti](#contribuzione-e-ringraziamenti)
- [Licenza e limitazione di responsabilità](#licenza-e-limitazione-di-responsabilità)

---

## Introduzione

**OniAnvil** (*dall'unione di **Oni**on-Skinning (animazione a buccia di cipolla) + **Anvil** (incudine per forgiare sprite)*) è un editor di pixel art e una suite di animazione open source ad altissime prestazioni scritto in Rust. Funziona in modo nativo con accelerazione hardware su Windows, Linux e macOS, e come applicazione WebAssembly a installazione zero direttamente nei moderni browser web.

La maggior parte degli editor esistenti tratta l'animazione come una sequenza di file immagine sconnessi, costringendo gli sviluppatori a noiosi passaggi di esportazione, ritaglio e ricomposizione in fogli sprite. **OniAnvil ribalta questo paradigma:** la texture dell'atlante dello sprite sheet di gioco (`atlas_rgba`) è la sorgente primaria e veritiera di ogni dato. Le celle dei fotogrammi, le tracce dei livelli, i tag di animazione e i punti di collisione rimangono sempre perfettamente sincronizzati con l'atlante in modo non distruttivo.

### Perché scegliere OniAnvil?
- 🚀 **Prestazioni native eccezionali in Rust**: avvio istantaneo in meno di un millisecondo, rendering nitidissimo gestito da `wgpu` ed `egui 0.31`, scorrimento fluido di lunghe sequenze temporali senza alcun ritardo.
- 🎬 **Sequenza temporale multilivello**: tracce di livelli ispirate ai software di montaggio video professionale (`V1, V2, V3...`) con composizione dall'alto verso il basso, unione verso il basso, fotogrammi collegati e moltiplicatori di durata (hold).
- 🧅 **Onion Skinning di prima classe**: visualizzazione simultanea di più fotogrammi con tinteggiature indipendenti per fotogrammi precedenti/successivi, curve di opacità personalizzabili e supporto per loop continui.
- 🎨 **Varianti colore (Skin) istantanee per personaggi**: crea varianti giorno/notte, colori di squadra o effetti elementali tramite rimappatura istantanea sull'atlante con importazione ed esportazione di file GPL, PAL ed HEX.
- 📦 **Esportazione diretta per motori di gioco**: esporta file PNG scalati con estrusione dei bordi per eliminare sbavature di filtraggio della texture nei motori 2D (Godot, Bevy, Raylib, Defold), insieme a metadati JSON standard per sprite sheet e GIF animate.

---

## Screenshot e tour visivo

<div align="center">

### 1. Area di lavoro principale e tela dello studio
*Area di lavoro pixel art ad alte prestazioni con viewport ad accelerazione hardware, sequenza temporale multilivello, anteprima in tempo reale, campioni di tavolozza e barre degli strumenti di precisione.*
<img src="screenshots/main.png" alt="Area di lavoro principale di OniAnvil" width="950" />

<br/><br/>

### 2. Sequenza temporale di animazione multilivello
*Timeline non lineare con righe di livelli sovrapposte (`V1, V2...`), istanze di fotogrammi collegate (`🔗`), moltiplicatori di durata (hold) e tag di animazione nominati con codice colore.*
<img src="screenshots/timeline.png" alt="Sequenza temporale multilivello di OniAnvil" width="950" />

<br/><br/>

### 3. Strumenti di precisione e barra delle azioni rapide
*Suite completa di 15 strumenti di disegno con modalità di ancoraggio standard e compatta, affiancata dalla barra superiore per la regolazione di dimensione del pennello, simmetria, ripetizione ai bordi e disegno Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Barra delle azioni di OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Strumenti OniAnvil Standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Strumenti OniAnvil Compatti" width="300" />
</p>

<br/><br/>

### 4. Anteprima animata in tempo reale
*Pannello di riproduzione dedicato con filtraggio dei tag, controllo della velocità FPS, cursore di zoom e selezione dello sfondo della finestra (a scacchiera, scuro a tinta unita, chiaro a tinta unita).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Anteprima dal vivo di OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zoom anteprima dal vivo di OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Gestione tavolozze, scambio colori e rampe gradiente
*Motore cromatico avanzato con riquadri campione per Primo piano/Sfondo, generazione automatica di rampe di colore interpolate tra due punti e scambio rapido di varianti skin della tavolozza.*
<img src="screenshots/palette_and_swaps.png" alt="Tavolozza e varianti colore di OniAnvil" width="950" />

<br/><br/>

### 6. Ricalco bozzetti di riferimento e Onion Skinning configurabile
*Carica bozzetti di concept o fotografie come sottofondo/sovrapposizione con controlli di trasformazione, campionamento intelligente del colore con ancoraggio alla tavolozza e ghosting multifotogramma.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Bozzetto e Onion Skinning in OniAnvil" width="950" />

<br/><br/>

### 7. Punti di ancoraggio (Pivot) e punti di azione di gioco
*Definisci il centro di origine dello sprite e punti di gioco personalizzati nominati (punti di ancoraggio delle armi, bocche da fuoco per proiettili, testa, passi) con indicatori interattivi sulla tela.*
<img src="screenshots/pivot_and_action_points.png" alt="Punti di ancoraggio e azione di OniAnvil" width="950" />

<br/><br/>

### 8. Ritaglio fogli sprite (Sheet Slicer) e ridimensionamento 9-Grid (9-Slice)
*Ritaglio automatico dei fotogrammi tramite autocorrelazione dei bordi dei pixel (ad es. ciclo di camminata di un gatto) e configurazione dei margini per caselle UI scalabili a 9 sezioni.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Ritaglio fogli sprite di OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Ridimensionamento 9-Slice di OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Effetti immagine e finestre modali dei filtri in tempo reale (con anteprima dal vivo)
*Tutte le finestre di dialogo degli effetti integrano l'anteprima dal vivo sulla tela, contagocce interattivi e annullamento non distruttivo.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generatore di contorno" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Sostituisci colore" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Tonalità, saturazione e contrasto" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtri matrice" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Rimappa sulla tavolozza" width="460" />
</p>

<br/><br/>

### 10. Griglia della tela e modalità di sfondo
*Alterna rapidamente tra griglia dei pixel, griglia delle tessere, scacchiera scura, scacchiera chiara e sfondi personalizzati a tinta unita.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Griglia e scacchiera scura" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Scacchiera chiara senza griglia" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Sfondo a tinta unita personalizzato" width="300" />
</p>

<br/><br/>

### 11. Procedura guidata di creazione nuovo progetto
*Configura le dimensioni della tela del progetto, il numero di fotogrammi, la disposizione a griglia delle colonne e scegli tra oltre 20 autentiche tavolozze rétro all'avvio.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Creazione nuovo progetto" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Tavolozze per il nuovo progetto" width="460" />
</p>

<br/><br/>

### 12. 35 temi autentici rétro e moderni
*Include Studio Dark, Studio Light, fosfori CRT Ambra/Verde/Ciano, console storiche (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) e temi di design.*
<p align="center">
  <img src="screenshots/themes.png" alt="Menu dei temi" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Ambra" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PlayStation 1" width="300" />
</p>

<br/><br/>

### 13. Sistema di menu e guida rapida alle scorciatoie da tastiera
<p align="center">
  <img src="screenshots/file_menu.png" alt="Menu File" width="180" />
  <img src="screenshots/edit_menu.png" alt="Menu Modifica" width="180" />
  <img src="screenshots/select_menu.png" alt="Menu Selezione" width="180" />
  <img src="screenshots/view_menu.png" alt="Menu Visualizza" width="180" />
  <img src="screenshots/project_menu.png" alt="Menu Progetto" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Finestra della guida alle scorciatoie da tastiera" width="700" />
</p>

</div>

---

## Guida completa agli strumenti

OniAnvil include 15 strumenti di precisione per disegno, forme geometriche, selezione e metadati. Le icone sono fornite dal set di icone open source **Font Awesome Free** *(Icone: CC BY 4.0, Font: SIL OFL 1.1)*.

| Strumento | Icona | Scorciatoia | Dimensione | Pixel Perfect | Descrizione |
|:---|:---:|:---:|:---:|:---:|:---|
| **Matita** | ✏️ | <kbd>B</kbd> | 1..32 px | Sì | Strumento di disegno principale. In modalità 1px supporta il tracciamento **Pixel-Perfect** (rimuove automaticamente i fastidiosi angoli a L scalettati). Supporta timbri personalizzati generati dalla selezione (`Ctrl/Cmd+B`). |
| **Gomma** | 🧹 | <kbd>E</kbd> | 1..32 px | No | Cancella i pixel riportandoli alla trasparenza (`[0, 0, 0, 0]`). Se è attiva una selezione, agisce rigorosamente all'interno dei suoi confini. |
| **Contagocce** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | No | Clic sinistro campiona il colore in Primo piano (Foreground); Clic destro campiona nello Sfondo (Background). Campiona da pixel della tela, livelli o bozzetto di riferimento. |
| **Secchiello** | 🪣 | <kbd>G</kbd> | N/D | No | Riempimento ad alta velocità basato su coda BFS con tolleranza colore configurabile (`0..=255`). Supporta modalità Contigua o Globale e delimitazione da maschera di selezione. |
| **Sfumatura** | 🌈 | <kbd>G</kbd> *(alterna)* | N/D | No | Gradienti lineari e radiali tra colore primario e secondario. Supporta dithering a matrice di Bayer (2x2, 4x4, 8x8) e Blue Noise con regolazione dell'intensità. |
| **Linea** | 📏 | <kbd>L</kbd> | 1..32 px | Sì | Algoritmo di tracciamento rette di Bresenham con anteprima di trascinamento in tempo reale. Tieni premuto <kbd>Shift</kbd> per bloccare gli angoli con incrementi di 45°/90°. |
| **Curva** | 〰️ | <kbd>C</kbd> | 1..32 px | Sì | Curva di Bézier quadratica a 3 clic. Il clic 1 fissa il punto iniziale, il clic 2 imposta il punto finale, il movimento del mouse regola la curvatura e il clic 3 applica la curva. |
| **Rettangolo** | ⬜ | <kbd>U</kbd> | 1..32 px | No | Rettangoli geometrici profilati o pieni. Tieni premuto <kbd>Shift</kbd> per vincolare la proporzione a quadrato 1:1. |
| **Ellisse** | ⭕ | <kbd>O</kbd> | 1..32 px | No | Ellissi profilate o piene tramite rasterizzazione con punto medio. Tieni premuto <kbd>Shift</kbd> per vincolare la forma a cerchio perfetto. |
| **Pennello dithering** | 🏁 | <kbd>D</kbd> | 1..32 px | No | Dipinge retini a scacchiera ordinati alternando il colore di primo piano e di sfondo senza introdurre colori aggiuntivi nella tavolozza. |
| **Strumento ombreggiatura** | 🌓 | <kbd>S</kbd> | 1..32 px | No | Schiarimento (Dodge) e scorrimento verso tinte più scure (Burn) progressivi basati sull'ordinamento dei colori nella tavolozza attiva. |
| **Selezione rettangolare** | ⬚ | <kbd>M</kbd> | N/D | No | Selezione rettangolare con perimetro dinamico a linee tratteggiate animate (marching ants). Supporta Aggiungi (<kbd>Shift</kbd>) e Sottrai (<kbd>Alt</kbd>). |
| **Lazo** | ➰ | <kbd>Q</kbd> | N/D | No | Selezione poligonale a mano libera. Utilizza una rasterizzazione del contorno 8-connessa con riempimento dell'esterno 4-connesso, garantendo selezioni solide al 100% senza buchi interni anche negli incroci. |
| **Bacchetta magica** | 🪄 | <kbd>W</kbd> | N/D | No | Seleziona regioni contigue (o globali) di colore corrispondente con soglia di tolleranza regolabile. |
| **Punto di ancoraggio e azione** | 🎯 | <kbd>P</kbd> | N/D | No | Posizionamento interattivo sulla tela dell'origine dello sprite e di punti di azione di gioco personalizzati (`weapon`, `head`, `feet`, `hitbox`) esportati nei metadati JSON. |

---

## Documentazione funzionale

### 1. Architettura unificata incentrata sull'atlante

In OniAnvil, l'atlante di texture master (`atlas_rgba`) rappresenta l'effettiva texture dello sprite sheet così come viene caricata dal motore di gioco:
- **Modalità fotogramma**: si concentra su un singolo fotogramma attivo, offrendo una tela priva di distrazioni. Qualsiasi modifica aggiorna immediatamente la cella corrispondente nell'atlante.
- **Modalità atlante**: visualizza l'intera griglia dello sprite sheet, consentendo regolazioni globali, verifiche di allineamento tra fotogrammi e modifiche in serie.
- **File compagno**: i progetti salvano un file `.png` pulito accompagnato da un leggero file sidecar `.onianvil.json` contenente layout dei fotogrammi, gerarchie di livelli, tag, moltiplicatori di durata e varianti skin.

### 2. Sequenza temporale (Timeline) multilivello

La timeline inferiore è modellata sui software di montaggio video non lineare:
- **Tracce dei livelli (`V1, V2, V3...`)**: i livelli sono impilati visivamente dall'alto verso il basso. La traccia posizionata più in alto nella sequenza temporale (`V1`) viene renderizzata sopra le altre sulla tela.
- **Controlli di traccia**:
  - Fai clic sulle etichette `V1` / `V2` per cambiare rapidamente il livello di disegno attivo.
  - Attiva/disattiva la visibilità del livello (icona dell'occhio) e il blocco delle modifiche (icona del lucchetto).
  - Regola l'opacità dei singoli livelli (`0%..=100%`).
  - Sposta il livello verso l'alto/basso nella gerarchia, duplica il livello e uniscilo verso il basso con la traccia sottostante (Merge Down).
  - Elimina il livello con finestra di dialogo di conferma protetta.
- **Clip dei fotogrammi**:
  - Moltiplicatore di durata (Hold): assegna durate personalizzate ai fotogrammi (es. tenuta `2×`, `3×` per pause o impatti scenici).
  - Fotogrammi collegati: collega fotogrammi ripetuti nell'animazione in modo che la modifica di un'istanza aggiorni automaticamente tutti i fotogrammi collegati.
  - Loop per tag: organizza le animazioni in tag nominati (`idle`, `walk`, `attack`) e riproduci in loop tag specifici.
  - Controlli di riproduzione: Avanti, Indietro, Ping-Pong e Ripetizione continua con frequenza FPS regolabile (`1..=120`).

### 3. Finestra di anteprima di animazione in tempo reale

Il pannello dock Preview sul lato destro riproduce le animazioni in tempo reale, in modo completamente indipendente dallo zoom o dalla posizione di panoramica della tela principale:
- **Filtraggio dei tag**: isola la riproduzione su un tag specifico (`walk`, `jump`) o visualizza l'intera sequenza di animazione.
- **Controlli di riproduzione**: Riproduci, Pausa, Fotogramma precedente, Fotogramma successivo, Ripetizione ad anello e velocità personalizzate (`0.25x..=4.0x`).
- **Modalità di visualizzazione**: passa istantaneamente tra scacchiera scura, scacchiera chiara, bianco puro, nero puro o sfondi personalizzati.

### 4. Motore avanzato di Onion Skinning

Crea animazioni fluide e armoniose grazie al sistema dedicato di onion skinning (carta da lucido):
- **Tinteggiatura personalizzata**: colori di evidenziazione indipendenti per i fotogrammi precedenti (predefinito: rosso) e per i fotogrammi successivi (predefinito: blu).
- **Attenuazione dell'opacità**: curva di opacità iniziale ed attenuazione progressiva configurabili per ciascun passaggio.
- **Loop continuo**: visualizza in trasparenza il primo fotogramma guardando l'ultimo (e viceversa) durante la creazione di animazioni cicliche perfette.
- **Visualizzazione rapida (Quick-Peek)**: tieni premuto il tasto <kbd>Spazio</kbd> senza trascinare per sbirciare all'istante i fotogrammi adiacenti o il bozzetto.

### 5. Livello bozzetto di riferimento e ricalco intelligente

Accelera il flusso di lavoro ricalcando disegni fatti a mano o fotografie di riferimento:
- **Formati**: supporta immagini di riferimento PNG, JPEG e WebP.
- **Modalità di fusione**: Sottofondo (sotto tutti i livelli pixel) o Sovrapposizione (sopra la tela di disegno).
- **Controlli di trasformazione**: zoom indipendente, scostamento di posizione, rotazione e cursore di opacità.
- **Campionamento intelligente**: usa il contagocce sul bozzetto con ancoraggio automatico opzionale al colore più vicino presente nella tavolozza di progetto.
- **Conversione rapida (1-Click Bake)**: proietta e rasterizza con un solo clic l'immagine di riferimento direttamente in pixel art sul fotogramma attivo.

### 6. Suite di selezione di precisione e linee tratteggiate animate

- **Linee tratteggiate animate (Marching Ants)**: le selezioni attive presentano un bordo tratteggiato a due colori ad alto contrasto con animazione continua.
- **Selezione lazo solida con riempimento esterno**: a differenza degli algoritmi ray-casting che creano buchi vuoti quando si sovrappongono tratti manuali, il lazo di OniAnvil rasterizza un confine 8-connesso chiuso e riempie l'area esterna, garantendo selezioni al 100% piene e senza buchi.
- **Operazioni sulla selezione**: Espandi, Contrai, Bordo (con matrici a cerchio, quadrato o diamante), Trasla, Inverti, Elimina, Rotazione libera e Taglia/Copia/Incolla negli appunti di sistema.
- **Copia e incolla tra fotogrammi**: seleziona un'area su un fotogramma, premi <kbd>Ctrl/Cmd+C</kbd>, passa a un altro fotogramma e premi <kbd>Ctrl/Cmd+V</kbd> per incollare alle medesime coordinate relative, indipendentemente dal layout di tastiera.

### 7. Effetti in tempo reale, filtri e RotSprite

Tutti gli effetti includono una casella di controllo **Anteprima dal vivo** che mostra in tempo reale le modifiche sulla tela e permette di annullare senza sporcare lo storico delle modifiche:
- **Sostituisci colore (`Shift+R`)**:
  - Contagocce interattivo: seleziona i colori sorgente e di destinazione direttamente dai pixel del fotogramma o dal pannello della tavolozza.
  - Barra della tavolozza rapida: clic sinistro per il colore sorgente, clic destro per il colore di destinazione.
  - Cursore di tolleranza e sostituzione cumulativa su tutti i fotogrammi dell'animazione.
- **Regolazione colore (`Ctrl/Cmd+U`)**: variazione in tempo reale di tonalità HSV, saturazione, luminosità e contrasto.
- **Generatore automatico di contorno**: crea contorni perfetti a livello di pixel (Esterno, Interno, Al centro) con angoli squadrati o a diamante.
- **Filtri di convoluzione 3×3**: sfocatura, nitidezza, rilevamento bordi e rilievo (emboss) con preservazione del canale alfa.
- **Rotazione libera (`Ctrl/Cmd+R` / `Ctrl/Cmd+Shift+R`)**: ruota regioni selezionate o interi fotogrammi a qualsiasi angolo con gli algoritmi **RotSprite** o **Nearest Neighbor**.
- **Rimappa sulla tavolozza**: riduci i colori della tela adattandoli alla tavolozza attiva tramite dithering Bayer 2x2, 4x4, 8x8 o Blue Noise.

### 8. Gestione tavolozze, scambio colori e varianti (Skin)

- **Tavolozze integrate**: oltre 20 autentiche tavolozze rétro tra cui Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized e Sweetie 16.
- **Importazione ed esportazione**: supporto completo per file GIMP Palette (`.gpl`), PAL (`.pal`), elenchi esadecimali (`.hex`) e strisce di campioni grafici (`.png`).
- **Scambio tavolozze (Skin)**: definisci più varianti cromatiche per un singolo personaggio (es. Standard, Armatura d'oro, Ombra, Ghiaccio). Il cambio di skin rimappa all'istante tutti i colori dei fotogrammi nell'intero atlante.
- **Generatore di rampe di colore**: genera sfumature morbide e coerenti tra due tonalità selezionate con numero di passaggi intermedi configurabile.

### 9. Ritaglio fogli sprite (Sheet Slicer), UI 9-Grid ed esportazione per motori di gioco

- **Ritaglio fogli sprite (Sheet Slicer)**: il rilevamento dei contorni tramite autocorrelazione suddivide automaticamente fogli sprite compositi in fotogrammi di animazione omogenei.
- **Ridimensionamento a 9 sezioni (9-Slice)**: imposta margini di taglio personalizzati (superiore, inferiore, sinistro, destro) per visualizzare in anteprima caselle e pulsanti per interfacce scalabili.
- **Esportazione PNG con estrusione dei bordi**: duplica i colori dei pixel perimetrali nei margini trasparenti per eliminare artefatti di filtraggio e sbavature visive nei motori 2D basati su OpenGL, DirectX, Vulkan o Godot.
- **Esportazione GIF animate**: genera GIF ottimizzate con conteggio dei loop personalizzabile, ritardi tra fotogrammi e sfondo trasparente.
- **Metadati JSON standard per sprite sheet**: esporta coordinate dei riquadri, tag, durate e punti di ancoraggio in un formato JSON compatibile con i più diffusi motori di gioco moderni.

### 10. 35 temi ergonomici e localizzazione multilingue

- **Temi curati nei dettagli**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, fosfori CRT (Ambra, Verde, Ciano), console storiche (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) e computer d'epoca (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Interfaccia multilingue**: supporto completo per oltre 37 lingue con parità totale delle chiavi del dizionario e passaggio immediato.

---

## Comandi e scorciatoie da tastiera

| Scorciatoia | Strumento / Azione | Descrizione |
|:---|:---|:---|
| <kbd>B</kbd> | **Matita** | Disegno di precisione a 1px con modalità opzionale Pixel-Perfect |
| <kbd>E</kbd> | **Gomma** | Cancella i pixel ripristinando la trasparenza |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Contagocce** | Campiona il colore dalla tela o dal bozzetto di riferimento |
| <kbd>G</kbd> | **Secchiello / Sfumatura** | Alterna tra secchiello di riempimento e sfumatura con dithering |
| <kbd>L</kbd> | **Linea** | Traccia linee rette con anteprima e vincolo angoli con Shift |
| <kbd>C</kbd> | **Curva** | Strumento curva quadratica di Bézier a 3 clic |
| <kbd>U</kbd> | **Rettangolo** | Rettangoli e quadrati geometrici con profilo o riempimento |
| <kbd>O</kbd> | **Ellisse** | Ellissi e cerchi geometrici con profilo o riempimento |
| <kbd>D</kbd> | **Pennello dithering** | Dipinge motivi a matrice con retino a scacchiera |
| <kbd>S</kbd> | **Strumento ombreggiatura** | Schiarisce (Dodge) o scurisce (Burn) lungo la rampa di tavolozza |
| <kbd>M</kbd> | **Selezione rettangolare** | Selezione rettangolare con perimetro a linee tratteggiate animate |
| <kbd>Q</kbd> | **Lazo** | Selezione poligonale a mano libera con riempimento pieno |
| <kbd>W</kbd> | **Bacchetta magica** | Seleziona aree continue di colore corrispondente |
| <kbd>P</kbd> | **Punti di ancoraggio (Pivot)** | Posiziona l'origine dello sprite e i punti di azione di gioco |
| <kbd>X</kbd> | **Scambia colori** | Scambia il colore di primo piano con quello di sfondo |
| <kbd>[</kbd> / <kbd>]</kbd> | **Dimensione pennello** | Riduci / Aumenta la dimensione del pennello (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copia** | Copia i pixel selezionati negli appunti di sistema |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Taglia** | Taglia i pixel selezionati negli appunti di sistema |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Incolla** | Incolla i pixel dagli appunti nel fotogramma attivo |
| <kbd>Canc</kbd> / <kbd>Backspace</kbd> | **Elimina** | Cancella i pixel all'interno della selezione attiva |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Seleziona tutto** | Seleziona l'intero fotogramma attivo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Deseleziona** | Rimuove la maschera di selezione attiva |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Inverti selezione** | Inverte la maschera di selezione attiva |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplica fotogramma** | Duplica il fotogramma attivo nella timeline |
| <kbd>Spazio</kbd> | **Riproduci / Pausa** | Avvia o ferma la riproduzione dell'animazione |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Sostituisci colore** | Apre la finestra di sostituzione colore con anteprima dal vivo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Regolazione colore** | Apre la finestra di regolazione HSV e contrasto |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Rotazione libera** | Apre la finestra di rotazione libera (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Annulla** | Annulla l'ultima operazione effettuata sulla tela |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Ripeti** | Ripristina l'azione precedentemente annullata |
| <kbd>Spazio</kbd> + Trascina / Tasto centrale | **Panoramica tela** | Scorrimento continuo lungo le coordinate della tela |
| <kbd>Ctrl/Cmd</kbd> + Rotellina / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Zoom nitido al pixel centrato sulla posizione del cursore |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Modalità vista** | Passa tra Vista Fotogramma (<kbd>1</kbd>) e Vista Atlante (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Adatta tela** | Centra e adatta automaticamente il fotogramma allo schermo |
| <kbd>F1</kbd> | **Guida** | Apre la finestra con il riepilogo delle scorciatoie da tastiera |

---

## Guida introduttiva

### Eseguibili precompilati

Scarica i pacchetti binari autonomi precompilati direttamente dalla **[pagina delle versioni GitHub](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: scarica `OniAnvil-windows-x86_64.zip`, estrai l'archivio e fai doppio clic su `OniAnvil.exe`.
- **Linux**: scarica l'archivio `OniAnvil-linux-x86_64.tar.gz` (portatile) oppure installa il pacchetto Debian/Ubuntu `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: scarica `OniAnvil-macos-universal.zip`, estrai il file e trascina `OniAnvil.app` nella cartella Applicazioni. Il binario universale funziona in modo nativo sia su architetture Apple Silicon (M1/M2/M3/M4) sia su Mac con processori Intel.
- **Web**: eseguilo subito nel browser senza dover installare alcunché: **[Avvia l'applicazione WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=it)**.
  > [!WARNING]
  > La versione WebAssembly per browser ha uno stato **Sperimentale**. A causa delle restrizioni della sandbox del browser, delle differenze tra i driver grafici WebGL/WebGPU e dell'accesso agli appunti, potrebbero verificarsi anomalie grafiche o rallentamenti rispetto alla versione nativa per computer. Per un lavoro di produzione continuativo, si raccomanda l'uso delle versioni desktop native.

---

### Compilazione dal codice sorgente

#### Prerequisiti
- **Toolchain Rust**: installa Rust (consigliata versione 1.80+) tramite [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dipendenze di compilazione per Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compila ed esegui la versione desktop nativa
```bash
# Clona il repository
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Esegui la suite di test
cargo test

# Compila ed avvia l'eseguibile ottimizzato
cargo run --release
```

#### 2. Compila la versione WebAssembly (tramite Trunk)
```bash
# Installa il target di compilazione wasm32 e lo strumento Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Avvia il server di sviluppo locale con ricaricamento a caldo
trunk serve

# Compila i file di produzione ottimizzati nella cartella docs/app/
./build_wasm.sh        # Su sistemi Linux/macOS
./build_wasm.ps1       # Su Windows PowerShell
```

---

## Formato dei file e architettura dei metadati

OniAnvil rifiuta l'uso di formati binari proprietari e opachi. Ogni progetto è composto da due file trasparenti e facilmente tracciabili tramite sistemi di controllo versione:

```
my_character/
├── my_character.png            # Atlante sprite sheet standard PNG (apribile in qualsiasi motore o visualizzatore)
└── my_character.onianvil.json   # File sidecar dei metadati del progetto
```

Il file compagno `.onianvil.json` memorizza:
- Configurazione della griglia della tela: dimensioni dei fotogrammi, numero di colonne, conteggio totale dei fotogrammi.
- Gerarchia dei livelli: nomi delle tracce, livelli di opacità, flag di visibilità e stati di blocco.
- Tag di animazione: nome, indici dei fotogrammi iniziale e finale, direzione di riproduzione (Avanti, Indietro, Ping-Pong), durate personalizzate.
- Definizioni della tavolozza: colori attivi, varianti skin denominate, indice della skin attualmente selezionata.
- Stato del bozzetto di riferimento: percorso dell'immagine, scala, scostamento di traslazione, rotazione, opacità e modalità di livello.
- Punti di ancoraggio: punti di azione personalizzati, agganci per le armi e centri di collisione dello sprite.

---

## Contribuzione e ringraziamenti

Suggerimenti, segnalazioni di bug e contributi al codice sono sempre benvenuti!
1. Crea un fork del repository su GitHub.
2. Crea un ramo per la tua funzionalità: `git checkout -b feature/nuovo-strumento-pixel`.
3. Registra le tue modifiche: `git commit -am 'Aggiungi nuovo strumento pixel'`.
4. Invia le modifiche al tuo ramo: `git push origin feature/nuovo-strumento-pixel`.
5. Apri una Pull Request.

### Ringraziamenti
- Lo sprite dimostrativo iniziale **"Cat 2D Pixel Art"** è stato realizzato da **xzany** ([pagina del progetto su itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Realizzato con la fantastica libreria grafica GUI **[egui](https://github.com/emilk/egui)** e l'ecosistema grafico **[wgpu](https://wgpu.rs/)**.
- Icone dell'interfaccia fornite da **[Font Awesome Free](https://fontawesome.com)** di @fontawesome (Icone: CC BY 4.0, Font: SIL OFL 1.1, Codice: Licenza MIT).

---

## Licenza e limitazione di responsabilità

Questo progetto è rilasciato sotto i termini della **[Licenza MIT](../LICENSE)**. Sei libero di utilizzare, modificare, distribuire ed integrare OniAnvil in progetti di sviluppo di videogiochi commerciali o personali.

### Esclusione di garanzia e limitazione di responsabilità
> [!IMPORTANT]
> **OniAnvil** viene distribuito **gratuitamente** sulla base del principio **"COSÌ COM'È" ("AS IS")** e **"COME DISPONIBILE"**, senza garanzie o condizioni di alcun tipo, esplicite o implicite (incluse, a titolo esemplificativo, garanzie di commerciabilità, idoneità per uno scopo specifico o non violazione dei diritti altrui).
> 
> In nessun caso l'autore (o gli autori), i detentori del copyright o i collaboratori potranno essere ritenuti responsabili per qualsiasi danno diretto, indiretto, incidentale, speciale, esemplare o consequenziale (inclusi, senza limitazione, perdita di dati, danneggiamento di file di progetto, guasti hardware, mancati guadagni o interruzione dell'attività economica) derivante in qualsiasi modo dall'uso o dall'impossibilità di usare questo software, anche se informati della possibilità di tali danni. L'utente si assume ogni responsabilità e rischio derivanti dall'utilizzo del programma.

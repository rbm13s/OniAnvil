<div align="center">

<img src="assets/icon.jpg" alt="Logo OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Studio d'animation pixel art et pelure d'oignon (Onion-Skin) axé sur les atlas prêts pour le jeu**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Téléchargements](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Moteur de rendu](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Langues](https://img.shields.io/badge/Langues-37_Langues-purple)](#10-35-thèmes-ergonomiques-et-localisation-multilingue)
[![Plateformes](https://img.shields.io/badge/Plateformes-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#prise-en-main)
[![Démo Web](https://img.shields.io/badge/Application_en_ligne-Tester_dans_le_navigateur_(Expérimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=fr)
[![Licence : MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Tester OniAnvil dans le navigateur (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=fr)** *(⚠️ Version expérimentale : peut présenter des limitations et des bugs WebGL/WASM)* • **[🌐 Site officiel de documentation](https://rbm13s.github.io/OniAnvil/fr.html)**

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
**Français** •
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

## Sommaire

- [Introduction](#introduction)
- [Captures d'écran et visite visuelle](#captures-décran-et-visite-visuelle)
- [Guide complet des outils](#guide-complet-des-outils)
- [Documentation fonctionnelle](#documentation-fonctionnelle)
  - [1. Architecture double centrée sur l'atlas](#1-architecture-double-centrée-sur-latlas)
  - [2. Chronologie des calques multipiste](#2-chronologie-des-calques-multipiste)
  - [3. Aperçu de l'animation en direct et en temps réel](#3-aperçu-de-lanimation-en-direct-et-en-temps-réel)
  - [4. Moteur avancé de pelure d'oignon (Onion Skinning)](#4-moteur-avancé-de-pelure-doignon-onion-skinning)
  - [5. Calque intelligent de référence et de décalquage d'esquisse](#5-calque-intelligent-de-référence-et-de-décalquage-desquisse)
  - [6. Suite de sélection de précision et pointillés animés](#6-suite-de-sélection-de-précision-et-pointillés-animés)
  - [7. Effets en direct, filtres et RotSprite](#7-effets-en-direct-filtres-et-rotsprite)
  - [8. Gestion des palettes, substitution et skins de personnage](#8-gestion-des-palettes-substitution-et-skins-de-personnage)
  - [9. Découpeur de planche (Sheet Slicer), 9-Grid UI et export pour moteurs de jeu](#9-découpeur-de-planche-sheet-slicer-9-grid-ui-et-export-pour-moteurs-de-jeu)
  - [10. 35 thèmes ergonomiques et localisation multilingue](#10-35-thèmes-ergonomiques-et-localisation-multilingue)
- [Contrôles et raccourcis clavier](#contrôles-et-raccourcis-clavier)
- [Prise en main](#prise-en-main)
  - [Fichiers binaires autonomes](#fichiers-binaires-autonomes)
  - [Compilation à partir des sources](#compilation-à-partir-des-sources)
- [Format de fichier et architecture compagnon](#format-de-fichier-et-architecture-compagnon)
- [Contribution et remerciements](#contribution-et-remerciements)
- [Licence et exclusion de garantie](#licence-et-exclusion-de-garantie)

---

## Introduction

**OniAnvil** (*issu de **Oni**on-Skinning (pelure d'oignon) + **Anvil** (l'enclume pour forger des sprites)*) est un éditeur de pixel art et une suite d'animation open-source et haute performance conçu en Rust. Il fonctionne nativement avec accélération matérielle sur Windows, Linux et macOS, ainsi que directement dans les navigateurs modernes sous forme d'application WebAssembly sans aucune installation requise.

La majorité des éditeurs existants traitent l'animation comme un empilement de fichiers images déconnectés, forçant les développeurs à des opérations fastidieuses d'exportation, de découpage et de réassemblage en atlas. **OniAnvil inverse ce paradigme :** la texture atlas de la planche de sprites du jeu (`atlas_rgba`) est la source unique de vérité. Les cellules d'images (frames), les pistes de calques, les marqueurs d'animation (tags) et les points de pivot/collision restent synchronisés en permanence avec l'atlas de manière non destructive.

### Pourquoi OniAnvil ?
- 🚀 **Performances natives Rust ultra-rapides** : Démarrage instantané en moins d'une milliseconde, rendu net propulsé par `wgpu` et `egui 0.31`, fluidité absolue lors du défilement des longues chronologies.
- 🎬 **Chronologie des calques multipiste** : Pistes de calques inspirées des logiciels de montage vidéo non linéaire (`V1, V2, V3...`) avec composition du haut vers le bas, fusion vers le bas, images liées et durées de maintien personnalisables.
- 🧅 **Pelure d'oignon de premier ordre** : Rémanence multi-images (ghosting) avec teintes avant/arrière indépendantes, courbes d'atténuation de l'opacité configurables et bouclage d'animation continu.
- 🎨 **Variantes de palette instantanées (Skins)** : Créez des variantes jour/nuit, des couleurs d'équipe ou des apparences élémentaires grâce au remappage en direct de l'atlas et à l'import/export GPL, PAL et HEX.
- 📦 **Export direct pour moteurs de jeu** : Exportez des PNG mis à l'échelle avec extrusion des bordures pour éliminer les raccords de filtrage de texture dans les moteurs 2D (Godot, Bevy, Raylib, Defold), accompagnés de métadonnées JSON standard et de GIF animés.

---

## Captures d'écran et visite visuelle

<div align="center">

### 1. Espace de travail principal et zone de dessin
*Espace de travail pixel art haute performance avec canevas à accélération matérielle, chronologie multipiste, aperçu en direct en temps réel, nuancier de palette et barres d'outils de précision.*
<img src="screenshots/main.png" alt="Espace de travail principal OniAnvil" width="950" />

<br/><br/>

### 2. Chronologie d'animation multipiste
*Chronologie non linéaire avec empilement de pistes de calques (`V1, V2...`), instances d'images liées (`🔗`), multiplicateurs de maintien et marqueurs d'animation colorés avec nommage.*
<img src="screenshots/timeline.png" alt="Chronologie multipiste OniAnvil" width="950" />

<br/><br/>

### 3. Outils de précision et barre d'actions rapides
*Suite complète de 15 outils de dessin avec modes d'ancrage standard et compact, accompagnée de la barre d'actions supérieure pour la taille de pinceau, la symétrie, le bouclage des bords et le mode Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Barre d'actions OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Outils OniAnvil Standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Outils OniAnvil Compact" width="300" />
</p>

<br/><br/>

### 4. Aperçu de l'animation en direct et en temps réel
*Panneau d'aperçu dédié avec filtrage par marqueur (tag), contrôle de vitesse FPS, curseur de zoom et bascule d'arrière-plan (damier, fond sombre uni, fond clair uni).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Aperçu en direct OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zoom de l'aperçu en direct OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Gestion des palettes, substitution et rampes de couleurs
*Moteur chromatique complet avec boîtes de sélection Premier plan / Arrière-plan, génération automatique de dégradés bicolores et variantes de palettes nommées.*
<img src="screenshots/palette_and_swaps.png" alt="Palettes et variantes OniAnvil" width="950" />

<br/><br/>

### 6. Calque de décalquage d'esquisse et pelure d'oignon configurable
*Chargez des esquisses ou photographies de référence en calque sous-jacent ou supérieur avec réglages de transformation, pipette intelligente avec magnétisme vers la palette et rémanence multi-images.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Esquisse et pelure d'oignon OniAnvil" width="950" />

<br/><br/>

### 7. Points de pivot et points d'action de collision
*Définissez l'origine du sprite et des points de gameplay personnalisés (ancres d'armes, points d'émission de projectiles, tête, pas) avec gizmos interactifs sur le canevas.*
<img src="screenshots/pivot_and_action_points.png" alt="Points de pivot et d'action OniAnvil" width="950" />

<br/><br/>

### 8. Découpeur de planche (Sheet Slicer) et mise à l'échelle 9-Grid (9-Slice)
*Découpage automatique des contours par autocorrélation (ex. cycle de marche d'un chat) et configuration des marges pour interfaces modulaires extensibles en 9 zones.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Découpeur de planche OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Mise à l'échelle 9-Slice OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Effets et filtres d'image en temps réel (avec prévisualisation en direct)
*Toutes les boîtes de dialogue d'effets disposent d'un aperçu en direct sur le canevas, de pipettes interactives et d'une annulation sans altération de l'historique.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Générateur de contour" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Remplacement de couleur" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Teinte Saturation Contraste" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtres matriciels" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Remappage vers la palette" width="460" />
</p>

<br/><br/>

### 10. Grilles et modes d'arrière-plan du canevas
*Basculez facilement entre grille de pixels, grille de tuiles, damier sombre, damier clair et couleurs unies personnalisées.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Grille et damier sombre" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Damier clair sans grille" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Fond uni personnalisé" width="300" />
</p>

<br/><br/>

### 11. Assistant de création d'un nouveau projet
*Configurez les dimensions du canevas, le nombre d'images, la disposition en colonnes et choisissez parmi plus de 20 palettes rétro authentiques dès le démarrage.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Assistant nouveau projet" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Choix des palettes du projet" width="460" />
</p>

<br/><br/>

### 12. 35 thèmes rétro et modernes authentiques
*Comprend Studio Dark, Studio Light, phosphores CRT Ambre/Vert/Cyan, consoles rétro (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis/Mega Drive) et thèmes graphistes.*
<p align="center">
  <img src="screenshots/themes.png" alt="Menu des thèmes" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Thème CRT Ambre" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Thème PSX" width="300" />
</p>

<br/><br/>

### 13. Menu principal et aide intégrée aux raccourcis
<p align="center">
  <img src="screenshots/file_menu.png" alt="Menu Fichier" width="180" />
  <img src="screenshots/edit_menu.png" alt="Menu Édition" width="180" />
  <img src="screenshots/select_menu.png" alt="Menu Sélection" width="180" />
  <img src="screenshots/view_menu.png" alt="Menu Affichage" width="180" />
  <img src="screenshots/project_menu.png" alt="Menu Projet" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Fenêtre d'aide des raccourcis clavier" width="700" />
</p>

</div>

---

## Guide complet des outils

OniAnvil inclut 15 outils de précision pour le dessin, les formes géométriques, la sélection et la gestion des métadonnées. Les icônes proviennent de la suite open-source **Font Awesome Free** *(Icônes : CC BY 4.0, Polices : SIL OFL 1.1)*.

| Outil | Icône | Raccourci | Taille de pinceau | Pixel-Perfect | Description |
|:---|:---:|:---:|:---:|:---:|:---|
| **Crayon** | ✏️ | <kbd>B</kbd> | 1..32 px | Oui | Outil de dessin principal. Le mode 1px prend en charge le tracé **Pixel-Perfect** (élimine automatiquement les angles disgracieux en escalier en « L »). Permet d'utiliser les sélections comme pinceaux-tampons (`Ctrl/Cmd+B`). |
| **Gomme** | 🧹 | <kbd>E</kbd> | 1..32 px | Non | Efface les pixels jusqu'à la transparence complète (`[0, 0, 0, 0]`). Limitée à la zone de sélection si un masque est actif. |
| **Pipette** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Non | Le clic gauche prélève la couleur pour le Premier plan (PP) ; le clic droit pour l'Arrière-plan (AP). Échantillonne depuis le canevas, les calques ou l'esquisse de référence. |
| **Remplissage** | 🪣 | <kbd>G</kbd> | N/D | Non | Remplissage algorithmique haute vitesse basé sur une file d'attente (BFS) avec tolérance chromatique réglable (`0..=255`). Prend en charge les modes Contigu ou Global et le confinement à la sélection. |
| **Dégradé** | 🌈 | <kbd>G</kbd> *(bascule)* | N/D | Non | Dégradés linéaires et radiaux entre la couleur de premier plan et celle d'arrière-plan. Supporte le tramage matriciel Bayer (2x2, 4x4, 8x8) et Blue Noise avec intensité réglable. |
| **Ligne** | 📏 | <kbd>L</kbd> | 1..32 px | Oui | Tracé de lignes droites via l'algorithme de Bresenham avec prévisualisation en direct. Maintenir <kbd>Maj</kbd> verrouille l'angle par incréments de 45°/90°. |
| **Courbe** | 〰️ | <kbd>C</kbd> | 1..32 px | Oui | Courbe de Bézier quadratique en 3 clics. Le 1er clic définit le point de départ, le 2e le point d'arrivée, le mouvement de la souris ajuste la courbure et le 3e clic valide la courbe. |
| **Rectangle** | ⬜ | <kbd>U</kbd> | 1..32 px | Non | Rectangles géométriques pleins ou avec contour. Maintenir <kbd>Maj</kbd> contraint les proportions à un carré 1:1. |
| **Ellipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Non | Ellipses pleines ou avec contour par rastérisation au point médian. Maintenir <kbd>Maj</kbd> contraint la forme à un cercle parfait. |
| **Pinceau de tramage** | 🏁 | <kbd>D</kbd> | 1..32 px | Non | Peint avec des motifs de tramage ordonnés en damier entre les couleurs de premier plan et d'arrière-plan sans ajouter de couleurs supplémentaires à la palette. |
| **Outil d'ombrage** | 🌓 | <kbd>S</kbd> | 1..32 px | Non | Éclaircissement (Dodge) et assombrissement (Burn) progressifs basés sur l'ordre des couleurs dans la palette active. |
| **Sélection rectangulaire** | ⬚ | <kbd>M</kbd> | N/D | Non | Sélection d'une région rectangulaire délimitée par des pointillés animés (« marching ants »). Supporte l'addition (<kbd>Maj</kbd>) et la soustraction (<kbd>Alt</kbd>). |
| **Sélection lasso** | ➰ | <kbd>Q</kbd> | N/D | Non | Lasso polygonal à main levée. Utilise une rastérisation de contour 8-connectée avec remplissage extérieur 4-connecté, garantissant une sélection 100 % pleine, sans trous, même sur des boucles qui se croisent. |
| **Baguette magique** | 🪄 | <kbd>W</kbd> | N/D | Non | Sélectionne des zones contiguës (ou globales) de même couleur avec seuil de tolérance réglable. |
| **Point de pivot & d'action** | 🎯 | <kbd>P</kbd> | N/D | Non | Positionnement interactif sur le canevas de l'origine du sprite et des points d'action de jeu personnalisés (`weapon`, `head`, `feet`, `hitbox`) exportés dans les métadonnées JSON. |

---

## Documentation fonctionnelle

### 1. Architecture double centrée sur l'atlas

Dans OniAnvil, l'atlas de texture principal (`atlas_rgba`) représente la véritable texture de la planche de sprites telle qu'elle est chargée par votre moteur de jeu :
- **Mode Image (Frame Mode)** : Se concentre sur une seule image d'animation active pour un dessin sans distraction. Les modifications apportées sont immédiatement répercutées dans la cellule correspondante de la texture atlas.
- **Mode Atlas (Atlas Mode)** : Affiche la grille complète de la planche de sprites, permettant les ajustements macroscopiques, le contrôle de l'alignement entre images et les modifications par lot.
- **Fichier compagnon (Sidecar)** : Les projets enregistrent des planches de sprites en `.png` propre, accompagnées d'un fichier compagnon `.onianvil.json` léger contenant les dimensions des images, la hiérarchie des calques, les marqueurs (tags), les durées de maintien et les variantes de palettes.

### 2. Chronologie des calques multipiste

La chronologie inférieure est conçue sur le modèle des logiciels professionnels de montage vidéo non linéaire :
- **Pistes de calques (`V1, V2, V3...`)** : Les calques s'empilent visuellement de haut en bas. La piste située le plus haut dans la chronologie (`V1`) est dessinée au-dessus dans le canevas d'image.
- **Contrôles de piste** :
  - Cliquez sur les badges `V1` / `V2` pour changer instantanément de calque de dessin actif.
  - Basculez la visibilité du calque (icône d'œil) et le verrouillage contre les modifications (icône de cadenas).
  - Réglez l'opacité individuelle de chaque calque (`0%..=100%`).
  - Déplacez le calque vers le haut ou vers le bas dans la hiérarchie, dupliquez-le ou fusionnez-le vers le calque inférieur (Merge Down).
  - Supprimez un calque en toute sécurité grâce à une boîte de dialogue de confirmation.
- **Séquences d'images (Clips)** :
  - Multiplicateur de maintien (Hold) : attribuez des durées d'affichage personnalisées (ex. maintien `2×`, `3×` pour des temps d'arrêt ou des impacts).
  - Images liées : liez des images répétitives au fil de l'animation pour que la modification de l'une d'elles mette automatiquement à jour toutes les instances liées.
  - Bouclage par marqueur (Tag) : organisez vos animations en blocs nommés (`idle`, `walk`, `attack`) et bouclez sur des marqueurs spécifiques pendant la lecture.
  - Commandes de lecture : Lecture avant, arrière, va-et-vient (Ping-Pong) et boucle avec fréquence d'images réglable (`1..=120` FPS).

### 3. Aperçu de l'animation en direct et en temps réel

Le panneau d'aperçu dédié sur la droite joue vos animations en temps réel, indépendamment du niveau de zoom ou du défilement de votre canevas :
- **Filtrage par marqueur** : Isolez la lecture d'un marqueur spécifique (`walk`, `jump`) ou prévisualisez l'ensemble de la séquence d'animation.
- **Commandes de lecture** : Lecture, Pause, Image précédente, Image suivante, bascule de boucle et vitesses de lecture personnalisées (`0.25x..=4.0x`).
- **Modes d'affichage** : Basculez entre damier sombre, damier clair, blanc pur, noir pur ou arrière-plan uni personnalisé.

### 4. Moteur avancé de pelure d'oignon (Onion Skinning)

Façonnez des animations fluides grâce au système dédié de pelure d'oignon :
- **Teintes personnalisées** : Couleurs de rémanence indépendantes pour les images précédentes (par défaut : Rouge) et les images suivantes (par défaut : Bleu).
- **Courbe d'atténuation d'opacité** : Définissez une opacité de base et une courbe d'atténuation pas-à-pas personnalisables.
- **Bouclage continu** : Activez la rémanence entre la première et la dernière image pour le travail sur des boucles parfaites.
- **Aperçu rapide (Quick-Peek)** : Maintenez la touche <kbd>Espace</kbd> sans déplacer la souris pour prévisualiser instantanément les images adjacentes.

### 5. Calque intelligent de référence et de décalquage d'esquisse

Accélérez votre flux de travail en décalquant des esquisses faites à la main ou des photos de référence :
- **Formats pris en charge** : Importez des images de référence aux formats PNG, JPEG et WebP.
- **Modes de fusion** : Calque sous-jacent (sous vos calques de pixels) ou Calque supérieur (au-dessus de vos calques).
- **Outils de transformation** : Zoom indépendant, décalage de position, rotation et curseur d'opacité.
- **Échantillonnage intelligent des couleurs** : Utilisez la pipette sur le calque d'esquisse avec alignement automatique facultatif sur la couleur la plus proche de la palette active du projet.
- **Conversion en 1 clic (Bake)** : Aplatissez instantanément l'image de référence redimensionnée sous forme de pixel art sur l'image active.

### 6. Suite de sélection de précision et pointillés animés

- **Pointillés animés (Marching Ants)** : Les sélections actives sont mises en valeur par un périmètre animé bicolore à fort contraste.
- **Sélection lasso polygonale pleine** : Contrairement aux algorithmes naïfs de lancer de rayons qui créent des trous non sélectionnés lors du tracé de boucles qui se chevauchent, le lasso d'OniAnvil rastérise un contour 8-connecté fermé et remplit l'extérieur, garantissant une sélection 100 % pleine et sans trous intérieurs.
- **Opérations sur la sélection** : Dilater, Contracter, Bordure (avec formes de noyau Circulaire, Carrée ou Losange), Translation, Inverser, Supprimer, Rotation libre et Copier/Couper/Coller via le presse-papiers système.
- **Copier-coller entre images** : Sélectionnez une région sur une image, faites <kbd>Ctrl/Cmd+C</kbd>, changez d'image et faites <kbd>Ctrl/Cmd+V</kbd> pour coller exactement aux mêmes coordonnées relatives, quelle que soit la disposition de votre clavier.

### 7. Effets en direct, filtres et RotSprite

Toutes les fenêtres d'effets disposent d'une case à cocher **Aperçu en direct (Live Preview)** permettant de visualiser les modifications directement sur le canevas et d'annuler sans polluer l'historique :
- **Remplacer une couleur (`Shift+R`)** :
  - Pipette interactive : cliquez sur l'icône de pipette et prélevez des couleurs directement sur les pixels du canevas ou dans le dock de palette.
  - Bandeau de sélection rapide : clic gauche pour choisir la couleur source (« De »), clic droit pour la couleur cible (« Vers »).
  - Curseur de tolérance et remplacement groupé sur toutes les images de l'animation.
- **Ajustement des couleurs (`Ctrl/Cmd+U`)** : Décalage de la teinte HSV en temps réel, saturation, luminosité et réglage du contraste.
- **Générateur de contour automatique** : Créez des contours nets pour vos personnages (Extérieur, Intérieur, Centré) avec jointures d'angles carrées ou en losange.
- **Filtres de convolution 3×3** : Filtres de flou, de netteté, de détection de contours et de gaufrage avec préservation du canal alpha.
- **Rotation libre (`Ctrl/Cmd+Shift+R`)** : Faites pivoter les zones sélectionnées ou des images complètes à n'importe quel angle grâce aux algorithmes classiques **RotSprite** ou du **Plus proche voisin**.
- **Remappage vers la palette** : Adaptez les couleurs du canevas à la palette active en utilisant le tramage Bayer 2x2, 4x4, 8x8 ou Blue Noise.

### 8. Gestion des palettes, substitution et skins de personnage

- **Palettes intégrées** : Plus de 20 palettes rétro authentiques dont Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized et Sweetie 16.
- **Importation et exportation** : Prise en charge des formats GIMP Palette (`.gpl`), PAL (`.pal`), palettes hexadécimales (`.hex`) et nuanciers d'images (`.png`).
- **Substitution de palette (Skins)** : Définissez plusieurs variantes de couleurs pour un même personnage (ex. Défaut, Armure d'or, Ombre, Glace). L'alternance des skins remappe instantanément les couleurs sur l'ensemble de l'atlas.
- **Générateur de rampes de couleurs** : Interpolez automatiquement des dégradés harmonieux entre deux teintes avec un nombre d'étapes intermédiaires personnalisable.

### 9. Découpeur de planche (Sheet Slicer), 9-Grid UI et export pour moteurs de jeu

- **Découpeur de planche (Sheet Slicer)** : La détection de contours par autocorrélation découpe automatiquement les planches brutes en cellules régulières ou ajustées aux limites des sprites.
- **Mise à l'échelle 9-Grid (9-Slice)** : Définissez des marges de découpe personnalisables (haut, bas, gauche, droite) pour prévisualiser des boîtes d'interface et des boutons modulaires sans distorsion des coins.
- **Export PNG avec extrusion des bordures** : Prolonge les pixels de bordure dans les marges transparentes afin d'éliminer les artefacts de couture et de filtrage de texture dans OpenGL, DirectX, Vulkan et les moteurs de jeu 2D (tels que Godot).
- **Export GIF animé** : Générez des GIF animés optimisés avec nombre de boucles personnalisable, délais entre images et transparence.
- **JSON de planche de sprites standard** : Exporte les coordonnées des images, les marqueurs, les durées et les points de pivot au format JSON standard compatible avec les moteurs de jeu 2D modernes.

### 10. 35 thèmes ergonomiques et localisation multilingue

- **Thèmes soigneusement conçus** : Studio Dark, Studio Light, Noir absolu, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Phosphores CRT (Ambre, Vert, Cyan), Consoles rétro (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) et Ordinateurs rétro (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Localisation complète** : Basculez en un clic entre le **Français**, l'**Anglais** et 35 autres langues avec une parfaite conformité des clés du dictionnaire.

---

## Contrôles et raccourcis clavier

| Raccourci | Outil / Action | Description |
|:---|:---|:---|
| <kbd>B</kbd> | **Crayon** | Dessin de précision 1px avec mode Pixel-Perfect facultatif |
| <kbd>E</kbd> | **Gomme** | Efface les pixels jusqu'à la transparence |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Pipette** | Prélève la couleur sur le canevas ou l'esquisse de référence |
| <kbd>G</kbd> | **Remplissage / Dégradé** | Bascule entre le remplissage et le dégradé avec tramage |
| <kbd>L</kbd> | **Ligne** | Trace des lignes droites avec prévisualisation et alignement par Maj |
| <kbd>C</kbd> | **Courbe** | Outil de courbe de Bézier quadratique en 3 clics |
| <kbd>U</kbd> | **Rectangle** | Formes rectangulaires et carrées avec contour ou pleines |
| <kbd>O</kbd> | **Ellipse** | Formes elliptiques et circulaires avec contour ou pleines |
| <kbd>D</kbd> | **Pinceau de tramage** | Peint des motifs matriciels de tramage en damier |
| <kbd>S</kbd> | **Outil d'ombrage** | Éclaircissement / assombrissement progressif le long de la rampe de palette |
| <kbd>M</kbd> | **Sélection rectangulaire** | Sélection rectangulaire avec pointillés animés |
| <kbd>Q</kbd> | **Sélection lasso** | Sélection polygonale pleine à main levée |
| <kbd>W</kbd> | **Baguette magique** | Sélectionne les régions contiguës de même couleur |
| <kbd>P</kbd> | **Points de pivot & d'action** | Positionne l'origine du sprite et les points d'action de jeu |
| <kbd>X</kbd> | **Intervertir les couleurs** | Inverse la couleur de premier plan et celle d'arrière-plan |
| <kbd>[</kbd> / <kbd>]</kbd> | **Taille du pinceau** | Diminue / augmente la taille du pinceau (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copier** | Copie les pixels sélectionnés dans le presse-papiers système |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Couper** | Coupe les pixels sélectionnés dans le presse-papiers système |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Coller** | Colle les pixels du presse-papiers dans l'image active |
| <kbd>Suppr</kbd> / <kbd>Retour arrière</kbd> | **Supprimer** | Efface les pixels à l'intérieur de la sélection active |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Tout sélectionner** | Sélectionne tous les pixels de l'image active |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Échap</kbd> | **Désélectionner** | Efface le masque de sélection actif |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Maj</kbd> + <kbd>I</kbd> | **Inverser la sélection** | Inverse le masque de sélection actif |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Maj</kbd> + <kbd>D</kbd> | **Dupliquer l'image** | Duplique l'image active dans la chronologie |
| <kbd>Espace</kbd> | **Lecture / Pause** | Démarre ou suspend la lecture dans la chronologie |
| <kbd>Maj</kbd> + <kbd>R</kbd> | **Remplacer la couleur** | Ouvre la fenêtre de remplacement de couleur avec aperçu en direct |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Ajustement chromatique** | Ouvre la fenêtre de réglage HSV et de contraste |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Maj</kbd> + <kbd>R</kbd> | **Rotation libre** | Ouvre la fenêtre de rotation libre d'angle arbitraire (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Annuler** | Annule la dernière action ou le dernier trait sur le canevas |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Maj</kbd>+<kbd>Z</kbd> | **Rétablir** | Rétablit l'action précédemment annulée |
| <kbd>Espace</kbd> + Glisser / BMF | **Panoramique** | Déplacement fluide et infini sur les coordonnées du canevas |
| <kbd>Ctrl/Cmd</kbd> + Molette / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Zoom net au pixel centré sur le curseur de la souris |
| <kbd>Pavé 1</kbd> / <kbd>Pavé 2</kbd> | **Mode d'affichage** | Bascule entre la vue Image (<kbd>1</kbd>) et la vue Atlas (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Ajuster au canevas** | Centre et adapte automatiquement l'image active dans l'espace de travail |
| <kbd>F1</kbd> | **Aide** | Ouvre la fenêtre d'aide des raccourcis clavier |

---

## Prise en main

### Fichiers binaires autonomes

Téléchargez les binaires précompilés et autonomes depuis la **[page des versions GitHub](https://github.com/rbm13s/OniAnvil/releases/latest)** :

- **Windows** : Téléchargez `OniAnvil-windows-x86_64.zip`, décompressez et double-cliquez sur `OniAnvil.exe`.
- **Linux** : Téléchargez `OniAnvil-linux-x86_64.tar.gz` (portable) ou installez le paquet Debian/Ubuntu `onianvil_*_amd64.deb` :
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS** : Téléchargez `OniAnvil-macos-universal.zip`, décompressez et glissez `OniAnvil.app` dans votre dossier Applications. Le binaire universel fonctionne nativement sur Apple Silicon (M1/M2/M3/M4) et sur les Mac Intel.
- **Web** : Lancez l'application directement dans votre navigateur sans rien installer : **[Ouvrir l'application WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=fr)**.
  > [!WARNING]
  > La version WebAssembly (dans le navigateur) a le statut **Expérimental**. En raison des contraintes du bac à sable du navigateur, des variations d'implémentation de WebGL/WebGPU et des limitations d'accès au presse-papiers, vous pouvez rencontrer de légers artefacts graphiques ou des ralentissements par rapport à la version native pour ordinateur. Pour un travail en production, l'utilisation des versions de bureau natives est fortement recommandée.

---

### Compilation à partir des sources

#### Prérequis
- **Chaîne d'outils Rust** : Installez Rust (version 1.80+ recommandée) via [rustup.rs](https://rustup.rs/) :
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dépendances de compilation Linux** (Debian/Ubuntu) :
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compiler et exécuter la version de bureau native
```bash
# Cloner le dépôt
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Lancer la suite de tests
cargo test

# Compiler et lancer le binaire optimisé en mode release
cargo run --release
```

#### 2. Compiler la version WebAssembly (via Trunk)
```bash
# Ajouter la cible de compilation wasm32 et l'outil Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Démarrer le serveur de développement local avec rechargement à chaud
trunk serve

# Générer les fichiers optimisés pour la production dans docs/app/
./build_wasm.sh        # Sous Linux/macOS
./build_wasm.ps1       # Sous Windows PowerShell
```

---

## Format de fichier et architecture compagnon

OniAnvil n'utilise aucun format binaire propriétaire fermé. Chaque projet se compose de deux fichiers clairs et faciles à versionner :

```
mon_personnage/
├── mon_personnage.png            # Atlas de planche de sprites PNG standard (ouvrable dans n'importe quel moteur ou lecteur)
└── mon_personnage.onianvil.json   # Fichier compagnon de métadonnées
```

Le fichier compagnon `.onianvil.json` enregistre :
- La configuration de la grille du canevas : dimensions des images, nombre de colonnes, nombre total d'images.
- La hiérarchie des calques : noms des pistes, niveaux d'opacité, indicateurs de visibilité, états verrouillés.
- Les marqueurs d'animation (tags) : nom, indices des images de début et de fin, direction de lecture (Avant, Arrière, Ping-Pong), durées des images.
- Les définitions de palettes : échantillons de couleurs actifs, variantes de skins nommées, index du skin actif.
- L'état du calque d'esquisse de référence : chemin du fichier, échelle, décalage panoramique, rotation, opacité, mode de calque.
- Les points de pivot : points d'action personnalisés, ancres pour armes et centres de collision.

---

## Contribution et remerciements

Les contributions, signalements de bogues et propositions de fonctionnalités sont les bienvenus !
1. Forkez le dépôt sur GitHub.
2. Créez une branche dédiée à votre fonctionnalité : `git checkout -b feature/mon-nouvel-outil`.
3. Validez vos modifications : `git commit -am 'Ajout d'un nouvel outil pixel'`.
4. Poussez vers la branche : `git push origin feature/mon-nouvel-outil`.
5. Ouvrez une Pull Request.

### Remerciements
- Sprite de démonstration animé **"Cat 2D Pixel Art"** créé par **xzany** ([page du projet sur itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Conçu avec la fantastique bibliothèque d'interface graphique **[egui](https://github.com/emilk/egui)** et l'écosystème graphique **[wgpu](https://wgpu.rs/)**.
- Icônes de l'interface propulsées par la suite **[Font Awesome Free](https://fontawesome.com)** par @fontawesome (Icônes : CC BY 4.0, Polices : SIL OFL 1.1, Code : Licence MIT).

---

## Licence et exclusion de garantie

Ce projet est distribué sous **[Licence MIT](../LICENSE)**. Vous êtes libre de l'utiliser, de le modifier, de le distribuer et d'intégrer OniAnvil dans vos projets de développement de jeux, tant commerciaux que personnels.

### Exclusion de garantie et limitation de responsabilité
> [!IMPORTANT]
> **OniAnvil** est distribué **gratuitement** sur une base **« EN L'ÉTAT » (« AS IS »)** et **« SELON DISPONIBILITÉ » (« AS AVAILABLE »)**, sans garantie ni condition d'aucune sorte, expresse ou implicite.
> 
> En aucun cas l'auteur ou les auteurs, les détenteurs des droits d'auteur ou les contributeurs ne pourront être tenus responsables de tout dommage direct, indirect, accessoire, spécial, exemplaire ou consécutif (y compris, mais sans s'y limiter, la perte de données, la corruption de fichiers, une défaillance matérielle, des pertes de profits ou une interruption d'activité) découlant de quelque manière que ce soit de l'utilisation ou de l'impossibilité d'utiliser ce logiciel, même si la possibilité de tels dommages a été expressément signalée. Vous assumez l'entière responsabilité et tous les risques liés à l'utilisation de ce programme.

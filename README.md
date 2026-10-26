<div align="center">

<img src="docs/assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Pixel Sprite & Onion-Skin Animation Studio built around Game-Ready Atlases**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Languages](https://img.shields.io/badge/Languages-37_Locales-purple)](#10-35-ergonomic-themes--multilingual-localization)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#getting-started)
[![Web Demo](https://img.shields.io/badge/Online_App-Try_in_Browser_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

<br/>

👉 **[🚀 Try OniAnvil in Browser (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/)** *(⚠️ Experimental build: may contain WebGL/WASM limitations and bugs)* • **[🌐 Official Documentation Website](https://rbm13s.github.io/OniAnvil/)**

<br/>

**Languages / Языки / Мови / Sprachen / 语言:**<br/>
**English** •
[Русский](docs/README-ru.md) •
[Українська](docs/README-uk.md) •
[Беларуская](docs/README-be.md) •
[Қазақша](docs/README-kk.md) •
[ქართული](docs/README-ka.md) •
[Հայերեն](docs/README-hy.md) •
[Azərbaycan](docs/README-az.md) •
[Oʻzbekcha](docs/README-uz.md) •
[Deutsch](docs/README-de.md) •
[Français](docs/README-fr.md) •
[Español](docs/README-es.md) •
[Italiano](docs/README-it.md) •
[Português](docs/README-pt.md) •
[Polski](docs/README-pl.md) •
[Čeština](docs/README-cs.md) •
[Slovenčina](docs/README-sk.md) •
[Slovenščina](docs/README-sl.md) •
[Български](docs/README-bg.md) •
[Српски](docs/README-sr.md) •
[Hrvatski](docs/README-hr.md) •
[Nederlands](docs/README-nl.md) •
[Dansk](docs/README-da.md) •
[Norsk](docs/README-no.md) •
[Svenska](docs/README-sv.md) •
[Suomi](docs/README-fi.md) •
[Eesti](docs/README-et.md) •
[Latviešu](docs/README-lv.md) •
[Lietuvių](docs/README-lt.md) •
[Magyar](docs/README-hu.md) •
[Română](docs/README-ro.md) •
[Türkçe](docs/README-tr.md) •
[Ελληνικά](docs/README-el.md) •
[简体中文](docs/README-zh-cn.md) •
[繁體中文](docs/README-zh-tw.md) •
[日本語](docs/README-ja.md) •
[한국어](docs/README-ko.md)

</div>

---

## Table of Contents

- [Introduction](#introduction)
- [Screenshots & Visual Tour](#screenshots--visual-tour)
- [Comprehensive Tools Reference](#comprehensive-tools-reference)
- [Functional Documentation](#functional-documentation)
  - [1. Dual Atlas-Centric Architecture](#1-dual-atlas-centric-architecture)
  - [2. Multi-Track Layer Timeline](#2-multi-track-layer-timeline)
  - [3. Real-Time Animation Live Preview](#3-real-time-animation-live-preview)
  - [4. Advanced Onion Skinning Engine](#4-advanced-onion-skinning-engine)
  - [5. Smart Reference & Sketch Tracing Layer](#5-smart-reference--sketch-tracing-layer)
  - [6. Precision Selection Suite & Marching Ants](#6-precision-selection-suite--marching-ants)
  - [7. Live FX, Filters & RotSprite](#7-live-fx-filters--rotsprite)
  - [8. Palette Management, Swapping & Character Skins](#8-palette-management-swapping--character-skins)
  - [9. Sheet Slicer, 9-Grid UI & Game Engine Export](#9-sheet-slicer-9-grid-ui--game-engine-export)
  - [10. 35 Ergonomic Themes & Multilingual Localization](#10-35-ergonomic-themes--multilingual-localization)
- [Controls & Keyboard Shortcuts](#controls--keyboard-shortcuts)
- [Getting Started](#getting-started)
  - [Standalone Binaries](#standalone-binaries)
  - [Building from Source](#building-from-source)
- [File Format & Companion Architecture](#file-format--companion-architecture)
- [Contributing & Acknowledgements](#contributing--acknowledgements)
- [License](#license)

---

## Introduction

**OniAnvil** (*from **Oni**on-Skinning + **Anvil** Sprite Forge*) is an open-source, high-performance pixel art editor and animation suite built in Rust. It runs natively with hardware-accelerated rendering on Windows, Linux, and macOS, and as a zero-installation WebAssembly application directly inside modern web browsers.

Most existing sprite editors treat animation as a stack of disconnected image files, forcing developers into tedious export, slicing, and repackaging steps. **OniAnvil flips this paradigm:** the game sprite sheet atlas texture (`atlas_rgba`) is the primary source of truth. Frame cells, layer tracks, animation tags, and collision pivots are non-destructively maintained in sync with the atlas at all times.

### Why OniAnvil?
- 🚀 **Blazing Fast Native Rust Performance**: Instant sub-millisecond startup, crisp rendering powered by `wgpu` and `egui 0.31`, zero lag when scrubbing long timelines.
- 🎬 **Multi-Track Layer Timeline**: NLE-inspired layer tracks (`V1, V2, V3...`) with top-over-bottom compositing, merge down, linked frames, and variable hold durations.
- 🧅 **First-Class Onion Skinning**: Multi-frame ghosting with independent forward/backward tinting, customizable opacity curves, and loop wrap-around.
- 🎨 **Instant Character Palette Skins**: Create day/night variants, team colors, or elemental skins with live atlas remapping and GPL/PAL/HEX import and export.
- 📦 **Direct Engine Export**: Export scaled PNGs with extruded borders to eliminate texture bleeding in 2D engines (Godot, Bevy, Raylib, Defold), along with standard sprite sheet JSON metadata and animated GIFs.

---

## Screenshots & Visual Tour

<div align="center">

### 1. Main Canvas & Studio Workspace
*High-performance pixel workspace featuring hardware-accelerated viewport, multi-track timeline, real-time live preview, palette swatches, and precision toolbars.*
<img src="docs/screenshots/main.png" alt="OniAnvil Main Workspace" width="950" />

<br/><br/>

### 2. Multi-Track Animation Timeline
*Non-linear multi-track timeline with stacked layer rows (`V1, V2...`), linked frame cel instances (`🔗`), hold multipliers, and named color-coded animation tags.*
<img src="docs/screenshots/timeline.png" alt="OniAnvil Multi-Track Timeline" width="950" />

<br/><br/>

### 3. Precision Tools & Action Toolbar
*Full 15-tool drawing suite with standard and compact docking modes, alongside the top quick-action toolbar for brush size, symmetry, wrap-around, and pixel-perfect toggles.*
<p align="center">
  <img src="docs/screenshots/toolbar.png" alt="OniAnvil Action Toolbar" width="600" /><br/><br/>
  <img src="docs/screenshots/tools.png" alt="OniAnvil Tools Standard" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/tools_compact.png" alt="OniAnvil Tools Compact" width="300" />
</p>

<br/><br/>

### 4. Real-Time Live Animation Preview
*Dedicated live playback dock with tag filtering, variable FPS speed controls, zoom slider, and viewport background toggles (checkerboard, solid dark, solid light).*
<p align="center">
  <img src="docs/screenshots/live_preview.png" alt="OniAnvil Live Preview" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/live_preview_scroll_test.png" alt="OniAnvil Live Preview Zoom" width="460" />
</p>

<br/><br/>

### 5. Palette Management, Swapping & Color Ramps
*Comprehensive color engine with Foreground/Background swatch boxes, automated two-point color ramp generation, and named palette skin swaps.*
<img src="docs/screenshots/palette_and_swaps.png" alt="OniAnvil Palette and Swaps" width="950" />

<br/><br/>

### 6. Reference Sketch Tracing & Configurable Onion Skinning
*Load concept sketches or photographs as an underlay/overlay with transform controls, smart color sampling with palette snap, and multi-frame onion skin ghosting.*
<img src="docs/screenshots/sketch_and_onion_skin.png" alt="OniAnvil Sketch and Onion Skin" width="950" />

<br/><br/>

### 7. Pivot Points & Collision Action Points
*Define sprite origin center and custom named gameplay points (weapon anchors, projectile emitters, head, footstep) with interactive canvas gizmos.*
<img src="docs/screenshots/pivot_and_action_points.png" alt="OniAnvil Pivot and Action Points" width="950" />

<br/><br/>

### 8. Sprite Sheet Slicer & 9-Grid (9-Slice) Scaler
*Automated sprite boundary autocorrelation slicing (e.g. cat walk cycle) alongside scalable 9-slice UI box margin configuration.*
<p align="center">
  <img src="docs/screenshots/slicer.png" alt="OniAnvil Sheet Slicer" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/3x3.png" alt="OniAnvil 9-Slice Sizer" width="460" />
</p>

<br/><br/>

### 9. Real-Time Image FX & Filter Modals (with Live Preview)
*All effect dialogs feature live canvas preview, interactive eyedroppers, and non-destructive cancellation.*
<p align="center">
  <img src="docs/screenshots/outline_generator.png" alt="Outline Generator" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/replace_color.png" alt="Replace Color" width="460" />
</p>
<p align="center">
  <img src="docs/screenshots/hue_saturation_contrast.png" alt="Hue Saturation Contrast" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/matrix_filter.png" alt="Matrix Filter" width="460" />
</p>
<p align="center">
  <img src="docs/screenshots/remap_to_palette.png" alt="Remap to Palette" width="460" />
</p>

<br/><br/>

### 10. Canvas Grid & Background Modes
*Switch seamlessly between pixel grids, tile grids, dark checkerboard, light checkerboard, and custom solid backdrop colors.*
<p align="center">
  <img src="docs/screenshots/grid_and_dark_checkerboard.png" alt="Grid and Dark Checkerboard" width="300" />
  &nbsp;&nbsp;
  <img src="docs/screenshots/no_grid_light_checkerboard.png" alt="Light Checkerboard" width="300" />
  &nbsp;&nbsp;
  <img src="docs/screenshots/custom_solid_background.png" alt="Custom Solid Background" width="300" />
</p>

<br/><br/>

### 11. New Project Creation Wizard
*Configure project canvas dimensions, frames, column grid layout, and pick from over 20 authentic retro palettes on project startup.*
<p align="center">
  <img src="docs/screenshots/new_project.png" alt="New Project Wizard" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/screenshots/new_project_palettes.png" alt="New Project Palettes" width="460" />
</p>

<br/><br/>

### 12. 35 Authentic Retro & Modern Themes
*Includes Studio Dark, Studio Light, CRT Amber/Green/Cyan phosphors, retro consoles (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis), and designer themes.*
<p align="center">
  <img src="docs/screenshots/themes.png" alt="Themes Menu" width="300" />
  &nbsp;&nbsp;
  <img src="docs/screenshots/crt_orange_theme.png" alt="CRT Amber Theme" width="300" />
  &nbsp;&nbsp;
  <img src="docs/screenshots/psx_theme.png" alt="PSX Theme" width="300" />
</p>

<br/><br/>

### 13. Menu System & In-App Keyboard Help
<p align="center">
  <img src="docs/screenshots/file_menu.png" alt="File Menu" width="180" />
  <img src="docs/screenshots/edit_menu.png" alt="Edit Menu" width="180" />
  <img src="docs/screenshots/select_menu.png" alt="Select Menu" width="180" />
  <img src="docs/screenshots/view_menu.png" alt="View Menu" width="180" />
  <img src="docs/screenshots/project_menu.png" alt="Project Menu" width="180" />
</p>
<p align="center">
  <img src="docs/screenshots/help.png" alt="Keyboard Shortcuts Help Modal" width="700" />
</p>

</div>

---

## Comprehensive Tools Reference

OniAnvil includes 15 precision drawing, shape, selection, and metadata tools. Icons are rendered using the open-source **Font Awesome Free** icon suite *(Icons: CC BY 4.0, Fonts: SIL OFL 1.1)*.

| Tool | Icon | Hotkey | Brush Size | Pixel Perfect | Description |
|:---|:---:|:---:|:---:|:---:|:---|
| **Pencil** | ✏️ | <kbd>B</kbd> | 1..32 px | Yes | Primary drawing tool. 1px mode supports **Pixel-Perfect** drawing (automatically eliminates ugly staircase L-corners). Supports custom selection stamp brushes (`Ctrl/Cmd+B`). |
| **Eraser** | 🧹 | <kbd>E</kbd> | 1..32 px | No | Erases pixels back to transparent (`[0, 0, 0, 0]`). Bounded by selection mask if active. |
| **Eyedropper** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | No | Left-click samples color to Primary (Foreground); Right-click samples to Secondary (Background). Can sample from canvas pixels, layers, or reference sketch. |
| **Flood Fill** | 🪣 | <kbd>G</kbd> | N/A | No | High-speed queue-based BFS fill with configurable Color Tolerance (`0..=255`). Supports Contiguous vs Global fill and selection bounding. |
| **Gradient** | 🌈 | <kbd>G</kbd> *(toggle)* | N/A | No | Linear and Radial gradients between Primary and Secondary colors. Supports Bayer (2x2, 4x4, 8x8) and Blue Noise dithering with strength control. |
| **Line** | 📏 | <kbd>L</kbd> | 1..32 px | Yes | Bresenham line algorithm with live drag preview. Hold <kbd>Shift</kbd> to lock angles to 45°/90° increments. |
| **Curve** | 〰️ | <kbd>C</kbd> | 1..32 px | Yes | 3-click quadratic Bézier curve. Click 1 sets Start Point, Click 2 sets End Point, mouse motion bends curvature, Click 3 commits the curve. |
| **Rectangle** | ⬜ | <kbd>U</kbd> | 1..32 px | No | Outlined or Filled geometric rectangles. Hold <kbd>Shift</kbd> to constrain to a 1:1 square. |
| **Ellipse** | ⭕ | <kbd>O</kbd> | 1..32 px | No | Outlined or Filled ellipses using midpoint rasterization. Hold <kbd>Shift</kbd> to constrain to a perfect circle. |
| **Dither Brush** | 🏁 | <kbd>D</kbd> | 1..32 px | No | Paints alternating checkerboard dither patterns between Foreground and Background colors without introducing extra palette entries. |
| **Shading Brush** | 🌓 | <kbd>S</kbd> | 1..32 px | No | Incremental Dodge (lighten) and Burn (darken) shading based on active palette swatch ordering. |
| **Rect Select** | ⬚ | <kbd>M</kbd> | N/A | No | Rectangular marquee selection with animated marching ants boundary. Supports Add (<kbd>Shift</kbd>) and Subtract (<kbd>Alt</kbd>). |
| **Lasso Select** | ➰ | <kbd>Q</kbd> | N/A | No | Freehand polygon lasso. Uses 8-connected boundary rasterization with 4-connected exterior flood fill, guaranteeing 100% solid, hole-free selections even on loops. |
| **Magic Wand** | 🪄 | <kbd>W</kbd> | N/A | No | Selects contiguous (or global) regions of matching color with adjustable tolerance threshold. |
| **Pivot & Action** | 🎯 | <kbd>P</kbd> | N/A | No | Interactive canvas placement of Sprite Origin and custom gameplay Action Points (`weapon`, `head`, `feet`, `hitbox`) exported to JSON metadata. |

---

## Functional Documentation

### 1. Dual Atlas-Centric Architecture

In OniAnvil, the master texture atlas (`atlas_rgba`) represents the true sprite sheet texture as loaded by your game engine.
- **Frame Mode**: Centers on a single active animation frame, providing a distraction-free pixel canvas. Edits immediately update the corresponding cell in the atlas texture.
- **Atlas Mode**: Displays the entire sprite sheet grid, enabling macro adjustments, cross-frame alignment checks, and batch editing.
- **Companion File**: Projects save clean `.png` sprite sheets accompanied by a lightweight `.onianvil.json` companion file containing frame layouts, layer hierarchies, tags, hold multipliers, and palette skins.

### 2. Multi-Track Layer Timeline

The bottom timeline is modeled after professional non-linear video editing software:
- **Layer Tracks (`V1, V2, V3...`)**: Layers stack visually from top to bottom. The track placed higher in the timeline (`V1`) renders on top in the frame canvas.
- **Track Controls**:
  - Click `V1` / `V2` badges to quickly switch the active drawing layer.
  - Toggle layer visibility (Eye icon) and layer editing lock (Lock icon).
  - Adjust individual layer opacity (`0%..=100%`).
  - Move layer up/down in hierarchy, duplicate layer, and merge layer down into the track below.
  - Delete layer with confirmation dialog.
- **Frame Clips**:
  - Hold multiplier: assign custom frame durations (e.g. `2×`, `3×` hold for pauses or impact frames).
  - Linked frames: link repetitive frames across the animation so editing one automatically updates all linked instances.
  - Tag looping: organize animations into named tags (`idle`, `walk`, `attack`) and loop specific tags during playback.
  - Playback controls: Forward, Reverse, Ping-Pong, and Loop playback with adjustable FPS (`1..=120`).

### 3. Real-Time Animation Live Preview

The right-hand Preview dock plays your animations in real time, completely independent of your canvas zoom or pan position:
- **Tag Filtering**: Isolate playback to specific tags (`walk`, `jump`) or preview the entire animation sequence.
- **Playback Controls**: Play, Pause, Step Previous, Step Next, Loop toggle, and custom playback speeds (`0.25x..=4.0x`).
- **Display Modes**: Toggle between dark checkerboard, light checkerboard, pure white, pure black, or custom canvas backgrounds.

### 4. Advanced Onion Skinning Engine

Sculpt fluid animations using the dedicated onion skinning system:
- **Custom Tinting**: Independent tint colors for previous frames (default: Red) and next frames (default: Blue).
- **Opacity Falloff**: Configurable base opacity and step-by-step opacity attenuation curves.
- **Loop Wrap-Around**: Enable ghosting across the first and last frames of looping animations.
- **Quick-Peek**: Hold <kbd>Space</kbd> without dragging to instantly preview adjacent frames.

### 5. Smart Reference & Sketch Tracing Layer

Accelerate your workflow by tracing hand-drawn sketches or reference photographs:
- **Formats**: Load PNG, JPEG, and WebP reference images.
- **Blending Modes**: Underlay (beneath your pixel layers) or Overlay (above your pixel layers).
- **Transform Controls**: Independent zoom, position offset, rotation, and opacity slider.
- **Smart Color Sampling**: Use the Eyedropper on the sketch layer with optional automatic snapping to the nearest color in your active project palette.
- **1-Click Bake**: Flatten the scaled reference image directly into pixel art on the active frame.

### 6. Precision Selection Suite & Marching Ants

- **Animated Marching Ants**: Active selections display animated high-visibility dual-tone dashed perimeters.
- **Solid Flood-Fill Lasso Selection**: Unlike naive ray-casting algorithms that create unselected holes when drawing overlapping loops, OniAnvil's lasso rasterizes a closed 8-connected boundary and flood-fills the exterior, guaranteeing 100% solid, hole-free selections.
- **Selection Operations**: Expand, Contract, Border (with Circle, Square, and Diamond kernel shapes), Translate, Invert, Delete, Free Rotate, and System Clipboard Copy/Cut/Paste.
- **Cross-Frame Copy & Paste**: Select regions on one frame, press <kbd>Ctrl/Cmd+C</kbd>, switch frames, and press <kbd>Ctrl/Cmd+V</kbd> to paste at identical relative coordinates across any keyboard layout.

### 7. Live FX, Filters & RotSprite

All image effects include a real-time **Live Preview** checkbox that previews modifications on the canvas and allows seamless cancellation without polluting the undo history stack:
- **Color Replace (`Shift+R`)**:
  - Interactive Eyedropper: click the pipette icon and pick colors directly from frame pixels or the palette dock.
  - Quick-pick palette strip: left-click to pick "From" color, right-click to pick "To" color.
  - Tolerance slider and batch replacement across all animation frames.
- **Color Adjust (`Ctrl/Cmd+U`)**: Real-time HSV Hue shift, Saturation, Value brightness, and Contrast adjustment.
- **Auto-Outline Generator**: Create pixel-crisp character outlines (Outside, Inside, Centered) with Square or Diamond corner joins.
- **3×3 Convolution Filters**: Blur, Sharpen, Edge Detect, and Emboss filters with preserve-alpha toggle.
- **Arbitrary Rotation (`Ctrl/Cmd+R`)**: Rotate selected regions or full frames by any degree using the classic **RotSprite** or **Nearest Neighbor** algorithms.
- **Remap to Palette**: Dither canvas colors into active palette using Bayer 2x2, 4x4, 8x8, or Blue Noise.

### 8. Palette Management, Swapping & Character Skins

- **Built-in Palettes**: 25+ authentic retro palettes including DawnBringer (DB8, DB16, DB32), Windows 95 (256), Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized, and Sweetie 16.
- **Import & Export**: Support for GIMP Palette (`.gpl`), PAL (`.pal`), Hex palette (`.hex`), and image swatches (`.png`).
- **Palette Swapping (Skins)**: Define multiple palette skin variants for a single character (e.g., Default, Golden Armor, Shadow, Frozen). Swapping skins instantly remaps all frame colors across the atlas.
- **Color Ramp Generator**: Automatically interpolate smooth color gradients between two chosen hues with configurable intermediate step count.

### 9. Sheet Slicer, 9-Grid UI & Game Engine Export

- **Sheet Slicer**: Autocorrelation boundary detection automatically slices raw sprite sheets into uniform or tightly bounded animation frames.
- **9-Grid (9-Slice) Scaling**: Set customizable slice margins (top, bottom, left, right) to preview scalable UI boxes and button panels.
- **Extruded Border PNG Export**: Extrudes edge pixel colors into transparent margins, eliminating texture filtering seams and bleeding in OpenGL, DirectX, Vulkan, and 2D game engines (such as Godot).
- **Animated GIF Export**: Generate optimized animated GIFs with custom loop counts, frame delays, and transparent backgrounds.
- **Standard Sprite Sheet JSON**: Exports frame slices, tags, durations, and pivot coordinates matching the standard sprite sheet JSON schema used across modern 2D game engines.

### 10. 35 Ergonomic Themes & Multilingual Localization

- **Curated Themes**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Amber CRT, Green CRT, Cyan CRT, Retro Consoles (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast), and Retro Computers (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **37 Languages**: Comprehensive native UI localization across 37 languages with 100% dictionary key parity.

---

## Controls & Keyboard Shortcuts

| Shortcut | Tool / Action | Description |
|:---|:---|:---|
| <kbd>B</kbd> | **Pencil** | 1px precision drawing with optional Pixel-Perfect mode |
| <kbd>E</kbd> | **Eraser** | Erase pixels back to transparency |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Eyedropper** | Sample color from canvas pixels or reference sketch |
| <kbd>G</kbd> | **Flood Fill / Gradient** | Toggle between Flood Fill and Dither Gradient |
| <kbd>L</kbd> | **Line** | Draw straight lines with live preview and Shift snap |
| <kbd>C</kbd> | **Curve** | 3-click quadratic Bézier curve tool |
| <kbd>U</kbd> | **Rectangle** | Geometric rectangle and square outline/fill |
| <kbd>O</kbd> | **Ellipse** | Geometric ellipse and circle outline/fill |
| <kbd>D</kbd> | **Dither Brush** | Paint checkerboard dither matrix patterns |
| <kbd>S</kbd> | **Shading Brush** | Incremental Dodge / Burn shading across palette ramp |
| <kbd>M</kbd> | **Rect Select** | Rectangular area selection with marching ants |
| <kbd>Q</kbd> | **Lasso Select** | Freehand solid polygon selection |
| <kbd>W</kbd> | **Magic Wand** | Select contiguous color regions |
| <kbd>P</kbd> | **Pivot Points** | Position sprite origin and action points |
| <kbd>X</kbd> | **Swap Colors** | Swap Primary and Secondary colors |
| <kbd>[</kbd> / <kbd>]</kbd> | **Brush Size** | Decrease / Increase brush size (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copy** | Copy selected pixels to system clipboard |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Cut** | Cut selected pixels to system clipboard |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Paste** | Paste clipboard pixels into active frame |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Delete** | Clear pixels inside active selection |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Select All** | Select all pixels in the active frame |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Deselect** | Clear active selection mask |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Invert Selection** | Invert active selection mask |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplicate Frame** | Duplicate active frame in timeline |
| <kbd>Space</kbd> | **Play / Pause** | Toggle animation playback in timeline |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Replace Color** | Open Color Replace modal with live preview |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Color Adjust** | Open HSV Hue / Saturation / Contrast modal |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Free Rotate** | Open arbitrary angle rotation modal (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Undo** | Undo last canvas stroke or tool action |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Redo** | Redo previously undone action |
| <kbd>Space</kbd> + Drag / MMB | **Pan Canvas** | Smooth infinite panning across canvas coordinates |
| <kbd>Ctrl/Cmd</kbd> + Wheel / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Pixel-crisp zoom centered around mouse cursor |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **View Mode** | Toggle between Frame View (<kbd>1</kbd>) and Atlas View (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Fit Canvas** | Auto-fit active frame to workspace center |
| <kbd>F1</kbd> | **Help** | Open Keyboard Shortcuts modal |

---

## Getting Started

### Standalone Binaries

Download pre-compiled, self-contained binaries from the **[GitHub Releases Page](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Download `OniAnvil-windows-x86_64.zip`, extract, and double-click `OniAnvil.exe`.
- **Linux**: Download `OniAnvil-linux-x86_64.tar.gz` (portable) or install the `onianvil_*_amd64.deb` Debian/Ubuntu package:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Download `OniAnvil-macos-universal.zip`, extract, and drag `OniAnvil.app` to your Applications folder. Universal binary runs natively on both Apple Silicon (M1/M2/M3/M4) and Intel Macs.
- **Web**: Run directly in your browser without installing anything: **[Launch WebAssembly App](https://rbm13s.github.io/OniAnvil/app/)**.
  > [!WARNING]
  > The WebAssembly (browser) build has an **Experimental** status. Due to browser sandbox constraints, WebGL/WebGPU differences, and clipboard limitations, you may encounter minor graphical glitches or performance bottlenecks compared to the native desktop version. For production work, native desktop builds are strongly recommended.

---

### Building from Source

#### Prerequisites
- **Rust Toolchain**: Install Rust (1.80+ recommended) via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux Build Dependencies** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compile & Run Native Desktop Version
```bash
# Clone the repository
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Run test suite
cargo test

# Build and run optimized release binary
cargo run --release
```

#### 2. Compile WebAssembly Version (via Trunk)
```bash
# Install wasm32 compilation target and Trunk builder
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Run local development server with hot-reload
trunk serve

# Build optimized release files into docs/app/
./build_wasm.sh        # On Linux/macOS
./build_wasm.ps1       # On Windows PowerShell
```

---

## File Format & Companion Architecture

OniAnvil avoids proprietary binary formats. Every project consists of two clean, version-controllable files:

```
my_character/
├── my_character.png            # Standard PNG sprite sheet atlas (openable in any engine or viewer)
└── my_character.onianvil.json   # Companion metadata sidecar
```

The `.onianvil.json` sidecar stores:
- Canvas grid configuration: frame dimensions, column count, frame count.
- Layer hierarchies: track names, opacity levels, visibility flags, locked states.
- Animation tags: name, start/end frame indices, playback direction (Forward, Reverse, Ping-Pong), frame durations.
- Palette definitions: active color swatches, named skin variations, active skin index.
- Reference sketch state: image path, scale, pan offset, rotation, opacity, layer mode.
- Pivot points: custom action points, weapon anchors, and collision centers.

---

## Contributing & Acknowledgements

Contributions, bug reports, and feature proposals are warmly welcome!
1. Fork the repository on GitHub.
2. Create a feature branch: `git checkout -b feature/my-new-feature`.
3. Commit your changes: `git commit -am 'Add new pixel tool'`.
4. Push to the branch: `git push origin feature/my-new-feature`.
5. Open a Pull Request.

### Acknowledgements
- Starter demo sprite **"Cat 2D Pixel Art"** created by **xzany** ([itch.io project page](https://xzany.itch.io/cat-2d-pixel-art)).
- Built with the fantastic **[egui](https://github.com/emilk/egui)** GUI library and **[wgpu](https://wgpu.rs/)** graphics ecosystem.
- UI iconography powered by **[Font Awesome Free](https://fontawesome.com)** by @fontawesome (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License).
- Curated retro palettes include **DB8**, **DB16**, and **DB32** created by **DawnBringer** (Richard Fhager) ([DawnBringer Palettes](https://github.com/geoffb/dawnbringer-palettes)), and the classic **Windows 95 (256)** palette.

---

## License & Disclaimer

This project is licensed under the **[MIT License](LICENSE)**. You are free to use, modify, distribute, and integrate OniAnvil into commercial and personal game development pipelines.

### Disclaimer of Warranty & Limitation of Liability
> [!IMPORTANT]
> **OniAnvil** is distributed **free of charge** on an **"AS IS"** and **"AS AVAILABLE"** basis, without warranties or conditions of any kind, either express or implied.
> 
> Under no circumstances shall the author(s), copyright holders, or contributors be held liable for any direct, indirect, incidental, special, exemplary, or consequential damages (including, but not limited to, loss of data, file corruption, hardware failure, lost profits, or business interruption) arising in any way out of the use of or inability to use this software, even if advised of the possibility of such damage. You assume all responsibility and risk for using this program.

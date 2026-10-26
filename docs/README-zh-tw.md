<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**專為遊戲圖集打造的高效能像素藝術與洋蔥皮逐格動畫工作室**

[![構建與發布](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![下載量](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![繪製器](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![介面語言](https://img.shields.io/badge/介面語言-37_種語言-purple)](#10-35-款人體工學主題與全套在地化)
[![平台](https://img.shields.io/badge/平台-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#快速入門)
[![線上版](https://img.shields.io/badge/線上應用-瀏覽器即開即用_(實驗性)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=zh-tw)
[![授權條款: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 在瀏覽器中試用 OniAnvil (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=zh-tw)** *(⚠️ 實驗性版本：可能存在 WebGL/WASM 瀏覽器沙箱限制及相容性問題)* • **[🌐 官方文件網站](https://rbm13s.github.io/OniAnvil/zh-tw.html)**

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
[Türkçe](README-tr.md) •
[Ελληνικά](README-el.md) •
[简体中文](README-zh-cn.md) •
**繁體中文** •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## 目錄

- [簡介](#簡介)
- [螢幕截圖與視覺導覽](#螢幕截圖與視覺導覽)
- [完整工具參考手冊](#完整工具參考手冊)
- [功能全景文件](#功能全景文件)
  - [1. 以圖集為核心的雙重架構](#1-以圖集為核心的雙重架構)
  - [2. 多軌道圖層時間軸](#2-多軌道圖層時間軸)
  - [3. 即時動畫播放預覽](#3-即時動畫播放預覽)
  - [4. 進階洋蔥皮引擎](#4-進階洋蔥皮引擎)
  - [5. 智慧參考圖與草圖描摹層](#5-智慧參考圖與草圖描摹層)
  - [6. 高精度選取套件與行軍蟻虛線](#6-高精度選取套件與行軍蟻虛線)
  - [7. 即時特效、濾鏡與 RotSprite](#7-即時特效濾鏡與-rotsprite)
  - [8. 調色盤管理、色彩置換與角色皮膚](#8-調色盤管理色彩置換與角色皮膚)
  - [9. 圖集切片器、九宮格 UI 與遊戲引擎匯出](#9-圖集切片器九宮格-ui-與遊戲引擎匯出)
  - [10. 35 款人體工學主題與全套在地化](#10-35-款人體工學主題與全套在地化)
- [快捷鍵與操作指南](#快捷鍵與操作指南)
- [快速入門](#快速入門)
  - [獨立執行版 (預先編譯二進位程式)](#獨立執行版-預先編譯二進位程式)
  - [從原始碼編譯](#從原始碼編譯)
- [檔案格式與伴隨檔案架構](#檔案格式與伴隨檔案架構)
- [參與貢獻與致謝](#參與貢獻與致謝)
- [授權條款與免責聲明](#授權條款與免責聲明)

---

## 簡介

**OniAnvil**（名稱源自 **Oni**on-Skinning [洋蔥皮透光] + **Anvil** [鐵砧/精靈鍛造台]）是一款基於 Rust 打造的開源、高效能像素藝術編輯器與逐格動畫套件。它在 Windows、Linux 和 macOS 上以硬體加速的原生效能執行，同時也能作為無須安裝的 WebAssembly 應用直接在現代網頁瀏覽器中運行。

目前多數精靈編輯器通常將動畫視為一堆零散獨立的影像檔案，迫使開發者經歷繁瑣的匯出、切片與重新打包步驟。**OniAnvil 顛覆了這項傳統思維：** 遊戲精靈圖集材質（`atlas_rgba`）即為專案唯一的真實來源（Source of Truth）。關鍵影格儲存格、圖層軌道、動畫標籤以及碰撞軸心點隨時與圖集保持非破壞性即時同步。

### 為什麼選擇 OniAnvil？
- 🚀 **極致飛快的原生 Rust 效能**：亞毫秒級瞬間啟動，由 `wgpu` 與 `egui 0.31` 驅動的清晰銳利圖形繪製，滑動長距離複雜時間軸毫無延遲。
- 🎬 **多軌道圖層時間軸**：源自專業非線性剪輯（NLE）概念的圖層軌道（`V1, V2, V3...`），支援由上而下圖層合成、向下合併、關聯副本影格以及自由設定的停留持續時間（Hold）。
- 🧅 **頂級支援的洋蔥皮引擎**：多影格透光殘影，支援獨立設定的前後影格著色（前向紅/後向藍）、自訂不透明度衰減曲線與首尾無縫循環回繞。
- 🎨 **即時角色調色盤皮膚換色**：一鍵建立日夜變體、隊伍配色或元素外觀，支援全圖集動態像素重對應與 GPL/PAL/HEX/PNG 調色盤匯入匯出。
- 📦 **直供遊戲引擎的標準匯出**：支援匯出邊緣外擴（像素擠出）的等比 PNG，徹底根除 2D 遊戲引擎（如 Godot、Bevy、Raylib、Defold）中的雙線性濾波接縫黑邊與溢色，同步輸出標準 Sprite Sheet JSON 中繼資料與最佳化動態 GIF。

---

## 螢幕截圖與視覺導覽

<div align="center">

### 1. 主畫布與工作室工作區
*高效能像素工作空間，包含硬體加速視口、多軌道時間軸、即時動畫預覽、調色盤色票與高精度工具列。*
<img src="screenshots/main.png" alt="OniAnvil 主工作區" width="950" />

<br/><br/>

### 2. 多軌道動畫時間軸
*非線性多軌道時間軸，具備層疊圖層列（`V1, V2...`）、關聯影格實例（`🔗`）、停留倍率（Hold）與彩色命名動畫標籤。*
<img src="screenshots/timeline.png" alt="OniAnvil 多軌道時間軸" width="950" />

<br/><br/>

### 3. 精密繪圖工具與頂部操作列
*完整 15 種專業繪圖工具，支援標準與精簡停駐模式，配有用於筆刷尺寸、對稱、循環回繞與像素修正（Pixel-Perfect）切換的頂部快捷工具列。*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil 頂部操作列" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil 標準工具列" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil 精簡工具列" width="300" />
</p>

<br/><br/>

### 4. 即時動畫播放預覽
*專屬獨立即時播放面板，支援依標籤篩選、可變 FPS 影格率控制、縮放滑桿以及視口背景切換（棋盤格、純深色、純淺色）。*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil 即時預覽" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil 預覽縮放" width="460" />
</p>

<br/><br/>

### 5. 調色盤管理、色階漸層與角色皮膚
*全功能色彩引擎，支援前景色/背景色票切換、兩點自動化色彩漸層階梯生成以及具名調色盤外觀替換。*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil 調色盤與外觀換色" width="950" />

<br/><br/>

### 6. 參考草圖描摹與可配置洋蔥皮
*支援將概念草圖或照片載入為底層底圖/頂層覆蓋層，具備變形控制、帶調色盤吸附的智慧取色以及多影格洋蔥皮透光殘影。*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil 草圖與洋蔥皮" width="950" />

<br/><br/>

### 7. 中心軸心點與動作點/碰撞錨點
*透過互動式畫布控制桿，直覺設定精靈原點中心與自訂命名的遊戲事件點（武器錨點、發射點、頭部、腳步判定等）。*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil 軸心點與動作點" width="950" />

<br/><br/>

### 8. 精靈圖集切片器與九宮格 (9-Grid) 縮放
*基於自動相關邊界偵測的自動圖集切片（例如貓咪行走循環），以及可縮放的九宮格 UI 邊界切片設定。*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil 圖集切片器" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 九宮格縮放" width="460" />
</p>

<br/><br/>

### 9. 即時影像特效與濾鏡面板 (支援即時預覽)
*所有特效對話方塊皆支援畫布即時預覽、互動式滴管取色以及無損復原取消。*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="輪廓產生器" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="取代顏色" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="色相飽和度對比度" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="矩陣濾鏡" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="重對應至調色盤" width="460" />
</p>

<br/><br/>

### 10. 畫布網格與背景顯示模式
*在像素網格、圖塊網格、深色棋盤格、淺色棋盤格與自訂純色背景之間無縫切換。*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="網格與深色棋盤格" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="淺色棋盤格" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="自訂純色背景" width="300" />
</p>

<br/><br/>

### 11. 新增專案精靈
*在建立專案時快速設定畫布尺寸、影格數、網格欄數配置，並可直接從 20 多種經典復古調色盤中挑選。*
<p align="center">
  <img src="screenshots/new_project.png" alt="新增專案精靈" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="新增專案調色盤" width="460" />
</p>

<br/><br/>

### 12. 35 款復古主機與現代主題
*包含工作室深色、淺色、CRT 琥珀/綠色/青色螢光幕、經典復古主機（PlayStation 1, Game Boy, SNES, Dreamcast, Genesis）以及現代設計師主題。*
<p align="center">
  <img src="screenshots/themes.png" alt="主題選單" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT 琥珀色主題" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX 主題" width="300" />
</p>

<br/><br/>

### 13. 選單系統與內建快捷鍵說明
<p align="center">
  <img src="screenshots/file_menu.png" alt="檔案選單" width="180" />
  <img src="screenshots/edit_menu.png" alt="編輯選單" width="180" />
  <img src="screenshots/select_menu.png" alt="選取選單" width="180" />
  <img src="screenshots/view_menu.png" alt="檢視選單" width="180" />
  <img src="screenshots/project_menu.png" alt="專案選單" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="快捷鍵說明視窗" width="700" />
</p>

</div>

---

## 完整工具參考手冊

OniAnvil 包含 15 款專業繪圖、幾何形狀、選取與中繼資料配置工具。介面圖示採用開源的 **Font Awesome Free** 圖示套件 *(圖示: CC BY 4.0, 字型: SIL OFL 1.1)*。

| 工具 | 圖示 | 快捷鍵 | 筆刷尺寸 | 像素修正 (Pixel-Perfect) | 說明描述 |
|:---|:---:|:---:|:---:|:---:|:---|
| **鉛筆** | ✏️ | <kbd>B</kbd> | 1..32 px | 是 | 基本繪圖工具。1px 模式下支援 **像素修正 (Pixel-Perfect)**（自動消除難看的 L 形階梯狀轉角孤點）。支援使用選取印章建立自訂筆刷（`Ctrl/Cmd+B`）。 |
| **橡皮擦** | 🧹 | <kbd>E</kbd> | 1..32 px | 否 | 將像素抹除為完全透明（`[0, 0, 0, 0]`）。若有使用中選取範圍，則僅在選取邊界內擦除。 |
| **滴管** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | 否 | 左鍵挑色至主要前景色（Foreground）；右鍵挑色至次要背景色（Background）。可從畫布像素、圖層或參考草圖挑取色彩。 |
| **油漆桶** | 🪣 | <kbd>G</kbd> | 無 | 否 | 基於佇列的高速 BFS 泛洪填滿演算法，具備可自訂色彩容差（`0..=255`）。支援相連區域填滿與全域同色填滿，受選取範圍限制。 |
| **漸層** | 🌈 | <kbd>G</kbd> *(切換)* | 無 | 否 | 在前景色與背景色之間繪製線性或放射狀漸層。支援 Bayer（2x2, 4x4, 8x8）仿色抖動與藍噪音（Blue Noise）仿色，強度可微調。 |
| **直線** | 📏 | <kbd>L</kbd> | 1..32 px | 是 | 基於布雷森漢姆（Bresenham）演算法繪製直線並提供即時拖曳預覽。按住 <kbd>Shift</kbd> 鍵可鎖定為 45°/90° 倍數角度。 |
| **曲線** | 〰️ | <kbd>C</kbd> | 1..32 px | 是 | 三擊式二次貝茲（Bézier）像素曲線工具。第 1 次點擊指定起點，第 2 次點擊指定終點，移動滑鼠拉動彎曲弧度，第 3 次點擊確認並套用曲線。 |
| **矩形** | ⬜ | <kbd>U</kbd> | 1..32 px | 否 | 繪製邊框輪廓或實心填滿幾何矩形。按住 <kbd>Shift</kbd> 可鎖定為 1:1 正方形。 |
| **橢圓** | ⭕ | <kbd>O</kbd> | 1..32 px | 否 | 採用中點點陣化演算法繪製邊框或實心橢圓。按住 <kbd>Shift</kbd> 可鎖定為正圓形。 |
| **仿色筆刷** | 🏁 | <kbd>D</kbd> | 1..32 px | 否 | 在前景色與背景色之間繪製規律棋盤格仿色紋理，無須在調色盤中引入多餘的中間過渡色。 |
| **光影工具** | 🌓 | <kbd>S</kbd> | 1..32 px | 否 | 依據目前使用中調色盤的色塊排序，逐步進行變亮（Dodge）或加深（Burn）光影漸變繪製。 |
| **矩形選取** | ⬚ | <kbd>M</kbd> | 無 | 否 | 具備動態雙色「行軍蟻虛線」邊框的矩形選取工具。支援按住 <kbd>Shift</kbd> 加選與按住 <kbd>Alt</kbd> 減選。 |
| **套索選取** | ➰ | <kbd>Q</kbd> | 無 | 否 | 徒手多邊形套索。採用 8-連通邊界點陣化與 4-連通外部泛洪填滿演算法，確保即使在自相交套索迴圈時也能獲得 100% 紮實無空洞的選取範圍。 |
| **魔術棒** | 🪄 | <kbd>W</kbd> | 無 | 否 | 選取相連連續區域（或全域比對）的相同顏色像素，容差門檻值可調。 |
| **軸心與動作點** | 🎯 | <kbd>P</kbd> | 無 | 否 | 互動式在畫布上放置精靈中心原點與自訂遊戲動作點（如武器掛點、發射點、頭部、腳步判定等），可完整匯出為 JSON 中繼資料。 |

---

## 功能全景文件

### 1. 以圖集為核心的雙重架構

在 OniAnvil 中，主材質圖集（`atlas_rgba`）即代表遊戲引擎直接載入的真實精靈表（Sprite Sheet）材質：
- **單影格模式 (Frame Mode)**：聚焦於單一目前活動動畫影格，提供專注無干擾的像素繪圖畫布。所有編輯會立即即時同步更新至圖集材質中的對應儲存格。
- **圖集模式 (Atlas Mode)**：完整檢視整張精靈表網格，便於全域宏觀調整、跨影格對齊檢查與批次編輯。
- **伴隨檔案 (Companion File)**：專案以標準的 `.png` 精靈圖集圖片儲存，並附帶一個輕量透明的 `.onianvil.json` 伴隨中繼資料檔案，用於記錄影格配置、圖層階層、標籤、停留倍率及調色盤外觀。

### 2. 多軌道圖層時間軸

底部時間軸借鏡了專業非線性視訊剪輯軟體的設計哲學：
- **圖層軌道 (`V1, V2, V3...`)**：圖層在視覺上由上至下層疊堆疊。在時間軸中位於上方的軌道（`V1`）在影格畫布上具備最高繪圖優先權（繪製在最頂層）。
- **軌道控制面板**：
  - 點擊 `V1` / `V2` 徽章標籤，可在不同繪圖圖層間快速切換。
  - 切換圖層可見度（眼睛圖示）以及編輯鎖定狀態（鎖頭圖示）。
  - 精確調節個別圖層的不透明度（`0%..=100%`）。
  - 在階層中向上/向下移動圖層、複製圖層，以及將圖層向下合併（Merge Down）至下方軌道。
  - 具備二次確認對話方塊的安全刪除圖層。
- **動畫影格與片段剪輯**：
  - 停留倍率（Hold）：為單影格指定自訂持續倍率（例如針對打擊頓挫或關鍵姿勢設定 `2×`、`3×` 停留）。
  - 關聯影格（Linked Frames）：在動畫不同位置引用相同影格實例（`🔗`），編輯其中任一處會自動同步更新所有關聯影格。
  - 標籤循環（Tag Looping）：將動畫組織為具名彩色標籤（如 `idle`、`walk`、`attack`），並在播放時限定循環指定標籤。
  - 播放控制：支援正向、反向、乒乓往復（Ping-Pong）與循環播放，影格率支援從 `1` 至 `120` FPS 自由微調。

### 3. 即時動畫播放預覽

右側專用的預覽停駐面板完全獨立於主畫布的縮放與視角平移位置，以真實速率即時播放動畫：
- **標籤篩選**：可單獨隔離播放特定動畫標籤（如 `walk`、`jump`），或預覽完整連續影格序列。
- **播放控制**：播放、暫停、單格後退、單格前進、循環切換，以及自訂播放變速（`0.25x..=4.0x`）。
- **視口背景模式**：自由切換深色棋盤格、淺色棋盤格、純白色、純黑色或自訂畫布底色。

### 4. 進階洋蔥皮引擎

利用專屬的洋蔥皮系統精雕細琢出流暢細膩的逐格動畫：
- **自訂色調**：前向影格（預設：紅色）與後向影格（預設：藍色）支援獨立的著色顏色。
- **透明度衰減**：可設定基礎不透明度及逐級平滑衰減曲線。
- **循環回繞 (Loop Wrap-Around)**：支援在循環動畫的首尾影格之間建立跨邊界透光殘影。
- **快速預覽 (Quick-Peek)**：無須拖曳滑鼠，按住 <kbd>Space</kbd> 空白鍵即可瞬間預覽相鄰影格動態。

### 5. 智慧參考圖與草圖描摹層

匯入手繪草稿或參考照片，大幅加速像素美術創作流程：
- **支援格式**：載入 PNG、JPEG 與 WebP 參考影像。
- **混合模式**：底層模式（Underlay，置於像素圖層下方）或覆蓋模式（Overlay，懸浮於像素圖層上方）。
- **變形控制**：獨立的縮放滑桿、位置位移、旋轉角度與不透明度調節。
- **智慧取色採樣**：使用滴管挑取草圖層顏色時，支援可選的自動吸附至目前專案使用中調色盤最接近的色彩。
- **一鍵烘焙 (1-Click Bake)**：一鍵將縮放對齊後的參考影像直接點陣化烘焙為使用中影格上的純粹像素畫。

### 6. 高精度選取套件與行軍蟻虛線

- **動態行軍蟻虛線**：使用中的選取範圍顯示清晰醒目的高對比雙色動態虛線邊界。
- **紮實泛洪多邊形套索**：相較於一般射線法在繪製重疊迴圈時會產生未選取的空心漏洞，OniAnvil 的套索先點陣化閉合的 8-連通邊界，再透過 4-連通演算法對外部進行泛洪填滿，百分之百保證選取內部紮實無孔洞。
- **豐富選取操作**：擴展、收縮、邊框（支援圓形、方形與菱形核心形狀）、平移、反向選取、刪除、自由旋轉以及系統剪貼簿複製/剪下/貼上。
- **跨影格複製與貼上**：在某一影格選定區域，按下 <kbd>Ctrl/Cmd+C</kbd>，切換至任一其他影格並按下 <kbd>Ctrl/Cmd+V</kbd>，即可跨任何鍵盤配置依相同相對座標精準貼上像素。

### 7. 即時特效、濾鏡與 RotSprite

所有影像特效皆內建即時**即時預覽 (Live Preview)** 核取方塊，直接在畫布上呈現處理結果，點擊取消不會污染復原歷史記錄：
- **取代顏色 (`Shift+R`)**：
  - 互動式滴管：點擊滴管圖示，直接從畫布像素或調色盤色票挑取色彩。
  - 快捷調色盤列：左鍵點擊挑取「原顏色」，右鍵點擊挑取「目標替換色」。
  - 容差調節滑桿與跨所有動畫影格一鍵批次取代。
- **色彩調整 (`Ctrl/Cmd+U`)**：即時調整 HSV 色相偏移、飽和度、明度與對比度。
- **自動外輪廓產生器**：為角色一鍵生成銳利的像素邊框（外側、內側、置中），支援方形或菱形轉角連接。
- **3×3 卷積矩陣濾鏡**：提供模糊、銳化、邊緣偵測與浮雕濾鏡，支援保留 Alpha 透明通道。
- **任意角度自由旋轉 (`Ctrl/Cmd+R` / `Ctrl/Cmd+Shift+R`)**：使用經典的 **RotSprite** 或 **最近鄰 (Nearest Neighbor)** 演算法將選取範圍或整格旋轉任意角度。
- **調色盤重對應**：使用 Bayer 2x2, 4x4, 8x8 矩陣或藍噪音（Blue Noise）仿色抖動將畫布影像精準色彩量化至目前調色盤。

### 8. 調色盤管理、色彩置換與角色皮膚

- **內建調色盤**：包含 20 多款真實復古調色盤，如 Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized 與 Sweetie 16。
- **匯入與匯出**：支援 GIMP 調色盤 (`.gpl`)、JASC PAL (`.pal`)、Hex 純文字十六進位 (`.hex`) 以及色票影像檔案 (`.png`)。
- **調色盤外觀替換 (Skins)**：為同一角色定義多種調色盤外觀變體（例如：預設、黃金重甲、暗影、冰霜）。切換外觀即可瞬間重對應並更新整個圖集上的所有對應影格。
- **漸層色階產生器**：在選定的兩種顏色之間結合 HSV 色相偏移自動插值產生平滑過渡色階，過渡步數自由微調。

### 9. 圖集切片器、九宮格 UI 與遊戲引擎匯出

- **圖集切片器 (Sheet Slicer)**：基於影像輪廓的自動相關性偵測演算法，自動將原始精靈表裁切為均勻對齊或緊湊邊界框的獨立動畫影格。
- **九宮格 (9-Grid / 9-Slice) 縮放**：設定自訂切片邊距（上、下、左、右），即時預覽可自由伸縮的 UI 彈出視窗與按鈕面板。
- **外擴像素 (Extruded Border) PNG 匯出**：自動將邊緣像素色彩向外擠出至透明間隙中，徹底消除 OpenGL、DirectX、Vulkan 以及 2D 遊戲引擎（如 Godot）在材質濾波時產生的接縫黑邊與溢色瑕疵。
- **動畫 GIF 匯出**：產生高度最佳化的動態 GIF，支援自訂循環次數、影格延遲時間與透明背景處理。
- **標準 Sprite Sheet JSON 匯出**：匯出影格切片、標籤、持續時長與軸心點座標，完全符合現代主流 2D 遊戲引擎通用的精靈圖集 JSON 資料結構規範。

### 10. 35 款人體工學主題與全套在地化

- **精選主題庫**：工作室深色 (Studio Dark)、工作室淺色 (Studio Light)、純黑 (Pitch Black)、賽博龐克霓虹 (Cyberpunk Neon)、合成波 (Synthwave)、北歐極光 (Nord Frost)、Dracula、東京之夜 (Tokyo Night)、Catppuccin Mocha、Gruvbox Dark、Monokai Pro、Solarized 深色/淺色、CRT 螢光幕（琥珀色、綠色、青色）、經典復古主機（Game Boy, Game Boy Light, Super Game Boy, 紅白機/NES, 超級任天堂/SNES, 世嘉 Genesis/Mega Drive, 初代 PlayStation 1, PlayStation 2, Dreamcast）與復古電腦（Commodore 64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, 經典 Windows 98）。
- **多語言全套在地化**：支援在全球 37 種語言之間即時切換，包含 **繁體中文** 與 **簡體中文**，所有詞條字典鍵實現 100% 完整對齊覆蓋。

---

## 快捷鍵與操作指南

| 快捷鍵 | 工具 / 操作 | 說明描述 |
|:---|:---|:---|
| <kbd>B</kbd> | **鉛筆** | 1px 高精度繪圖，支援可選的像素修正 (Pixel-Perfect) 模式 |
| <kbd>E</kbd> | **橡皮擦** | 將像素抹除為透明色 |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **滴管** | 從畫布像素或參考草圖挑取顏色 |
| <kbd>G</kbd> | **油漆桶 / 漸層** | 在油漆桶填滿與仿色漸層工具之間切換 |
| <kbd>L</kbd> | **直線** | 繪製像素直線，具備即時預覽，支援 Shift 角度吸附 |
| <kbd>C</kbd> | **曲線** | 三擊式二次貝茲像素曲線工具 |
| <kbd>U</kbd> | **矩形** | 幾何矩形與正方形的邊框描邊/實心填滿 |
| <kbd>O</kbd> | **橢圓** | 幾何橢圓與正圓形的邊框描邊/實心填滿 |
| <kbd>D</kbd> | **仿色筆刷** | 繪製規律棋盤格矩陣仿色圖樣 |
| <kbd>S</kbd> | **光影工具** | 沿調色盤順位逐步加深 (Burn) 或變亮 (Dodge) |
| <kbd>M</kbd> | **矩形選取** | 具備雙色行軍蟻虛線的矩形區域框選 |
| <kbd>Q</kbd> | **套索選取** | 徒手繪製紮實多邊形閉合選取 |
| <kbd>W</kbd> | **魔術棒** | 選取相連或全域相符的同色像素區域 |
| <kbd>P</kbd> | **軸心點與動作點** | 定位精靈原點中心與遊戲玩法事件錨點 |
| <kbd>X</kbd> | **切換前景色/背景色** | 交換主要前景色與次要背景色 |
| <kbd>[</kbd> / <kbd>]</kbd> | **筆刷尺寸** | 縮小 / 放大筆刷尺寸 (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **複製** | 將選取的像素區域複製至系統剪貼簿 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **剪下** | 將選取的像素區域剪下至系統剪貼簿 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **貼上** | 將剪貼簿像素貼上至目前使用中影格 |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **刪除** | 清除使用中選取範圍內的所有像素 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **全選** | 選取目前使用中影格的所有像素 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **取消選取** | 清除目前使用中的選取遮罩 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **反向選取** | 反轉目前使用中的選取遮罩 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **複製影格** | 在時間軸中建立目前影格的複本 |
| <kbd>Space</kbd> | **播放 / 暫停** | 切換時間軸動畫播放或暫停 |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **取代顏色** | 開啟支援即時預覽的取代顏色對話方塊 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **色彩調整** | 開啟 HSV 色相 / 飽和度 / 對比度調節對話方塊 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **自由旋轉** | 開啟任意角度旋轉對話方塊 (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **復原** | 復原上一步畫布筆畫或工具操作 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **重做** | 重做先前復原的操作 |
| <kbd>Space</kbd> + 拖曳 / 滑鼠中鍵 | **平移畫布** | 平滑無限平移畫布視口 |
| <kbd>Ctrl/Cmd</kbd> + 滾輪 / <kbd>+</kbd> / <kbd>-</kbd> | **縮放** | 以滑鼠游標為中心進行清晰平滑的像素縮放 |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **檢視模式切換** | 在單影格檢視 (<kbd>1</kbd>) 與圖集全景檢視 (<kbd>2</kbd>) 之間切換 |
| <kbd>F</kbd> | **畫布置中適應** | 自動將目前使用中影格置中並適應至工作區 |
| <kbd>F1</kbd> | **說明** | 開啟快捷鍵說明對話方塊 |

---

## 快速入門

### 獨立執行版 (預先編譯二進位程式)

直接從 **[GitHub Releases 發布頁面](https://github.com/rbm13s/OniAnvil/releases/latest)** 下載已預先編譯好的獨立執行檔：

- **Windows**: 下載 `OniAnvil-windows-x86_64.zip`，解壓縮並連按兩下執行 `OniAnvil.exe`。
- **Linux**: 下載 `OniAnvil-linux-x86_64.tar.gz`（免安裝可攜版）或安裝 `onianvil_*_amd64.deb` Debian/Ubuntu 套件：
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: 下載 `OniAnvil-macos-universal.zip`，解壓縮後將 `OniAnvil.app` 拖移至「應用程式」資料夾。通用二進位檔案原生支援 Apple Silicon (M1/M2/M3/M4) 與 Intel 架構處理器。
- **Web**: 無須安裝任何軟體，直接在現代瀏覽器中執行：**[啟動 WebAssembly 應用](https://rbm13s.github.io/OniAnvil/app/?lang=zh-tw)**。
  > [!WARNING]
  > WebAssembly (網頁端) 版本目前處於 **實驗性 (Experimental)** 狀態。由於瀏覽器沙箱機制、WebGL/WebGPU 差異以及系統剪貼簿存取限制，可能會遇到輕微的圖形顯示異常或效能瓶頸。對於正式生產創作，強烈建議使用原生桌面版。

---

### 從原始碼編譯

#### 環境準備
- **Rust Toolchain**: 建議透過官方 [rustup.rs](https://rustup.rs/) 安裝最新穩定版 Rust（建議 1.80+）：
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux 編譯相依套件** (Debian/Ubuntu 環境):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. 編譯並執行原生桌面版
```bash
# 複製儲存庫
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# 執行自動化測試套件
cargo test

# 編譯並執行經過完整最佳化的 Release 二進位程式
cargo run --release
```

#### 2. 編譯 WebAssembly 版本 (透過 Trunk)
```bash
# 安裝 wasm32 編譯目標與 Trunk 建置工具
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# 啟動支援熱重載的本機開發伺服器
trunk serve

# 將經過完整最佳化的生產檔案建置至 docs/app/ 目錄
./build_wasm.sh        # Linux / macOS 環境
./build_wasm.ps1       # Windows PowerShell 環境
```

---

## 檔案格式與伴隨檔案架構

OniAnvil 拒絕使用不透明的專有二進位格式。每個專案皆由兩個乾淨、對版本控制極為友善的檔案構成：

```
my_character/
├── my_character.png            # 標準 PNG 精靈圖集（可在任何遊戲引擎或影像檢視器中直接開啟）
└── my_character.onianvil.json   # 伴隨中繼資料側車（Sidecar）檔案
```

`.onianvil.json` 側車檔案記錄了：
- 畫布網格設定：影格尺寸、欄數、總影格數。
- 圖層階層體系：軌道名稱、不透明度數值、可見度旗標、鎖定狀態。
- 動畫標籤資料：標籤名稱、起始與結束影格索引、播放方向（正向、反向、乒乓往復）、影格持續時間。
- 調色盤定義：使用中顏色色票、具名外觀皮膚變體、目前使用中外觀索引。
- 參考草圖狀態：影像路徑、縮放比例、平移位移、旋轉角度、不透明度、圖層混合模式。
- 軸心點與錨點：自訂遊戲玩法動作點、武器錨點與碰撞中心原點。

---

## 參與貢獻與致謝

熱烈歡迎各種形式的貢獻、問題回報（Bug Reports）與新功能提案！
1. 在 GitHub 上 Fork 本儲存庫。
2. 建立您的特性分支：`git checkout -b feature/my-new-feature`。
3. 提交您的修改：`git commit -am 'Add new pixel tool'`。
4. 推送至您的分支：`git push origin feature/my-new-feature`。
5. 提出 Pull Request。

### 致謝
- 專案開箱即用的範例小貓精靈動畫 **"Cat 2D Pixel Art"** 由 **xzany** 創作（[itch.io 專案頁面](https://xzany.itch.io/cat-2d-pixel-art)）。
- 基於出色的 **[egui](https://github.com/emilk/egui)** GUI 介面庫與 **[wgpu](https://wgpu.rs/)** 圖形渲染生態建置。
- UI 圖示由 @fontawesome 的開源 **[Font Awesome Free](https://fontawesome.com)** 圖示庫提供支援（圖示：CC BY 4.0，字型：SIL OFL 1.1，程式碼：MIT License）。

---

## 授權條款與免責聲明

本專案遵循 **[MIT 授權條款](../LICENSE)** 開源。您可以免費使用於個人及商業遊戲開發流程中，亦可自由修改、散布與整合。

### 擔保免責與責任限制聲明
> [!IMPORTANT]
> **OniAnvil** 是一款**免費**散布且依**「現狀」 (AS IS)** 與**「現有」 (AS AVAILABLE)** 基礎提供的軟體，不包含任何明示或默示的擔保或條件。
> 
> 在任何情況下，作者、版權持有人或貢獻者均不對因使用或無法使用本軟體以任何方式引發的任何直接、間接、附帶、特殊、懲戒性或衍生性損害（包括但不限於資料遺失、檔案損毀、硬體故障、利潤損失或業務中斷）承擔任何法律責任，無論其基於何種責任理論，即使已被告知此類損害之可能性。使用本軟體的所有風險由您自行承擔。

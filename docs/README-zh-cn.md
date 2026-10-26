<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**专为游戏图集打造的高性能像素艺术与洋葱皮定格动画工作室**

[![构建与发布](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![下载量](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![渲染器](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![界面语言](https://img.shields.io/badge/界面语言-37_种语言-purple)](#10-35-款人体工学主题与全套本地化)
[![平台](https://img.shields.io/badge/平台-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#快速上手)
[![在线版](https://img.shields.io/badge/在线应用-浏览器即开即用_(实验性)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=zh-cn)
[![许可证: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 在浏览器中试用 OniAnvil (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=zh-cn)** *(⚠️ 实验性版本：可能存在 WebGL/WASM 浏览器沙箱限制及问题)* • **[🌐 官方文档网站](https://rbm13s.github.io/OniAnvil/zh-cn.html)**

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
**简体中文** •
[繁體中文](README-zh-tw.md) •
[日本語](README-ja.md) •
[한국어](README-ko.md)

</div>

---

## 目录

- [介绍](#介绍)
- [截图与视觉导览](#截图与视觉导览)
- [全面工具参考手册](#全面工具参考手册)
- [功能全景文档](#功能全景文档)
  - [1. 以图集为核心的双重架构](#1-以图集为核心的双重架构)
  - [2. 多轨道图层时间轴](#2-多轨道图层时间轴)
  - [3. 实时动画播放预览](#3-实时动画播放预览)
  - [4. 高级洋葱皮引擎](#4-高级洋葱皮引擎)
  - [5. 智能参考图与草图描摹层](#5-智能参考图与草图描摹层)
  - [6. 高精度选区套件与黑白蚂蚁线](#6-高精度选区套件与黑白蚂蚁线)
  - [7. 实时特效、滤镜与 RotSprite](#7-实时特效滤镜与-rotsprite)
  - [8. 调色板管理、换色与角色皮肤](#8-调色板管理换色与角色皮肤)
  - [9. 图集切片器、九宫格 UI 与游戏引擎导出](#9-图集切片器九宫格-ui-与游戏引擎导出)
  - [10. 35 款人体工学主题与全套本地化](#10-35-款人体工学主题与全套本地化)
- [快捷键与操作指南](#快捷键与操作指南)
- [快速上手](#快速上手)
  - [独立运行版 (预编译二进制程序)](#独立运行版-预编译二进制程序)
  - [从源码编译](#从源码编译)
- [文件格式与伴随文件架构](#文件格式与伴随文件架构)
- [参与贡献与致谢](#参与贡献与致谢)
- [许可证与免责声明](#许可证与免责声明)

---

## 介绍

**OniAnvil**（名称源自 **Oni**on-Skinning [洋葱皮透光] + **Anvil** [铁砧/精灵锻造台]）是一款基于 Rust 打造的开源、高性能像素艺术编辑器与逐帧动画工作室套件。它在 Windows、Linux 和 macOS 上以硬件加速的原生性能运行，同时也能作为无需安装的 WebAssembly 应用直接在现代网页浏览器中运行。

目前多数像素精灵编辑器通常将动画处理为一堆分散独立的图像文件，迫使开发者经历繁琐的导出、切片与重新打包流程。**OniAnvil 颠覆了这一范式：** 游戏精灵图集纹理（`atlas_rgba`）即为项目的唯一样本真相（Source of Truth）。关键帧单元格、图层轨道、动画标签以及碰撞轴心点始终与图集保持非破坏性实时同步。

### 为什么选择 OniAnvil？
- 🚀 **极致飞快的原生 Rust 性能**：亚毫秒级瞬间启动，由 `wgpu` 与 `egui 0.31` 驱动的清晰锐利图形渲染，拖动长距离复杂时间轴毫无卡顿。
- 🎬 **多轨道图层时间轴**：源自专业非线性剪辑（NLE）理念的图层轨道（`V1, V2, V3...`），支持自上而下图层合成、向下合并、关联副本帧以及自由配置的停顿持续时长（Hold）。
- 🧅 **头等支持的洋葱皮引擎**：多帧透光残影，支持独立配置的前后帧着色（前向红/后向蓝）、自定义不透明度衰减曲线与首尾无缝循环回绕。
- 🎨 **即时角色调色板皮肤换色**：一键生成日间/夜间变体、阵营队伍颜色或元素皮肤，支持全图集动态像素重映射与 GPL/PAL/HEX/PNG 调色板导入导出。
- 📦 **直供游戏引擎的规范导出**：支持导出边缘外扩（像素挤出）的缩放 PNG，从根源消除 2D 游戏引擎（如 Godot、Bevy、Raylib、Defold）中的双线性滤波接缝黑边与渗色，同步输出标准 Sprite Sheet JSON 元数据与优化动画 GIF。

---

## 截图与视觉导览

<div align="center">

### 1. 主画布与工作室工作区
*高性能像素工作空间，包含硬件加速视口、多轨道时间轴、实时动画预览、调色板色卡与高精度工具栏。*
<img src="screenshots/main.png" alt="OniAnvil 主工作区" width="950" />

<br/><br/>

### 2. 多轨道动画时间轴
*非线性多轨道时间轴，具备层叠图层行（`V1, V2...`）、关联帧实例（`🔗`）、停顿倍率（Hold）与彩色命名动画标签。*
<img src="screenshots/timeline.png" alt="OniAnvil 多轨道时间轴" width="950" />

<br/><br/>

### 3. 精密绘图工具与顶部操作栏
*完整 15 种专业绘图工具，支持标准与紧凑停靠模式，配有用于笔刷尺寸、对称、循环回绕与像素修正（Pixel-Perfect）切换的顶部快捷工具栏。*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil 顶部操作栏" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil 标准工具栏" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil 紧凑工具栏" width="300" />
</p>

<br/><br/>

### 4. 实时动画播放预览
*专属独立实时播放面板，支持按标签过滤、可变 FPS 帧率控制、缩放滑块以及视口背景切换（棋盘格、纯深色、纯浅色）。*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil 实时预览" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil 预览缩放" width="460" />
</p>

<br/><br/>

### 5. 调色板管理、色阶渐变与角色皮肤
*全功能色彩引擎，支持前景色/背景色卡切换、两点自动化色彩渐变坡道生成以及命名调色板皮肤置换。*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil 调色板与皮肤换色" width="950" />

<br/><br/>

### 6. 参考草图描摹与可配置洋葱皮
*支持将概念草图或照片导入为底层底图/顶层覆盖层，具备变换控制、带调色板吸附的智能取色以及多帧洋葱皮透光残影。*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil 草图与洋葱皮" width="950" />

<br/><br/>

### 7. 中心轴心点与动作点/碰撞锚点
*通过交互式画布手柄，直观设定精灵原点中心与自定义命名的玩法事件点（武器锚点、飞行道具发射口、头部、脚步判定等）。*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil 轴心点与动作点" width="950" />

<br/><br/>

### 8. 精灵图集切片器与九宫格 (9-Grid) 缩放
*基于自动相关边界检测的自动图集切片（例如猫咪行走循环），以及可缩放的九宫格 UI 边距切片配置。*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil 图集切片器" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 九宫格缩放" width="460" />
</p>

<br/><br/>

### 9. 实时图像特效与滤镜面板 (支持即时预览)
*所有特效对话框均支持画布实时预览、交互式吸管取色以及无损撤销取消。*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="轮廓生成器" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="替换颜色" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="色相饱和度对比度" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="矩阵滤镜" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="重映射至调色板" width="460" />
</p>

<br/><br/>

### 10. 画布网格与背景显示模式
*在像素网格、图块瓦片网格、深色棋盘格、浅色棋盘格与自定义纯色背景之间无缝切换。*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="网格与深色棋盘格" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="浅色棋盘格" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="自定义纯色背景" width="300" />
</p>

<br/><br/>

### 11. 新建项目向导
*在创建项目时快速配置画布尺寸、帧数、网格列数布局，并可直接从 20 多种经典复古调色板中挑选。*
<p align="center">
  <img src="screenshots/new_project.png" alt="新建项目向导" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="新建项目调色板" width="460" />
</p>

<br/><br/>

### 12. 35 款复古主机与现代主题
*包含工作室深色、浅色、CRT 琥珀/绿色/青色荧光屏、经典复古主机（PlayStation 1, Game Boy, SNES, Dreamcast, Genesis）以及现代设计师主题。*
<p align="center">
  <img src="screenshots/themes.png" alt="主题菜单" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT 琥珀色主题" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX 主题" width="300" />
</p>

<br/><br/>

### 13. 菜单系统与内置快捷键帮助
<p align="center">
  <img src="screenshots/file_menu.png" alt="文件菜单" width="180" />
  <img src="screenshots/edit_menu.png" alt="编辑菜单" width="180" />
  <img src="screenshots/select_menu.png" alt="选择菜单" width="180" />
  <img src="screenshots/view_menu.png" alt="视图菜单" width="180" />
  <img src="screenshots/project_menu.png" alt="项目菜单" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="快捷键帮助窗口" width="700" />
</p>

</div>

---

## 全面工具参考手册

OniAnvil 包含 15 款专业绘图、几何图形、选区及元数据配置工具。界面图标采用开源的 **Font Awesome Free** 图标套件 *(图标: CC BY 4.0, 字体: SIL OFL 1.1)*。

| 工具 | 图标 | 快捷键 | 笔刷尺寸 | 像素修正 (Pixel-Perfect) | 描述说明 |
|:---|:---:|:---:|:---:|:---:|:---|
| **铅笔** | ✏️ | <kbd>B</kbd> | 1..32 px | 是 | 基础绘图工具。1px 模式下支持 **像素修正 (Pixel-Perfect)**（自动消除恼人的 L 形转折锯齿孤点）。支持使用选区印章创建自定义画笔（`Ctrl/Cmd+B`）。 |
| **橡皮擦** | 🧹 | <kbd>E</kbd> | 1..32 px | 否 | 将像素擦除为完全透明（`[0, 0, 0, 0]`）。若有活动选区，则仅在选区范围内擦除。 |
| **吸管** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | 否 | 左键取色至首选前景色（Foreground）；右键取色至次选背景色（Background）。可从画布像素、图层或参考草图拾取颜色。 |
| **油漆桶** | 🪣 | <kbd>G</kbd> | 无 | 否 | 基于队列的高速 BFS 泛洪填充算法，具备可调颜色容差（`0..=255`）。支持相连区域填充与全局同色填充，受选区约束。 |
| **渐变** | 🌈 | <kbd>G</kbd> *(切换)* | 无 | 否 | 在前景色与背景色之间绘制线性与径向渐变。支持 Bayer（2x2, 4x4, 8x8）矩阵抖动与蓝噪声（Blue Noise）抖动，强度可调。 |
| **直线** | 📏 | <kbd>L</kbd> | 1..32 px | 是 | 基于布兰森汉姆（Bresenham）算法绘制直线并提供实时拖拽预览。按住 <kbd>Shift</kbd> 键可锁定为 45°/90° 整数倍角度。 |
| **曲线** | 〰️ | <kbd>C</kbd> | 1..32 px | 是 | 三击式二次贝塞尔（Bézier）像素曲线工具。第 1 次点击设定起点，第 2 次点击设定终点，移动鼠标调整弯曲曲率，第 3 次点击确认提交曲线。 |
| **矩形** | ⬜ | <kbd>U</kbd> | 1..32 px | 否 | 绘制线框轮廓或实心填充几何矩形。按住 <kbd>Shift</kbd> 可等比约束为 1:1 正方形。 |
| **椭圆** | ⭕ | <kbd>O</kbd> | 1..32 px | 否 | 采用中点光栅化算法绘制线框或实心椭圆。按住 <kbd>Shift</kbd> 可约束为完美正圆。 |
| **抖动笔刷** | 🏁 | <kbd>D</kbd> | 1..32 px | 否 | 在前景色与背景色之间绘制规则棋盘格有序抖动纹理，无需在调色板中引入多余的中间过渡色。 |
| **明暗着色工具** | 🌓 | <kbd>S</kbd> | 1..32 px | 否 | 根据当前活动调色板的色块排序，进行逐步加深（Burn）或减淡（Dodge）光影过渡绘制。 |
| **矩形选区** | ⬚ | <kbd>M</kbd> | 无 | 否 | 带动态黑白相间“走马灯蚂蚁线”边框的矩形框选工具。支持按住 <kbd>Shift</kbd> 加选与按住 <kbd>Alt</kbd> 减选。 |
| **套索选择** | ➰ | <kbd>Q</kbd> | 无 | 否 | 徒手多边形套索。采用 8-连通边界光栅化与 4-连通外部泛洪填充算法，确保即使在绘制自相交套索环时也能获得 100% 实心无孔洞的选区。 |
| **魔棒工具** | 🪄 | <kbd>W</kbd> | 无 | 否 | 选择相连连续区域（或全局匹配）的相同颜色像素，容差阈值可调。 |
| **轴心与动作点** | 🎯 | <kbd>P</kbd> | 无 | 否 | 交互式在画布上放置精灵中心原点与自定义游戏动作点（如武器挂点、飞行道具发射口、头部、脚步判定等），可完整导出为 JSON 元数据。 |

---

## 功能全景文档

### 1. 以图集为核心的双重架构

在 OniAnvil 中，主纹理图集（`atlas_rgba`）即代表游戏引擎直接加载的真正精灵表（Sprite Sheet）纹理：
- **单帧模式 (Frame Mode)**：聚焦于单个当前活动动画帧，提供专注纯粹的像素绘制画布。所作修改会即刻实时同步更新至图集纹理中的对应单元格。
- **图集模式 (Atlas Mode)**：完整平铺展示整张精灵表网格，便于全局宏观调整、跨帧对齐检查与批量编辑。
- **伴随文件 (Companion File)**：项目以标准的 `.png` 精灵图集图片保存，并附带一个轻量级透明的 `.onianvil.json` 伴随元数据文件，用于存储帧布局、图层层级、标签、停顿倍率及调色板皮肤。

### 2. 多轨道图层时间轴

底部时间轴借鉴了专业非线性视频剪辑软件的设计哲学：
- **图层轨道 (`V1, V2, V3...`)**：图层在视觉上自上而下层叠堆砌。在时间轴中位于上方的轨道（`V1`）在单帧画布上具有最高绘制优先级（渲染在最上方）。
- **轨道控制面板**：
  - 点击 `V1` / `V2` 徽章标签，可在不同绘制图层间快速切换。
  - 切换图层可见性（眼睛图标）以及编辑锁定状态（锁形图标）。
  - 精确调节各个图层的不透明度（`0%..=100%`）。
  - 在层级中向上/向下移动图层、复制图层，以及将图层向下合并（Merge Down）到下方轨道。
  - 带二次确认对话框的安全删除图层。
- **动画帧与片段剪辑**：
  - 停顿倍率（Hold）：为单帧分配自定义持续倍率（例如针对打击停顿或关键 pose 设置 `2×`、`3×` 停顿）。
  - 关联帧（Linked Frames）：在动画不同位置引用相同帧实例（`🔗`），编辑其中任意一个会自动同步更新所有关联帧。
  - 标签循环（Tag Looping）：将动画组织为具名彩色标签（如 `idle`、`walk`、`attack`），并在回放时限定循环指定标签。
  - 播放控制：支持正放、倒放、乒乓往复（Ping-Pong）与循环播放，帧率支持从 `1` 至 `120` FPS 自由微调。

### 3. 实时动画播放预览

右侧专用的预览停靠坞完全独立于主画布的缩放和视图平移位置，以真实速率实时播放动画：
- **标签过滤**：可单独隔离播放特定动画标签（如 `walk`、`jump`），或预览整个连续帧序列。
- **回放控制**：播放、暂停、单帧后退、单帧前进、循环切换，以及自定义播放变速（`0.25x..=4.0x`）。
- **视口背景模式**：自由切换深色棋盘格、浅色棋盘格、纯白色、纯黑色或自定义画布底色。

### 4. 高级洋葱皮引擎

利用专用的洋葱皮系统精雕细琢出流畅细腻的帧动画：
- **自定义色调**：前向帧（默认：红色）与后向帧（默认：蓝色）支持独立的着色颜色。
- **透明度衰减**：可配置基础不透明度及逐级平滑衰减曲线。
- **循环回绕 (Loop Wrap-Around)**：支持在循环动画的首尾帧之间建立跨边界透光残影。
- **快速预览 (Quick-Peek)**：无需拖动鼠标，按住 <kbd>Space</kbd> 空格键即可瞬间预览相邻帧动态。

### 5. 智能参考图与草图描摹层

导入手绘草稿或参考照片，大幅加快像素美术绘制流程：
- **支持格式**：加载 PNG、JPEG 和 WebP 参考图像。
- **混合模式**：底层模式（Underlay，置于像素图层下方）或覆盖模式（Overlay，悬浮于像素图层上方）。
- **变换控制**：独立的缩放滑块、位置偏移、旋转角度与不透明度调节。
- **智能取色采样**：使用吸管拾取草图层颜色时，支持可选的自动吸附至当前项目活动调色板中最接近的颜色。
- **一键烘焙 (1-Click Bake)**：一键将缩放对齐后的参考图直接光栅化烘焙为活动帧上的纯正像素画。

### 6. 高精度选区套件与黑白蚂蚁线

- **动态走马灯蚂蚁线**：活动选区显示清晰醒目的高对比度黑白双色动态虚线周长。
- **实心泛洪多边形套索**：相较于常规光线投射算法在绘制自重叠回路时会产生未选中的中空孔洞，OniAnvil 的套索先光栅化闭合的 8-连通边界，再通过 4-连通算法对外部进行泛洪填充，百分之百保证选区内部实心无空洞。
- **丰富选区操作**：扩展、收缩、边框（支持圆形、方形和菱形核形状）、平移、反向选择、删除、自由旋转以及系统剪贴板复制/剪切/粘贴。
- **跨帧复制与粘贴**：在某一帧选定区域，按下 <kbd>Ctrl/Cmd+C</kbd>，切换到任意其他帧并按下 <kbd>Ctrl/Cmd+V</kbd>，即可跨任意键盘布局按相同相对坐标精准粘贴像素。

### 7. 实时特效、滤镜与 RotSprite

所有图像特效均内置实时**即时预览 (Live Preview)** 复选框，直接在画布上显示处理结果，点击取消不会污染撤销历史记录：
- **颜色替换 (`Shift+R`)**：
  - 交互式吸管：点击吸管图标，直接从画布像素或调色板色卡上挑选颜色。
  - 快捷调色板条：左键单击选取“原颜色”，右键单击选取“目标替换色”。
  - 容差调节滑块与跨全动画帧一键批量替换。
- **色彩调整 (`Ctrl/Cmd+U`)**：实时调节 HSV 色相偏移、饱和度、明度与对比度。
- **自动外轮廓生成器**：为角色一键生成清晰的像素描边（外侧、内侧、居中），支持方形或菱形转角连接。
- **3×3 卷积矩阵滤镜**：提供模糊、锐化、边缘检测与浮雕滤镜，支持保留 Alpha 透明通道。
- **任意角度自由旋转 (`Ctrl/Cmd+R` / `Ctrl/Cmd+Shift+R`)**：使用经典的 **RotSprite** 或 **最近邻 (Nearest Neighbor)** 算法将选区或整帧旋转任意角度。
- **调色板重映射**：使用 Bayer 2x2, 4x4, 8x8 矩阵或蓝噪声（Blue Noise）抖动将画布图像精准色彩量化到当前调色板。

### 8. 调色板管理、换色与角色皮肤

- **内置调色板**：包含 20 多款真实复古调色板，如 Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized 和 Sweetie 16。
- **导入与导出**：支持 GIMP 调色板 (`.gpl`)、JASC PAL (`.pal`)、Hex 纯文本十六进制 (`.hex`) 以及色板图像文件 (`.png`)。
- **调色板换肤 (Skins)**：为同一角色定义多种调色板皮肤变体（例如：默认、黄金装甲、暗影、冰霜）。切换皮肤即可瞬间重映射并更新整个图集上的所有对应帧。
- **渐变色阶生成器**：在选定的两种颜色之间结合 HSV 色相偏移自动插值生成平滑过渡色阶，过渡步数自由调节。

### 9. 图集切片器、九宫格 UI 与游戏引擎导出

- **图集切片器 (Sheet Slicer)**：基于图像轮廓的自动相关性检测算法，自动将原始精灵表裁切为均匀对齐或紧凑包围框的独立动画帧。
- **九宫格 (9-Grid / 9-Slice) 缩放**：设定自定义切片边距（上、下、左、右），实时预览可自由伸缩的 UI 弹窗框与按钮面板。
- **外扩像素 (Extruded Border) PNG 导出**：自动将边缘像素颜色向外挤出到透明间隙中，从根本上根除 OpenGL、DirectX、Vulkan 以及 2D 游戏引擎（如 Godot）在纹理滤波时产生的接缝黑边与渗色瑕疵。
- **动画 GIF 导出**：生成高度优化的动画 GIF，支持自定义循环次数、帧延迟时间与透明背景处理。
- **标准 Sprite Sheet JSON 导出**：导出帧切片、标签、持续时长与轴心点坐标，完全符合现代主流 2D 游戏引擎通用的精灵图集 JSON 数据结构规范。

### 10. 35 款人体工学主题与全套本地化

- **精选主题库**：工作室深色 (Studio Dark)、工作室浅色 (Studio Light)、纯黑 (Pitch Black)、赛博朋克霓虹 (Cyberpunk Neon)、合成波 (Synthwave)、北欧极光 (Nord Frost)、Dracula、东京之夜 (Tokyo Night)、Catppuccin Mocha、Gruvbox Dark、Monokai Pro、Solarized 深色/浅色、CRT 荧光屏（琥珀色、绿色、青色）、经典复古主机（Game Boy, Game Boy Light, Super Game Boy, 红白机/NES, 超级任天堂/SNES, 世嘉 Genesis/Mega Drive, 初代 PlayStation 1, PlayStation 2, Dreamcast）与复古电脑（Commodore 64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, 经典 Windows 98）。
- **多语言全套本地化**：支持在全球 37 种语言之间即时切换，包含 **简体中文** 与 **繁体中文**，所有词条字典键实现 100% 完整对齐覆盖。

---

## 快捷键与操作指南

| 快捷键 | 工具 / 操作 | 描述说明 |
|:---|:---|:---|
| <kbd>B</kbd> | **铅笔** | 1px 高精度绘图，支持可选的像素修正 (Pixel-Perfect) 模式 |
| <kbd>E</kbd> | **橡皮擦** | 将像素擦除为透明色 |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **吸管** | 从画布像素或参考草图拾取颜色 |
| <kbd>G</kbd> | **油漆桶 / 渐变** | 在油漆桶填充与抖动渐变工具之间切换 |
| <kbd>L</kbd> | **直线** | 绘制像素直线，具备实时预览，支持 Shift 角度吸附 |
| <kbd>C</kbd> | **曲线** | 三击式二次贝塞尔像素曲线工具 |
| <kbd>U</kbd> | **矩形** | 几何矩形与正方形的线框描边/实心填充 |
| <kbd>O</kbd> | **椭圆** | 几何椭圆与正圆形的线框描边/实心填充 |
| <kbd>D</kbd> | **抖动笔刷** | 绘制规则棋盘格矩阵抖动图案 |
| <kbd>S</kbd> | **明暗着色工具** | 沿调色板顺位逐步加深 (Burn) 或减淡 (Dodge) |
| <kbd>M</kbd> | **矩形选区** | 带黑白走马灯蚂蚁线的矩形区域框选 |
| <kbd>Q</kbd> | **套索选择** | 徒手绘制实心多边形闭合选区 |
| <kbd>W</kbd> | **魔棒工具** | 选取连续或全局相匹配的同色像素区域 |
| <kbd>P</kbd> | **轴心点与动作点** | 定位精灵原点中心与游戏玩法事件锚点 |
| <kbd>X</kbd> | **切换前景色/背景色** | 交换首选前景色与次选背景色 |
| <kbd>[</kbd> / <kbd>]</kbd> | **笔刷尺寸** | 缩小 / 放大画笔尺寸 (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **复制** | 将选中的像素区域复制至系统剪贴板 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **剪切** | 将选中的像素区域剪切至系统剪贴板 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **粘贴** | 将剪贴板像素粘贴至当前活动帧 |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **删除** | 清除活动选区内的所有像素 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **全选** | 选取当前活动帧的所有像素 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **取消选择** | 清除当前活动选区遮罩 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **反向选择** | 反转当前活动选区遮罩 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **复制帧** | 在时间轴中创建当前帧的副本 |
| <kbd>Space</kbd> | **播放 / 暂停** | 切换时间轴动画播放或暂停 |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **替换颜色** | 打开支持即时预览的替换颜色对话框 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **调整颜色** | 打开 HSV 色相 / 饱和度 / 对比度调节对话框 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **自由旋转** | 打开任意角度旋转对话框 (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **撤销** | 撤销上一步画布笔画或工具操作 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **重做** | 恢复先前撤销的操作 |
| <kbd>Space</kbd> + 拖拽 / 鼠标中键 | **平移画布** | 平滑无限平移画布视口 |
| <kbd>Ctrl/Cmd</kbd> + 滚轮 / <kbd>+</kbd> / <kbd>-</kbd> | **缩放** | 以鼠标指针为中心进行清晰平滑的像素缩放 |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **视图模式切换** | 在单帧视图 (<kbd>1</kbd>) 与图集全景视图 (<kbd>2</kbd>) 之间切换 |
| <kbd>F</kbd> | **画布居中适应** | 自动将当前活动帧居中并适配至工作区 |
| <kbd>F1</kbd> | **帮助** | 打开快捷键说明帮助弹窗 |

---

## 快速上手

### 独立运行版 (预编译二进制程序)

直接从 **[GitHub Releases 发布页面](https://github.com/rbm13s/OniAnvil/releases/latest)** 下载已预先编译好的独立可执行文件：

- **Windows**: 下载 `OniAnvil-windows-x86_64.zip`，解压并双击运行 `OniAnvil.exe`。
- **Linux**: 下载 `OniAnvil-linux-x86_64.tar.gz`（免安装便携版）或安装 `onianvil_*_amd64.deb` Debian/Ubuntu 软件包：
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: 下载 `OniAnvil-macos-universal.zip`，解压后将 `OniAnvil.app` 拖入“应用程序”文件夹中。通用二进制文件原生支持 Apple Silicon (M1/M2/M3/M4) 与 Intel 架构处理器。
- **Web**: 无需安装任何软件，直接在现代浏览器中运行：**[启动 WebAssembly 应用](https://rbm13s.github.io/OniAnvil/app/?lang=zh-cn)**。
  > [!WARNING]
  > WebAssembly (网页端) 版本目前处于 **实验性 (Experimental)** 状态。由于浏览器沙箱环境机制、WebGL/WebGPU 差异以及系统剪贴板访问限制，可能会遇到轻微的图形显示瑕疵或性能瓶颈。对于正式生产创作，强烈推荐使用原生桌面版。

---

### 从源码编译

#### 环境准备
- **Rust Toolchain**: 推荐通过官方 [rustup.rs](https://rustup.rs/) 安装最新稳定版 Rust（建议 1.80+）：
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux 编译依赖** (Debian/Ubuntu 环境):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. 编译并运行原生桌面版
```bash
# 克隆仓库
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# 运行自动化测试套件
cargo test

# 编译并运行经过全面优化的 Release 二进制程序
cargo run --release
```

#### 2. 编译 WebAssembly 版本 (通过 Trunk)
```bash
# 安装 wasm32 编译目标与 Trunk 构建工具
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# 启动支持热重载的本地开发服务器
trunk serve

# 将经过全面优化的生产文件构建到 docs/app/ 目录
./build_wasm.sh        # Linux / macOS 环境
./build_wasm.ps1       # Windows PowerShell 环境
```

---

## 文件格式与伴随文件架构

OniAnvil 拒绝使用不透明的专有二进制格式。每个项目均由两个干净、对版本控制极度友好的文件构成：

```
my_character/
├── my_character.png            # 标准 PNG 精灵图集（可在任何游戏引擎或图像查看器中打开）
└── my_character.onianvil.json   # 伴随元数据侧车（Sidecar）文件
```

`.onianvil.json` 侧车文件保存了：
- 画布网格配置：帧尺寸、列数、总帧数。
- 图层层级体系：轨道名称、不透明度数值、可见性标记、锁定状态。
- 动画标签数据：标签名称、起止帧索引、播放方向（正向、反向、乒乓往复）、帧持续时长。
- 调色板定义：活动颜色色板、具名皮肤变体、当前活动皮肤索引。
- 参考草图状态：图片路径、缩放比例、平移位移、旋转角度、不透明度、图层混合模式。
- 轴心点与锚点：自定义玩法动作点、武器挂点与碰撞中心原点。

---

## 参与贡献与致谢

热烈欢迎各种形式的贡献、问题反馈（Bug Reports）与新功能提案！
1. 在 GitHub 上 Fork 本仓库。
2. 创建您的特性分支：`git checkout -b feature/my-new-feature`。
3. 提交您的修改：`git commit -am 'Add new pixel tool'`。
4. 推送至您的分支：`git push origin feature/my-new-feature`。
5. 提交 Pull Request。

### 致谢
- 项目开箱即用的示例小猫精灵动画 **"Cat 2D Pixel Art"** 由 **xzany** 创作（[itch.io 项目主页](https://xzany.itch.io/cat-2d-pixel-art)）。
- 基于出色的 **[egui](https://github.com/emilk/egui)** GUI 界面库与 **[wgpu](https://wgpu.rs/)** 图形渲染生态构建。
- UI 图标由 @fontawesome 的开源 **[Font Awesome Free](https://fontawesome.com)** 图标库提供支持（图标：CC BY 4.0，字体：SIL OFL 1.1，代码：MIT License）。

---

## 许可证与免责声明

本项目遵循 **[MIT 许可证](../LICENSE)** 开源。您可以免费用于个人及商业游戏开发管线中，亦可自由修改、分发与整合。

### 担保免责与责任限制声明
> [!IMPORTANT]
> **OniAnvil** 是一款**免费**分发且按**“现状” (AS IS)** 与**“现有” (AS AVAILABLE)** 基础提供的软件，不包含任何明示或暗示的保证或条件。
> 
> 在任何情况下，作者、版权持有人或贡献者均不对因使用或无法使用本软件以任何方式引发的任何直接、间接、附带、特殊、惩戒性或后果性损害（包括但不限于数据丢失、文件损坏、硬件故障、利润损失或业务中断）承担任何法律责任，无论其基于何种责任理论，即使已被告知此类损害的可能性。使用本软件的所有风险由您自行承担。

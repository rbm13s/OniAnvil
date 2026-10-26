<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil ロゴ" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**ゲーム用スプライトアトラスに最適化されたピクセルアート＆オニオンスキンアニメーションスタジオ**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Languages](https://img.shields.io/badge/Languages-37_Locales-purple)](#10-35種類のテーマと多言語ローカライズ)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#はじめ方)
[![Web Demo](https://img.shields.io/badge/Online_App-Try_in_Browser_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ja)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 ブラウザでOniAnvilを試す (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ja)** *(⚠️ 実験的ビルド: WebGL/WASMの制約や不具合が含まれる可能性があります)* • **[🌐 公式ドキュメントサイト](https://rbm13s.github.io/OniAnvil/ja.html)**

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
[繁體中文](README-zh-tw.md) •
**日本語** •
[한국어](README-ko.md)

</div>

---

## 目次

- [はじめに](#はじめに)
- [スクリーンショット＆ビジュアルツアー](#スクリーンショットビジュアルツアー)
- [包括的ツールリファレンス](#包括的ツールリファレンス)
- [機能詳細ドキュメント](#機能詳細ドキュメント)
  - [1. アトラス中心のデュアルアーキテクチャ](#1-アトラス中心のデュアルアーキテクチャ)
  - [2. マルチトラックレイヤータイムライン](#2-マルチトラックレイヤータイムライン)
  - [3. リアルタイムアニメーションプレビュー](#3-リアルタイムアニメーションプレビュー)
  - [4. 高度なオニオンスキンエンジン](#4-高度なオニオンスキンエンジン)
  - [5. 参照スケッチ＆トレースレイヤー](#5-参照スケッチトレースレイヤー)
  - [6. 高精度な選択ツールとマーチングアンツ](#6-高精度な選択ツールとマーチングアンツ)
  - [7. リアルタイムエフェクト・フィルター・RotSprite](#7-リアルタイムエフェクトフィルターrotsprite)
  - [8. パレット管理・スワップ・キャラクタースキン](#8-パレット管理スワップキャラクタースキン)
  - [9. シートスライサー・9スライスUI・ゲームエンジン出力](#9-シートスライサー9スライスuiゲームエンジン出力)
  - [10. 35種類のテーマと多言語ローカライズ](#10-35種類のテーマと多言語ローカライズ)
- [操作方法＆キーボードショートカット](#操作方法キーボードショートカット)
- [はじめ方](#はじめ方)
  - [ビルド済みバイナリ](#ビルド済みバイナリ)
  - [ソースコードからのビルド](#ソースコードからのビルド)
- [ファイル形式とコンパニオン構造](#ファイル形式とコンパニオン構造)
- [貢献と謝辞](#貢献と謝辞)
- [ライセンスと免責事項](#ライセンスと免責事項)

---

## はじめに

**OniAnvil**（**Oni**on-Skinning〈オニオンスキン〉+ **Anvil**〈金床：スプライトを鍛造する場所〉）は、Rustで構築されたオープンソースの超高速ピクセルアートエディタ＆アニメーション制作スイートです。Windows、Linux、macOS上でハードウェアアクセラレーションによるネイティブ描画で快適に動作するほか、モダンなWebブラウザ上でもインストール不要のWebAssemblyアプリケーションとして直接動作します。

多くの既存スプライトエディタは、アニメーションを個別の画像ファイルの集まりとして扱うため、ゲーム開発者はスライシングやパッキング、再エクスポートといった煩雑な作業を強いられていました。**OniAnvilはこのパラダイムを根本から刷新します**。ゲームのスプライトシートアトラステクスチャ（`atlas_rgba`）が常に第一の「信頼できる唯一の情報源（Single Source of Truth）」です。フレームセル、レイヤートラック、アニメーションタグ、当たり判定ピボットは、常にアトラスと完全かつ非破壊に同期し続けます。

### なぜOniAnvilなのか？
- 🚀 **Rustによる圧倒的なネイティブパフォーマンス**: 1ミリ秒未満の瞬時起動、`wgpu`と`egui 0.31`による鮮明なハードウェア描画、長大なタイムラインのシークでも一切のラグなし。
- 🎬 **マルチトラックレイヤータイムライン**: ノンリニア動画編集ソフトに着想を得たレイヤートラック（`V1, V2, V3...`）。上から下への合成、下のレイヤーと統合、リンクフレーム、可変ホールド（コマ打ち）に対応。
- 🧅 **ファーストクラスのオニオンスキン**: 前後フレームの個別色付け（前：赤、後：青）、滑らかな不透明度減衰カーブ、ループ境界をまたぐゴースト表示に対応。
- 🎨 **瞬時に切り替え可能なキャラクタースキン（パレットスワップ）**: アトラス全体のカラーリマップにより、昼夜バリエーションやチームカラー、属性スキンを瞬時に生成。GPL/PAL/HEXパレットの入出力に対応。
- 📦 **ゲームエンジンへ直接エクスポート**: 2Dエンジン（Godot、Bevy、Raylib、Defoldなど）でのテクスチャブリーディング（隙間の滲み）を防ぐ押し出し境界（Extruded border）付きPNG、標準Sprite Sheet JSONメタデータ、最適化GIFアニメーションを出力可能。

---

## スクリーンショット＆ビジュアルツアー

<div align="center">

### 1. メインキャンバス＆スタジオワークスペース
*ハードウェアアクセラレーション対応ビューポート、マルチトラックタイムライン、リアルタイムプレビュー、パレットスウォッチ、高精度ツールバーを備えた高性能ピクセルワークスペース。*
<img src="screenshots/main.png" alt="OniAnvil メインワークスペース" width="950" />

<br/><br/>

### 2. マルチトラックアニメーションタイムライン
*レイヤー行の積み重ね（`V1, V2...`）、リンクフレームインスタンス（`🔗`）、ホールド倍率設定、色分けされたアニメーションタグを備えたノンリニアマルチトラックタイムライン。*
<img src="screenshots/timeline.png" alt="OniAnvil マルチトラックタイムライン" width="950" />

<br/><br/>

### 3. 高精度ツール群＆クイックアクションツールバー
*標準・コンパクトドッキングに対応した全15種類の描画ツール群と、ブラシサイズ、対称軸、ラップアラウンド、ピクセルパーフェクトの切り替えを行う上部ツールバー。*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil アクションツールバー" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil 標準ツールパネル" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil コンパクトツールパネル" width="300" />
</p>

<br/><br/>

### 4. リアルタイムアニメーションプレビュー
*タグフィルタリング、可変FPS再生制御、ズームスライダー、ビューポート背景切り替え（市松模様、ダーク単色、ライト単色）を備えた独立ライブプレビュードック。*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil ライブプレビュー" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil ライブプレビューのズーム" width="460" />
</p>

<br/><br/>

### 5. パレット管理・カラースワップ・カラーランプ
*前景色／背景色のスウォッチボックス、2色間の自動カラーランプ生成、名前付きパレットスキンスワップを備えた総合カラーエンジン。*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil パレットとスキンスワップ" width="950" />

<br/><br/>

### 6. 参照スケッチの下絵トレース＆高機能オニオンスキン
*コンセプトスケッチや写真画像を変形制御付きで下絵／上絵として読み込み、パレット吸着付きスポイトによるサンプリングと複数フレームのオニオンスキン表示が可能。*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil スケッチとオニオンスキン" width="950" />

<br/><br/>

### 7. ピボットポイント＆当たり判定アクションポイント
*スプライトの原点中心（Pivot）とカスタム命名のゲームプレイポイント（武器の持ち手、弾の発射位置、頭部、足音など）をキャンバスギズモでインタラクティブに定義。*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil ピボットとアクションポイント" width="950" />

<br/><br/>

### 8. スプライトシートスライサー＆9スライス（9-Grid）UIスケーラー
*オートコリレーションによる境界自動検出スライシング（猫の歩行サイクルなど）と、拡縮可能な9スライスUIボックスのマージン設定。*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil シートスライサー" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9スライス設定" width="460" />
</p>

<br/><br/>

### 9. リアルタイム画像エフェクト＆フィルター（ライブプレビュー対応）
*すべてのエフェクトダイアログがキャンバス上のリアルタイムプレビュー、スポイトによるカラーピッキング、非破壊キャンセルに対応。*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="アウトラインジェネレーター" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="色の置換" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="色相・彩度・コントラスト調整" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="行列畳み込みフィルター" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="パレットへのリマップ" width="460" />
</p>

<br/><br/>

### 10. キャンバスグリッド＆背景表示モード
*ピクセルグリッド、タイルグリッド、暗い市松模様、明るい市松模様、カスタム単色背景をシームレスに切り替え。*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="グリッドと暗い市松模様" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="グリッドなし明るい市松模様" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="カスタム単色背景" width="300" />
</p>

<br/><br/>

### 11. 新規プロジェクト作成ウィザード
*キャンバスの寸法、フレーム数、カラムグリッド配置を設定し、起動時に20種類以上の本格レトロパレットから選択可能。*
<p align="center">
  <img src="screenshots/new_project.png" alt="新規プロジェクトウィザード" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="プロジェクトパレット選択" width="460" />
</p>

<br/><br/>

### 12. 35種類の本格レトロ＆モダンテーマ
*Studio Dark、Studio Light、CRTアンバー/グリーン/シアン蛍光体、往年のレトロ家庭用ゲーム機（PlayStation 1、Game Boy、SNES、Dreamcast、Genesis）、デザイナーテーマを収録。*
<p align="center">
  <img src="screenshots/themes.png" alt="テーマ選択メニュー" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRTアンバーテーマ" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSXテーマ" width="300" />
</p>

<br/><br/>

### 13. メニューシステム＆キーボードショートカットヘルプ
<p align="center">
  <img src="screenshots/file_menu.png" alt="ファイルメニュー" width="180" />
  <img src="screenshots/edit_menu.png" alt="編集メニュー" width="180" />
  <img src="screenshots/select_menu.png" alt="選択メニュー" width="180" />
  <img src="screenshots/view_menu.png" alt="表示メニュー" width="180" />
  <img src="screenshots/project_menu.png" alt="プロジェクトメニュー" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="ショートカットヘルプモーダル" width="700" />
</p>

</div>

---

## 包括的ツールリファレンス

OniAnvilには、描画・図形・選択・メタデータ配置のための15種類の高精度ツールが備わっています。アイコンはオープンソースの **Font Awesome Free** アイコンスイートを使用しています *(Icons: CC BY 4.0, Fonts: SIL OFL 1.1)*。

| ツール | アイコン | ショートカット | ブラシサイズ | ピクセルパーフェクト | 説明 |
|:---|:---:|:---:|:---:|:---:|:---|
| **鉛筆** | ✏️ | <kbd>B</kbd> | 1..32 px | あり | メインの描画ツール。1pxモードでは**ピクセルパーフェクト**描画をサポート（余計なL字角の階段状アーティファクトを自動排除）。選択範囲からのカスタムブラシ作成スタンプ（`Ctrl/Cmd+B`）に対応。 |
| **消しゴム** | 🧹 | <kbd>E</kbd> | 1..32 px | なし | ピクセルを透明（`[0, 0, 0, 0]`）に消去。選択範囲マスクが有効な場合は範囲内のみ消去。 |
| **スポイト** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | なし | 左クリックで前景色（メイン）、右クリックで背景色（サブ）を抽出。キャンバス、レイヤー、参照スケッチから色を取得可能。 |
| **塗りつぶし** | 🪣 | <kbd>G</kbd> | なし | なし | キューベースの高速BFS塗りつぶし。色の許容値（Tolerance: `0..=255`）設定、隣接領域のみ／全体の同色塗りつぶし、選択範囲による制限に対応。 |
| **グラデーション** | 🌈 | <kbd>G</kbd> *(切替)* | なし | なし | 前景色と背景色の間で線形および放射状グラデーションを描画。Bayer（2x2, 4x4, 8x8）およびBlue Noiseによるディザリング強度調整に対応。 |
| **直線** | 📏 | <kbd>L</kbd> | 1..32 px | あり | リアルタイムプレビュー付きブレゼンハム直線アルゴリズム。<kbd>Shift</kbd>を押すと角度を45°/90°にスナップ。 |
| **曲線** | 〰️ | <kbd>C</kbd> | 1..32 px | あり | 3クリック操作の2次ベジェ曲線ツール。クリック1で始点、クリック2で終点を設定し、マウス移動で湾曲を調整、クリック3で曲線を確定。 |
| **長方形** | ⬜ | <kbd>U</kbd> | 1..32 px | なし | 輪郭線または塗りつぶしの幾何学的長方形。<kbd>Shift</kbd>を押しながらドラッグすると1:1の正方形に固定。 |
| **楕円** | ⭕ | <kbd>O</kbd> | 1..32 px | なし | 中点アルゴリズムによる輪郭線または塗りつぶしの楕円。<kbd>Shift</kbd>キーで正円に固定。 |
| **ディザブラシ** | 🏁 | <kbd>D</kbd> | 1..32 px | なし | パレットに余計な中間色を追加することなく、前景色と背景色の交互市松模様ディザパターンで描画。 |
| **シェーディングツール** | 🌓 | <kbd>S</kbd> | 1..32 px | なし | アクティブパレットの並び順に基づいて、段階的なハイライト（Dodge・覆い焼き）とシャドウ（Burn・焼き込み）を適用。 |
| **矩形選択** | ⬚ | <kbd>M</kbd> | なし | なし | アニメーションするマーチングアンツ（点線）表示の矩形選択ツール。<kbd>Shift</kbd>で追加、<kbd>Alt</kbd>で除外。 |
| **なげなわ選択** | ➰ | <kbd>Q</kbd> | なし | なし | フリーハンド多角形なげなわ。8近傍境界ラスタライズと外部4近傍フラッドフィルを採用し、交差ループを描いた場合でも内部に穴が開かない100%完全な中実選択を保証。 |
| **自動選択 (マジックワンド)** | 🪄 | <kbd>W</kbd> | なし | なし | 許容値の調整が可能な、隣接（またはキャンバス全体）の近似色ピクセル領域選択。 |
| **ピボット＆アクションポイント** | 🎯 | <kbd>P</kbd> | なし | なし | スプライト原点（Pivot）およびゲームプレイ用アクションポイント（武器の持ち手、頭部、足元、ヒットボックス）をキャンバス上でインタラクティブに配置し、JSONメタデータへ出力。 |

---

## 機能詳細ドキュメント

### 1. アトラス中心のデュアルアーキテクチャ

OniAnvilでは、マスターテクスチャアトラス（`atlas_rgba`）がゲームエンジンに直接読み込まれる真のスプライトシートテクスチャそのものを表しています。
- **フレームモード（Frame Mode）**: 作業中の単一アニメーションフレームに焦点を当て、余計な情報のない集中できるピクセルキャンバスを提供します。編集内容は即座にアトラステクスチャ内の対応するセルへ反映されます。
- **アトラスモード（Atlas Mode）**: スプライトシート全体のグリッドを表示し、全体的な配置確認、フレーム間の整合性チェック、一括編集を可能にします。
- **コンパニオンファイル構造**: プロジェクトはプレーンな `.png` スプライトシート画像と、フレーム配置、レイヤー階層、タグ、ホールド倍率、パレットスキン情報を収めた軽量な `.onianvil.json` メタデータファイルとして透過的に保存されます。

### 2. マルチトラックレイヤータイムライン

下部に配置されたタイムラインは、プロ用ノンリニア動画編集ソフトウェア（NLE）の設計思想を踏襲しています。
- **レイヤートラック（`V1, V2, V3...`）**: レイヤーは視覚的に上から下へと積み重なります。タイムラインの上位にあるトラック（`V1`）が、フレームキャンバス上で最前面に合成描画されます。
- **トラックコントロール**:
  - `V1` / `V2` バッジをクリックして、アクティブな描画レイヤーを素早く切り替え。
  - レイヤーの表示・非表示の切り替え（目のアイコン）および編集ロック（鍵のアイコン）。
  - 個別レイヤーの不透明度調整（`0%..=100%`）。
  - 階層内の上下移動、レイヤーの複製、直下のトラックへの統合（下のレイヤーと結合）。
  - 確認ダイアログ付きの安全なレイヤー削除。
- **フレームクリップ機能**:
  - ホールド倍率（Hold Multiplier）: 個々のフレームに任意の表示ウェイト（例: タメやヒットストップ用の `2×`, `3×` ホールド）を設定可能。
  - リンクフレーム: アニメーション内の繰り返しフレームをリンクさせ、1箇所を編集するとすべてのリンクインスタンスへ自動的に同期。
  - アニメーションタグ: アニメーションを名前付きタグ（`idle`, `walk`, `attack`）に整理し、再生中に特定タグのみをループ再生。
  - 再生制御: 順再生、逆再生、ピンポン（往復）、ループ再生、および可変FPS（`1..=120`）に対応。

### 3. リアルタイムアニメーションプレビュー

右側のプレビュードックは、キャンバスの拡大率やスクロール位置に一切左右されることなく、アニメーションをリアルタイムでループ再生します。
- **タグフィルタリング**: 特定のタグ（`walk`, `jump` など）だけを絞り込んでプレビューするか、タイムライン全体のシーケンスを通しで確認可能。
- **再生コントロール**: 再生、一時停止、前フレームへ移動、次フレームへ移動、ループ切り替え、再生速度倍率（`0.25x..=4.0x`）。
- **表示モード**: 暗い市松模様、明るい市松模様、純白、完全な黒、カスタム単色背景を自由に切り替え。

### 4. 高度なオニオンスキンエンジン

オニオンスキン専用システムにより、滑らかで躍動感のあるアニメーションを直感的に制作できます。
- **独立カラーティント**: 前のフレーム（デフォルト: 赤）と次のフレーム（デフォルト: 青）にそれぞれ独立した表示色を設定可能。
- **不透明度の減衰カーブ**: 基本不透明度と、ステップごとの段階的減衰量を細かく調整可能。
- **ループラップアラウンド**: ループアニメーションの制作時に、先頭フレームと末尾フレームの間を透過表示。
- **クイックプレビュー（Quick-Peek）**: マウスをドラッグせずに <kbd>Space</kbd> キーを押し続けるだけで、隣接フレームを瞬時に確認。

### 5. 参照スケッチ＆トレースレイヤー

手描きの下絵スケッチや写真資料を読み込んでトレースすることで、制作作業を大幅に加速できます。
- **対応フォーマット**: PNG、JPEG、WebP 形式の画像をインポート可能。
- **合成モード**: アンダーレイ（ピクセルレイヤーの背面）またはオーバーレイ（ピクセルレイヤーの前面）。
- **変形コントロール**: 独立した拡大縮小、位置オフセット、回転角度、不透明度スライダー。
- **スマートカラーサンプリング**: スケッチ上でスポイトツールを使用する際、アクティブなプロジェクトパレット内の最も近い色へ自動吸着（スナップ）させるオプションを装備。
- **ワンクリック焼き込み（1-Click Bake）**: 拡縮・配置した参照画像を、アクティブフレームのピクセルアートとして1クリックでキャンバスへ定着。

### 6. 高精度な選択ツールとマーチングアンツ

- **アニメーションするマーチングアンツ**: アクティブな選択範囲は、高視認性の2色点線ループアニメーション（PhotoshopやAseprite同等のマーチングアンツ）で表示されます。
- **中実フラッドフィルなげなわ選択**: 従来の単純なレイキャスティング方式では自己交差ループを描いた際に内部が抜けてしまう問題がありましたが、OniAnvilのなげなわツールは閉じた8近傍境界をラスタライズし、外側領域を4近傍フラッドフィルすることで、どんな形状でも100%隙間のない中実選択を保証します。
- **豊富な選択範囲操作**: 拡大、縮小、境界線化（円形、正方形、菱形のカーネル形状を選択可能）、平行移動、反転、消去、自由回転、システムクリップボードを介したコピー／切り取り／貼り付け。
- **フレーム間をまたぐコピー＆ペースト**: あるフレームで領域を選択して <kbd>Ctrl/Cmd+C</kbd> を押し、別のフレームに切り替えて <kbd>Ctrl/Cmd+V</kbd> を押すだけで、同一の相対座標へ正確にペースト可能。あらゆるキーボード配列で安定して動作します。

### 7. リアルタイムエフェクト・フィルター・RotSprite

すべての画像エフェクトにはリアルタイムな **ライブプレビュー（Live Preview）** チェックボックスが備わっており、アンドゥ履歴を汚すことなく、キャンバス上で結果を確認しながら安全に適用・キャンセルできます。
- **色の置換（`Shift+R`）**:
  - インタラクティブスポイト: スポイトアイコンをクリックして、フレームのピクセルやパレットドックから直接色を採取。
  - クイックパレットストリップ: 左クリックで「変更前」色、右クリックで「変更後」色を選択。
  - 許容値スライダーと、全アニメーションフレームに対する一括置換（バッチ処理）に対応。
- **色調補正（`Ctrl/Cmd+U`）**: HSV色相シフト、彩度、明度、コントラストをリアルタイムに調整。
- **自動アウトラインジェネレーター**: ドット絵キャラクターの外枠（外側、内側、中央）を正方形または菱形の角結合でシャープに生成。
- **3×3 行列畳み込みフィルター**: ぼかし（Blur）、シャープ化（Sharpen）、輪郭検出（Edge Detect）、エンボス（Emboss）を透明度保護付きで適用可能。
- **自由回転（`Ctrl/Cmd+R`）**: 選択範囲またはフレーム全体を、伝説的な **RotSprite** アルゴリズムまたは Nearest Neighbor アルゴリズムを用いて任意の角度で美しく回転。
- **パレットへのリマップ**: Bayer 2x2, 4x4, 8x8、または Blue Noise ディザリングを用いて、キャンバスのフルカラーをアクティブパレットへ減色量子化。

### 8. パレット管理・スワップ・キャラクタースキン

- **内蔵レトロパレット**: Game Boy、PICO-8、Commodore 64、NES、Endesga 32、Solarized、Sweetie 16 など、20種類以上の厳選パレットを初期収録。
- **インポート＆エクスポート**: GIMP Palette（`.gpl`）、JASC PAL（`.pal`）、Hex（`.hex`）、画像カラーストリップ（`.png`）の相互読み込み・書き出しに対応。
- **パレットスワップ（スキンシステム）**: 1体のキャラクターに対して複数のカラーパレットバリエーション（例: 通常、黄金の鎧、シャドウ、氷結）を定義。スキンを切り替えるだけで、アトラス全体の全フレームカラーが一瞬で再マッピングされます。
- **カラーランプジェネレーター**: 指定した2色の色相間を滑らかに補間するグラデーションステップを自動生成。

### 9. シートスライサー・9スライスUI・ゲームエンジン出力

- **シートスライサー**: 自己相関による境界認識アルゴリズムが、未加工のスプライトシート画像を規則的なアニメーションフレームへと自動分割。
- **9スライス（9-Grid）スケーラー**: 上下左右の境界マージンを設定し、四隅を歪ませることなく自在に拡縮できるUIウィンドウやボタンパネルをプレビュー。
- **押し出し境界（Extruded Border）PNGエクスポート**: 各セルの外周ピクセルを透明マージン方向へ1px押し出して複製出力。OpenGL、DirectX、Vulkan、Godot Engineなどのテクスチャフィルタリング時に発生するタイルの継ぎ目（シーム）や滲みを完全に解消します。
- **最適化アニメーションGIF出力**: ループ回数指定、フレーム遅延、背景透過に対応したGIF生成エンジン。
- **標準Sprite Sheet JSON**: 現代の2Dゲームエンジンで広く採用されている標準Sprite Sheetメタデータスキーマに準拠したJSONファイルを出力。

### 10. 35種類のテーマと多言語ローカライズ

- **厳選された35種類のUIテーマ**: Studio Dark、Studio Light、Pitch Black、Cyberpunk Neon、Synthwave、Nord Frost、Dracula、Tokyo Night、Catppuccin Mocha、Gruvbox Dark、Monokai Pro、Solarized Dark、Solarized Light、CRTアンバー/グリーン/シアン蛍光体、レトロゲーム機（Game Boy、Game Boy Light、Super Game Boy、NES/ファミコン、スーパーファミコン、メガドライブ、PlayStation 1、PlayStation 2、Dreamcast）、レトロPC（C64、ZX Spectrum、Amiga Workbench、Atari ST、MSX、Apple II、DOS CGA、Mac Classic、Windows 98）。
- **37言語の国際化ローカライズ**: すべてのUIテキスト、ツールチップ、ダイアログメッセージが多言語に対応。

---

## 操作方法＆キーボードショートカット

| ショートカット | ツール / 操作 | 説明 |
|:---|:---|:---|
| <kbd>B</kbd> | **鉛筆** | 1px精度の描画。オプションでピクセルパーフェクトモード対応 |
| <kbd>E</kbd> | **消しゴム** | ピクセルを透明に消去 |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **スポイト** | キャンバスピクセルまたは下絵スケッチから色を抽出 |
| <kbd>G</kbd> | **塗りつぶし / グラデーション** | 塗りつぶしツールとディザグラデーションツールの切り替え |
| <kbd>L</kbd> | **直線** | リアルタイムプレビューとShift角度固定付きの直線描画 |
| <kbd>C</kbd> | **曲線** | 3クリック操作の2次ベジェ曲線描画ツール |
| <kbd>U</kbd> | **長方形** | 輪郭線および塗りつぶしの長方形／正方形を描画 |
| <kbd>O</kbd> | **楕円** | 輪郭線および塗りつぶしの楕円／正円を描画 |
| <kbd>D</kbd> | **ディザブラシ** | 市松模様のマトリックスディザパターンで描画 |
| <kbd>S</kbd> | **シェーディングツール** | パレットの並び順に従って段階的に明暗（覆い焼き／焼き込み）を付与 |
| <kbd>M</kbd> | **矩形選択** | マーチングアンツ表示を伴う矩形範囲の選択 |
| <kbd>Q</kbd> | **なげなわ選択** | フリーハンドによる隙間のない多角形中実選択 |
| <kbd>W</kbd> | **自動選択 (マジックワンド)** | 色の一致による連続領域の選択 |
| <kbd>P</kbd> | **ピボットポイント** | スプライト原点およびアクションポイントの配置 |
| <kbd>X</kbd> | **色を入れ替え** | 前景色と背景色の入れ替え |
| <kbd>[</kbd> / <kbd>]</kbd> | **ブラシサイズ** | ブラシサイズを縮小／拡大 (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **コピー** | 選択範囲のピクセルをシステムクリップボードへコピー |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **切り取り** | 選択範囲のピクセルを切り取ってクリップボードへ保存 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **貼り付け** | クリップボードのピクセルをアクティブフレームへ貼り付け |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **削除** | アクティブな選択範囲内のピクセルを消去 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **すべて選択** | アクティブフレーム内のすべてのピクセルを選択 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **選択解除** | アクティブな選択範囲マスクをクリア |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **選択範囲を反転** | 選択範囲マスクを反転 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **フレーム複製** | タイムライン内のアクティブフレームを複製 |
| <kbd>Space</kbd> | **再生 / 一時停止** | タイムラインアニメーションの再生・停止を切り替え |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **色の置換** | リアルタイムプレビュー付きの色置換ダイアログを開く |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **色調補正** | HSV色相／彩度／コントラスト調整ダイアログを開く |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **自由回転** | 任意角度回転モーダルを開く（RotSprite対応） |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **元に戻す (Undo)** | 直前の描画ストロークや操作を取り消す |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **やり直し (Redo)** | 取り消した操作を再び適用する |
| <kbd>Space</kbd> + ドラッグ / 中ボタンドラッグ | **パン移動** | キャンバス座標を滑らかに無段階スクロール |
| <kbd>Ctrl/Cmd</kbd> + ホイール / <kbd>+</kbd> / <kbd>-</kbd> | **ズーム** | カーソル位置を中心としたピクセル鮮明な拡大縮小 |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **表示モード切替** | フレーム表示（<kbd>1</kbd>）とアトラス表示（<kbd>2</kbd>）の切り替え |
| <kbd>F</kbd> | **全体表示** | アクティブフレームを画面中央に自動フィット |
| <kbd>F1</kbd> | **ヘルプ** | キーボードショートカット一覧モーダルを表示 |

---

## はじめ方

### ビルド済みバイナリ

コンパイル済みのスタンドアロンバイナリを **[GitHub Releases ページ](https://github.com/rbm13s/OniAnvil/releases/latest)** からダウンロードできます。

- **Windows**: `OniAnvil-windows-x86_64.zip` をダウンロードし、展開して `OniAnvil.exe` を実行します。
- **Linux**: `OniAnvil-linux-x86_64.tar.gz`（ポータブル版）をダウンロードするか、Debian/Ubuntu向けパッケージ `onianvil_*_amd64.deb` をインストールします。
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: `OniAnvil-macos-universal.zip` をダウンロードし、展開した `OniAnvil.app` を「アプリケーション」フォルダへドラッグ＆ドロップします。ユニバーサルバイナリのため、Apple Silicon（M1/M2/M3/M4）およびIntel搭載Macの双方でネイティブ動作します。
- **Web**: インストール不要でブラウザから直接実行できます: **[WebAssembly版を起動](https://rbm13s.github.io/OniAnvil/app/?lang=ja)**。
  > [!WARNING]
  > WebAssembly（ブラウザ）版は **実験的（Experimental）** ステータスです。ブラウザのサンドボックス制限やWebGL/WebGPUドライバの差異、クリップボードの制約により、デスクトップネイティブ版と比較して描画の不具合やパフォーマンス低下が発生する場合があります。本格的な制作作業には、ネイティブデスクトップ版のご利用を強くお勧めします。

---

### ソースコードからのビルド

#### 前提条件
- **Rustツールチェーン**: [rustup.rs](https://rustup.rs/) から最新のRust（1.80以上推奨）をインストールしてください。
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux用ビルド依存関係**（Debian/Ubuntu系）:
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. ネイティブデスクトップ版のコンパイルと実行
```bash
# リポジトリのクローン
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# テストスイートの実行
cargo test

# 最適化リリースバイナリのビルドと起動
cargo run --release
```

#### 2. WebAssembly版のコンパイル (Trunk使用)
```bash
# wasm32コンパイルターゲットとTrunkビルダーのインストール
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# ホットリロード対応ローカル開発サーバーの起動
trunk serve

# docs/app/ へ向けたリリースビルドの生成
./build_wasm.sh        # Linux/macOS環境
./build_wasm.ps1       # Windows PowerShell環境
```

---

## ファイル形式とコンパニオン構造

OniAnvilは不透明な独自バイナリ形式を使用しません。すべてのプロジェクトは、Gitなどのバージョン管理と極めて相性の良い2つのクリーンなファイルで構成されます。

```
my_character/
├── my_character.png            # 標準PNGスプライトシートアトラス (あらゆるゲームエンジンやビューアで直接閲覧可能)
└── my_character.onianvil.json   # コンパニオンメタデータファイル
```

サイドカーとなる `.onianvil.json` には以下のデータが保存されます。
- キャンバスグリッド設定: フレーム寸法、列数、総フレーム数。
- レイヤー階層: トラック名、不透明度、可視性フラグ、ロック状態。
- アニメーションタグ: タグ名、開始／終了フレームインデックス、再生方向（順再生、逆再生、ピンポン）、フレーム表示時間。
- パレット定義: アクティブなカラースウォッチ、名前付きスキンバリエーション、現在選択中のスキンインデックス。
- 参照スケッチ状態: 画像ファイルパス、拡大率、位置オフセット、回転角度、不透明度、配置レイヤーモード。
- ピボットポイント: カスタムアクションポイント、武器アンカー位置、当たり判定原点。

---

## 貢献と謝辞

バグ報告、新機能のご提案、プルリクエストを心より歓迎いたします！
1. GitHubで本リポジトリをフォークします。
2. 機能ブランチを作成します: `git checkout -b feature/my-new-feature`
3. 変更をコミットします: `git commit -am 'Add new pixel tool'`
4. ブランチをプッシュします: `git push origin feature/my-new-feature`
5. プルリクエストを作成します。

### 謝辞
- スターターデモスプライト **"Cat 2D Pixel Art"** — 作成者: **xzany** 様（[itch.ioプロジェクトページ](https://xzany.itch.io/cat-2d-pixel-art)）。
- 素晴らしいGUIライブラリ **[egui](https://github.com/emilk/egui)** およびグラフィックスエコシステム **[wgpu](https://wgpu.rs/)** を基盤に構築されています。
- UIアイコン群は @fontawesome による **[Font Awesome Free](https://fontawesome.com)** を採用しています（Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License）。

---

## ライセンスと免責事項

本プロジェクトは **[MIT License](LICENSE)** のもとで公開されています。商用・個人利用を問わず、ゲーム開発パイプラインやプロジェクトへ自由に導入・改変・再配布していただけます。

### 保証の否認および責任の制限（免責条項）
> [!IMPORTANT]
> **OniAnvil** は、明示または黙示を問わず、いかなる種類の保証（商品性、特定目的への適合性、権利の非侵害性を含みますがこれらに限定されません）もなく、**「現状のまま（AS IS）」** かつ **「提供可能な限度（AS AVAILABLE）」** で **無償** にて提供されます。
> 
> 著作者、著作権者、または貢献者は、本ソフトウェアの使用または使用不能から生じるいかなる直接損害、間接損害、付随的損害、特別損害、懲罰的損害、または結果的損害（データの喪失、プロジェクトファイルの破損、ハードウェアの故障、逸失利益、業務の中断を含みますがこれらに限定されません）について、契約責任、無過失責任、不法行為責任（過失を含みます）を問わず、またそのような損害の可能性を事前に知らされていた場合であっても、一切の責任を負いません。本ソフトウェアの使用に伴うすべてのリスクおよび責任は、利用者が自ら負うものとします。

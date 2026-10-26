<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil 로고" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**게임용 스프라이트 아틀라스 중심의 픽셀 스프라이트 & 어니언 스키닝 애니메이션 스튜디오**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Languages](https://img.shields.io/badge/Languages-37_Locales-purple)](#10-35가지-인체공학적-테마-및-다국어-지원)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#시작하기)
[![Web Demo](https://img.shields.io/badge/Online_App-Try_in_Browser_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ko)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 브라우저에서 OniAnvil 체험하기 (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=ko)** *(⚠️ 실험적 빌드: WebGL/WASM의 제한 사항 및 버그가 포함될 수 있습니다)* • **[🌐 공식 문서 웹사이트](https://rbm13s.github.io/OniAnvil/ko.html)**

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
[日本語](README-ja.md) •
**한국어**

</div>

---

## 목차

- [소개](#소개)
- [스크린샷 및 둘러보기](#스크린샷-및-둘러보기)
- [상세 도구 레퍼런스](#상세-도구-레퍼런스)
- [기능별 상세 문서](#기능별-상세-문서)
  - [1. 아틀라스 중심의 듀얼 아키텍처](#1-아틀라스-중심의-듀얼-아키텍처)
  - [2. 멀티트랙 레이어 타임라인](#2-멀티트랙-레이어-타임라인)
  - [3. 실시간 애니메이션 라이브 미리보기](#3-실시간-애니메이션-라이브-미리보기)
  - [4. 고급 어니언 스키닝 엔진](#4-고급-어니언-스키닝-엔진)
  - [5. 스마트 레퍼런스 및 스케치 트레이싱 레이어](#5-스마트-레퍼런스-및-스케치-트레이싱-레이어)
  - [6. 정밀 선택 도구 및 행진하는 개미 애니메이션](#6-정밀-선택-도구-및-행진하는-개미-애니메이션)
  - [7. 라이브 FX, 필터 및 RotSprite](#7-라이브-fx-필터-및-rotsprite)
  - [8. 팔레트 관리, 스왑 및 캐릭터 스킨](#8-팔레트-관리-스왑-및-캐릭터-스킨)
  - [9. 시트 슬라이서, 9-그리드 UI 및 게임 엔진 내보내기](#9-시트-슬라이서-9-그리드-ui-및-게임-엔진-내보내기)
  - [10. 35가지 인체공학적 테마 및 다국어 지원](#10-35가지-인체공학적-테마-및-다국어-지원)
- [조작법 및 단축키 안내](#조작법-및-단축키-안내)
- [시작하기](#시작하기)
  - [독립 실행형 바이너리](#독립-실행형-바이너리)
  - [소스 코드에서 빌드하기](#소스-코드에서-빌드하기)
- [파일 포맷 및 컴패니언 아키텍처](#파일-포맷-및-컴패니언-아키텍처)
- [기여 및 감사의 글](#기여-및-감사의-글)
- [라이선스 및 보증 면책](#라이선스-및-보증-면책)

---

## 소개

**OniAnvil**(**Oni**on-Skinning〈어니언 스키닝〉+ **Anvil**〈모루: 스프라이트를 단조하는 작업대〉)은 Rust 언어로 개발된 오픈 소스 고성능 픽셀 아트 에디터 및 애니메이션 제작 스위트입니다. Windows, Linux, macOS에서 하드웨어 가속 렌더링을 통해 네이티브 데스크톱으로 부드럽게 작동하며, 최신 웹 브라우저에서도 설치 없이 실행 가능한 WebAssembly 애플리케이션으로 바로 동작합니다.

기존의 많은 스프라이트 에디터들은 애니메이션을 단순히 여러 장의 이미지 파일 묶음으로 취급하기 때문에, 게임 개발자는 슬라이싱, 패킹, 재내보내기라는 번거로운 과정을 반복해야 했습니다. **OniAnvil은 이러한 개발 패러다임을 혁신합니다.** 게임에서 실제로 로드되는 스프라이트 시트 아틀라스 텍스처(`atlas_rgba`)가 항상 개발의 '단일 진실 공급원(Single Source of Truth)'이 됩니다. 프레임 셀, 레이어 트랙, 애니메이션 태그 및 충돌 피벗 포인트는 언제나 아틀라스와 실시간으로 완벽하게 동기화됩니다.

### 왜 OniAnvil인가?
- 🚀 **Rust 기반의 초고속 네이티브 성능**: 1밀리초 미만의 초고속 실행 속도, `wgpu` 및 `egui 0.31` 기반의 선명한 하드웨어 가속 렌더링, 긴 타임라인 스크러빙 시에도 렉 없는 부드러움.
- 🎬 **멀티트랙 레이어 타임라인**: 전문 비선형 비디오 편집기(NLE) 방식의 레이어 트랙(`V1, V2, V3...`). 상단에서 하단으로의 합성, 아래 레이어와 병합, 링크된 프레임, 가변 홀드(프레임 지속 시간) 지원.
- 🧅 **최고 수준의 어니언 스키닝**: 이전/이후 프레임의 독립적 색상 틴트(기본: 이전-빨강, 이후-파랑), 부드러운 불투명도 감쇠 곡선, 루프 경계를 넘나드는 고스트 표시 지원.
- 🎨 **즉각적인 캐릭터 팔레트 스킨(스왑)**: 아틀라스의 색상 리매핑을 통해 낮/밤 변형, 팀 컬러, 속성 스킨을 즉시 생성. GPL/PAL/HEX 팔레트 가져오기 및 내보내기 완벽 지원.
- 📦 **게임 엔진으로 직접 내보내기**: 2D 게임 엔진(Godot, Bevy, Raylib, Defold 등)에서 텍스처 필터링 시 발생하는 틈새 번짐 현상을 방지하는 외곽선 확장(Extruded border) PNG, 표준 Sprite Sheet JSON 메타데이터, 최적화된 GIF 애니메이션 출력.

---

## 스크린샷 및 둘러보기

<div align="center">

### 1. 메인 캔버스 & 스튜디오 작업 공간
*하드웨어 가속 뷰포트, 멀티트랙 타임라인, 실시간 라이브 미리보기, 팔레트 색상환, 정밀 툴바를 갖춘 고성능 픽셀 작업 공간.*
<img src="screenshots/main.png" alt="OniAnvil 메인 작업 공간" width="950" />

<br/><br/>

### 2. 멀티트랙 애니메이션 타임라인
*계층화된 레이어 행(`V1, V2...`), 링크된 프레임 셀 인스턴스(`🔗`), 홀드(지속 시간) 배수 설정, 색상으로 구분된 애니메이션 태그를 갖춘 비선형 멀티트랙 타임라인.*
<img src="screenshots/timeline.png" alt="OniAnvil 멀티트랙 타임라인" width="950" />

<br/><br/>

### 3. 정밀 도구 모음 & 빠른 실행 툴바
*표준 및 컴팩트 도킹 모드를 지원하는 15종 전체 그리기 도구와 브러시 크기, 대칭축, 래핑, 픽셀 퍼펙트를 토글하는 상단 툴바.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="OniAnvil 액션 툴바" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="OniAnvil 표준 도구 패널" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="OniAnvil 컴팩트 도구 패널" width="300" />
</p>

<br/><br/>

### 4. 실시간 애니메이션 라이브 미리보기
*태그 필터링, 가변 FPS 재생 제어, 줌 슬라이더 및 뷰포트 배경 전환(체커보드, 다크 단색, 라이트 단색)을 지원하는 독립형 라이브 재생 패널.*
<p align="center">
  <img src="screenshots/live_preview.png" alt="OniAnvil 라이브 미리보기" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="OniAnvil 라이브 미리보기 줌" width="460" />
</p>

<br/><br/>

### 5. 팔레트 관리, 색상 스왑 & 램프 생성
*전경색/배경색 견본 박스, 2색 간 자동 그라디언트 램프 생성 및 명명된 팔레트 스킨 스왑을 갖춘 종합 색상 엔진.*
<img src="screenshots/palette_and_swaps.png" alt="OniAnvil 팔레트 및 스킨 스왑" width="950" />

<br/><br/>

### 6. 레퍼런스 스케치 트레이싱 & 설정 가능한 어니언 스키닝
*컨셉 스케치나 사진을 변형 제어 기능과 함께 밑그림/덮개로 불러오고, 팔레트 스냅이 적용된 스포이트 색상 추출 및 다중 프레임 어니언 스킨 지원.*
<img src="screenshots/sketch_and_onion_skin.png" alt="OniAnvil 스케치 및 어니언 스키닝" width="950" />

<br/><br/>

### 7. 피벗 포인트 & 충돌/액션 포인트
*스프라이트 원점(Origin) 및 커스텀 액션 포인트(무기 장착점, 투사체 발사점, 머리, 발소리 등)를 인터랙티브 캔버스 기즈모로 지정.*
<img src="screenshots/pivot_and_action_points.png" alt="OniAnvil 피벗 및 액션 포인트" width="950" />

<br/><br/>

### 8. 스프라이트 시트 슬라이서 & 9-그리드(9-슬라이스) UI 스케일러
*자기상관 경계 감지를 통한 자동 슬라이싱(예: 고양이 걷기 사이클) 및 크기 조정 가능한 9-슬라이스 UI 박스 여백 설정.*
<p align="center">
  <img src="screenshots/slicer.png" alt="OniAnvil 시트 슬라이서" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="OniAnvil 9-슬라이스 설정" width="460" />
</p>

<br/><br/>

### 9. 실시간 이미지 FX & 필터 모달 (라이브 미리보기 지원)
*캔버스 실시간 미리보기, 스포이트 색상 선택, 비파괴 취소를 지원하는 모든 이펙트 다이얼로그.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="외곽선 생성기" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="색상 교체" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="색조, 채도, 명도 및 대비 조절" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="행렬 합성곱 필터" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="팔레트에 색상 맞추기" width="460" />
</p>

<br/><br/>

### 10. 캔버스 그리드 & 배경 모드
*픽셀 그리드, 타일 그리드, 다크 체커보드, 라이트 체커보드, 커스텀 단색 배경 간의 매끄러운 전환.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="그리드 및 다크 체커보드" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="그리드 없는 라이트 체커보드" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="커스텀 단색 배경" width="300" />
</p>

<br/><br/>

### 11. 새 프로젝트 생성 마법사
*캔버스 크기, 프레임 수, 열 그리드 배치를 설정하고 20종 이상의 정통 레트로 팔레트 중에서 선택.*
<p align="center">
  <img src="screenshots/new_project.png" alt="새 프로젝트 마법사" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="프로젝트 팔레트 선택" width="460" />
</p>

<br/><br/>

### 12. 35종의 정통 레트로 & 모던 테마
*Studio Dark, Studio Light, CRT 앰버/그린/시안 형광체, 레트로 콘솔(PlayStation 1, Game Boy, SNES, Dreamcast, Genesis), 디자이너 테마 수록.*
<p align="center">
  <img src="screenshots/themes.png" alt="테마 선택 메뉴" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="CRT 앰버 테마" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="PSX 테마" width="300" />
</p>

<br/><br/>

### 13. 메뉴 시스템 & 인앱 단축키 도움말
<p align="center">
  <img src="screenshots/file_menu.png" alt="파일 메뉴" width="180" />
  <img src="screenshots/edit_menu.png" alt="편집 메뉴" width="180" />
  <img src="screenshots/select_menu.png" alt="선택 메뉴" width="180" />
  <img src="screenshots/view_menu.png" alt="보기 메뉴" width="180" />
  <img src="screenshots/project_menu.png" alt="프로젝트 메뉴" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="단축키 도움말 모달" width="700" />
</p>

</div>

---

## 상세 도구 레퍼런스

OniAnvil에는 드로잉, 도형, 선택 및 메타데이터 지정을 위한 15가지 정밀 도구가 탑재되어 있습니다. 아이콘은 오픈 소스 **Font Awesome Free** 아이콘 세트를 사용합니다 *(Icons: CC BY 4.0, Fonts: SIL OFL 1.1)*.

| 도구 | 아이콘 | 단축키 | 브러시 크기 | 픽셀 퍼펙트 | 설명 |
|:---|:---:|:---:|:---:|:---:|:---|
| **연필** | ✏️ | <kbd>B</kbd> | 1..32 px | 지원 | 기본 드로잉 도구. 1px 모드에서 **픽셀 퍼펙트(Pixel-Perfect)** 드로잉 지원 (불필요한 L자형 계단 현상을 자동 제거). 선택 영역으로 생성한 커스텀 브러시 스탬프(`Ctrl/Cmd+B`) 지원. |
| **지우개** | 🧹 | <kbd>E</kbd> | 1..32 px | 미지원 | 픽셀을 완전한 투명(`[0, 0, 0, 0]`) 상태로 지웁니다. 선택 영역이 활성화된 경우 해당 영역 내부만 지워집니다. |
| **스포이트** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | 미지원 | 좌클릭 시 전경색(Primary), 우클릭 시 배경색(Secondary)으로 색상을 추출합니다. 캔버스, 레이어, 참조 스케치에서 색상을 가져올 수 있습니다. |
| **페인트통** | 🪣 | <kbd>G</kbd> | 해당 없음 | 미지원 | 허용 오차(Tolerance: `0..=255`) 설정이 가능한 큐 기반 초고속 BFS 채우기. 인접 영역만 채우기 또는 전체 영역 채우기 지원. |
| **그라디언트** | 🌈 | <kbd>G</kbd> *(전환)* | 해당 없음 | 미지원 | 전경색과 배경색 간의 선형 및 방사형 그라디언트를 생성합니다. Bayer(2x2, 4x4, 8x8) 및 Blue Noise 디더링 강도 조절을 지원합니다. |
| **직선** | 📏 | <kbd>L</kbd> | 1..32 px | 지원 | 드래그 실시간 미리보기를 지원하는 브레젠험(Bresenham) 직선 알고리즘. <kbd>Shift</kbd>를 누르면 45°/90° 각도로 고정됩니다. |
| **곡선** | 〰️ | <kbd>C</kbd> | 1..32 px | 지원 | 3단계 클릭 2차 베지에(Bézier) 곡선 도구. 1차 클릭으로 시작점, 2차 클릭으로 끝점을 지정하고 마우스 이동으로 곡률을 조절한 후 3차 클릭으로 확정합니다. |
| **직사각형** | ⬜ | <kbd>U</kbd> | 1..32 px | 미지원 | 윤곽선 또는 채워진 기하학적 직사각형을 그립니다. <kbd>Shift</kbd>를 누르고 드래그하면 1:1 정사각형으로 고정됩니다. |
| **타원** | ⭕ | <kbd>O</kbd> | 1..32 px | 미지원 | 중점(Midpoint) 알고리즘 기반의 윤곽선 또는 채워진 타원을 그립니다. <kbd>Shift</kbd>를 누르면 정원으로 고정됩니다. |
| **디더링 브러시** | 🏁 | <kbd>D</kbd> | 1..32 px | 미지원 | 팔레트에 불필요한 중간색을 추가하지 않고, 전경색과 배경색의 교차 체커보드 디더 패턴으로 음영을 표현합니다. |
| **셰이딩 도구** | 🌓 | <kbd>S</kbd> | 1..32 px | 미지원 | 활성 팔레트의 색상 순서에 따라 단계별 닷지(밝게, Dodge) 및 번(어둡게, Burn) 음영 효과를 적용합니다. |
| **사각 선택** | ⬚ | <kbd>M</kbd> | 해당 없음 | 미지원 | 움직이는 점선(행진하는 개미) 테두리가 표시되는 직사각형 영역 선택 도구. <kbd>Shift</kbd>로 추가, <kbd>Alt</kbd>로 제외. |
| **올가미 선택** | ➰ | <kbd>Q</kbd> | 해당 없음 | 미지원 | 자유형 다각형 올가미 도구. 8방향 경계선 래스터화와 4방향 외부 플러드 필 알고리즘을 사용하여 겹치는 루프를 그려도 내부에 구멍이 생기지 않는 100% 완전한 솔리드 선택을 보장합니다. |
| **마술봉** | 🪄 | <kbd>W</kbd> | 해당 없음 | 미지원 | 조절 가능한 허용 오차 임계값을 기준으로 인접한(또는 캔버스 전체의) 동일/유사 색상 영역을 선택합니다. |
| **피벗 및 액션 포인트** | 🎯 | <kbd>P</kbd> | 해당 없음 | 미지원 | 스프라이트 원점(Origin) 및 커스텀 게임플레이 액션 포인트(무기 장착점, 머리, 발, 피격 판정 등)를 캔버스에 배치하고 JSON 메타데이터로 내보냅니다. |

---

## 기능별 상세 문서

### 1. 아틀라스 중심의 듀얼 아키텍처

OniAnvil에서는 마스터 텍스처 아틀라스(`atlas_rgba`)가 게임 엔진에서 로드되는 실제 스프라이트 시트 텍스처를 나타냅니다.
- **프레임 모드 (Frame Mode)**: 작업 중인 단일 애니메이션 프레임에 집중하여 방해 없는 정밀 픽셀 캔버스를 제공합니다. 편집 내용은 아틀라스 텍스처 내 해당 셀에 즉각 반영됩니다.
- **아틀라스 모드 (Atlas Mode)**: 전체 스프라이트 시트 그리드를 표시하여 전체적인 정렬 확인, 프레임 간 크기 비교 및 일괄 편집을 수행할 수 있습니다.
- **컴패니언 파일 구조**: 프로젝트는 깨끗한 `.png` 스프라이트 시트 이미지와 프레임 레이아웃, 레이어 계층 구조, 태그, 홀드 배수, 팔레트 스킨을 담은 가벼운 `.onianvil.json` 메타데이터 파일로 안전하게 저장됩니다.

### 2. 멀티트랙 레이어 타임라인

하단 타임라인은 전문 논리니어 비디오 편집 소프트웨어(NLE)의 설계를 모델로 제작되었습니다.
- **레이어 트랙 (`V1, V2, V3...`)**: 레이어는 위에서 아래로 시각적으로 쌓입니다. 타임라인 상단에 위치한 트랙(`V1`)이 프레임 캔버스에서 가장 위에 합성 렌더링됩니다.
- **트랙 제어**:
  - `V1` / `V2` 배지를 클릭하여 활성 드로잉 레이어를 빠르게 전환.
  - 레이어 표시/숨김(눈 아이콘) 및 편집 잠금(자물쇠 아이콘) 토글.
  - 개별 레이어의 불투명도 조절 (`0%..=100%`).
  - 계층 구조 내에서 위/아래 이동, 레이어 복제, 아래 레이어와 병합(Merge Down).
  - 확인 대화상자를 통한 안전한 레이어 삭제.
- **프레임 클립 기능**:
  - 홀드(Hold) 배수: 프레임별 지속 시간(예: 타격감 연출이나 멈춤 동작을 위한 `2×`, `3×` 홀드) 지정.
  - 링크된 프레임: 반복되는 프레임을 링크하여 한 곳을 수정하면 링크된 모든 인스턴스가 자동으로 동기화.
  - 애니메이션 태그: 애니메이션을 명명된 태그(`idle`, `walk`, `attack`)로 정리하고 특정 태그만 반복 재생.
  - 재생 컨트롤: 정방향, 역방향, 핑퐁(왕복), 루프 재생 및 가변 FPS(`1..=120`) 조절.

### 3. 실시간 애니메이션 라이브 미리보기

우측의 미리보기 도크는 캔버스의 줌 배율이나 스크롤 위치와 관계없이 애니메이션을 실시간으로 재생합니다.
- **태그 필터링**: 특정 태그(`walk`, `jump` 등)만 분리하여 재생하거나 전체 애니메이션 시퀀스를 미리보기.
- **재생 컨트롤**: 재생, 일시정지, 이전 프레임 이동, 다음 프레임 이동, 루프 전환 및 재생 속도 조절(`0.25x..=4.0x`).
- **디스플레이 모드**: 다크 체커보드, 라이트 체커보드, 순수 흰색, 순수 검정 또는 커스텀 단색 배경 전환.

### 4. 고급 어니언 스키닝 엔진

전용 어니언 스키닝 시스템을 통해 유기적이고 역동적인 애니메이션을 조각하듯 작업할 수 있습니다.
- **독립적인 색상 틴트**: 이전 프레임(기본값: 빨강)과 다음 프레임(기본값: 파랑)에 대한 독립적인 색조 지정.
- **불투명도 감쇠 곡선**: 기본 투명도와 단계별 투명도 감쇠량을 정밀하게 조정 가능.
- **루프 래핑**: 순환 애니메이션 작업 시 첫 프레임과 마지막 프레임 사이를 자연스럽게 투과 표시.
- **빠른 미리보기 (Quick-Peek)**: 마우스를 드래그하지 않고 <kbd>Space</kbd> 키를 누르고 있는 동안 인접 프레임을 즉시 확인.

### 5. 스마트 레퍼런스 및 스케치 트레이싱 레이어

손으로 그린 스케치나 레퍼런스 사진을 불러와 트레이싱하여 작업 속도를 극대화할 수 있습니다.
- **지원 포맷**: PNG, JPEG, WebP 이미지 포맷 지원.
- **합성 모드**: 언더레이(픽셀 레이어 뒤쪽) 또는 오버레이(픽셀 레이어 앞쪽).
- **변형 컨트롤**: 독립적인 줌, 위치 오프셋, 회전 각도 및 불투명도 조절 슬라이더.
- **스마트 색상 추출**: 스케치 위에서 스포이트 도구를 사용할 때 활성 프로젝트 팔레트의 가장 가까운 색상으로 자동 스냅(흡착).
- **원클릭 굽기 (1-Click Bake)**: 배치된 크기의 레퍼런스 이미지를 클릭 한 번으로 활성 프레임의 픽셀 아트로 바로 변환.

### 6. 정밀 선택 도구 및 행진하는 개미 애니메이션

- **애니메이션 행진하는 개미 테두리**: 활성 선택 영역은 높은 시인성의 2색 점선 루프 애니메이션(Photoshop/Aseprite 방식)으로 표시됩니다.
- **완전 채움 플러드 필 올가미 선택**: 겹치는 루프를 그릴 때 내부가 비어버리는 일반적인 레이캐스팅 알고리즘과 달리, OniAnvil의 올가미 도구는 8방향 닫힌 경계를 래스터화하고 외부 영역을 4방향으로 채워 어떤 형태든 100% 빈틈없는 솔리드 선택을 보장합니다.
- **풍부한 선택 영역 조작**: 확장, 축소, 테두리 변환(원형, 사각형, 다이아몬드 커널 형태 선택 가능), 이동, 반전, 삭제, 자유 회전, 시스템 클립보드 복사/잘라내기/붙여넣기.
- **프레임 간 복사 및 붙여넣기**: 한 프레임에서 영역을 선택하고 <kbd>Ctrl/Cmd+C</kbd>를 누른 뒤 다른 프레임으로 이동하여 <kbd>Ctrl/Cmd+V</kbd>를 누르면 동일한 상대 좌표에 정확히 붙여넣어집니다. 모든 키보드 레이아웃에서 안정적으로 작동합니다.

### 7. 라이브 FX, 필터 및 RotSprite

모든 이미지 이펙트에는 실시간 **라이브 미리보기(Live Preview)** 체크박스가 포함되어 있어 작업 취소 시 실행 취소(Undo) 기록을 오염시키지 않고 캔버스에서 변경 사항을 즉시 확인하면서 안전하게 적용할 수 있습니다.
- **색상 교체 (`Shift+R`)**:
  - 인터랙티브 스포이트: 스포이트 아이콘을 클릭하고 프레임 픽셀이나 팔레트 도크에서 직접 색상을 추출.
  - 빠른 팔레트 스트립: 좌클릭으로 "기존 색상", 우클릭으로 "변경 색상" 선택.
  - 허용 오차 슬라이더 및 모든 애니메이션 프레임에 걸친 일괄(Batch) 색상 교체 지원.
- **색조/채도/명도 조절 (`Ctrl/Cmd+U`)**: HSV 색조(Hue), 채도(Saturation), 명도(Value) 및 대비(Contrast) 실시간 조절.
- **자동 외곽선 생성기**: 바깥쪽, 안쪽, 중앙 배치와 직각(사각형) 또는 모따기(다이아몬드) 모서리를 지원하는 선명한 픽셀 캐릭터 외곽선 생성.
- **3×3 합성곱 행렬 필터**: 알파 채널 보존 옵션이 포함된 흐림(Blur), 선명화(Sharpen), 가장자리 감지(Edge Detect), 엠보싱(Emboss) 필터.
- **자유 회전 (`Ctrl/Cmd+R`)**: 전설적인 **RotSprite** 알고리즘 또는 Nearest Neighbor 알고리즘을 사용하여 선택 영역이나 프레임 전체를 임의의 각도로 회전.
- **팔레트에 맞추기 (Remap)**: Bayer 2x2, 4x4, 8x8 또는 Blue Noise 디더링을 적용하여 캔버스 색상을 활성 팔레트로 양자화.

### 8. 팔레트 관리, 스왑 및 캐릭터 스킨

- **내장 정통 레트로 팔레트**: Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized, Sweetie 16 등 20종 이상의 엄선된 팔레트 제공.
- **가져오기 및 내보내기**: GIMP Palette(`.gpl`), JASC PAL(`.pal`), Hex(`.hex`) 및 이미지 견본(`.png`) 상호 호환.
- **팔레트 스왑 (캐릭터 스킨 시스템)**: 한 캐릭터에 대해 여러 팔레트 스킨 변형(예: 기본, 황금 갑옷, 그림자, 결빙)을 정의. 스킨을 전환하면 아틀라스 전체의 모든 프레임 색상이 즉시 재매핑됩니다.
- **컬러 램프 생성기**: 선택한 두 가지 색상 사이에 설정한 단계 수만큼 부드러운 중간 계조 그라디언트를 자동 생성.

### 9. 시트 슬라이서, 9-그리드 UI 및 게임 엔진 내보내기

- **시트 슬라이서**: 자기상관 경계 감지 알고리즘이 가공되지 않은 통 스프라이트 시트를 일정한 애니메이션 프레임으로 자동 분할.
- **9-그리드 (9-슬라이스) 스케일링**: 상하좌우 여백을 설정하여 모서리 왜곡 없이 자유롭게 확장되는 UI 윈도우 및 버튼 패널을 미리보기.
- **외곽선 확장(Extruded Border) PNG 내보내기**: 각 프레임의 테두리 픽셀을 투명 여백으로 1px 확장 복제하여 내보냅니다. OpenGL, DirectX, Vulkan 및 Godot 엔진 등에서 텍스처 필터링 시 발생하는 타일 간 틈새 번짐(시밍) 현상을 완벽하게 방지합니다.
- **최적화된 애니메이션 GIF 출력**: 루프 횟수, 프레임 지연, 투명 배경을 지원하는 고품질 GIF 인코더.
- **표준 Sprite Sheet JSON**: 최신 2D 게임 엔진에서 표준으로 사용되는 스프라이트 시트 메타데이터 규격에 맞춰 슬라이스, 태그, 지속 시간 및 피벗 좌표를 내보냅니다.

### 10. 35가지 인체공학적 테마 및 다국어 지원

- **35종의 다채로운 테마 컬렉션**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, CRT 앰버/그린/시안 형광체, 레트로 콘솔(Game Boy, Game Boy Light, Super Game Boy, Famicom/NES, Super Famicom/SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast), 레트로 PC(C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **37개 언어의 다국어 지원**: 모든 UI 텍스트, 툴팁, 알림 메시지가 다국어로 완벽하게 번역되어 있습니다.

---

## 조작법 및 단축키 안내

| 단축키 | 도구 / 기능 | 설명 |
|:---|:---|:---|
| <kbd>B</kbd> | **연필** | 1px 정밀 드로잉. 픽셀 퍼펙트 모드 지원 |
| <kbd>E</kbd> | **지우개** | 픽셀을 투명하게 지우기 |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **스포이트** | 캔버스 픽셀 또는 참조 스케치에서 색상 추출 |
| <kbd>G</kbd> | **페인트통 / 그라디언트** | 채우기 도구와 디더 그라디언트 도구 간 전환 |
| <kbd>L</kbd> | **직선** | 실시간 미리보기 및 Shift 각도 고정을 지원하는 직선 그리기 |
| <kbd>C</kbd> | **곡선** | 3단계 클릭 2차 베지에 곡선 그리기 |
| <kbd>U</kbd> | **직사각형** | 윤곽선 및 채워진 직사각형/정사각형 그리기 |
| <kbd>O</kbd> | **타원** | 윤곽선 및 채워진 타원/정원 그리기 |
| <kbd>D</kbd> | **디더링 브러시** | 체커보드 패턴의 매트릭스 디더로 그리기 |
| <kbd>S</kbd> | **셰이딩 도구** | 팔레트 순서에 따라 단계별 명암(닷지/번) 부여 |
| <kbd>M</kbd> | **사각 선택** | 행진하는 개미 애니메이션을 동반한 직사각형 영역 선택 |
| <kbd>Q</kbd> | **올가미 선택** | 빈틈없는 자유형 다각형 솔리드 선택 |
| <kbd>W</kbd> | **마술봉** | 동일/유사 색상 영역 선택 |
| <kbd>P</kbd> | **피벗 포인트** | 스프라이트 원점 및 액션 포인트 배치 |
| <kbd>X</kbd> | **색상 전환** | 전경색과 배경색 맞바꾸기 |
| <kbd>[</kbd> / <kbd>]</kbd> | **브러시 크기** | 브러시 크기 감소 / 증가 (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **복사** | 선택 영역의 픽셀을 시스템 클립보드로 복사 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **잘라내기** | 선택 영역의 픽셀을 클립보드로 잘라내기 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **붙여넣기** | 클립보드의 픽셀을 활성 프레임에 붙여넣기 |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **삭제** | 활성 선택 영역 내부의 픽셀 지우기 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **전체 선택** | 활성 프레임의 모든 픽셀 선택 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **선택 해제** | 활성 선택 마스크 지우기 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **선택 반전** | 활성 선택 마스크 반전 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **프레임 복제** | 타임라인의 활성 프레임 복제 |
| <kbd>Space</kbd> | **재생 / 일시정지** | 타임라인 애니메이션 재생 및 정지 토글 |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **색상 교체** | 실시간 미리보기를 지원하는 색상 교체 창 열기 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **색조/채도 조절** | HSV 색조 / 채도 / 대비 조절 창 열기 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **자유 회전** | 임의 각도 회전 모달창 열기 (RotSprite 지원) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **실행 취소 (Undo)** | 마지막 캔버스 작업 취소 |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **다시 실행 (Redo)** | 취소한 작업 다시 적용 |
| <kbd>Space</kbd> + 드래그 / 휠 클릭 드래그 | **캔버스 이동 (Pan)** | 캔버스 화면 부드러운 무한 이동 |
| <kbd>Ctrl/Cmd</kbd> + 휠 / <kbd>+</kbd> / <kbd>-</kbd> | **확대/축소 (Zoom)** | 커서 위치를 중심으로 선명한 픽셀 단위 줌 |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **보기 모드 전환** | 프레임 뷰(<kbd>1</kbd>)와 아틀라스 뷰(<kbd>2</kbd>) 간 전환 |
| <kbd>F</kbd> | **캔버스 맞춤** | 활성 프레임을 작업 공간 중앙에 자동 맞춤 |
| <kbd>F1</kbd> | **도움말** | 키보드 단축키 안내 모달 열기 |

---

## 시작하기

### 독립 실행형 바이너리

사전 컴파일된 독립 실행형 바이너리를 **[GitHub Releases 페이지](https://github.com/rbm13s/OniAnvil/releases/latest)**에서 다운로드할 수 있습니다:

- **Windows**: `OniAnvil-windows-x86_64.zip`을 다운로드하고 압축을 푼 뒤 `OniAnvil.exe`를 더블 클릭하여 실행합니다.
- **Linux**: `OniAnvil-linux-x86_64.tar.gz`(포터블 버전)를 다운로드하거나 Debian/Ubuntu용 `onianvil_*_amd64.deb` 패키지를 설치합니다:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: `OniAnvil-macos-universal.zip`을 다운로드하고 압축을 해제한 후 `OniAnvil.app`을 응용 프로그램 폴더로 드래그합니다. 유니버설 바이너리로 빌드되어 Apple Silicon(M1/M2/M3/M4) 및 Intel Mac 모두에서 네이티브로 실행됩니다.
- **Web**: 설치 없이 웹 브라우저에서 바로 사용 가능합니다: **[WebAssembly 앱 실행하기](https://rbm13s.github.io/OniAnvil/app/?lang=ko)**.
  > [!WARNING]
  > WebAssembly(브라우저) 빌드는 **실험적(Experimental)** 단계입니다. 브라우저 샌드박스 제한, WebGL/WebGPU 드라이버 차이, 클립보드 제약으로 인해 데스크톱 네이티브 버전에 비해 그래픽 결함이나 성능 병목이 발생할 수 있습니다. 상용 프로덕션 작업에는 네이티브 데스크톱 빌드를 적극 권장합니다.

---

### 소스 코드에서 빌드하기

#### 필수 조건
- **Rust 툴체인**: [rustup.rs](https://rustup.rs/)를 통해 Rust(1.80 이상 권장)를 설치합니다:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Linux 빌드 종속성** (Debian/Ubuntu 계열):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. 네이티브 데스크톱 버전 컴파일 및 실행
```bash
# 리포지토리 복제
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# 테스트 실행
cargo test

# 최적화된 릴리스 바이너리 빌드 및 실행
cargo run --release
```

#### 2. WebAssembly 버전 컴파일 (Trunk 사용)
```bash
# wasm32 타깃 및 Trunk 빌더 설치
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# 핫 리로드를 지원하는 로컬 개발 서버 실행
trunk serve

# docs/app/ 경로로 최적화된 배포 파일 빌드
./build_wasm.sh        # Linux/macOS 환경
./build_wasm.ps1       # Windows PowerShell 환경
```

---

## 파일 포맷 및 컴패니언 아키텍처

OniAnvil은 불투명한 독점 바이너리 포맷을 사용하지 않습니다. 모든 프로젝트는 버전 관리에 최적화된 두 개의 깔끔한 파일로 구성됩니다:

```
my_character/
├── my_character.png            # 표준 PNG 스프라이트 시트 아틀라스 (모든 엔진 및 뷰어에서 직접 열람 가능)
└── my_character.onianvil.json   # 프로젝트 메타데이터 컴패니언 파일
```

`.onianvil.json` 파일에는 다음 정보가 저장됩니다:
- 캔버스 그리드 설정: 프레임 크기, 열 개수, 총 프레임 수.
- 레이어 계층 구조: 트랙 이름, 불투명도, 가시성 플래그, 잠금 상태.
- 애니메이션 태그: 태그 이름, 시작/종료 프레임 인덱스, 재생 방향(정방향, 역방향, 핑퐁), 프레임별 지속 시간.
- 팔레트 정의: 활성 색상 견본, 명명된 스킨 변형 목록, 활성 스킨 인덱스.
- 참조 스케치 상태: 이미지 파일 경로, 확대율, 위치 오프셋, 회전 각도, 불투명도, 레이어 합성 모드.
- 피벗 포인트: 커스텀 액션 포인트, 무기 장착 좌표, 충돌 및 피격 원점.

---

## 기여 및 감사의 글

버그 제보, 기능 개선 아이디어, 풀 리퀘스트를 언제나 환영합니다!
1. GitHub에서 리포지토리를 포크(Fork)합니다.
2. 기능 브랜치를 만듭니다: `git checkout -b feature/my-new-feature`
3. 변경 사항을 커밋합니다: `git commit -am 'Add new pixel tool'`
4. 브랜치에 푸시합니다: `git push origin feature/my-new-feature`
5. 풀 리퀘스트(Pull Request)를 제출합니다.

### 감사의 글
- 스타터 데모 스프라이트 **"Cat 2D Pixel Art"** 제작자: **xzany** 님 ([itch.io 프로젝트 페이지](https://xzany.itch.io/cat-2d-pixel-art)).
- 훌륭한 GUI 프레임워크 **[egui](https://github.com/emilk/egui)** 및 그래픽 라이브러리 **[wgpu](https://wgpu.rs/)** 생태계를 기반으로 구축되었습니다.
- UI 아이콘은 @fontawesome의 **[Font Awesome Free](https://fontawesome.com)**를 사용했습니다 (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License).

---

## 라이선스 및 보증 면책

본 프로젝트는 **[MIT License](LICENSE)**에 따라 배포됩니다. 상업용 및 개인용 게임 개발 프로젝트에서 OniAnvil을 자유롭게 사용, 수정, 배포 및 통합할 수 있습니다.

### 보증 면책 및 책임의 한계
> [!IMPORTANT]
> **OniAnvil**은 명시적이든 묵시적이든 어떠한 형태의 보증(상품성, 특정 목적에의 적합성, 권리 비침해를 포함하되 이에 국한되지 않음) 없이 **무료**로 **"있는 그대로(AS IS)"** 및 **"이용 가능한 상태(AS AVAILABLE)"**로 제공됩니다.
> 
> 어떠한 경우에도 원작자, 저작권자 또는 기여자는 본 소프트웨어의 사용 또는 사용 불능으로 인해 발생하는 직간접적, 우발적, 특수적, 징벌적 또는 결과적 손해(데이터 손실, 프로젝트 파일 손상, 하드웨어 장애, 영업 이익 손실, 업무 중단 등을 포함하되 이에 국한되지 않음)에 대해 계약, 엄격 책임, 불법 행위(과실 포함) 등 어떠한 법적 책임 원인에 의해서도 책임을 지지 않으며, 이는 그러한 손해의 발생 가능성을 사전에 알고 있었던 경우에도 마찬가지입니다. 본 소프트웨어의 사용에 따른 모든 위험과 책임은 전적으로 사용자가 부담합니다.

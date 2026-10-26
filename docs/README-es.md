<div align="center">

<img src="assets/icon.jpg" alt="Logo de OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Estudio de animación pixel art y papel cebolla (Onion-Skin) basado en atlas para videojuegos**

[![Compilación y lanzamientos](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Descargas](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderizador](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Idiomas](https://img.shields.io/badge/Idiomas-37_Idiomas-purple)](#10-35-temas-ergonómicos-y-localización-multilingüe)
[![Plataformas](https://img.shields.io/badge/Plataformas-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#primeros-pasos)
[![Demo Web](https://img.shields.io/badge/App_en_línea-Probar_en_navegador_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=es)
[![Licencia: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Probar OniAnvil en el navegador (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=es)** *(⚠️ Versión experimental: puede contener limitaciones y fallos de WebGL/WASM)* • **[🌐 Sitio oficial de documentación](https://rbm13s.github.io/OniAnvil/es.html)**

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
**Español** •
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

## Contenido

- [Introducción](#introducción)
- [Capturas de pantalla y recorrido visual](#capturas-de-pantalla-y-recorrido-visual)
- [Referencia completa de herramientas](#referencia-completa-de-herramientas)
- [Documentación funcional](#documentación-funcional)
  - [1. Arquitectura dual centrada en el atlas](#1-arquitectura-dual-centrada-en-el-atlas)
  - [2. Línea de tiempo de capas multipista](#2-línea-de-tiempo-de-capas-multipista)
  - [3. Vista previa de animación en vivo y en tiempo real](#3-vista-previa-de-animación-en-vivo-y-en-tiempo-real)
  - [4. Motor avanzado de papel cebolla (Onion Skinning)](#4-motor-avanzado-de-papel-cebolla-onion-skinning)
  - [5. Capa inteligente de referencia y calco de bocetos](#5-capa-inteligente-de-referencia-y-calco-de-bocetos)
  - [6. Suite de selección de precisión y hormigas marchantes](#6-suite-de-selección-de-precisión-y-hormigas-marchantes)
  - [7. Efectos en vivo, filtros y RotSprite](#7-efectos-en-vivo-filtros-y-rotsprite)
  - [8. Gestión de paletas, intercambio y skins de personajes](#8-gestión-de-paletas-intercambio-y-skins-de-personajes)
  - [9. Cortador de hojas (Sheet Slicer), 9-Grid UI y exportación para motores de videojuegos](#9-cortador-de-hojas-sheet-slicer-9-grid-ui-y-exportación-para-motores-de-videojuegos)
  - [10. 35 temas ergonómicos y localización multilingüe](#10-35-temas-ergonómicos-y-localización-multilingüe)
- [Controles y atajos de teclado](#controles-y-atajos-de-teclado)
- [Primeros pasos](#primeros-pasos)
  - [Binarios independientes](#binarios-independientes)
  - [Compilación desde el código fuente](#compilación-desde-el-código-fuente)
- [Formato de archivo y arquitectura complementaria](#formato-de-archivo-y-arquitectura-complementaria)
- [Contribución y agradecimientos](#contribución-y-agradecimientos)
- [Licencia y exención de responsabilidad](#licencia-y-exención-de-responsabilidad)

---

## Introducción

**OniAnvil** (*nombre derivado de **Oni**on-Skinning (papel cebolla) + **Anvil** (el yunque para forjar sprites)*) es un editor de pixel art y suite de animación de código abierto y alto rendimiento programado en Rust. Se ejecuta de forma nativa con aceleración gráfica por hardware en Windows, Linux y macOS, y también directamente en navegadores web modernos como aplicación WebAssembly sin necesidad de instalación.

La mayoría de editores tradicionales tratan la animación como una pila de archivos de imagen inconexos, obligando a los desarrolladores a tareas tediosas de exportación, recorte y reensamblado en atlas. **OniAnvil invierte este paradigma:** la textura del atlas de la hoja de sprites del juego (`atlas_rgba`) es la fuente principal de verdad. Las celdas de fotogramas, las pistas de capas, las etiquetas de animación (tags) y los puntos de pivote/colisión se mantienen sincronizados con el atlas en todo momento de manera no destructiva.

### ¿Por qué OniAnvil?
- 🚀 **Rendimiento nativo ultrarrápido en Rust**: Inicio instantáneo en menos de un milisegundo, renderizado nítido impulsado por `wgpu` y `egui 0.31`, y fluidez total al desplazarse por líneas de tiempo extensas.
- 🎬 **Línea de tiempo de capas multipista**: Pistas de capas inspiradas en programas de edición de vídeo no lineal (`V1, V2, V3...`) con composición de arriba a abajo, fusión hacia abajo, fotogramas vinculados y multiplicadores de duración personalizables.
- 🧅 **Papel cebolla de primer nivel**: Efecto fantasma (ghosting) multifotograma con tintes anteriores y posteriores independientes, curvas de atenuación de opacidad configurables y bucle continuo.
- 🎨 **Variantes de paleta instantáneas (Skins)**: Crea variantes de día/noche, colores de equipo o apariencias elementales mediante el remapeo en vivo del atlas e importación/exportación en formatos GPL, PAL y HEX.
- 📦 **Exportación directa para motores de videojuegos**: Exporta archivos PNG escalados con extrusión de bordes para eliminar artefactos de filtrado de texturas en motores 2D (Godot, Bevy, Raylib, Defold), junto con metadatos JSON estándar y GIF animados.

---

## Capturas de pantalla y recorrido visual

<div align="center">

### 1. Espacio de trabajo principal y lienzo
*Espacio de trabajo pixel art de alto rendimiento con lienzo acelerado por hardware, línea de tiempo multipista, vista previa en tiempo real, muestras de paleta y barras de herramientas de precisión.*
<img src="screenshots/main.png" alt="Espacio de trabajo principal de OniAnvil" width="950" />

<br/><br/>

### 2. Línea de tiempo de animación multipista
*Línea de tiempo no lineal con capas apiladas (`V1, V2...`), instancias de fotogramas vinculados (`🔗`), multiplicadores de duración (hold) y etiquetas de animación con nombres y colores.*
<img src="screenshots/timeline.png" alt="Línea de tiempo multipista de OniAnvil" width="950" />

<br/><br/>

### 3. Herramientas de precisión y barra de acciones rápidas
*Suite completa de 15 herramientas de dibujo con modos de acoplamiento estándar y compacto, junto con la barra superior de acciones rápidas para tamaño de pincel, simetría, modo envolvente y Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Barra de herramientas de OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Herramientas de OniAnvil Estándar" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Herramientas de OniAnvil Compacto" width="300" />
</p>

<br/><br/>

### 4. Vista previa de animación en vivo y en tiempo real
*Panel de reproducción interactivo dedicado con filtrado por etiquetas, control de velocidad FPS, deslizador de zoom y alternancia de fondos (tablero de ajedrez, fondo oscuro sólido, fondo claro sólido).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Vista previa en vivo de OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zoom en vista previa de OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Gestión de paletas, intercambio de colores y rampas de gradiente
*Motor de color integral con selectores de color Principal y de Fondo, generación automática de rampas de color de dos puntos y variantes de paleta con nombres.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta y variantes de OniAnvil" width="950" />

<br/><br/>

### 6. Capa de calco de bocetos y papel cebolla configurable
*Carga bocetos o fotografías de referencia como capa inferior o superpuesta con controles de transformación, cuentagotas inteligente con ajuste automático a la paleta y efecto fantasma multifotograma.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Boceto y papel cebolla de OniAnvil" width="950" />

<br/><br/>

### 7. Puntos de pivote y puntos de acción de colisión
*Define el origen del sprite y puntos de juego personalizados (anclajes de armas, emisores de proyectiles, cabeza, pisadas) con gizmos interactivos en el lienzo.*
<img src="screenshots/pivot_and_action_points.png" alt="Puntos de pivote y acción de OniAnvil" width="950" />

<br/><br/>

### 8. Cortador de hojas (Sheet Slicer) y escalador 9-Grid (9-Slice)
*Detección automática de límites de fotogramas mediante autocorrelación (p. ej. ciclo de caminata de un gato) y configuración de márgenes para cajas de interfaz 9-slice escalables.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Cortador de hojas de OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Escalador 9-Slice de OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Efectos y filtros de imagen en tiempo real (con vista previa en vivo)
*Todos los cuadros de diálogo de efectos cuentan con vista previa en vivo en el lienzo, cuentagotas interactivo y cancelación sin alterar el historial de deshacer.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Generador de contorno" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Reemplazar color" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Tono Saturación Contraste" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtros matriciales" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Remapear a paleta" width="460" />
</p>

<br/><br/>

### 10. Cuadrículas y modos de fondo del lienzo
*Alterna sin problemas entre cuadrícula de píxeles, cuadrícula de mosaicos, tablero de ajedrez oscuro, tablero claro y colores de fondo personalizados.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Cuadrícula y tablero oscuro" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Tablero claro sin cuadrícula" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Fondo sólido personalizado" width="300" />
</p>

<br/><br/>

### 11. Asistente de creación de nuevos proyectos
*Configura las dimensiones del lienzo, fotogramas, disposición de columnas y elige entre más de 20 paletas retro auténticas al iniciar un proyecto.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Asistente de nuevo proyecto" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletas de nuevo proyecto" width="460" />
</p>

<br/><br/>

### 12. 35 temas retro y modernos auténticos
*Incluye Studio Dark, Studio Light, fósforos CRT Ámbar/Verde/Cian, consolas retro (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis/Mega Drive) y temas de diseño.*
<p align="center">
  <img src="screenshots/themes.png" alt="Menú de temas" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Ámbar" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PSX" width="300" />
</p>

<br/><br/>

### 13. Sistema de menús y ayuda de teclado integrada
<p align="center">
  <img src="screenshots/file_menu.png" alt="Menú Archivo" width="180" />
  <img src="screenshots/edit_menu.png" alt="Menú Edición" width="180" />
  <img src="screenshots/select_menu.png" alt="Menú Selección" width="180" />
  <img src="screenshots/view_menu.png" alt="Menú Ver" width="180" />
  <img src="screenshots/project_menu.png" alt="Menú Proyecto" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Ventana de ayuda de atajos de teclado" width="700" />
</p>

</div>

---

## Referencia completa de herramientas

OniAnvil incluye 15 herramientas de precisión para dibujo, geometría, selección y metadatos. Los iconos provienen de la suite de código abierto **Font Awesome Free** *(Iconos: CC BY 4.0, Fuentes: SIL OFL 1.1)*.

| Herramienta | Icono | Atajo | Tamaño de pincel | Pixel Perfect | Descripción |
|:---|:---:|:---:|:---:|:---:|:---|
| **Lápiz** | ✏️ | <kbd>B</kbd> | 1..32 px | Sí | Herramienta básica de dibujo. En modo 1px admite trazado **Pixel-Perfect** (elimina esquinas en L indeseadas en escalera). Permite usar selecciones como pinceles de estampa (`Ctrl/Cmd+B`). |
| **Borrador** | 🧹 | <kbd>E</kbd> | 1..32 px | No | Borra píxeles hasta hacerlos transparentes (`[0, 0, 0, 0]`). Limitado por la máscara de selección si está activa. |
| **Cuentagotas** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | No | Clic izquierdo toma color para el Frente (PP); Clic derecho para el Fondo (FP). Muestrea desde el lienzo, capas o boceto de referencia. |
| **Bote de pintura** | 🪣 | <kbd>G</kbd> | N/D | No | Relleno por algoritmo BFS basado en cola de alta velocidad con umbral de tolerancia ajustable (`0..=255`). Admite modo Contiguo o Global y restricción por selección. |
| **Degradado** | 🌈 | <kbd>G</kbd> *(alternar)* | N/D | No | Degradados lineales y radiales entre el color principal y de fondo. Admite tramado ordenado Bayer (2x2, 4x4, 8x8) y Blue Noise con control de intensidad. |
| **Línea** | 📏 | <kbd>L</kbd> | 1..32 px | Sí | Algoritmo de línea de Bresenham con vista previa interactiva. Mantener <kbd>Mayús</kbd> bloquea ángulos en incrementos de 45°/90°. |
| **Curva** | 〰️ | <kbd>C</kbd> | 1..32 px | Sí | Curva de Bézier cuadrática en 3 clics. Clic 1 define inicio, Clic 2 define fin, mover el ratón curva la línea y Clic 3 confirma la curva. |
| **Rectángulo** | ⬜ | <kbd>U</kbd> | 1..32 px | No | Rectángulos geométricos con contorno o rellenos. Mantener <kbd>Mayús</kbd> restringe a un cuadrado proporcional 1:1. |
| **Elipse** | ⭕ | <kbd>O</kbd> | 1..32 px | No | Elipses con contorno o rellenas mediante rasterización de punto medio. Mantener <kbd>Mayús</kbd> fuerza un círculo perfecto. |
| **Pincel de tramado** | 🏁 | <kbd>D</kbd> | 1..32 px | No | Pinta patrones de tramado en tablero de ajedrez alternando los colores frontal y de fondo sin añadir colores adicionales a la paleta. |
| **Herramienta de sombreado** | 🌓 | <kbd>S</kbd> | 1..32 px | No | Aclarado (Dodge) y oscurecido (Burn) incremental basado en el orden de colores de la paleta activa. |
| **Selección rectangular** | ⬚ | <kbd>M</kbd> | N/D | No | Selección de área rectangular con borde dinámico de hormigas marchantes. Admite añadir (<kbd>Mayús</kbd>) y restar (<kbd>Alt</kbd>). |
| **Selección de lazo** | ➰ | <kbd>Q</kbd> | N/D | No | Lazo poligonal a mano alzada. Utiliza rasterización de contorno 8-conectada con relleno exterior 4-conectado, garantizando selecciones 100% sólidas sin huecos, incluso en bucles cruzados. |
| **Varita mágica** | 🪄 | <kbd>W</kbd> | N/D | No | Selecciona regiones contiguas (o globales) del mismo color con umbral de tolerancia ajustable. |
| **Punto de pivote y acción** | 🎯 | <kbd>P</kbd> | N/D | No | Colocación interactiva en el lienzo del origen del sprite y puntos de acción de juego personalizados (`weapon`, `head`, `feet`, `hitbox`) exportados en metadatos JSON. |

---

## Documentación funcional

### 1. Arquitectura dual centrada en el atlas

En OniAnvil, la textura principal del atlas de sprites (`atlas_rgba`) representa la textura real tal como la cargará tu motor de juego:
- **Modo Fotograma (Frame Mode)**: Se centra en un único fotograma de animación activo para dibujar sin distracciones. Cualquier edición se refleja de inmediato en la celda correspondiente de la textura del atlas.
- **Modo Atlas (Atlas Mode)**: Muestra la cuadrícula completa de la hoja de sprites, lo que permite ajustes macro, comprobaciones de alineación entre fotogramas y edición por lotes.
- **Archivo complementario (Sidecar)**: Los proyectos se guardan como archivos de imagen `.png` limpios junto con un archivo complementario ligero `.onianvil.json` que contiene las dimensiones de fotogramas, jerarquía de capas, etiquetas, duraciones y variantes de paleta.

### 2. Línea de tiempo de capas multipista

La línea de tiempo inferior está diseñada según los principios del software profesional de edición de vídeo no lineal:
- **Pistas de capas (`V1, V2, V3...`)**: Las capas se apilan visualmente de arriba hacia abajo. La pista colocada más arriba en la línea de tiempo (`V1`) se dibuja encima en el lienzo del fotograma.
- **Controles de capa**:
  - Haz clic en las etiquetas `V1` / `V2` para alternar rápidamente la capa de dibujo activa.
  - Alterna la visibilidad de la capa (icono de ojo) y el bloqueo frente a modificaciones (icono de candado).
  - Ajusta la opacidad individual de cada capa (`0%..=100%`).
  - Mueve la capa hacia arriba o abajo en la jerarquía, duplica la capa o fusiónala hacia abajo (Merge Down).
  - Elimina capas de forma segura con un cuadro de diálogo de confirmación.
- **Clips de fotogramas**:
  - Multiplicador de duración (Hold): asigna tiempos de permanencia personalizados (p. ej. duración `2×`, `3×` para pausas o fotogramas de impacto).
  - Fotogramas vinculados (`🔗`): enlaza fotogramas repetitivos en la animación para que al editar uno se actualicen automáticamente todas las instancias vinculadas.
  - Bucles por etiqueta: organiza animaciones en bloques con nombre (`idle`, `walk`, `attack`) y reproduce en bucle etiquetas específicas.
  - Controles de reproducción: Reproducción hacia adelante, atrás, ping-pong y bucle con velocidad ajustable (`1..=120` FPS).

### 3. Vista previa de animación en vivo y en tiempo real

El panel de vista previa dedicado en el lateral derecho reproduce tus animaciones en tiempo real de forma independiente del zoom o paneo de tu lienzo:
- **Filtrado por etiquetas**: Aísla la reproducción a etiquetas específicas (`walk`, `jump`) o previsualiza toda la secuencia de animación.
- **Controles de reproducción**: Botones de Play, Pausa, fotograma anterior, fotograma siguiente, alternar bucle y multiplicadores de velocidad (`0.25x..=4.0x`).
- **Modos de visualización**: Alterna entre tablero de ajedrez oscuro, tablero claro, blanco puro, negro puro o fondos sólidos personalizados.

### 4. Motor avanzado de papel cebolla (Onion Skinning)

Crea animaciones fluidas con el sistema especializado de papel cebolla:
- **Tintes personalizados**: Colores de tinte independientes para fotogramas anteriores (por defecto: Rojo) y posteriores (por defecto: Azul).
- **Curva de atenuación de opacidad**: Opacidad base configurable y curvas de atenuación escalonada.
- **Bucle continuo**: Habilita la visualización fantasma entre el primer y último fotograma en ciclos continuos.
- **Vista rápida (Quick-Peek)**: Mantén pulsada la barra <kbd>Espacio</kbd> sin arrastrar el ratón para previsualizar instantáneamente los fotogramas adyacentes.

### 5. Capa inteligente de referencia y calco de bocetos

Acelera tu flujo de trabajo calcando bocetos a mano o fotos de referencia:
- **Formatos compatibles**: Carga imágenes de referencia en PNG, JPEG y WebP.
- **Modos de capa**: Capa inferior (debajo de tus capas de píxeles) o Superpuesta (encima de tus capas).
- **Controles de transformación**: Zoom independiente, desplazamiento de posición, rotación y deslizador de opacidad.
- **Muestreo inteligente de color**: Usa el cuentagotas sobre la capa de boceto con ajuste magnético automático opcional al color más cercano de la paleta activa.
- **Horneado en 1 clic (Bake)**: Convierte la imagen de referencia escalada directamente en pixel art en el fotograma activo.

### 6. Suite de selección de precisión y hormigas marchantes

- **Hormigas marchantes dinámicas**: Las selecciones activas se resaltan con un perímetro animado bicolor de alto contraste.
- **Lazo poligonal sólido sin huecos**: A diferencia de algoritmos ingenuos de trazado de rayos que dejan huecos en bucles superpuestos, el lazo de OniAnvil rasteriza un límite 8-conectado cerrado y rellena el exterior, garantizando selecciones 100% sólidas.
- **Operaciones de selección**: Expandir, Contraer, Borde (con formas de núcleo Circular, Cuadrada y Diamante), Trasladar, Invertir, Borrar, Rotación libre y Copiar/Cortar/Pegar en el portapapeles del sistema.
- **Copiar y pegar entre fotogramas**: Selecciona una región en un fotograma, pulsa <kbd>Ctrl/Cmd+C</kbd>, cambia de fotograma y pulsa <kbd>Ctrl/Cmd+V</kbd> para pegar exactamente en las mismas coordenadas relativas en cualquier distribución de teclado.

### 7. Efectos en vivo, filtros y RotSprite

Todos los cuadros de diálogo de efectos incluyen una casilla de **Vista previa en vivo (Live Preview)** que muestra el resultado en el lienzo en tiempo real y permite cancelar sin ensuciar el historial de deshacer:
- **Reemplazar color (`Shift+R`)**:
  - Cuentagotas interactivo: haz clic en el icono de la pipeta y selecciona colores directamente de los píxeles del lienzo o de la paleta.
  - Tira de selección rápida de paleta: clic izquierdo para color de origen («De»), clic derecho para color de destino («A»).
  - Deslizador de tolerancia y sustitución por lotes en todos los fotogramas de la animación.
- **Ajuste de color (`Ctrl/Cmd+U`)**: Desplazamiento de tono HSV en tiempo real, saturación, brillo y contraste.
- **Generador automático de contorno**: Genera contornos de píxeles nítidos para personajes (Exterior, Interior, Centrado) con esquinas cuadradas o en diamante.
- **Filtros de convolución 3×3**: Desenfoque, Enfoque, Detección de bordes y Relieve con preservación del canal alfa.
- **Rotación libre (`Ctrl/Cmd+Shift+R`)**: Rota áreas seleccionadas o fotogramas completos en cualquier ángulo usando el algoritmo clásico **RotSprite** o **Vecino más cercano**.
- **Remapear a paleta**: Cuantiza los colores del lienzo a la paleta activa mediante tramado Bayer 2x2, 4x4, 8x8 o Blue Noise.

### 8. Gestión de paletas, intercambio y skins de personajes

- **Paletas integradas**: Más de 20 paletas retro auténticas, incluyendo Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized y Sweetie 16.
- **Importación y exportación**: Compatibilidad con paletas GIMP (`.gpl`), PAL (`.pal`), hexadecimales (`.hex`) y tiras de imagen (`.png`).
- **Intercambio de paleta (Skins)**: Define múltiples variantes de color para un mismo personaje (p. ej. Por defecto, Armadura dorada, Sombra, Hielo). Al cambiar de skin se remapean al instante todos los colores en el atlas.
- **Generador de rampas de color**: Interpola gradientes suaves entre dos tonos seleccionados con un número de pasos intermedios configurable.

### 9. Cortador de hojas (Sheet Slicer), 9-Grid UI y exportación para motores de videojuegos

- **Cortador de hojas (Sheet Slicer)**: Detección de límites por autocorrelación que recorta automáticamente hojas en fotogramas regulares o ajustados al contorno.
- **Escalado 9-Grid (9-Slice)**: Configura márgenes de corte personalizados (superior, inferior, izquierdo, derecho) para previsualizar paneles y botones de interfaz sin distorsionar las esquinas.
- **Exportación PNG con extrusión de bordes**: Extiende los píxeles de los bordes hacia los márgenes transparentes para evitar artefactos y costuras de filtrado en OpenGL, DirectX, Vulkan y motores 2D (como Godot).
- **Exportación de GIF animado**: Genera GIF optimizados con control de repeticiones en bucle, retrasos entre fotogramas y transparencia.
- **JSON de hoja de sprites estándar**: Exporta coordenadas de fotogramas, etiquetas, duraciones y puntos de pivote compatibles con el formato JSON estándar de los motores modernos.

### 10. 35 temas ergonómicos y localización multilingüe

- **Temas seleccionados**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, Fósforos CRT (Ámbar, Verde, Cian), Consolas retro (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) y Ordenadores retro (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Localización multilingüe**: Alterna con un solo clic entre **Español**, **Inglés** y otros 35 idiomas con total coherencia en todas las cadenas.

---

## Controles y atajos de teclado

| Atajo | Herramienta / Acción | Descripción |
|:---|:---|:---|
| <kbd>B</kbd> | **Lápiz** | Dibujo de precisión de 1px con modo Pixel-Perfect opcional |
| <kbd>E</kbd> | **Borrador** | Borra píxeles hasta hacerlos transparentes |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Cuentagotas** | Muestrea color del lienzo o boceto de referencia |
| <kbd>G</kbd> | **Bote de pintura / Degradado** | Alterna entre bote de pintura y degradado con tramado |
| <kbd>L</kbd> | **Línea** | Dibuja líneas rectas con vista previa y bloqueo de ángulo con Mayús |
| <kbd>C</kbd> | **Curva** | Herramienta de curva Bézier cuadrática de 3 clics |
| <kbd>U</kbd> | **Rectángulo** | Formas rectangulares y cuadradas con contorno o rellenas |
| <kbd>O</kbd> | **Elipse** | Formas elípticas y circulares con contorno o rellenas |
| <kbd>D</kbd> | **Pincel de tramado** | Pinta patrones de tramado matricial en tablero de ajedrez |
| <kbd>S</kbd> | **Herramienta de sombreado** | Aclarado / oscurecido gradual a lo largo de la paleta |
| <kbd>M</kbd> | **Selección rectangular** | Selección de área rectangular con hormigas marchantes |
| <kbd>Q</kbd> | **Selección de lazo** | Selección poligonal sólida a mano alzada |
| <kbd>W</kbd> | **Varita mágica** | Selecciona regiones de color continuo |
| <kbd>P</kbd> | **Punto de pivote y acción** | Posiciona el origen del sprite y los puntos de acción de juego |
| <kbd>X</kbd> | **Intercambiar colores** | Alterna los colores principal y secundario |
| <kbd>[</kbd> / <kbd>]</kbd> | **Tamaño de pincel** | Reduce / aumenta el tamaño del pincel (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copiar** | Copia píxeles seleccionados al portapapeles del sistema |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Cortar** | Corta píxeles seleccionados al portapapeles del sistema |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Pegar** | Pega los píxeles del portapapeles en el fotograma activo |
| <kbd>Supr</kbd> / <kbd>Retroceso</kbd> | **Eliminar** | Borra los píxeles dentro de la selección activa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Seleccionar todo** | Selecciona todos los píxeles del fotograma activo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Deseleccionar** | Limpia la máscara de selección activa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Mayús</kbd> + <kbd>I</kbd> | **Invertir selección** | Invierte la máscara de selección activa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Mayús</kbd> + <kbd>D</kbd> | **Duplicar fotograma** | Duplica el fotograma activo en la línea de tiempo |
| <kbd>Espacio</kbd> | **Reproducir / Pausa** | Inicia o pausa la reproducción de la animación |
| <kbd>Mayús</kbd> + <kbd>R</kbd> | **Reemplazar color** | Abre el cuadro de diálogo de reemplazo de color con vista previa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Ajuste de color** | Abre el cuadro de diálogo de HSV y contraste |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Mayús</kbd> + <kbd>R</kbd> | **Rotación libre** | Abre el cuadro de diálogo de rotación libre por RotSprite |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Deshacer** | Deshace la última acción o trazo en el lienzo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Mayús</kbd>+<kbd>Z</kbd> | **Rehacer** | Rehace la acción previamente deshecha |
| <kbd>Espacio</kbd> + Arrastrar / Botón central | **Panorámica** | Desplazamiento suave e infinito por el lienzo |
| <kbd>Ctrl/Cmd</kbd> + Rueda / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Zoom nítido al píxel centrado en el cursor del ratón |
| <kbd>Teclado num. 1</kbd> / <kbd>Teclado num. 2</kbd> | **Modo de vista** | Alterna entre vista de Fotograma (<kbd>1</kbd>) y Atlas (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Ajustar lienzo** | Centra y ajusta automáticamente el fotograma en el espacio de trabajo |
| <kbd>F1</kbd> | **Ayuda** | Abre el cuadro modal de atajos de teclado |

---

## Primeros pasos

### Binarios independientes

Descarga binarios precompilados y autónomos desde la **[página de versiones en GitHub](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: Descarga `OniAnvil-windows-x86_64.zip`, descomprime y haz doble clic en `OniAnvil.exe`.
- **Linux**: Descarga `OniAnvil-linux-x86_64.tar.gz` (portable) o instala el paquete de Debian/Ubuntu `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: Descarga `OniAnvil-macos-universal.zip`, descomprime y arrastra `OniAnvil.app` a la carpeta Aplicaciones. El binario universal funciona de manera nativa tanto en Apple Silicon (M1/M2/M3/M4) como en equipos Mac basados en Intel.
- **Web**: Ejecuta la aplicación directamente en tu navegador sin instalar nada: **[Abrir aplicación WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=es)**.
  > [!WARNING]
  > La versión web (WebAssembly) tiene estado **Experimental**. Debido a las limitaciones del sandbox del navegador, diferencias de controladores WebGL/WebGPU y restricciones del portapapeles del sistema, pueden presentarse pequeños fallos visuales o menor rendimiento frente a la versión de escritorio nativa. Para trabajo en producción, se recomienda encarecidamente utilizar las versiones de escritorio nativas.

---

### Compilación desde el código fuente

#### Requisitos previos
- **Cadena de herramientas de Rust**: Instala Rust (versión 1.80+ recomendada) mediante [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dependencias de compilación en Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compilar y ejecutar la versión nativa de escritorio
```bash
# Clonar el repositorio
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Ejecutar la suite de pruebas
cargo test

# Compilar y ejecutar el binario optimizado en release
cargo run --release
```

#### 2. Compilar la versión WebAssembly (con Trunk)
```bash
# Añadir el destino de compilación wasm32 y el empaquetador Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Iniciar servidor local de desarrollo con recarga en vivo
trunk serve

# Generar archivos optimizados de producción en docs/app/
./build_wasm.sh        # En Linux/macOS
./build_wasm.ps1       # En Windows PowerShell
```

---

## Formato de archivo y arquitectura complementaria

OniAnvil evita el uso de formatos binarios propietarios y cerrados. Cada proyecto se guarda en dos archivos claros y listos para control de versiones:

```
mi_personaje/
├── mi_personaje.png            # Atlas de hoja de sprites PNG estándar (compatible con cualquier motor o visor)
└── mi_personaje.onianvil.json   # Archivo complementario de metadatos
```

El archivo complementario `.onianvil.json` almacena:
- Configuración de cuadrícula del lienzo: dimensiones de fotogramas, recuento de columnas, número total de fotogramas.
- Jerarquía de capas: nombres de pistas, niveles de opacidad, indicadores de visibilidad y estados de bloqueo.
- Etiquetas de animación: nombre, índices de inicio/fin, dirección de reproducción (Adelante, Atrás, Ping-Pong) y duraciones.
- Definiciones de paleta: muestras de color activas, variantes de skin con nombre e índice de skin activo.
- Estado del boceto de referencia: ruta de la imagen, escala, desplazamiento, rotación, opacidad y modo de capa.
- Puntos de pivote: puntos de acción personalizados, anclajes de armas y centros de colisión.

---

## Contribución y agradecimientos

¡Las contribuciones, informes de errores y propuestas de funciones son bienvenidos!
1. Haz un fork del repositorio en GitHub.
2. Crea una rama para tu función: `git checkout -b feature/mi-nueva-herramienta`.
3. Haz commit de tus cambios: `git commit -am 'Añadir nueva herramienta de píxeles'`.
4. Envía los cambios a tu rama: `git push origin feature/mi-nueva-herramienta`.
5. Abre una Pull Request.

### Agradecimientos
- Sprite de demostración animado **"Cat 2D Pixel Art"** creado por **xzany** ([página del proyecto en itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Construido con la fantástica biblioteca de interfaz gráfica **[egui](https://github.com/emilk/egui)** y el ecosistema de gráficos **[wgpu](https://wgpu.rs/)**.
- Iconografía de interfaz proporcionada por **[Font Awesome Free](https://fontawesome.com)** por @fontawesome (Iconos: CC BY 4.0, Fuentes: SIL OFL 1.1, Código: Licencia MIT).

---

## Licencia y exención de responsabilidad

Este proyecto está bajo la **[Licencia MIT](../LICENSE)**. Eres libre de usar, modificar, distribuir e integrar OniAnvil en proyectos de videojuegos comerciales y personales.

### Exención de garantía y limitación de responsabilidad
> [!IMPORTANT]
> **OniAnvil** se distribuye de manera **gratuita** bajo la condición **«TAL CUAL» («AS IS»)** y **«SEGÚN DISPONIBILIDAD» («AS AVAILABLE»)**, sin garantías ni condiciones de ningún tipo, ya sean expresas o implícitas.
> 
> En ningún caso los autores, titulares de derechos de autor o colaboradores serán responsables de daños directos, indirectos, incidentales, especiales, ejemplares o consecuentes (incluyendo, entre otros, la pérdida de datos, corrupción de archivos, fallos de hardware, lucro cesante o interrupción del negocio) derivados de cualquier forma del uso o de la imposibilidad de uso de este software, incluso si se advirtió de la posibilidad de tales daños. El usuario asume toda la responsabilidad y el riesgo derivados del uso de este programa.

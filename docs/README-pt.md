<div align="center">

<img src="assets/icon.jpg" alt="Logotipo do OniAnvil" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**Estúdio de pixel art e animação com onion-skinning baseado em atlas prontos para motores de jogos**

[![Compilação e Lançamento](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderizador](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Idiomas](https://img.shields.io/badge/Idiomas-37_Localizações-purple)](#10-35-temas-ergonômicos-e-localização-multilíngue)
[![Plataformas](https://img.shields.io/badge/Plataformas-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#primeiros-passos)
[![App Online](https://img.shields.io/badge/App_Online-Experimente_no_Navegador_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang=pt)
[![Licença: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Experimente o OniAnvil no Navegador (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang=pt)** *(⚠️ Versão experimental: sujeita a limitações gráficas e falhas de WebGL/WASM)* • **[🌐 Site Oficial de Documentação](https://rbm13s.github.io/OniAnvil/pt.html)**

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
**Português** •
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

## Sumário

- [Introdução](#introdução)
- [Capturas de tela e tour visual](#capturas-de-tela-e-tour-visual)
- [Referência completa de ferramentas](#referência-completa-de-ferramentas)
- [Documentação funcional](#documentação-funcional)
  - [1. Arquitetura unificada centrada em atlas](#1-arquitetura-unificada-centrada-em-atlas)
  - [2. Linha do tempo de camadas multifaixa](#2-linha-do-tempo-de-camadas-multifaixa)
  - [3. Janela de pré-visualização de animação em tempo real](#3-janela-de-pré-visualização-de-animação-em-tempo-real)
  - [4. Motor avançado de Onion Skinning](#4-motor-avançado-de-onion-skinning)
  - [5. Camada de esboço de referência e traçado inteligente](#5-camada-de-esboço-de-referência-e-traçado-inteligente)
  - [6. Suíte de seleção de precisão e formigas marchantes](#6-suíte-de-seleção-de-precisão-e-formigas-marchantes)
  - [7. Efeitos em tempo real, filtros e RotSprite](#7-efeitos-em-tempo-real-filtros-e-rotsprite)
  - [8. Gerenciamento de paletas, troca de cores e variações de personagens (Skins)](#8-gerenciamento-de-paletas-troca-de-cores-e-variações-de-personagens-skins)
  - [9. Cortador de folhas de sprites (Sheet Slicer), UI 9-Grid e exportação para motores de jogos](#9-cortador-de-folhas-de-sprites-sheet-slicer-ui-9-grid-e-exportação-para-motores-de-jogos)
  - [10. 35 temas ergonômicos e localização multilíngue](#10-35-temas-ergonômicos-e-localização-multilíngue)
- [Controles e atalhos de teclado](#controles-e-atalhos-de-teclado)
- [Primeiros passos](#primeiros-passos)
  - [Binários autônomos](#binários-autônomos)
  - [Compilação a partir do código-fonte](#compilação-a-partir-do-código-fonte)
- [Formato de arquivo e arquitetura complementar](#formato-de-arquivo-e-arquitetura-complementar)
- [Contribuição e agradecimentos](#contribuição-e-agradecimentos)
- [Licença e isenção de responsabilidade](#licença-e-isenção-de-responsabilidade)

---

## Introdução

**OniAnvil** (*formado pela fusão de **Oni**on-Skinning (animação com casca de cebola) + **Anvil** (bigorna para forjar sprites)*) é um editor de pixel art e suíte de animação de alto desempenho e código aberto desenvolvido em Rust. Funciona nativamente com aceleração por hardware no Windows, Linux e macOS, e como uma aplicação WebAssembly de instalação zero diretamente nos navegadores modernos.

A maioria dos editores tradicionais trata a animação como uma coleção de arquivos de imagem isolados, obrigando desenvolvedores a fluxos cansativos de exportação, recorte e montagem em folhas de sprites. **O OniAnvil inverte esse paradigma:** a textura do atlas do sprite sheet do jogo (`atlas_rgba`) é a fonte primária e incontestável de dados. As células dos quadros, as faixas de camadas, as tags de animação e os pivôs de colisão permanecem perfeitamente sincronizados com o atlas de maneira não destrutiva.

### Por que o OniAnvil?
- 🚀 **Desempenho Rust nativo ultrarrápido**: inicialização instantânea em menos de um milissegundo, renderização nítida impulsionada por `wgpu` e `egui 0.31`, navegação suave por linhas do tempo extensas sem qualquer atraso.
- 🎬 **Linha do tempo multifaixa de camadas**: faixas de camadas inspiradas em editores de vídeo profissionais (`V1, V2, V3...`) com composição de cima para baixo, mesclagem para baixo, quadros vinculados e multiplicadores de duração (hold).
- 🧅 **Onion Skinning de excelência**: visualização simultânea de múltiplos quadros vizinhos com colorização independente para quadros anteriores/posteriores, curvas de atenuação de opacidade e suporte para ciclos contínuos.
- 🎨 **Variações de cor (Skins) instantâneas para personagens**: crie variantes de dia/noite, equipes ou elementos mágicos com remapeamento instantâneo no atlas e suporte a importação/exportação de paletas GPL, PAL e HEX.
- 📦 **Exportação direta para motores de jogos**: exportação de PNGs escalados com extrusão de bordas para eliminar falhas de filtragem de texturas em engines 2D (Godot, Bevy, Raylib, Defold), além de metadados padrão Sprite Sheet JSON e GIFs animados.

---

## Capturas de tela e tour visual

<div align="center">

### 1. Área de trabalho principal e tela do estúdio
*Ambiente de trabalho de pixel art de alta performance com viewport acelerada por hardware, linha do tempo multifaixa, pré-visualização em tempo real, amostras de paleta e barras de ferramentas de precisão.*
<img src="screenshots/main.png" alt="Área de trabalho principal do OniAnvil" width="950" />

<br/><br/>

### 2. Linha do tempo de animação multifaixa
*Linha do tempo não linear com faixas de camadas empilhadas (`V1, V2...`), instâncias de quadros vinculadas (`🔗`), multiplicadores de duração e tags de animação organizadas por cor.*
<img src="screenshots/timeline.png" alt="Linha do tempo multifaixa do OniAnvil" width="950" />

<br/><br/>

### 3. Ferramentas de precisão e barra de ações rápidas
*Suíte com 15 ferramentas de desenho nos modos padrão e compacto, acompanhada da barra superior de ações rápidas para ajuste de tamanho do pincel, simetria, repetição de bordas e modo Pixel-Perfect.*
<p align="center">
  <img src="screenshots/toolbar.png" alt="Barra de ações rápidas do OniAnvil" width="600" /><br/><br/>
  <img src="screenshots/tools.png" alt="Ferramentas OniAnvil Padrão" width="300" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/tools_compact.png" alt="Ferramentas OniAnvil Compactas" width="300" />
</p>

<br/><br/>

### 4. Janela de pré-visualização de animação em tempo real
*Painel de reprodução independente com filtragem por tags de animação, controle de velocidade em FPS, controle deslizante de zoom e alternância de fundo (xadrez escuro, xadrez claro, cor sólida).*
<p align="center">
  <img src="screenshots/live_preview.png" alt="Pré-visualização em tempo real do OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/live_preview_scroll_test.png" alt="Zoom da pré-visualização do OniAnvil" width="460" />
</p>

<br/><br/>

### 5. Gerenciamento de paletas, troca de cores e gradientes
*Motor completo de cores com caixas de amostra para Primeiro Plano/Fundo, geração automática de rampas de gradiente entre dois pontos e troca de skins de paleta nomeadas.*
<img src="screenshots/palette_and_swaps.png" alt="Paleta e variações de skins do OniAnvil" width="950" />

<br/><br/>

### 6. Camada de esboço de referência e Onion Skinning configurável
*Carregue esboços de conceitos ou fotos como camada inferior/superior com controles de transformação, conta-gotas inteligente com encaixe à paleta e efeito fantasma de múltiplos quadros.*
<img src="screenshots/sketch_and_onion_skin.png" alt="Esboço e Onion Skinning no OniAnvil" width="950" />

<br/><br/>

### 7. Pontos de pivô e marcações de ação para jogos
*Defina o ponto de origem do sprite e pontos de ancoragem customizados para gameplay (arma, boca de fogo para projéteis, cabeça, passos) com marcadores interativos na tela.*
<img src="screenshots/pivot_and_action_points.png" alt="Pontos de pivô e ação do OniAnvil" width="950" />

<br/><br/>

### 8. Cortador de folhas de sprites (Sheet Slicer) e redimensionamento 9-Grid (9-Slice)
*Recorte automatizado por autocorrelação de bordas de pixels (ex.: ciclo de caminhada de gato) e configuração de margens para painéis e caixas escaláveis de UI.*
<p align="center">
  <img src="screenshots/slicer.png" alt="Cortador de folhas do OniAnvil" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/3x3.png" alt="Redimensionamento 9-Slice do OniAnvil" width="460" />
</p>

<br/><br/>

### 9. Efeitos de imagem e filtros com pré-visualização ao vivo (Live Preview)
*Todas as janelas de efeitos contam com pré-visualização em tempo real na tela, conta-gotas interativo e cancelamento seguro sem alterar o histórico de desfazer.*
<p align="center">
  <img src="screenshots/outline_generator.png" alt="Gerador de contorno" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/replace_color.png" alt="Substituição de cor" width="460" />
</p>
<p align="center">
  <img src="screenshots/hue_saturation_contrast.png" alt="Matiz, saturação e contraste" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/matrix_filter.png" alt="Filtros de matriz" width="460" />
</p>
<p align="center">
  <img src="screenshots/remap_to_palette.png" alt="Remapeamento para a paleta" width="460" />
</p>

<br/><br/>

### 10. Modos de grade e fundos de tela
*Alterne de forma fluida entre grade de pixels, grade de ladrilhos (tiles), xadrez escuro, xadrez claro e cores sólidas de fundo personalizadas.*
<p align="center">
  <img src="screenshots/grid_and_dark_checkerboard.png" alt="Grade e xadrez escuro" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/no_grid_light_checkerboard.png" alt="Xadrez claro sem grade" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/custom_solid_background.png" alt="Fundo sólido personalizado" width="300" />
</p>

<br/><br/>

### 11. Assistente de criação de novos projetos
*Defina dimensões da tela, quantidade de quadros, disposição das colunas do atlas e escolha entre mais de 20 autênticas paletas retrô logo no início.*
<p align="center">
  <img src="screenshots/new_project.png" alt="Assistente de novo projeto" width="460" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="screenshots/new_project_palettes.png" alt="Paletas de novo projeto" width="460" />
</p>

<br/><br/>

### 12. 35 temas retrô e modernos autênticos
*Inclui Studio Dark, Studio Light, fósforos CRT Âmbar/Verde/Ciano, consoles lendários (PlayStation 1, Game Boy, SNES, Dreamcast, Genesis) e paletas autorais.*
<p align="center">
  <img src="screenshots/themes.png" alt="Menu de temas" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/crt_orange_theme.png" alt="Tema CRT Âmbar" width="300" />
  &nbsp;&nbsp;
  <img src="screenshots/psx_theme.png" alt="Tema PlayStation 1" width="300" />
</p>

<br/><br/>

### 13. Sistema de menus e ajuda integrada de atalhos
<p align="center">
  <img src="screenshots/file_menu.png" alt="Menu Arquivo" width="180" />
  <img src="screenshots/edit_menu.png" alt="Menu Editar" width="180" />
  <img src="screenshots/select_menu.png" alt="Menu Seleção" width="180" />
  <img src="screenshots/view_menu.png" alt="Menu Exibir" width="180" />
  <img src="screenshots/project_menu.png" alt="Menu Projeto" width="180" />
</p>
<p align="center">
  <img src="screenshots/help.png" alt="Janela modal de atalhos de teclado" width="700" />
</p>

</div>

---

## Referência completa de ferramentas

O OniAnvil oferece 15 ferramentas de precisão para desenho, geometria, seleção e metadados. Os ícones utilizam a biblioteca aberta **Font Awesome Free** *(Ícones: CC BY 4.0, Fontes: SIL OFL 1.1)*.

| Ferramenta | Ícone | Atalho | Tamanho | Pixel Perfect | Descrição |
|:---|:---:|:---:|:---:|:---:|:---|
| **Lápis** | ✏️ | <kbd>B</kbd> | 1..32 px | Sim | Ferramenta principal de desenho livre. No tamanho 1px suporta **Pixel-Perfect** (elimina cantos em L e escadas indesejadas). Suporta carimbos customizados a partir de seleções (`Ctrl/Cmd+B`). |
| **Borracha** | 🧹 | <kbd>E</kbd> | 1..32 px | Não | Apaga pixels restaurando a transparência (`[0, 0, 0, 0]`). Age estritamente dentro da máscara se houver seleção ativa. |
| **Conta-gotas** | 💉 | <kbd>I</kbd> / <kbd>Alt</kbd> | 1 px | Não | Clique com botão esquerdo captura para Primeiro Plano (Foreground); botão direito para Fundo (Background). Coleta da tela, camadas ou do esboço de referência. |
| **Balde de tinta** | 🪣 | <kbd>G</kbd> | N/A | Não | Preenchimento rápido em fila (BFS) com tolerância de cor ajustável (`0..=255`). Suporta modo contíguo ou global e delimitação por seleção. |
| **Gradiente** | 🌈 | <kbd>G</kbd> *(alternar)* | N/A | Não | Gradientes lineares e radiais entre o primeiro plano e o fundo. Suporta pontilhado ordenado de Bayer (2x2, 4x4, 8x8) e Blue Noise com controle de força. |
| **Linha** | 📏 | <kbd>L</kbd> | 1..32 px | Sim | Algoritmo de linhas de Bresenham com pré-visualização ao arrastar. Segure <kbd>Shift</kbd> para travar em incrementos de 45°/90°. |
| **Curva** | 〰️ | <kbd>C</kbd> | 1..32 px | Sim | Curva quadrática de Bézier em 3 cliques. O clique 1 define o início, o clique 2 o fim, o arrasto molda a curva e o clique 3 consolida o traço. |
| **Retângulo** | ⬜ | <kbd>U</kbd> | 1..32 px | Não | Retângulos com contorno ou preenchidos. Segure <kbd>Shift</kbd> para travar proporções em quadrado 1:1. |
| **Elipse** | ⭕ | <kbd>O</kbd> | 1..32 px | Não | Elipses e círculos com contorno ou preenchidos via algoritmo de ponto médio. Segure <kbd>Shift</kbd> para círculo perfeito. |
| **Pincel de pontilhado** | 🏁 | <kbd>D</kbd> | 1..32 px | Não | Pinta padrões geométricos de pontilhado (dithering) alternando entre o primeiro plano e o fundo sem poluir a paleta com novas cores. |
| **Ferramenta de sombreamento** | 🌓 | <kbd>S</kbd> | 1..32 px | Não | Clareamento (Dodge) e escurecimento (Burn) incrementais baseados na ordem das amostras da paleta ativa. |
| **Seleção retangular** | ⬚ | <kbd>M</kbd> | N/A | Não | Seleção retangular delimitada por formigas marchantes em loop contínuo. Suporta Adicionar (<kbd>Shift</kbd>) e Subtrair (<kbd>Alt</kbd>). |
| **Seleção de laço** | ➰ | <kbd>Q</kbd> | N/A | Não | Laço poligonal à mão livre. Usa rasterização 8-conectada na borda e preenchimento 4-conectado no exterior, gerando seleções 100% sólidas sem buracos. |
| **Varinha mágica** | 🪄 | <kbd>W</kbd> | N/A | Não | Seleciona regiões contíguas (ou globais) de cor correspondente com limiar de tolerância configurável. |
| **Pivô e pontos de ação** | 🎯 | <kbd>P</kbd> | N/A | Não | Posicionamento interativo do centro de origem do sprite e pontos de ação de gameplay (`weapon`, `head`, `feet`, `hitbox`) exportados em JSON. |

---

## Documentação funcional

### 1. Arquitetura unificada centrada em atlas

No OniAnvil, o atlas de textura mestre (`atlas_rgba`) representa a imagem real da folha de sprites que será carregada pelo motor do jogo:
- **Modo Quadro**: foca no quadro de animação ativo, proporcionando uma área de desenho sem distrações. As alterações atualizam instantaneamente a célula correspondente no atlas.
- **Modo Atlas**: exibe a grade completa do sprite sheet, possibilitando ajustes macro, verificações de alinhamento entre quadros e edições em lote.
- **Arquivo de metadados**: os projetos são gravados como um arquivo `.png` padrão acompanhado de um arquivo complementar `.onianvil.json` com estrutura de camadas, tags, multiplicadores e skins de paleta.

### 2. Linha do tempo de camadas multifaixa

A linha do tempo inferior foi projetada nos moldes de softwares de edição de vídeo profissional (NLE):
- **Faixas de camadas (`V1, V2, V3...`)**: as camadas são empilhadas visualmente de cima para baixo. A faixa posicionada mais alto na linha do tempo (`V1`) é desenhada sobre as camadas inferiores na tela.
- **Controles de faixas**:
  - Clique nas legendas `V1` / `V2` para trocar rapidamente a camada de desenho ativa.
  - Alterne a visibilidade (ícone de olho) e o bloqueio contra edição (ícone de cadeado).
  - Controle a opacidade individual de cada camada (`0%..=100%`).
  - Mova a camada para cima/baixo, duplique-a e mescle com a faixa inferior (Merge Down).
  - Exclua camadas com janela de confirmação de segurança.
- **Quadros e clipes**:
  - Multiplicador de duração (Hold): atribua durações personalizadas (ex.: `2×`, `3×` de sustentação para pausas dramáticas ou impactos).
  - Quadros vinculados: vincule quadros repetidos ao longo da animação para que a edição em uma instância atualize automaticamente todas as outras.
  - Repetição por tags: estruture animações em blocos nomeados (`idle`, `walk`, `attack`) e reproduza tags específicas em ciclo.
  - Controles de reprodução: Avançar, Retroceder, Ping-Pong e Loop contínuo com velocidade ajustável (`1..=120` FPS).

### 3. Janela de pré-visualização de animação em tempo real

O painel de pré-visualização (Preview) à direita reproduz suas animações em tempo real, sem sofrer impacto de zoom ou navegação da tela de pintura:
- **Filtragem por tags**: isole a reprodução em uma tag específica (`walk`, `jump`) ou visualize a sequência inteira.
- **Controles de reprodução**: Play, Pause, Quadro anterior, Próximo quadro, Loop contínuo e taxas de velocidade customizadas (`0.25x..=4.0x`).
- **Modos de visualização**: alterne instantaneamente entre xadrez escuro, xadrez claro, branco puro, preto puro ou cores sólidas personalizadas.

### 4. Motor avançado de Onion Skinning

Modele animações fluidas e consistentes com o sistema dedicado de onion skinning (casca de cebola):
- **Colorização personalizada**: cores de destaque independentes para quadros anteriores (padrão: vermelho) e quadros posteriores (padrão: azul).
- **Curva de atenuação de opacidade**: opacidade base e coeficiente de redução configuráveis a cada passo de intervalo.
- **Ciclo contínuo (Wrap-Around)**: visualize o primeiro quadro a partir do último quadro (e vice-versa) ao produzir animações em loop fechado.
- **Espiada rápida (Quick-Peek)**: segure a tecla <kbd>Espaço</kbd> sem arrastar para espiar instantaneamente os quadros adjacentes ou o esboço.

### 5. Camada de esboço de referência e traçado inteligente

Acelere o fluxo de trabalho desenhando por cima de rascunhos feitos à mão ou fotos de referência:
- **Formatos suportados**: carregue arquivos PNG, JPEG e WebP.
- **Modos de mesclagem**: Camada inferior (sob as camadas de pixels) ou Sobreposição (acima das camadas de desenho).
- **Controles de transformação**: zoom independente, deslocamento de posição, rotação e controle deslizante de opacidade.
- **Conta-gotas inteligente**: colete cores do esboço com encaixe automático opcional à cor mais próxima da paleta ativa do projeto.
- **Rasterização em 1 clique (1-Click Bake)**: projete a imagem de referência escalada diretamente em pixel art no quadro ativo.

### 6. Suíte de seleção de precisão e formigas marchantes

- **Formigas marchantes animadas**: as seleções ativas são contornadas por linhas tracejadas dinâmicas de alto contraste em dois tons.
- **Seleção de laço contínua**: diferentemente de métodos clássicos por ray-casting que deixam lacunas quando há laços sobrepostos, o laço do OniAnvil rasteriza um contorno fechado 8-conectado e preenche a área externa, garantindo seleções 100% preenchidas.
- **Operações de seleção**: Expandir, Contrair, Borda (com formatos circular, quadrado e diamante), Transladar, Inverter, Deletar, Rotação livre e Copiar/Cortar/Colar na área de transferência.
- **Transferência entre quadros**: selecione uma área, pressione <kbd>Ctrl/Cmd+C</kbd>, mude de quadro e aperte <kbd>Ctrl/Cmd+V</kbd> para colar exatamente nas mesmas coordenadas relativas em qualquer layout de teclado.

### 7. Efeitos em tempo real, filtros e RotSprite

Todos os efeitos dispõem da caixa de seleção **Pré-visualização ao vivo**, permitindo visualizar as alterações na tela antes de aplicar e cancelar sem poluir o histórico de desfazer:
- **Substituir cor (`Shift+R`)**:
  - Conta-gotas interativo: clique no ícone da pipeta e selecione as cores de origem e destino na tela ou na barra de paletas.
  - Faixa rápida de paleta: clique esquerdo define a cor de origem, clique direito define a cor de destino.
  - Ajuste de tolerância e substituição em lote em todos os quadros de uma vez.
- **Ajuste de cores (`Ctrl/Cmd+U`)**: ajuste em tempo real de matriz (Hue), saturação, brilho e contraste no espaço HSV.
- **Gerador automático de contorno**: crie contornos nítidos ao redor dos sprites (Externo, Interno, Centralizado) com junções retas ou diagonais.
- **Filtros de matriz 3×3**: desfoque, nitidez, detecção de bordas e relevo (emboss) com preservação de canal alfa.
- **Rotação em ângulo livre (`Ctrl/Cmd+Shift+R`)**: gire áreas selecionadas ou quadros inteiros em qualquer grau com os algoritmos **RotSprite** ou **Vizinho Mais Próximo**.
- **Remapeamento para a paleta**: converta as cores da tela para a paleta ativa usando pontilhado de Bayer 2x2, 4x4, 8x8 ou Blue Noise.

### 8. Gerenciamento de paletas, troca de cores e variações de personagens (Skins)

- **Paletas integradas**: mais de 20 autênticas paletas retrô, incluindo Game Boy, PICO-8, Commodore 64, NES, Endesga 32, Solarized e Sweetie 16.
- **Importação e exportação**: suporte aos formatos GIMP Palette (`.gpl`), PAL (`.pal`), listas hexadecimais (`.hex`) e faixas de cores em imagem (`.png`).
- **Troca de paletas (Skins)**: configure variações cromáticas para um mesmo personagem (ex.: Padrão, Armadura Dourada, Sombra, Congelado). A troca de skin remapeia instantaneamente as cores em todo o atlas.
- **Gerador de rampas de cores**: gere transições suaves de gradiente entre dois tons escolhidos com número ajustável de degraus.

### 9. Cortador de folhas de sprites (Sheet Slicer), UI 9-Grid e exportação para motores de jogos

- **Cortador de folhas (Sheet Slicer)**: identificação de limites por autocorrelação recorta automaticamente folhas de sprites prontas em quadros regulares de animação.
- **Escala 9-Grid (9-Slice)**: defina margens de corte (superior, inferior, esquerda, direita) para visualizar e ajustar elementos de interface escaláveis (janelas, caixas de diálogo e botões).
- **Exportação PNG com extrusão de bordas**: duplica as cores dos pixels de borda nas margens transparentes, prevenindo artefatos de filtragem de texturas e emendas em OpenGL, DirectX, Vulkan e Godot.
- **Exportação de GIFs animados**: exportador otimizado com suporte a contagem de ciclos, intervalos por quadro e fundo transparente.
- **Metadados padrão Sprite Sheet JSON**: exportação de coordenadas de quadros, tags, durações e pontos de ancoragem no padrão adotado pelas principais engines 2D.

### 10. 35 temas ergonômicos e localização multilíngue

- **Seleção cuidadosa de temas**: Studio Dark, Studio Light, Pitch Black, Cyberpunk Neon, Synthwave, Nord Frost, Dracula, Tokyo Night, Catppuccin Mocha, Gruvbox Dark, Monokai Pro, Solarized Dark, Solarized Light, fósforos CRT (Âmbar, Verde, Ciano), consoles históricos (Game Boy, Game Boy Light, Super Game Boy, NES/Famicom, SNES, Genesis/Mega Drive, PlayStation 1, PlayStation 2, Dreamcast) e computadores clássicos (C64, ZX Spectrum, Amiga Workbench, Atari ST, MSX, Apple II, DOS CGA, Mac Classic, Windows 98).
- **Localização completa**: suporte integrado a mais de 37 idiomas com correspondência total de termos e troca instantânea sem reiniciar.

---

## Controles e atalhos de teclado

| Atalho | Ferramenta / Ação | Descrição |
|:---|:---|:---|
| <kbd>B</kbd> | **Lápis** | Desenho de precisão de 1px com suporte a Pixel-Perfect |
| <kbd>E</kbd> | **Borracha** | Apaga pixels restaurando a transparência |
| <kbd>I</kbd> / <kbd>Alt</kbd> | **Conta-gotas** | Coleta cores dos pixels da tela ou do esboço |
| <kbd>G</kbd> | **Balde de tinta / Gradiente** | Alterna entre preenchimento e gradiente com pontilhado |
| <kbd>L</kbd> | **Linha** | Desenha linhas retas com pré-visualização e trava por Shift |
| <kbd>C</kbd> | **Curva** | Ferramenta de curva quadrática de Bézier em 3 cliques |
| <kbd>U</kbd> | **Retângulo** | Retângulos e quadrados com contorno ou preenchimento |
| <kbd>O</kbd> | **Elipse** | Elipses e círculos com contorno ou preenchimento |
| <kbd>D</kbd> | **Pincel de pontilhado** | Pinta padrões matriciais xadrez com pontilhado |
| <kbd>S</kbd> | **Ferramenta de sombreamento** | Clareia (Dodge) ou escurece (Burn) na ordem da paleta |
| <kbd>M</kbd> | **Seleção retangular** | Seleção retangular com contorno animado de formigas |
| <kbd>Q</kbd> | **Seleção de laço** | Seleção poligonal à mão livre com preenchimento sólido |
| <kbd>W</kbd> | **Varinha mágica** | Seleciona áreas contíguas de cor correspondente |
| <kbd>P</kbd> | **Pontos de pivô** | Posiciona a origem do sprite e pontos de ação de jogo |
| <kbd>X</kbd> | **Alternar cores** | Inverte as cores de primeiro plano e fundo |
| <kbd>[</kbd> / <kbd>]</kbd> | **Tamanho do pincel** | Reduz / Aumenta o tamanho do pincel (`1..=32`) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>C</kbd> | **Copiar** | Copia os pixels selecionados para a área de transferência |
| <kbd>Ctrl/Cmd</kbd> + <kbd>X</kbd> | **Cortar** | Corta os pixels selecionados para a área de transferência |
| <kbd>Ctrl/Cmd</kbd> + <kbd>V</kbd> | **Colar** | Cola pixels da área de transferência no quadro ativo |
| <kbd>Delete</kbd> / <kbd>Backspace</kbd> | **Excluir** | Limpa pixels contidos na seleção ativa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>A</kbd> | **Selecionar tudo** | Seleciona todos os pixels do quadro ativo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>D</kbd> / <kbd>Esc</kbd> | **Desmarcar** | Limpa a máscara de seleção ativa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd> | **Inverter seleção** | Inverte a máscara de seleção ativa |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd> | **Duplicar quadro** | Cria cópia do quadro ativo na linha do tempo |
| <kbd>Espaço</kbd> | **Play / Pause** | Inicia ou pausa a reprodução na linha do tempo |
| <kbd>Shift</kbd> + <kbd>R</kbd> | **Substituir cor** | Abre janela de substituição com pré-visualização ao vivo |
| <kbd>Ctrl/Cmd</kbd> + <kbd>U</kbd> | **Ajuste de cores** | Abre janela de ajuste de matiz HSV e contraste |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Shift</kbd> + <kbd>R</kbd> | **Rotação livre** | Abre janela de rotação em qualquer ângulo (RotSprite) |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Z</kbd> | **Desfazer** | Desfaz a última ação realizada na tela |
| <kbd>Ctrl/Cmd</kbd> + <kbd>Y</kbd> / <kbd>Ctrl/Cmd</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> | **Refazer** | Refaz a ação que havia sido desfeita |
| <kbd>Espaço</kbd> + Arrastar / Botão do meio | **Mover tela** | Navegação panorâmica suave pelas coordenadas da tela |
| <kbd>Ctrl/Cmd</kbd> + Roda / <kbd>+</kbd> / <kbd>-</kbd> | **Zoom** | Zoom pixelizado focado na posição do cursor do mouse |
| <kbd>Num 1</kbd> / <kbd>Num 2</kbd> | **Modo de exibição** | Alterna entre Vista de Quadro (<kbd>1</kbd>) e Vista de Atlas (<kbd>2</kbd>) |
| <kbd>F</kbd> | **Ajustar tela** | Centraliza e enquadra o quadro perfeitamente na janela |
| <kbd>F1</kbd> | **Ajuda** | Abre o resumo completo de atalhos de teclado |

---

## Primeiros passos

### Binários autônomos

Baixe os pacotes binários pré-compilados diretamente na **[página de lançamentos do GitHub](https://github.com/rbm13s/OniAnvil/releases/latest)**:

- **Windows**: baixe `OniAnvil-windows-x86_64.zip`, descompacte os arquivos e execute `OniAnvil.exe`.
- **Linux**: baixe o pacote portátil `OniAnvil-linux-x86_64.tar.gz` ou instale o pacote Debian/Ubuntu `onianvil_*_amd64.deb`:
  ```bash
  sudo dpkg -i onianvil_*_amd64.deb
  ```
- **macOS**: baixe `OniAnvil-macos-universal.zip`, descompacte e mova o aplicativo `OniAnvil.app` para a pasta Aplicativos. O binário universal roda nativamente tanto nos Macs Apple Silicon (M1/M2/M3/M4) quanto nos modelos Intel.
- **Web**: execute diretamente no seu navegador sem instalar nada: **[Abrir aplicativo WebAssembly](https://rbm13s.github.io/OniAnvil/app/?lang=pt)**.
  > [!WARNING]
  > A versão WebAssembly para navegadores possui status **Experimental**. Devido às restrições de segurança do navegador, diferenças nos drivers WebGL/WebGPU e acesso à área de transferência, pequenos artefatos gráficos ou quedas de performance podem ocorrer. Para trabalhos regulares e produção, recomendamos o uso das versões nativas de computador.

---

### Compilação a partir do código-fonte

#### Pré-requisitos
- **Rust Toolchain**: instale o compilador Rust (recomendado versão 1.80+) por meio do site [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dependências de compilação no Linux** (Debian/Ubuntu):
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libssl-dev libgtk-3-dev libgl1-mesa-dev
  ```

#### 1. Compilar e executar versão nativa
```bash
# Clone o repositório
git clone https://github.com/rbm13s/OniAnvil.git
cd OniAnvil

# Execute a suíte de testes
cargo test

# Compile e execute a versão otimizada de lançamento
cargo run --release
```

#### 2. Compilar versão WebAssembly (via Trunk)
```bash
# Instale o alvo de compilação wasm32 e o empacotador Trunk
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Inicie o servidor local de desenvolvimento com recarregamento em tempo real
trunk serve

# Compile os arquivos de produção para a pasta docs/app/
./build_wasm.sh        # No Linux/macOS
./build_wasm.ps1       # No Windows PowerShell
```

---

## Formato de arquivo e arquitetura complementar

O OniAnvil não adota formatos binários fechados ou proprietários. Cada projeto consiste em dois arquivos claros e amigáveis ao controle de versões:

```
my_character/
├── my_character.png            # Imagem padrão PNG da folha de sprites (compatível com qualquer engine ou visualizador)
└── my_character.onianvil.json   # Arquivo lateral de metadados do projeto
```

O arquivo lateral `.onianvil.json` armazena:
- Configuração de grade da tela: dimensões dos quadros, quantidade de colunas e total de quadros.
- Estrutura de camadas: nomes das faixas, níveis de opacidade, sinalizadores de visibilidade e estado de bloqueio.
- Tags de animação: nome, quadros inicial e final, sentido de reprodução (Avançar, Retroceder, Ping-Pong) e durações individuais.
- Definições de paletas: cores ativas, variações de skins nomeadas e índice da skin selecionada.
- Estado da imagem de esboço: caminho do arquivo, escala, deslocamento, rotação, opacidade e modo de mesclagem.
- Pontos de pivô: pontos de ação customizados, engates de armas e centros de colisão do sprite.

---

## Contribuição e agradecimentos

Sugestões, relatórios de falhas e contribuições de código são muito bem-vindos!
1. Faça um Fork do repositório no GitHub.
2. Crie um branch para a sua funcionalidade: `git checkout -b feature/minha-nova-ferramenta`.
3. Salve suas alterações: `git commit -am 'Adiciona nova ferramenta de desenho'`.
4. Envie o branch: `git push origin feature/minha-nova-ferramenta`.
5. Abra um Pull Request.

### Agradecimentos
- O sprite de animação inicial do gato **"Cat 2D Pixel Art"** foi criado por **xzany** ([página do projeto no itch.io](https://xzany.itch.io/cat-2d-pixel-art)).
- Desenvolvido sobre a excelente biblioteca visual **[egui](https://github.com/emilk/egui)** e o ecossistema gráfico **[wgpu](https://wgpu.rs/)**.
- Ícones de interface fornecidos pela biblioteca **[Font Awesome Free](https://fontawesome.com)** por @fontawesome (Ícones: CC BY 4.0, Fontes: SIL OFL 1.1, Código: Licença MIT).

---

## Licença e isenção de responsabilidade

Este projeto está licenciado sob os termos da **[Licença MIT](../LICENSE)**. Você tem total liberdade para usar, modificar, distribuir e integrar o OniAnvil em jogos e projetos comerciais ou pessoais.

### Isenção de garantias e limitação de responsabilidade
> [!IMPORTANT]
> O **OniAnvil** é distribuído **gratuitamente** no estado em que se encontra (**"COMO ESTÁ"** / **"AS IS"**) e **"CONFORME DISPONÍVEL"**, sem garantias ou condições de qualquer tipo, expressas ou implícitas (incluindo, entre outras, garantias de comercialização, adequação a uma finalidade específica ou não violação).
> 
> Em hipótese alguma os autores, detentores dos direitos autorais ou colaboradores serão responsáveis por quaisquer danos diretos, indiretos, incidentais, especiais, exemplares ou consequenciais (incluindo, entre outros, perda de dados, corrupção de arquivos, falhas de hardware, lucros cessantes ou paralisação de atividades comerciais) decorrentes de qualquer forma do uso ou da impossibilidade de uso deste software, mesmo se avisados da possibilidade de tais danos. Você assume toda a responsabilidade e os riscos pelo uso deste programa.

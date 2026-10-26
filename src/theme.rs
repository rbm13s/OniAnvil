use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

// Drawing & Selection tools
pub const FA_PENCIL: &str = "\u{f303}";
pub const FA_ERASER: &str = "\u{f12d}";
pub const FA_EYEDROPPER: &str = "\u{f1fb}";
pub const FA_FLOOD_FILL: &str = "\u{f576}";
pub const FA_GRADIENT: &str = "\u{f5c3}";
pub const FA_LINE: &str = "\u{f715}";
pub const FA_CURVE: &str = "\u{f55b}";
pub const FA_RECTANGLE: &str = "\u{f0c8}";
pub const FA_ELLIPSE: &str = "\u{f111}";
pub const FA_DITHER: &str = "\u{f43c}";
pub const FA_SHADING: &str = "\u{f042}";
pub const FA_RECT_SELECT: &str = "\u{f5cb}";
pub const FA_LASSO: &str = "\u{f5ee}";
pub const FA_MAGIC_WAND: &str = "\u{f0d0}";
pub const FA_PIVOT: &str = "\u{f05b}";
pub const FA_SWAP: &str = "\u{f0ec}";
pub const FA_SORT: &str = "\u{f0dc}";

// History & Transforms
pub const FA_UNDO: &str = "\u{f2ea}";
pub const FA_REDO: &str = "\u{f2f9}";
pub const FA_FLIP_H: &str = "\u{f337}";
pub const FA_FLIP_V: &str = "\u{f338}";
pub const FA_ROTATE_90: &str = "\u{f2f9}";
pub const FA_OUTLINE: &str = "\u{f84c}";

// Playback & Animation Transport
pub const FA_PLAY: &str = "\u{f04b}";
pub const FA_PAUSE: &str = "\u{f04c}";
pub const FA_STEP_PREV: &str = "\u{f048}";
pub const FA_STEP_NEXT: &str = "\u{f051}";

// Navigation, Zoom & Viewport
pub const FA_ZOOM_IN: &str = "\u{f00e}";
pub const FA_ZOOM_OUT: &str = "\u{f010}";
pub const FA_FIT: &str = "\u{f31e}";
pub const FA_VIEW_FRAME: &str = "\u{f03e}";
pub const FA_VIEW_ATLAS: &str = "\u{f00a}";
pub const FA_GRID_PIXEL: &str = "\u{f84c}";
pub const FA_GRID_TILED: &str = "\u{f009}";

// Common Actions & Operations
pub const FA_PLUS: &str = "\u{f067}";
pub const FA_MINUS: &str = "\u{f068}";
pub const FA_CHECK: &str = "\u{f00c}";
pub const FA_COPY: &str = "\u{f0c5}";
pub const FA_SCISSORS: &str = "\u{f0c4}";
pub const FA_CLIPBOARD: &str = "\u{f0ea}";
pub const FA_TRASH: &str = "\u{f2ed}";
pub const FA_CLOSE: &str = "\u{f00d}";
pub const FA_TAG: &str = "\u{f02b}";
pub const FA_IMPORT: &str = "\u{f56f}";
pub const FA_EXPORT: &str = "\u{f56e}";
pub const FA_PALETTE: &str = "\u{f53f}";
pub const FA_SLIDERS: &str = "\u{f1de}";
pub const FA_BAKE: &str = "\u{f06d}";
pub const FA_RESET: &str = "\u{f2ea}";
pub const FA_FOLDER: &str = "\u{f07c}";
pub const FA_SAVE: &str = "\u{f0c7}";
pub const FA_EYE: &str = "\u{f06e}";
pub const FA_EYE_SLASH: &str = "\u{f070}";
pub const FA_LOCK: &str = "\u{f023}";
pub const FA_LOCK_OPEN: &str = "\u{f3c1}";
pub const FA_LAYER_GROUP: &str = "\u{f5fd}";
#[allow(dead_code)]
pub const FA_GLOBE: &str = "\u{f0ac}";
pub const FA_QUESTION: &str = "\u{f059}";
pub const FA_ARROW_UP: &str = "\u{f062}";
pub const FA_ARROW_DOWN: &str = "\u{f063}";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum AppTheme {
    #[default]
    StudioDark,
    StudioLight,
    System,
    CrtAmber,
    CrtGreen,
    CrtCyan,
    Famicom,
    MegaDrive,
    Snes,
    GameBoy,
    GameBoyLight,
    SuperGameBoy,
    ZxSpectrum,
    Commodore64,
    MacClassic,
    Pico8,
    CyberpunkNeon,
    Synthwave,
    NordFrost,
    Dracula,
    TokyoNight,
    CatppuccinMocha,
    GruvboxDark,
    MonokaiPro,
    SolarizedDark,
    SolarizedLight,
    Ps1,
    Dreamcast,
    Ps2,
    Windows98,
    AmigaWorkbench,
    AtariSt,
    Msx,
    Apple2,
    DosCga,
}


impl AppTheme {
    pub const ALL: [AppTheme; 35] = [
        AppTheme::StudioDark,
        AppTheme::StudioLight,
        AppTheme::System,
        AppTheme::CrtAmber,
        AppTheme::CrtGreen,
        AppTheme::CrtCyan,
        AppTheme::Famicom,
        AppTheme::MegaDrive,
        AppTheme::Snes,
        AppTheme::GameBoy,
        AppTheme::GameBoyLight,
        AppTheme::SuperGameBoy,
        AppTheme::Ps1,
        AppTheme::Dreamcast,
        AppTheme::Ps2,
        AppTheme::ZxSpectrum,
        AppTheme::Commodore64,
        AppTheme::AmigaWorkbench,
        AppTheme::AtariSt,
        AppTheme::Msx,
        AppTheme::Apple2,
        AppTheme::DosCga,
        AppTheme::MacClassic,
        AppTheme::Windows98,
        AppTheme::Pico8,
        AppTheme::CyberpunkNeon,
        AppTheme::Synthwave,
        AppTheme::NordFrost,
        AppTheme::Dracula,
        AppTheme::TokyoNight,
        AppTheme::CatppuccinMocha,
        AppTheme::GruvboxDark,
        AppTheme::MonokaiPro,
        AppTheme::SolarizedDark,
        AppTheme::SolarizedLight,
    ];

    pub fn i18n_key(self) -> &'static str {
        match self {
            Self::StudioDark => "theme.dark",
            Self::StudioLight => "theme.light",
            Self::System => "theme.system",
            Self::CrtAmber => "theme.amber",
            Self::CrtGreen => "theme.crt_green",
            Self::CrtCyan => "theme.crt_cyan",
            Self::Famicom => "theme.famicom",
            Self::MegaDrive => "theme.megadrive",
            Self::Snes => "theme.snes",
            Self::GameBoy => "theme.gameboy",
            Self::GameBoyLight => "theme.gb_light",
            Self::SuperGameBoy => "theme.super_gb",
            Self::Ps1 => "theme.ps1",
            Self::Dreamcast => "theme.dreamcast",
            Self::Ps2 => "theme.ps2",
            Self::ZxSpectrum => "theme.spectrum",
            Self::Commodore64 => "theme.c64",
            Self::AmigaWorkbench => "theme.amiga",
            Self::AtariSt => "theme.atari_st",
            Self::Msx => "theme.msx",
            Self::Apple2 => "theme.apple2",
            Self::DosCga => "theme.pc_dos",
            Self::MacClassic => "theme.mac_classic",
            Self::Windows98 => "theme.win98",
            Self::Pico8 => "theme.pico8",
            Self::CyberpunkNeon => "theme.cyberpunk",
            Self::Synthwave => "theme.synthwave",
            Self::NordFrost => "theme.nord",
            Self::Dracula => "theme.dracula",
            Self::TokyoNight => "theme.tokyo",
            Self::CatppuccinMocha => "theme.catppuccin",
            Self::GruvboxDark => "theme.gruvbox",
            Self::MonokaiPro => "theme.monokai",
            Self::SolarizedDark => "theme.solarized",
            Self::SolarizedLight => "theme.solarized_light",
        }
    }

    pub fn apply(self, ctx: &egui::Context) {
        let mut style: Style = (*ctx.style()).clone();

        if self == Self::System {
            let visuals = Visuals {
                window_corner_radius: CornerRadius::same(4),
                ..Default::default()
            };
            style.visuals = visuals;
            ctx.set_style(style);
            return;
        }

        let is_light = self == Self::StudioLight
            || self == Self::SolarizedLight
            || self == Self::MacClassic
            || self == Self::Snes
            || self == Self::Windows98
            || self == Self::Dreamcast
            || self == Self::AtariSt;
        let mut visuals = if is_light {
            Visuals::light()
        } else {
            Visuals::dark()
        };

        let (
            bg_panel,
            bg_backdrop,
            bg_widget_inactive,
            bg_widget_hovered,
            bg_widget_active,
            border_subtle,
            border_focused,
            text_primary,
            selection_bg,
        ) = match self {
            Self::StudioDark | Self::System => (
                Color32::from_rgb(28, 28, 28),
                Color32::from_rgb(18, 18, 18),
                Color32::from_rgb(44, 44, 44),
                Color32::from_rgb(58, 58, 58),
                Color32::from_rgb(76, 76, 76),
                Color32::from_rgb(54, 54, 54),
                Color32::from_rgb(130, 130, 130),
                Color32::from_rgb(220, 220, 220),
                Color32::from_rgb(92, 92, 92),
            ),
            Self::StudioLight => (
                Color32::from_rgb(244, 244, 248),
                Color32::from_rgb(224, 224, 230),
                Color32::from_rgb(232, 232, 240),
                Color32::from_rgb(212, 214, 226),
                Color32::from_rgb(180, 185, 205),
                Color32::from_rgb(175, 178, 195),
                Color32::from_rgb(90, 95, 120),
                Color32::from_rgb(16, 16, 24),
                Color32::from_rgb(60, 100, 210),
            ),
            Self::GameBoy => (
                Color32::from_rgb(48, 98, 48),
                Color32::from_rgb(15, 56, 15),
                Color32::from_rgb(62, 117, 62),
                Color32::from_rgb(82, 143, 82),
                Color32::from_rgb(139, 172, 15),
                Color32::from_rgb(28, 68, 28),
                Color32::from_rgb(155, 188, 15),
                Color32::from_rgb(155, 188, 15),
                Color32::from_rgb(139, 172, 15),
            ),
            Self::CyberpunkNeon => (
                Color32::from_rgb(18, 19, 36),
                Color32::from_rgb(9, 10, 20),
                Color32::from_rgb(30, 32, 58),
                Color32::from_rgb(42, 46, 82),
                Color32::from_rgb(0, 240, 255),
                Color32::from_rgb(255, 0, 85),
                Color32::from_rgb(0, 240, 255),
                Color32::from_rgb(230, 249, 255),
                Color32::from_rgb(255, 0, 85),
            ),
            Self::Synthwave => (
                Color32::from_rgb(36, 23, 52),
                Color32::from_rgb(22, 13, 34),
                Color32::from_rgb(54, 34, 78),
                Color32::from_rgb(74, 46, 106),
                Color32::from_rgb(247, 89, 144),
                Color32::from_rgb(104, 59, 140),
                Color32::from_rgb(255, 210, 117),
                Color32::from_rgb(255, 220, 140),
                Color32::from_rgb(224, 59, 139),
            ),
            Self::NordFrost => (
                Color32::from_rgb(46, 52, 64),
                Color32::from_rgb(36, 41, 51),
                Color32::from_rgb(59, 66, 82),
                Color32::from_rgb(67, 76, 94),
                Color32::from_rgb(76, 86, 106),
                Color32::from_rgb(76, 86, 106),
                Color32::from_rgb(136, 192, 208),
                Color32::from_rgb(236, 239, 244),
                Color32::from_rgb(136, 192, 208),
            ),
            Self::MonokaiPro => (
                Color32::from_rgb(34, 31, 34),
                Color32::from_rgb(25, 24, 26),
                Color32::from_rgb(45, 42, 46),
                Color32::from_rgb(64, 62, 65),
                Color32::from_rgb(255, 216, 102),
                Color32::from_rgb(64, 62, 65),
                Color32::from_rgb(255, 216, 102),
                Color32::from_rgb(252, 252, 250),
                Color32::from_rgb(255, 97, 136),
            ),
            Self::SolarizedDark => (
                Color32::from_rgb(7, 54, 66),
                Color32::from_rgb(0, 43, 54),
                Color32::from_rgb(11, 66, 80),
                Color32::from_rgb(19, 82, 99),
                Color32::from_rgb(38, 139, 210),
                Color32::from_rgb(26, 94, 112),
                Color32::from_rgb(42, 161, 152),
                Color32::from_rgb(147, 161, 161),
                Color32::from_rgb(42, 161, 152),
            ),
            Self::CrtAmber => (
                Color32::from_rgb(28, 18, 0),
                Color32::from_rgb(10, 6, 0),
                Color32::from_rgb(43, 29, 2),
                Color32::from_rgb(61, 42, 4),
                Color32::from_rgb(255, 176, 0),
                Color32::from_rgb(94, 64, 8),
                Color32::from_rgb(255, 176, 0),
                Color32::from_rgb(255, 176, 0),
                Color32::from_rgb(204, 136, 0),
            ),
            Self::CrtGreen => (
                Color32::from_rgb(0, 24, 8),
                Color32::from_rgb(0, 12, 4),
                Color32::from_rgb(0, 42, 16),
                Color32::from_rgb(0, 64, 24),
                Color32::from_rgb(0, 255, 65),
                Color32::from_rgb(0, 56, 20),
                Color32::from_rgb(51, 255, 102),
                Color32::from_rgb(51, 255, 102),
                Color32::from_rgb(0, 128, 48),
            ),
            Self::ZxSpectrum => (
                Color32::from_rgb(18, 18, 24),
                Color32::from_rgb(0, 0, 0),
                Color32::from_rgb(32, 32, 44),
                Color32::from_rgb(0, 0, 215),
                Color32::from_rgb(255, 255, 0),
                Color32::from_rgb(0, 180, 215),
                Color32::from_rgb(255, 0, 0),
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(215, 0, 0),
            ),
            Self::Commodore64 => (
                Color32::from_rgb(56, 48, 112),
                Color32::from_rgb(36, 30, 72),
                Color32::from_rgb(72, 64, 136),
                Color32::from_rgb(92, 82, 168),
                Color32::from_rgb(120, 105, 196),
                Color32::from_rgb(40, 32, 84),
                Color32::from_rgb(160, 144, 224),
                Color32::from_rgb(216, 208, 240),
                Color32::from_rgb(160, 144, 224),
            ),
            Self::MacClassic => (
                Color32::from_rgb(222, 222, 222),
                Color32::from_rgb(180, 180, 180),
                Color32::from_rgb(236, 236, 236),
                Color32::from_rgb(208, 208, 208),
                Color32::from_rgb(51, 102, 204),
                Color32::from_rgb(150, 150, 150),
                Color32::from_rgb(30, 30, 30),
                Color32::from_rgb(10, 10, 10),
                Color32::from_rgb(51, 102, 204),
            ),
            Self::Pico8 => (
                Color32::from_rgb(36, 24, 56),
                Color32::from_rgb(20, 12, 32),
                Color32::from_rgb(60, 36, 92),
                Color32::from_rgb(88, 50, 120),
                Color32::from_rgb(255, 0, 77),
                Color32::from_rgb(126, 37, 83),
                Color32::from_rgb(255, 119, 168),
                Color32::from_rgb(255, 241, 232),
                Color32::from_rgb(255, 119, 168),
            ),
            Self::Dracula => (
                Color32::from_rgb(40, 42, 54),
                Color32::from_rgb(33, 34, 44),
                Color32::from_rgb(52, 55, 70),
                Color32::from_rgb(68, 71, 90),
                Color32::from_rgb(189, 147, 249),
                Color32::from_rgb(68, 71, 90),
                Color32::from_rgb(139, 233, 253),
                Color32::from_rgb(248, 248, 242),
                Color32::from_rgb(189, 147, 249),
            ),
            Self::TokyoNight => (
                Color32::from_rgb(26, 27, 38),
                Color32::from_rgb(19, 20, 29),
                Color32::from_rgb(36, 40, 59),
                Color32::from_rgb(53, 59, 87),
                Color32::from_rgb(122, 162, 247),
                Color32::from_rgb(41, 46, 66),
                Color32::from_rgb(187, 154, 247),
                Color32::from_rgb(192, 202, 245),
                Color32::from_rgb(122, 162, 247),
            ),
            Self::CatppuccinMocha => (
                Color32::from_rgb(30, 30, 46),
                Color32::from_rgb(24, 24, 37),
                Color32::from_rgb(49, 50, 68),
                Color32::from_rgb(69, 71, 90),
                Color32::from_rgb(203, 166, 247),
                Color32::from_rgb(69, 71, 90),
                Color32::from_rgb(245, 194, 231),
                Color32::from_rgb(205, 214, 244),
                Color32::from_rgb(180, 190, 254),
            ),
            Self::GruvboxDark => (
                Color32::from_rgb(40, 40, 40),
                Color32::from_rgb(29, 32, 33),
                Color32::from_rgb(60, 56, 54),
                Color32::from_rgb(80, 73, 69),
                Color32::from_rgb(250, 189, 47),
                Color32::from_rgb(80, 73, 69),
                Color32::from_rgb(254, 128, 25),
                Color32::from_rgb(235, 219, 178),
                Color32::from_rgb(214, 93, 14),
            ),
            Self::SolarizedLight => (
                Color32::from_rgb(253, 246, 227),
                Color32::from_rgb(238, 232, 213),
                Color32::from_rgb(238, 232, 213),
                Color32::from_rgb(218, 210, 185),
                Color32::from_rgb(181, 137, 0),
                Color32::from_rgb(195, 186, 158),
                Color32::from_rgb(38, 139, 210),
                Color32::from_rgb(7, 54, 66),
                Color32::from_rgb(42, 161, 152),
            ),
            Self::CrtCyan => (
                Color32::from_rgb(0, 20, 28),
                Color32::from_rgb(0, 10, 16),
                Color32::from_rgb(0, 36, 50),
                Color32::from_rgb(0, 56, 76),
                Color32::from_rgb(0, 229, 255),
                Color32::from_rgb(0, 50, 68),
                Color32::from_rgb(0, 229, 255),
                Color32::from_rgb(51, 235, 255),
                Color32::from_rgb(0, 120, 150),
            ),
            Self::Famicom => (
                Color32::from_rgb(120, 24, 36),
                Color32::from_rgb(72, 12, 22),
                Color32::from_rgb(148, 36, 50),
                Color32::from_rgb(180, 48, 64),
                Color32::from_rgb(235, 195, 80),
                Color32::from_rgb(88, 16, 26),
                Color32::from_rgb(235, 195, 80),
                Color32::from_rgb(248, 242, 224),
                Color32::from_rgb(215, 170, 50),
            ),
            Self::MegaDrive => (
                Color32::from_rgb(24, 25, 28),
                Color32::from_rgb(14, 15, 18),
                Color32::from_rgb(38, 40, 46),
                Color32::from_rgb(54, 58, 68),
                Color32::from_rgb(220, 175, 40),
                Color32::from_rgb(48, 52, 62),
                Color32::from_rgb(0, 136, 255),
                Color32::from_rgb(235, 238, 245),
                Color32::from_rgb(0, 110, 220),
            ),
            Self::Snes => (
                Color32::from_rgb(208, 208, 218),
                Color32::from_rgb(180, 180, 194),
                Color32::from_rgb(222, 222, 230),
                Color32::from_rgb(192, 192, 206),
                Color32::from_rgb(79, 66, 132),
                Color32::from_rgb(160, 160, 178),
                Color32::from_rgb(141, 121, 178),
                Color32::from_rgb(28, 26, 36),
                Color32::from_rgb(108, 90, 168),
            ),
            Self::GameBoyLight => (
                Color32::from_rgb(22, 92, 78),
                Color32::from_rgb(10, 52, 44),
                Color32::from_rgb(32, 120, 102),
                Color32::from_rgb(45, 155, 132),
                Color32::from_rgb(72, 255, 216),
                Color32::from_rgb(16, 74, 62),
                Color32::from_rgb(72, 255, 216),
                Color32::from_rgb(5, 32, 26),
                Color32::from_rgb(45, 185, 156),
            ),
            Self::SuperGameBoy => (
                Color32::from_rgb(52, 44, 76),
                Color32::from_rgb(34, 28, 52),
                Color32::from_rgb(70, 60, 100),
                Color32::from_rgb(92, 78, 128),
                Color32::from_rgb(255, 198, 54),
                Color32::from_rgb(40, 32, 60),
                Color32::from_rgb(255, 110, 140),
                Color32::from_rgb(240, 235, 250),
                Color32::from_rgb(230, 80, 120),
            ),
            Self::Ps1 => (
                Color32::from_rgb(42, 43, 50),
                Color32::from_rgb(26, 27, 32),
                Color32::from_rgb(58, 60, 70),
                Color32::from_rgb(76, 78, 92),
                Color32::from_rgb(0, 168, 150),
                Color32::from_rgb(68, 70, 82),
                Color32::from_rgb(238, 90, 131),
                Color32::from_rgb(235, 236, 242),
                Color32::from_rgb(46, 67, 158),
            ),
            Self::Dreamcast => (
                Color32::from_rgb(238, 238, 242),
                Color32::from_rgb(218, 220, 226),
                Color32::from_rgb(226, 227, 233),
                Color32::from_rgb(210, 212, 222),
                Color32::from_rgb(255, 92, 0),
                Color32::from_rgb(186, 188, 198),
                Color32::from_rgb(255, 92, 0),
                Color32::from_rgb(24, 26, 34),
                Color32::from_rgb(255, 106, 16),
            ),
            Self::Ps2 => (
                Color32::from_rgb(18, 20, 30),
                Color32::from_rgb(10, 11, 18),
                Color32::from_rgb(30, 34, 52),
                Color32::from_rgb(44, 50, 78),
                Color32::from_rgb(0, 170, 255),
                Color32::from_rgb(38, 44, 68),
                Color32::from_rgb(0, 210, 255),
                Color32::from_rgb(225, 235, 255),
                Color32::from_rgb(20, 60, 180),
            ),
            Self::Windows98 => (
                Color32::from_rgb(198, 198, 198),
                Color32::from_rgb(0, 128, 128),
                Color32::from_rgb(212, 212, 212),
                Color32::from_rgb(228, 228, 228),
                Color32::from_rgb(0, 0, 128),
                Color32::from_rgb(128, 128, 128),
                Color32::from_rgb(0, 0, 128),
                Color32::from_rgb(0, 0, 0),
                Color32::from_rgb(0, 0, 128),
            ),
            Self::AmigaWorkbench => (
                Color32::from_rgb(0, 85, 170),
                Color32::from_rgb(0, 50, 105),
                Color32::from_rgb(18, 102, 192),
                Color32::from_rgb(32, 120, 216),
                Color32::from_rgb(255, 136, 0),
                Color32::from_rgb(0, 68, 140),
                Color32::from_rgb(255, 136, 0),
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(255, 136, 0),
            ),
            Self::AtariSt => (
                Color32::from_rgb(218, 220, 214),
                Color32::from_rgb(190, 194, 184),
                Color32::from_rgb(230, 232, 226),
                Color32::from_rgb(205, 208, 200),
                Color32::from_rgb(24, 98, 136),
                Color32::from_rgb(152, 156, 144),
                Color32::from_rgb(24, 98, 136),
                Color32::from_rgb(20, 22, 18),
                Color32::from_rgb(34, 116, 158),
            ),
            Self::Msx => (
                Color32::from_rgb(24, 28, 64),
                Color32::from_rgb(12, 14, 38),
                Color32::from_rgb(38, 44, 92),
                Color32::from_rgb(52, 60, 120),
                Color32::from_rgb(64, 224, 208),
                Color32::from_rgb(48, 54, 108),
                Color32::from_rgb(240, 80, 140),
                Color32::from_rgb(238, 242, 255),
                Color32::from_rgb(48, 180, 220),
            ),
            Self::Apple2 => (
                Color32::from_rgb(16, 22, 16),
                Color32::from_rgb(8, 12, 8),
                Color32::from_rgb(26, 36, 26),
                Color32::from_rgb(38, 54, 38),
                Color32::from_rgb(34, 204, 68),
                Color32::from_rgb(32, 48, 32),
                Color32::from_rgb(154, 76, 222),
                Color32::from_rgb(52, 230, 88),
                Color32::from_rgb(26, 140, 52),
            ),
            Self::DosCga => (
                Color32::from_rgb(0, 0, 140),
                Color32::from_rgb(0, 0, 68),
                Color32::from_rgb(0, 0, 180),
                Color32::from_rgb(20, 40, 210),
                Color32::from_rgb(0, 180, 180),
                Color32::from_rgb(0, 0, 210),
                Color32::from_rgb(255, 255, 85),
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(0, 160, 160),
            ),
        };

        visuals.panel_fill = bg_panel;
        visuals.window_fill = bg_panel;
        visuals.extreme_bg_color = bg_backdrop;
        visuals.faint_bg_color = bg_widget_inactive;
        visuals.code_bg_color = bg_widget_inactive;
        visuals.override_text_color = Some(text_primary);

        let widget_text = text_primary;

        visuals.widgets.noninteractive.bg_fill = bg_panel;
        visuals.widgets.noninteractive.weak_bg_fill = bg_panel;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_subtle);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, widget_text);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(2);

        visuals.widgets.inactive.bg_fill = bg_widget_inactive;
        visuals.widgets.inactive.weak_bg_fill = bg_widget_inactive;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_subtle);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, widget_text);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(3);

        visuals.widgets.hovered.bg_fill = bg_widget_hovered;
        visuals.widgets.hovered.weak_bg_fill = bg_widget_hovered;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, border_focused);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, if is_light { Color32::from_rgb(0, 0, 0) } else { Color32::WHITE });
        visuals.widgets.hovered.corner_radius = CornerRadius::same(3);

        visuals.widgets.active.bg_fill = bg_widget_active;
        visuals.widgets.active.weak_bg_fill = bg_widget_active;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, border_focused);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, if is_light { Color32::from_rgb(0, 0, 0) } else { Color32::WHITE });
        visuals.widgets.active.corner_radius = CornerRadius::same(3);

        visuals.widgets.open.bg_fill = bg_widget_hovered;
        visuals.widgets.open.weak_bg_fill = bg_widget_hovered;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, border_focused);
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, widget_text);
        visuals.widgets.open.corner_radius = CornerRadius::same(3);

        visuals.selection.bg_fill = selection_bg;
        visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

        visuals.window_corner_radius = CornerRadius::same(4);
        visuals.window_stroke = Stroke::new(1.0, border_subtle);

        style.spacing.item_spacing = egui::vec2(5.0, 4.0);
        style.spacing.button_padding = egui::vec2(6.0, 3.0);
        style.spacing.window_margin = egui::Margin::same(8);
        style.visuals = visuals;

        ctx.set_style(style);
    }
}

/// Returns true if the slice starts with a valid TrueType or OpenType header magic.
pub fn is_valid_font_bytes(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return false;
    }
    let tag = &bytes[0..4];
    tag == b"\x00\x01\x00\x00"
        || tag == b"OTTO"
        || tag == b"true"
        || tag == b"typ1"
        || tag == b"ttcf"
}

/// If given a TrueType Collection (`.ttc`, header `ttcf`), extracts the first face (face 0)
/// into a standalone TrueType (`.ttf`) buffer.
pub fn ensure_ttf_bytes(bytes: Vec<u8>) -> Vec<u8> {
    if bytes.len() < 16 || &bytes[0..4] != b"ttcf" {
        return bytes;
    }
    let num_fonts = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
    if num_fonts == 0 {
        return bytes;
    }
    let face0_offset = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
    if face0_offset + 12 > bytes.len() {
        return bytes;
    }
    let num_tables =
        u16::from_be_bytes([bytes[face0_offset + 4], bytes[face0_offset + 5]]) as usize;
    let dir_end = face0_offset + 12 + num_tables * 16;
    if dir_end > bytes.len() {
        return bytes;
    }

    let mut out = Vec::with_capacity(bytes.len() / 2);
    out.extend_from_slice(&bytes[face0_offset..face0_offset + 12]);
    let records_pos = out.len();
    out.resize(records_pos + num_tables * 16, 0);

    for i in 0..num_tables {
        let rec_src = face0_offset + 12 + i * 16;
        let tag = &bytes[rec_src..rec_src + 4];
        let checksum = &bytes[rec_src + 4..rec_src + 8];
        let orig_offset = u32::from_be_bytes([
            bytes[rec_src + 8],
            bytes[rec_src + 9],
            bytes[rec_src + 10],
            bytes[rec_src + 11],
        ]) as usize;
        let length = u32::from_be_bytes([
            bytes[rec_src + 12],
            bytes[rec_src + 13],
            bytes[rec_src + 14],
            bytes[rec_src + 15],
        ]) as usize;

        if orig_offset + length > bytes.len() {
            return bytes;
        }
        let new_offset = out.len() as u32;
        let rec_dst = records_pos + i * 16;
        out[rec_dst..rec_dst + 4].copy_from_slice(tag);
        out[rec_dst + 4..rec_dst + 8].copy_from_slice(checksum);
        out[rec_dst + 8..rec_dst + 12].copy_from_slice(&new_offset.to_be_bytes());
        out[rec_dst + 12..rec_dst + 16].copy_from_slice(&(length as u32).to_be_bytes());

        out.extend_from_slice(&bytes[orig_offset..orig_offset + length]);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    }
    out
}

#[cfg(not(target_arch = "wasm32"))]
fn read_system_font_bytes(path: &str) -> Option<Vec<u8>> {
    #[cfg(target_os = "windows")]
    if let Some(stripped) = path.strip_prefix("C:\\Windows\\Fonts\\") {
        if let Ok(windir) = std::env::var("WINDIR") {
            let p = std::path::Path::new(&windir).join("Fonts").join(stripped);
            if let Ok(bytes) = std::fs::read(&p) {
                return Some(bytes);
            }
        }
        if let Ok(sysroot) = std::env::var("SystemRoot") {
            let p = std::path::Path::new(&sysroot).join("Fonts").join(stripped);
            if let Ok(bytes) = std::fs::read(&p) {
                return Some(bytes);
            }
        }
    }
    std::fs::read(path).ok()
}

/// Configures fonts including Font Awesome 6 Solid for icons, system fonts, and web fonts.
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "fa_solid".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/fa-solid-900.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("fa_solid".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("fa_solid".to_owned());

    #[cfg(not(target_arch = "wasm32"))]
    {
        // 1. Windows / Desktop system fonts for emoji and UI symbols
        if let Some(bytes) = read_system_font_bytes("C:\\Windows\\Fonts\\seguiemj.ttf") {
            fonts.font_data.insert(
                "seguiemj".to_owned(),
                std::sync::Arc::new(FontData::from_owned(bytes)),
            );
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "seguiemj".to_owned());
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "seguiemj".to_owned());
        }

        // 2. Proportional UI fonts (Segoe UI, SF/Arial, DejaVu/Noto)
        for (font_key, path) in [
            ("sys_sans_win", "C:\\Windows\\Fonts\\segoeui.ttf"),
            ("sys_sans_mac", "/System/Library/Fonts/Supplemental/Arial.ttf"),
            ("sys_sans_linux_dejavu", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
            ("sys_sans_linux_liberation", "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf"),
            ("sys_sans_linux_noto", "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf"),
        ] {
            if let Some(raw) = read_system_font_bytes(path) {
                let ttf = ensure_ttf_bytes(raw);
                fonts
                    .font_data
                    .insert(font_key.to_owned(), std::sync::Arc::new(FontData::from_owned(ttf)));
                fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_key.to_owned());
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_key.to_owned());
                break;
            }
        }

        // 3. CJK and Caucasus (Georgian / Armenian) system fonts on Desktop
        for (font_key, path) in [
            ("caucasus_sylfaen", "C:\\Windows\\Fonts\\sylfaen.ttf"),
            ("cjk_msyh", "C:\\Windows\\Fonts\\msyh.ttc"),
            ("cjk_msjh", "C:\\Windows\\Fonts\\msjh.ttc"),
            ("cjk_malgun", "C:\\Windows\\Fonts\\malgun.ttf"),
            ("cjk_mac_pingfang", "/System/Library/Fonts/PingFang.ttc"),
            ("cjk_mac_sdgothic", "/System/Library/Fonts/AppleSDGothicNeo.ttc"),
            ("cjk_mac_arial_unicode", "/System/Library/Fonts/Supplemental/Arial Unicode.ttf"),
            ("cjk_linux_noto_ttc", "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"),
            ("caucasus_linux_dejavu", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
        ] {
            if let Some(raw) = read_system_font_bytes(path) {
                let ttf = ensure_ttf_bytes(raw);
                fonts
                    .font_data
                    .insert(font_key.to_owned(), std::sync::Arc::new(FontData::from_owned(ttf)));
                fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .push(font_key.to_owned());
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .push(font_key.to_owned());
            }
        }

        // 4. Fallback to local web/fonts files if available on disk
        for (font_key, path) in [
            ("noto_sans_georgian", "web/fonts/NotoSansGeorgian-Regular.ttf"),
            ("noto_sans_armenian", "web/fonts/NotoSansArmenian-Regular.ttf"),
            ("noto_sans_ext", "web/fonts/NotoSans-Regular.ttf"),
        ] {
            if !fonts.font_data.contains_key(font_key) {
                if let Ok(bytes) = std::fs::read(path) {
                    fonts
                        .font_data
                        .insert(font_key.to_owned(), std::sync::Arc::new(FontData::from_owned(bytes)));
                    fonts
                        .families
                        .entry(FontFamily::Proportional)
                        .or_default()
                        .push(font_key.to_owned());
                    fonts
                        .families
                        .entry(FontFamily::Monospace)
                        .or_default()
                        .push(font_key.to_owned());
                }
            }
        }
    }

    // 5. Dynamically downloaded web fonts (WASM / runtime)
    for (font_key, font_bytes) in crate::web::get_all_installed_fonts() {
        let ttf = ensure_ttf_bytes(font_bytes);
        if is_valid_font_bytes(&ttf) {
            fonts
                .font_data
                .insert(font_key.clone(), std::sync::Arc::new(FontData::from_owned(ttf)));
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .push(font_key.clone());
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .push(font_key);
        }
    }

    ctx.set_fonts(fonts);
}

/// Applies the default studio theme.
#[allow(dead_code)]
pub fn apply_studio_theme(ctx: &egui::Context) {
    setup_custom_fonts(ctx);
    AppTheme::StudioDark.apply(ctx);
}

/// Loads the application logo texture from embedded assets (`assets/icon.ico`).
pub fn load_app_logo_texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let bytes = include_bytes!("../assets/icon.ico");
    let dyn_img = image::load_from_memory(bytes).ok()?;
    let rgba = dyn_img.to_rgba8();
    let dims = [rgba.width() as usize, rgba.height() as usize];
    let color_img = egui::ColorImage::from_rgba_unmultiplied(dims, rgba.as_flat_samples().as_slice());
    Some(ctx.load_texture("onianvil_app_logo", color_img, egui::TextureOptions::LINEAR))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_themes_apply_and_have_valid_keys() {
        let ctx = egui::Context::default();
        for theme in AppTheme::ALL {
            let key = theme.i18n_key();
            assert!(!key.is_empty());
            theme.apply(&ctx);
        }
    }
}

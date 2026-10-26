pub mod fill;
pub mod filters;
pub mod selection;
pub mod shapes;
pub mod stroke;
pub mod transforms;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ToolKind {
    #[default]
    Pencil,
    Eraser,
    Eyedropper,
    FloodFill,
    Gradient,
    Line,
    Curve,
    Rectangle,
    Ellipse,
    DitherBrush,
    Shading,
    RectSelect,
    Lasso,
    MagicWand,
    PivotPoint,
}


impl ToolKind {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Pencil => crate::theme::FA_PENCIL,
            Self::Eraser => crate::theme::FA_ERASER,
            Self::Eyedropper => crate::theme::FA_EYEDROPPER,
            Self::FloodFill => crate::theme::FA_FLOOD_FILL,
            Self::Gradient => crate::theme::FA_GRADIENT,
            Self::Line => crate::theme::FA_LINE,
            Self::Curve => crate::theme::FA_CURVE,
            Self::Rectangle => crate::theme::FA_RECTANGLE,
            Self::Ellipse => crate::theme::FA_ELLIPSE,
            Self::DitherBrush => crate::theme::FA_DITHER,
            Self::Shading => crate::theme::FA_SHADING,
            Self::RectSelect => crate::theme::FA_RECT_SELECT,
            Self::Lasso => crate::theme::FA_LASSO,
            Self::MagicWand => crate::theme::FA_MAGIC_WAND,
            Self::PivotPoint => crate::theme::FA_PIVOT,
        }
    }

    pub fn i18n_name_key(self) -> &'static str {
        match self {
            Self::Pencil => "tools.pencil",
            Self::Eraser => "tools.eraser",
            Self::Eyedropper => "tools.eyedropper",
            Self::FloodFill => "tools.fill",
            Self::Gradient => "tools.gradient",
            Self::Line => "tools.line",
            Self::Curve => "tools.curve",
            Self::Rectangle => "tools.rect",
            Self::Ellipse => "tools.ellipse",
            Self::DitherBrush => "tools.dither",
            Self::Shading => "tools.shading",
            Self::RectSelect => "tools.select_rect",
            Self::Lasso => "tools.lasso",
            Self::MagicWand => "tools.magic_wand",
            Self::PivotPoint => "tools.pivot",
        }
    }

    pub fn i18n_title_key(self) -> &'static str {
        match self {
            Self::Pencil => "tools.pencil.title",
            Self::Eraser => "tools.eraser.title",
            Self::Eyedropper => "tools.eyedropper.title",
            Self::FloodFill => "tools.fill.title",
            Self::Gradient => "tools.gradient.title",
            Self::Line => "tools.line.title",
            Self::Curve => "tools.curve.title",
            Self::Rectangle => "tools.rect.title",
            Self::Ellipse => "tools.ellipse.title",
            Self::DitherBrush => "tools.dither.title",
            Self::Shading => "tools.shading.title",
            Self::RectSelect => "tools.select_rect.title",
            Self::Lasso => "tools.lasso.title",
            Self::MagicWand => "tools.magic_wand.title",
            Self::PivotPoint => "tools.pivot.title",
        }
    }

    #[allow(dead_code)]
    pub fn i18n_btn_key(self) -> &'static str {
        match self {
            Self::Pencil => "tools.pencil.btn",
            Self::Eraser => "tools.eraser.btn",
            Self::Eyedropper => "tools.eyedropper.btn",
            Self::FloodFill => "tools.fill.btn",
            Self::Gradient => "tools.gradient.btn",
            Self::Line => "tools.line.btn",
            Self::Curve => "tools.curve.btn",
            Self::Rectangle => "tools.rect.btn",
            Self::Ellipse => "tools.ellipse.btn",
            Self::DitherBrush => "tools.dither.btn",
            Self::Shading => "tools.shading.btn",
            Self::RectSelect => "tools.select_rect.btn",
            Self::Lasso => "tools.lasso.btn",
            Self::MagicWand => "tools.magic_wand.btn",
            Self::PivotPoint => "tools.pivot.btn",
        }
    }

    pub fn i18n_short_key(self) -> &'static str {
        match self {
            Self::Pencil => "tools.pencil.short",
            Self::Eraser => "tools.eraser.short",
            Self::Eyedropper => "tools.eyedropper.short",
            Self::FloodFill => "tools.fill.short",
            Self::Gradient => "tools.gradient.short",
            Self::Line => "tools.line.short",
            Self::Curve => "tools.curve.short",
            Self::Rectangle => "tools.rect.short",
            Self::Ellipse => "tools.ellipse.short",
            Self::DitherBrush => "tools.dither.short",
            Self::Shading => "tools.shading.short",
            Self::RectSelect => "tools.select_rect.short",
            Self::Lasso => "tools.lasso.short",
            Self::MagicWand => "tools.magic_wand.short",
            Self::PivotPoint => "tools.pivot.short",
        }
    }

    pub const ALL: [ToolKind; 15] = [
        ToolKind::Pencil,
        ToolKind::Eraser,
        ToolKind::Eyedropper,
        ToolKind::FloodFill,
        ToolKind::Gradient,
        ToolKind::Line,
        ToolKind::Curve,
        ToolKind::Rectangle,
        ToolKind::Ellipse,
        ToolKind::DitherBrush,
        ToolKind::Shading,
        ToolKind::RectSelect,
        ToolKind::Lasso,
        ToolKind::MagicWand,
        ToolKind::PivotPoint,
    ];

    pub fn supports_brush_size(self) -> bool {
        match self {
            Self::Pencil
            | Self::Eraser
            | Self::DitherBrush
            | Self::Shading
            | Self::Line
            | Self::Curve
            | Self::Rectangle
            | Self::Ellipse => true,
            Self::Eyedropper
            | Self::FloodFill
            | Self::Gradient
            | Self::MagicWand
            | Self::RectSelect
            | Self::Lasso
            | Self::PivotPoint => false,
        }
    }

    pub fn effective_brush_size(self, brush_size: u32) -> u32 {
        if self.supports_brush_size() {
            brush_size
        } else {
            1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum SymmetryMode {
    #[default]
    None,
    Horizontal,
    Vertical,
    Quad,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum DitherPattern {
    #[default]
    Checker50,
    Sparse25,
    HorizontalLines,
    VerticalLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GradientKind {
    #[default]
    Linear,
    Radial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GradientDither {
    #[default]
    None,
    Bayer4x4,
    Bayer8x8,
    Dither50,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomBrush {
    pub pixels: Vec<((i32, i32), [u8; 4])>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSettings {
    pub active_tool: ToolKind,
    pub fg_color: [u8; 4],
    pub bg_color: [u8; 4],
    pub brush_size: u32,
    pub pixel_perfect: bool,
    pub lock_alpha: bool,
    pub symmetry: SymmetryMode,
    pub dither_pattern: DitherPattern,
    pub filled_shape: bool,
    pub wand_global: bool,
    pub wrap_around: bool,
    pub batch_mode: bool,
    pub two_columns: bool,
    #[serde(skip)]
    pub custom_brush: Option<CustomBrush>,
    pub gradient_kind: GradientKind,
    pub gradient_dither: GradientDither,
    pub tolerance: u8,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            active_tool: ToolKind::Pencil,
            fg_color: [245, 244, 250, 255],
            bg_color: [20, 18, 29, 255],
            brush_size: 1,
            pixel_perfect: true,
            lock_alpha: false,
            symmetry: SymmetryMode::None,
            dither_pattern: DitherPattern::Checker50,
            filled_shape: false,
            wand_global: false,
            wrap_around: true,
            batch_mode: false,
            two_columns: true,
            custom_brush: None,
            gradient_kind: GradientKind::Linear,
            gradient_dither: GradientDither::None,
            tolerance: 0,
        }
    }
}

impl ToolSettings {
    pub fn swap_fg_bg(&mut self) {
        std::mem::swap(&mut self.fg_color, &mut self.bg_color);
    }

    pub fn effective_brush_size(&self) -> u32 {
        if self.active_tool.supports_brush_size() {
            self.brush_size
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_supports_brush_size_and_effective_size() {
        let mut settings = ToolSettings {
            brush_size: 14,
            active_tool: ToolKind::Pencil,
            ..Default::default()
        };

        // Brush tools support brush size
        assert!(ToolKind::Pencil.supports_brush_size());
        assert!(ToolKind::Eraser.supports_brush_size());
        assert!(ToolKind::DitherBrush.supports_brush_size());
        assert!(ToolKind::Shading.supports_brush_size());
        assert!(ToolKind::Line.supports_brush_size());
        assert!(ToolKind::Curve.supports_brush_size());
        assert!(ToolKind::Rectangle.supports_brush_size());
        assert!(ToolKind::Ellipse.supports_brush_size());

        assert_eq!(settings.effective_brush_size(), 14);

        // Eyedropper and tools without brush size report effective size 1
        settings.active_tool = ToolKind::Eyedropper;
        assert!(!ToolKind::Eyedropper.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        settings.active_tool = ToolKind::FloodFill;
        assert!(!ToolKind::FloodFill.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        settings.active_tool = ToolKind::MagicWand;
        assert!(!ToolKind::MagicWand.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        settings.active_tool = ToolKind::Lasso;
        assert!(!ToolKind::Lasso.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        settings.active_tool = ToolKind::RectSelect;
        assert!(!ToolKind::RectSelect.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        settings.active_tool = ToolKind::PivotPoint;
        assert!(!ToolKind::PivotPoint.supports_brush_size());
        assert_eq!(settings.effective_brush_size(), 1);

        // Switching back to Pencil restores configured size 14
        settings.active_tool = ToolKind::Pencil;
        assert_eq!(settings.effective_brush_size(), 14);
    }
}

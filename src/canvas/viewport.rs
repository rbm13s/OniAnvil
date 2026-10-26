use egui::{Pos2, Rect, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CanvasViewMode {
    #[default]
    Frame,
    Atlas,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum BackgroundMode {
    #[default]
    DarkCheckerboard,
    LightCheckerboard,
    SolidColor,
}


pub struct ViewportState {
    pub view_mode: CanvasViewMode,
    pub zoom: f32,
    pub pan_offset: Vec2,
    pub show_pixel_grid: bool,
    pub show_frame_grid: bool,
    pub tiled_preview_3x3: bool,
    pub background_mode: BackgroundMode,
    pub custom_bg_color: [u8; 4],
    pub hover_pixel: Option<(i32, i32)>,
    pub needs_fit_to_view: bool,
    /// Normalized symmetry X axis within the frame (`0.0..=1.0`, default `0.5`).
    pub symmetry_norm_x: f32,
    /// Normalized symmetry Y axis within the frame (`0.0..=1.0`, default `0.5`).
    pub symmetry_norm_y: f32,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            view_mode: CanvasViewMode::Frame,
            zoom: 12.0,
            pan_offset: Vec2::ZERO,
            show_pixel_grid: true,
            show_frame_grid: true,
            tiled_preview_3x3: false,
            background_mode: BackgroundMode::DarkCheckerboard,
            custom_bg_color: [56, 62, 74, 255],
            hover_pixel: None,
            needs_fit_to_view: true,
            symmetry_norm_x: 0.5,
            symmetry_norm_y: 0.5,
        }
    }
}

impl ViewportState {
    pub const ZOOM_PRESETS: [f32; 9] = [1.0, 2.0, 4.0, 8.0, 12.0, 16.0, 24.0, 32.0, 64.0];

    pub fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom = new_zoom.clamp(0.5, 128.0);
    }

    pub fn zoom_in(&mut self) {
        let next = if self.zoom < 2.0 {
            (self.zoom * 1.25).clamp(0.5, 128.0)
        } else {
            (self.zoom * 1.25).round().max(self.zoom + 1.0)
        };
        self.set_zoom(next);
    }

    pub fn zoom_out(&mut self) {
        let next = if self.zoom <= 2.0 {
            (self.zoom / 1.25).clamp(0.5, 128.0)
        } else {
            (self.zoom / 1.25).floor().min(self.zoom - 1.0).max(0.5)
        };
        self.set_zoom(next);
    }

    /// Smoothly zooms proportional to `scroll_y` while keeping the canvas point under `cursor_pos` stationary.
    pub fn zoom_by_wheel_at_cursor(&mut self, viewport_rect: Rect, cursor_pos: Pos2, scroll_y: f32) {
        let factor = (1.0 + scroll_y * 0.0022).clamp(0.75, 1.35);
        let old_zoom = self.zoom;
        let new_zoom = (old_zoom * factor).clamp(0.5, 128.0);
        if (new_zoom - old_zoom).abs() > f32::EPSILON {
            let ratio = new_zoom / old_zoom;
            let vp_center = viewport_rect.center();
            let old_canvas_center = vp_center + self.pan_offset;
            let rel_cursor = cursor_pos - old_canvas_center;
            self.pan_offset = cursor_pos - vp_center - rel_cursor * ratio;
            self.zoom = new_zoom;
        }
    }

    pub fn fit_to_rect(&mut self, viewport_rect: Rect, content_w: u32, content_h: u32) {
        let cw = (content_w.max(1)) as f32;
        let ch = (content_h.max(1)) as f32;
        let avail_w = (viewport_rect.width() - 64.0).max(32.0);
        let avail_h = (viewport_rect.height() - 64.0).max(32.0);

        let scale = (avail_w / cw).min(avail_h / ch);
        let snapped = if scale >= 1.0 {
            scale.floor().max(1.0)
        } else {
            0.5
        };
        self.zoom = snapped.clamp(0.5, 64.0);
        self.pan_offset = Vec2::ZERO;
        self.needs_fit_to_view = false;
    }

    /// Computes the screen-space `Rect` where the active frame or full atlas is drawn.
    pub fn canvas_screen_rect(&self, viewport_rect: Rect, content_w: u32, content_h: u32) -> Rect {
        let scaled_size = Vec2::new(content_w as f32 * self.zoom, content_h as f32 * self.zoom);
        let center = viewport_rect.center() + self.pan_offset;
        Rect::from_center_size(center, scaled_size)
    }

    /// Converts a screen position into integer pixel coordinates relative to the displayed content.
    pub fn screen_to_pixel(&self, canvas_rect: Rect, screen_pos: Pos2) -> (i32, i32) {
        let rel_x = (screen_pos.x - canvas_rect.min.x) / self.zoom;
        let rel_y = (screen_pos.y - canvas_rect.min.y) / self.zoom;
        (rel_x.floor() as i32, rel_y.floor() as i32)
    }
}

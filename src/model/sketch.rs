use image::RgbaImage;
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum SketchLayerMode {
    #[default]
    Underlay,
    Overlay,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum QuickPeekMode {
    #[default]
    PreviousFrame,
    SketchOverlay,
    Both,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SketchReferenceState {
    pub relative_path: Option<String>,
    pub embedded_base64: Option<String>,
    pub embed_in_json: bool,
    pub visible: bool,
    pub layer_mode: SketchLayerMode,
    pub opacity: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
    pub rotation_deg: f32,
    pub flip_h: bool,
    pub flip_v: bool,
    #[serde(default = "default_true")]
    pub snap_sample_to_palette: bool,
    #[serde(default = "default_true")]
    pub bilinear_filter: bool,
}

impl Default for SketchReferenceState {
    fn default() -> Self {
        Self {
            relative_path: None,
            embedded_base64: None,
            embed_in_json: false,
            visible: true,
            layer_mode: SketchLayerMode::Underlay,
            opacity: 0.45,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            rotation_deg: 0.0,
            flip_h: false,
            flip_v: false,
            snap_sample_to_palette: true,
            bilinear_filter: true,
        }
    }
}

impl SketchReferenceState {
    pub fn reset_transform(&mut self) {
        self.offset_x = 0.0;
        self.offset_y = 0.0;
        self.scale = 1.0;
        self.rotation_deg = 0.0;
        self.flip_h = false;
        self.flip_v = false;
    }

    fn half_extents(
        &self,
        frame_w: u32,
        frame_h: u32,
        sketch_w: u32,
        sketch_h: u32,
    ) -> (f32, f32, f32, f32) {
        let fw = frame_w.max(1) as f32;
        let fh = frame_h.max(1) as f32;
        let sw = sketch_w.max(1) as f32;
        let sh = sketch_h.max(1) as f32;

        let base_fit = (fw / sw).min(fh / sh);
        let safe_scale = self.scale.max(0.02);
        let hx = sw * base_fit * safe_scale * 0.5;
        let hy = sh * base_fit * safe_scale * 0.5;
        let cx = fw * 0.5 + self.offset_x;
        let cy = fh * 0.5 + self.offset_y;
        (cx, cy, hx, hy)
    }

    /// Computes the 4 transformed corners `[TL, TR, BR, BL]` in frame-pixel coordinates `(fx, fy)`.
    pub fn transformed_frame_quad(
        &self,
        frame_w: u32,
        frame_h: u32,
        sketch_w: u32,
        sketch_h: u32,
    ) -> [(f32, f32); 4] {
        let (cx, cy, hx, hy) = self.half_extents(frame_w, frame_h, sketch_w, sketch_h);
        let rad = self.rotation_deg.to_radians();
        let (sin_t, cos_t) = rad.sin_cos();

        let local_corners = [(-hx, -hy), (hx, -hy), (hx, hy), (-hx, hy)];
        let mut out = [(0.0, 0.0); 4];
        for (i, (sx, sy)) in local_corners.iter().copied().enumerate() {
            let fx = cx + sx * cos_t - sy * sin_t;
            let fy = cy + sx * sin_t + sy * cos_t;
            out[i] = (fx, fy);
        }
        out
    }

    /// Inverse-transforms a frame-pixel point `(fx, fy)` into the high-resolution `sketch_img`
    /// and returns `Some([r, g, b, a])` if it lands inside the sketch with `a > 0`.
    pub fn sample_rgba_at_frame_pos(
        &self,
        sketch_img: &RgbaImage,
        frame_w: u32,
        frame_h: u32,
        fx: f32,
        fy: f32,
    ) -> Option<[u8; 4]> {
        let sw = sketch_img.width();
        let sh = sketch_img.height();
        if sw == 0 || sh == 0 {
            return None;
        }

        let (cx, cy, hx, hy) = self.half_extents(frame_w, frame_h, sw, sh);
        if hx <= 1e-5 || hy <= 1e-5 {
            return None;
        }

        let dx = fx - cx;
        let dy = fy - cy;
        let rad = self.rotation_deg.to_radians();
        let (sin_t, cos_t) = rad.sin_cos();

        let sx = dx * cos_t + dy * sin_t;
        let sy = -dx * sin_t + dy * cos_t;

        let mut u = sx / (2.0 * hx) + 0.5;
        let mut v = sy / (2.0 * hy) + 0.5;

        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        if self.flip_h {
            u = 1.0 - u;
        }
        if self.flip_v {
            v = 1.0 - v;
        }

        let px = ((u * sw as f32).floor() as u32).min(sw - 1);
        let py = ((v * sh as f32).floor() as u32).min(sh - 1);
        let p = *sketch_img.get_pixel(px, py);
        if p[3] > 0 {
            Some([p[0], p[1], p[2], p[3]])
        } else {
            None
        }
    }
}

/// Finds the nearest color in `palette` to `target_rgba` using weighted RGB Euclidean distance.
pub fn nearest_palette_color(target_rgba: [u8; 4], palette: &[[u8; 4]]) -> [u8; 4] {
    if palette.is_empty() {
        return [target_rgba[0], target_rgba[1], target_rgba[2], 255];
    }

    let r0 = target_rgba[0] as i32;
    let g0 = target_rgba[1] as i32;
    let b0 = target_rgba[2] as i32;

    let mut best = palette[0];
    let mut best_dist = i64::MAX;

    for &col in palette {
        let dr = (col[0] as i32 - r0) as i64;
        let dg = (col[1] as i32 - g0) as i64;
        let db = (col[2] as i32 - b0) as i64;
        // Perceptual luma weights (2, 4, 3)
        let dist = 2 * dr * dr + 4 * dg * dg + 3 * db * db;
        if dist < best_dist {
            best_dist = dist;
            best = col;
        }
    }
    best
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnionSkinSettings {
    pub enabled: bool,
    pub prev_frames: u32,
    pub next_frames: u32,
    pub opacity: f32,
    pub color_tint: bool,
    pub quick_peek_mode: QuickPeekMode,
}

impl Default for OnionSkinSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            prev_frames: 1,
            next_frames: 1,
            opacity: 0.35,
            color_tint: true,
            quick_peek_mode: QuickPeekMode::PreviousFrame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn test_sketch_inverse_sampling_and_palette_snap() {
        let mut sketch_img = RgbaImage::from_pixel(64, 64, Rgba([0, 0, 0, 0]));
        for y in 0..64 {
            for x in 0..64 {
                sketch_img.put_pixel(x, y, Rgba([200, 50, 50, 255]));
            }
        }
        let state = SketchReferenceState::default();
        let sampled = state.sample_rgba_at_frame_pos(&sketch_img, 32, 32, 16.0, 16.0);
        assert_eq!(sampled, Some([200, 50, 50, 255]));

        let palette = vec![[20, 20, 20, 255], [195, 55, 48, 255], [40, 200, 60, 255]];
        let snapped = nearest_palette_color(sampled.unwrap(), &palette);
        assert_eq!(snapped, [195, 55, 48, 255]);
    }
}

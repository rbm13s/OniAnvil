use crate::model::palette::PaletteDitherMode;
use image::RgbaImage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BakePaletteSource {
    #[default]
    ActivePalette,
    Predefined(usize),
    OriginalColors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PixelPrecisionMode {
    /// Dominant / Mode color in cell footprint. Eliminates AI gradient transitions, noise, and anti-aliasing blur.
    #[default]
    Dominant,
    /// Exact center point sample.
    Center,
    /// Box-filtered area average.
    Average,
    /// Median filter: selects median sample per cell. Eliminates salt-and-pepper noise while preserving sharp edges.
    Median,
    /// Contrast / Outline priority: prioritizes sharp stroke/contour pixels over background to keep 1px lines unbroken.
    Contrast,
    /// Max Chroma / Vivid: picks the most saturated sample in the cell to prevent micro-details from washing out.
    MaxChroma,
    /// Gaussian / Bell-weighted area filtering with center emphasis.
    Gaussian,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BgRemovalMode {
    #[default]
    Contiguous,
    Everywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackgroundRemovalTarget {
    Pixelated,
    Original,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BakeSketchSettings {
    pub palette_source: BakePaletteSource,
    pub dither_mode: PaletteDitherMode,
    pub dither_strength: f32,
    pub edge_protection: bool,
    pub crisp_pixels_and_lines: bool,
    pub pixel_precision: PixelPrecisionMode,
    pub alpha_threshold: u8,

    // Background Removal
    pub remove_background: bool,
    pub bg_color: [u8; 3],
    pub bg_tolerance: u8,
    pub bg_contiguous_only: bool,
    pub bg_target: BackgroundRemovalTarget,

    // Pre-Bake Color Adjustments
    pub brightness: i32,
    pub contrast: i32,
    pub saturation: i32,

    // Post-Processing
    pub clean_orphan_pixels: bool,
    pub generate_outline: bool,
    pub outline_color: [u8; 4],

    // Scope & Destination
    pub lock_alpha: bool,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub set_as_active_palette: bool,
}

impl Default for BakeSketchSettings {
    fn default() -> Self {
        Self {
            palette_source: BakePaletteSource::ActivePalette,
            dither_mode: PaletteDitherMode::None,
            dither_strength: 32.0,
            edge_protection: true,
            crisp_pixels_and_lines: true,
            pixel_precision: PixelPrecisionMode::Dominant,
            alpha_threshold: 128,
            remove_background: false,
            bg_color: [0, 0, 0],
            bg_tolerance: 28,
            bg_contiguous_only: true,
            bg_target: BackgroundRemovalTarget::Pixelated,
            brightness: 0,
            contrast: 0,
            saturation: 0,
            clean_orphan_pixels: false,
            generate_outline: false,
            outline_color: [0, 0, 0, 255],
            lock_alpha: false,
            all_frames: false,
            in_selection_only: false,
            set_as_active_palette: false,
        }
    }
}

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

    /// Samples a frame pixel cell at `(lx, ly)` according to `precision`, `alpha_threshold`,
    /// `crisp_lines` contrast enhancement, and optional target `palette` (used for mode/majority voting in Dominant mode).
    #[allow(clippy::too_many_arguments)]
    pub fn sample_pixel_area(
        &self,
        sketch_img: &RgbaImage,
        frame_w: u32,
        frame_h: u32,
        lx: i32,
        ly: i32,
        precision: PixelPrecisionMode,
        alpha_threshold: u8,
        palette: Option<&[[u8; 4]]>,
        crisp_lines: bool,
    ) -> Option<[u8; 4]> {
        let fx = lx as f32 + 0.5;
        let fy = ly as f32 + 0.5;

        match precision {
            PixelPrecisionMode::Center => {
                let sampled = self.sample_rgba_at_frame_pos(sketch_img, frame_w, frame_h, fx, fy)?;
                if sampled[3] >= alpha_threshold {
                    if let Some(pal) = palette {
                        Some(nearest_palette_color(sampled, pal))
                    } else {
                        Some([sampled[0], sampled[1], sampled[2], 255])
                    }
                } else {
                    None
                }
            }
            PixelPrecisionMode::Average => {
                let mut sum_r = 0u64;
                let mut sum_g = 0u64;
                let mut sum_b = 0u64;
                let mut total_a = 0u64;
                let mut total_samples = 0usize;
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);

                for &dy in &[-0.35, 0.0, 0.35] {
                    for &dx in &[-0.35, 0.0, 0.35] {
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                            }
                            let a = p[3] as u64;
                            sum_r += p[0] as u64 * a;
                            sum_g += p[1] as u64 * a;
                            sum_b += p[2] as u64 * a;
                            total_a += a;
                        }
                        total_samples += 1;
                    }
                }

                if total_samples == 0 || total_a == 0 || valid_samples.is_empty() {
                    return None;
                }
                let avg_a = (total_a / total_samples as u64) as u8;
                if avg_a < alpha_threshold {
                    return None;
                }

                // If crisp_lines is enabled and there is high contrast in the cell (e.g. 1px outline against light fill),
                // prioritize the sharp stroke color rather than blending into washed-out gray.
                if crisp_lines && valid_samples.len() >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut dark_sample = valid_samples[0];
                    for &s in &valid_samples {
                        let luma = 0.299 * s[0] as f32 + 0.587 * s[1] as f32 + 0.114 * s[2] as f32;
                        if luma < min_luma {
                            min_luma = luma;
                            dark_sample = s;
                        }
                        if luma > max_luma {
                            max_luma = luma;
                        }
                    }
                    if (max_luma - min_luma) >= 60.0 {
                        let col = [dark_sample[0], dark_sample[1], dark_sample[2], 255];
                        return if let Some(pal) = palette {
                            Some(nearest_palette_color(col, pal))
                        } else {
                            Some(col)
                        };
                    }
                }

                let avg_r = (sum_r / total_a).min(255) as u8;
                let avg_g = (sum_g / total_a).min(255) as u8;
                let avg_b = (sum_b / total_a).min(255) as u8;
                let col = [avg_r, avg_g, avg_b, 255];
                if let Some(pal) = palette {
                    Some(nearest_palette_color(col, pal))
                } else {
                    Some(col)
                }
            }
            PixelPrecisionMode::Dominant => {
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if valid_samples.is_empty() || transparent_count >= 6 {
                    return None;
                }

                // If crisp lines enabled, find stroke/contour features
                let mut dark_sample_opt = None;
                if crisp_lines && valid_samples.len() >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut darkest = valid_samples[0];
                    for &s in &valid_samples {
                        let luma = 0.299 * s[0] as f32 + 0.587 * s[1] as f32 + 0.114 * s[2] as f32;
                        if luma < min_luma {
                            min_luma = luma;
                            darkest = s;
                        }
                        if luma > max_luma {
                            max_luma = luma;
                        }
                    }
                    if (max_luma - min_luma) >= 60.0 {
                        dark_sample_opt = Some(darkest);
                    }
                }

                if let Some(pal) = palette {
                    let mut votes: Vec<([u8; 4], usize)> = Vec::new();
                    for &p in &valid_samples {
                        let snapped = nearest_palette_color(p, pal);
                        let bonus = if let Some(ds) = dark_sample_opt {
                            if (p[0] as i32 - ds[0] as i32).abs() + (p[1] as i32 - ds[1] as i32).abs() + (p[2] as i32 - ds[2] as i32).abs() < 40 {
                                2
                            } else {
                                1
                            }
                        } else {
                            1
                        };
                        if let Some(entry) = votes.iter_mut().find(|(c, _)| *c == snapped) {
                            entry.1 += bonus;
                        } else {
                            votes.push((snapped, bonus));
                        }
                    }
                    votes.sort_by(|a, b| b.1.cmp(&a.1));
                    votes.first().map(|(c, _)| *c)
                } else {
                    let mut clusters: Vec<([u8; 4], usize)> = Vec::new();
                    for &p in &valid_samples {
                        let bonus = if let Some(ds) = dark_sample_opt {
                            if (p[0] as i32 - ds[0] as i32).abs() + (p[1] as i32 - ds[1] as i32).abs() + (p[2] as i32 - ds[2] as i32).abs() < 40 {
                                2
                            } else {
                                1
                            }
                        } else {
                            1
                        };
                        if let Some(entry) = clusters.iter_mut().find(|(c, _)| {
                            let dr = (c[0] as i32 - p[0] as i32).abs();
                            let dg = (c[1] as i32 - p[1] as i32).abs();
                            let db = (c[2] as i32 - p[2] as i32).abs();
                            dr + dg + db <= 24
                        }) {
                            entry.1 += bonus;
                        } else {
                            clusters.push(([p[0], p[1], p[2], 255], bonus));
                        }
                    }
                    clusters.sort_by(|a, b| b.1.cmp(&a.1));
                    clusters.first().map(|(c, _)| *c)
                }
            }
            PixelPrecisionMode::Median => {
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if valid_samples.is_empty() || transparent_count >= 6 {
                    return None;
                }

                valid_samples.sort_by(|a, b| {
                    let la = 0.299 * a[0] as f32 + 0.587 * a[1] as f32 + 0.114 * a[2] as f32;
                    let lb = 0.299 * b[0] as f32 + 0.587 * b[1] as f32 + 0.114 * b[2] as f32;
                    la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
                });

                let med = valid_samples[valid_samples.len() / 2];
                let col = [med[0], med[1], med[2], 255];
                if let Some(pal) = palette {
                    Some(nearest_palette_color(col, pal))
                } else {
                    Some(col)
                }
            }
            PixelPrecisionMode::Contrast => {
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if valid_samples.is_empty() || transparent_count >= 6 {
                    return None;
                }

                let mut min_luma = 255.0f32;
                let mut max_luma = 0.0f32;
                let mut darkest = valid_samples[0];
                for &s in &valid_samples {
                    let luma = 0.299 * s[0] as f32 + 0.587 * s[1] as f32 + 0.114 * s[2] as f32;
                    if luma < min_luma {
                        min_luma = luma;
                        darkest = s;
                    }
                    if luma > max_luma {
                        max_luma = luma;
                    }
                }

                let chosen = if (max_luma - min_luma) >= 40.0 {
                    [darkest[0], darkest[1], darkest[2], 255]
                } else {
                    valid_samples.sort_by(|a, b| {
                        let la = 0.299 * a[0] as f32 + 0.587 * a[1] as f32 + 0.114 * a[2] as f32;
                        let lb = 0.299 * b[0] as f32 + 0.587 * b[1] as f32 + 0.114 * b[2] as f32;
                        la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    let med = valid_samples[valid_samples.len() / 2];
                    [med[0], med[1], med[2], 255]
                };

                if let Some(pal) = palette {
                    Some(nearest_palette_color(chosen, pal))
                } else {
                    Some(chosen)
                }
            }
            PixelPrecisionMode::MaxChroma => {
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if valid_samples.is_empty() || transparent_count >= 6 {
                    return None;
                }

                let mut max_chroma = -1i32;
                let mut best_sample = valid_samples[0];
                for &s in &valid_samples {
                    let r = s[0] as i32;
                    let g = s[1] as i32;
                    let b = s[2] as i32;
                    let chroma = (r.max(g).max(b)) - (r.min(g).min(b));
                    if chroma > max_chroma {
                        max_chroma = chroma;
                        best_sample = s;
                    }
                }

                let chosen = if max_chroma >= 15 {
                    [best_sample[0], best_sample[1], best_sample[2], 255]
                } else {
                    [valid_samples[0][0], valid_samples[0][1], valid_samples[0][2], 255]
                };

                if let Some(pal) = palette {
                    Some(nearest_palette_color(chosen, pal))
                } else {
                    Some(chosen)
                }
            }
            PixelPrecisionMode::Gaussian => {
                let mut sum_r = 0.0f32;
                let mut sum_g = 0.0f32;
                let mut sum_b = 0.0f32;
                let mut total_w = 0.0f32;
                let mut valid_samples: Vec<[u8; 4]> = Vec::with_capacity(9);

                for &(dy, wy) in &[(-0.35, 1.0), (0.0, 2.0), (0.35, 1.0)] {
                    for &(dx, wx) in &[(-0.35, 1.0), (0.0, 2.0), (0.35, 1.0)] {
                        let w = wx * wy;
                        if let Some(p) = self.sample_rgba_at_frame_pos(
                            sketch_img,
                            frame_w,
                            frame_h,
                            fx + dx,
                            fy + dy,
                        ) {
                            if p[3] >= alpha_threshold {
                                valid_samples.push(p);
                                let alpha_weight = (p[3] as f32 / 255.0) * w;
                                sum_r += p[0] as f32 * alpha_weight;
                                sum_g += p[1] as f32 * alpha_weight;
                                sum_b += p[2] as f32 * alpha_weight;
                                total_w += alpha_weight;
                            }
                        }
                    }
                }

                if total_w < 1.0 || valid_samples.is_empty() {
                    return None;
                }

                if crisp_lines && valid_samples.len() >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut dark_sample = valid_samples[0];
                    for &s in &valid_samples {
                        let luma = 0.299 * s[0] as f32 + 0.587 * s[1] as f32 + 0.114 * s[2] as f32;
                        if luma < min_luma {
                            min_luma = luma;
                            dark_sample = s;
                        }
                        if luma > max_luma {
                            max_luma = luma;
                        }
                    }
                    if (max_luma - min_luma) >= 60.0 {
                        let col = [dark_sample[0], dark_sample[1], dark_sample[2], 255];
                        return if let Some(pal) = palette {
                            Some(nearest_palette_color(col, pal))
                        } else {
                            Some(col)
                        };
                    }
                }

                let col = [
                    (sum_r / total_w).clamp(0.0, 255.0).round() as u8,
                    (sum_g / total_w).clamp(0.0, 255.0).round() as u8,
                    (sum_b / total_w).clamp(0.0, 255.0).round() as u8,
                    255,
                ];
                if let Some(pal) = palette {
                    Some(nearest_palette_color(col, pal))
                } else {
                    Some(col)
                }
            }
        }
    }

}

/// Adjusts brightness (-100..=100), contrast (-100..=100), and saturation (-100..=100) of an RGBA pixel.
pub fn adjust_rgba_color(
    rgba: [u8; 4],
    brightness: i32,
    contrast: i32,
    saturation: i32,
) -> [u8; 4] {
    if brightness == 0 && contrast == 0 && saturation == 0 {
        return rgba;
    }

    let mut r = rgba[0] as f32;
    let mut g = rgba[1] as f32;
    let mut b = rgba[2] as f32;

    if brightness != 0 {
        let b_delta = (brightness as f32 / 100.0) * 255.0;
        r += b_delta;
        g += b_delta;
        b += b_delta;
    }

    if contrast != 0 {
        let c = (contrast.clamp(-99, 99) as f32) * 2.55;
        let c_factor = (259.0 * (c + 255.0)) / (255.0 * (259.0 - c));
        r = c_factor * (r - 128.0) + 128.0;
        g = c_factor * (g - 128.0) + 128.0;
        b = c_factor * (b - 128.0) + 128.0;
    }

    let r_clamped = r.clamp(0.0, 255.0) as u8;
    let g_clamped = g.clamp(0.0, 255.0) as u8;
    let b_clamped = b.clamp(0.0, 255.0) as u8;

    if saturation != 0 {
        let (h, s, l) = crate::model::palette::rgb_to_hsl(r_clamped, g_clamped, b_clamped);
        let s_new = (s * (1.0 + saturation as f32 / 100.0)).clamp(0.0, 1.0);
        let (nr, ng, nb) = crate::model::palette::hsl_to_rgb(h, s_new, l);
        return [nr, ng, nb, rgba[3]];
    }

    [r_clamped, g_clamped, b_clamped, rgba[3]]
}

/// Computes Manhattan color distance between two RGB colors (0..=765).
pub fn color_distance_rgb(c1: [u8; 3], c2: [u8; 3]) -> u32 {
    let dr = (c1[0] as i32 - c2[0] as i32).unsigned_abs();
    let dg = (c1[1] as i32 - c2[1] as i32).unsigned_abs();
    let db = (c1[2] as i32 - c2[2] as i32).unsigned_abs();
    dr + dg + db
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

    #[test]
    fn test_sample_pixel_area_dominant_and_alpha_threshold() {
        let mut sketch_img = RgbaImage::from_pixel(64, 64, Rgba([0, 0, 0, 0]));
        // Put solid red in top-left quarter, but add some noisy border pixels
        for y in 0..32 {
            for x in 0..32 {
                sketch_img.put_pixel(x, y, Rgba([200, 50, 50, 255]));
            }
        }
        // One noisy pixel in the cell
        sketch_img.put_pixel(16, 16, Rgba([50, 180, 50, 255]));

        let state = SketchReferenceState::default();
        let palette = vec![[20, 20, 20, 255], [195, 55, 48, 255], [40, 200, 60, 255]];

        // Dominant mode should outvote the noise and pick the red palette color
        let sampled = state.sample_pixel_area(
            &sketch_img,
            32,
            32,
            8,
            8,
            PixelPrecisionMode::Dominant,
            128,
            Some(&palette),
            false,
        );
        assert_eq!(sampled, Some([195, 55, 48, 255]));

        // Check alpha threshold filtering
        sketch_img.put_pixel(48, 48, Rgba([200, 50, 50, 64])); // Alpha 64
        let below_thresh = state.sample_pixel_area(
            &sketch_img,
            32,
            32,
            24,
            24,
            PixelPrecisionMode::Center,
            128,
            Some(&palette),
            false,
        );
        assert_eq!(below_thresh, None);
    }

    #[test]
    fn test_adjust_rgba_color_and_distance() {
        let col = [100, 100, 100, 255];
        let brighter = adjust_rgba_color(col, 20, 0, 0);
        assert!(brighter[0] > 100);

        let c1 = [0, 0, 0];
        let c2 = [10, 10, 10];
        assert_eq!(color_distance_rgb(c1, c2), 30);
    }

    #[test]
    fn test_all_sampling_precision_modes() {
        let state = SketchReferenceState::default();
        let mut sketch = RgbaImage::from_pixel(16, 16, Rgba([200, 200, 200, 255]));
        sketch.put_pixel(8, 8, Rgba([255, 0, 0, 255])); // bright red accent / dot

        let modes = [
            PixelPrecisionMode::Dominant,
            PixelPrecisionMode::Center,
            PixelPrecisionMode::Average,
            PixelPrecisionMode::Median,
            PixelPrecisionMode::Contrast,
            PixelPrecisionMode::MaxChroma,
            PixelPrecisionMode::Gaussian,
        ];
        for mode in modes {
            let res = state.sample_pixel_area(&sketch, 16, 16, 8, 8, mode, 128, None, true);
            assert!(res.is_some(), "Mode {:?} should yield a sampled color", mode);
        }

        // MaxChroma should pick the vivid red pixel over the light gray background
        let chroma_res = state.sample_pixel_area(&sketch, 16, 16, 8, 8, PixelPrecisionMode::MaxChroma, 128, None, false);
        assert_eq!(chroma_res, Some([255, 0, 0, 255]));
    }
}

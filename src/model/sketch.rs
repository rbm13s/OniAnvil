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

#[derive(Clone, Copy, Debug)]
pub struct SketchSampleContext {
    pub cx: f32,
    pub cy: f32,
    pub inv_2hx: f32,
    pub inv_2hy: f32,
    pub sin_t: f32,
    pub cos_t: f32,
    pub flip_h: bool,
    pub flip_v: bool,
    pub sw: u32,
    pub sh: u32,
}

impl SketchSampleContext {
    #[inline(always)]
    pub fn sample_rgba(&self, sketch_img: &RgbaImage, fx: f32, fy: f32) -> Option<[u8; 4]> {
        let dx = fx - self.cx;
        let dy = fy - self.cy;
        let sx = dx * self.cos_t + dy * self.sin_t;
        let sy = -dx * self.sin_t + dy * self.cos_t;

        let mut u = sx * self.inv_2hx + 0.5;
        let mut v = sy * self.inv_2hy + 0.5;

        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        if self.flip_h {
            u = 1.0 - u;
        }
        if self.flip_v {
            v = 1.0 - v;
        }

        let px = ((u * self.sw as f32).floor() as u32).min(self.sw - 1);
        let py = ((v * self.sh as f32).floor() as u32).min(self.sh - 1);
        let p = *sketch_img.get_pixel(px, py);
        if p[3] > 0 {
            Some([p[0], p[1], p[2], p[3]])
        } else {
            None
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

    /// Returns the cache key tuple capturing all transform state that affects pixel grid sampling.
    pub fn cache_key(&self) -> (u32, u32, i32, i32, i32, bool, bool) {
        (
            self.scale.to_bits(),
            self.rotation_deg.to_bits(),
            (self.offset_x * 100.0).round() as i32,
            (self.offset_y * 100.0).round() as i32,
            self.bilinear_filter as i32,
            self.flip_h,
            self.flip_v,
        )
    }

    /// Precomputes inverse-transform constants for rapid sampling of many pixels in a frame.
    pub fn get_sample_context(
        &self,
        frame_w: u32,
        frame_h: u32,
        sketch_w: u32,
        sketch_h: u32,
    ) -> Option<SketchSampleContext> {
        if sketch_w == 0 || sketch_h == 0 {
            return None;
        }
        let (cx, cy, hx, hy) = self.half_extents(frame_w, frame_h, sketch_w, sketch_h);
        if hx <= 1e-5 || hy <= 1e-5 {
            return None;
        }
        let rad = self.rotation_deg.to_radians();
        let (sin_t, cos_t) = rad.sin_cos();
        Some(SketchSampleContext {
            cx,
            cy,
            inv_2hx: 1.0 / (2.0 * hx),
            inv_2hy: 1.0 / (2.0 * hy),
            sin_t,
            cos_t,
            flip_h: self.flip_h,
            flip_v: self.flip_v,
            sw: sketch_w,
            sh: sketch_h,
        })
    }

    pub fn fit_width(&mut self, frame_w: u32, frame_h: u32, sketch_w: u32, sketch_h: u32) {
        if sketch_w == 0 || sketch_h == 0 {
            return;
        }
        let fw = frame_w.max(1) as f32;
        let fh = frame_h.max(1) as f32;
        let sw = sketch_w as f32;
        let sh = sketch_h as f32;
        let base_fit = (fw / sw).min(fh / sh);
        if base_fit > 1e-6 {
            self.scale = (fw / sw) / base_fit;
            self.offset_x = 0.0;
            self.offset_y = 0.0;
            self.rotation_deg = 0.0;
        }
    }

    pub fn fit_height(&mut self, frame_w: u32, frame_h: u32, sketch_w: u32, sketch_h: u32) {
        if sketch_w == 0 || sketch_h == 0 {
            return;
        }
        let fw = frame_w.max(1) as f32;
        let fh = frame_h.max(1) as f32;
        let sw = sketch_w as f32;
        let sh = sketch_h as f32;
        let base_fit = (fw / sw).min(fh / sh);
        if base_fit > 1e-6 {
            self.scale = (fh / sh) / base_fit;
            self.offset_x = 0.0;
            self.offset_y = 0.0;
            self.rotation_deg = 0.0;
        }
    }

    pub fn fit_to_pixels(&mut self, frame_w: u32, frame_h: u32, sketch_w: u32, sketch_h: u32) {
        if sketch_w == 0 || sketch_h == 0 {
            return;
        }
        let fw = frame_w.max(1) as f32;
        let fh = frame_h.max(1) as f32;
        let sw = sketch_w as f32;
        let sh = sketch_h as f32;
        let base_fit = (fw / sw).min(fh / sh);
        if base_fit > 1e-6 {
            self.scale = 1.0 / base_fit;
            let target_x0 = ((frame_w as i32 - sketch_w as i32) / 2) as f32;
            let target_y0 = ((frame_h as i32 - sketch_h as i32) / 2) as f32;
            self.offset_x = target_x0 - (fw - sw) * 0.5;
            self.offset_y = target_y0 - (fh - sh) * 0.5;
            self.rotation_deg = 0.0;
            self.bilinear_filter = false;
        }
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
        let ctx = self.get_sample_context(frame_w, frame_h, sketch_img.width(), sketch_img.height())?;
        ctx.sample_rgba(sketch_img, fx, fy)
    }

    /// Samples a frame pixel cell at `(lx, ly)` according to `precision`, `alpha_threshold`,
    /// `crisp_lines` contrast enhancement, and optional target `palette`.
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
        let ctx = self.get_sample_context(frame_w, frame_h, sketch_img.width(), sketch_img.height())?;
        Self::sample_pixel_area_with_ctx(sketch_img, &ctx, lx, ly, precision, alpha_threshold, palette, crisp_lines)
    }

    /// Zero-allocation, high-throughput area sampler taking precomputed `SketchSampleContext`.
    #[allow(clippy::too_many_arguments)]
    pub fn sample_pixel_area_with_ctx(
        sketch_img: &RgbaImage,
        ctx: &SketchSampleContext,
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
                let sampled = ctx.sample_rgba(sketch_img, fx, fy)?;
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
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;

                for &dy in &[-0.35, 0.0, 0.35] {
                    for &dx in &[-0.35, 0.0, 0.35] {
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
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

                if total_samples == 0 || total_a == 0 || num_valid == 0 {
                    return None;
                }
                let avg_a = (total_a / total_samples as u64) as u8;
                if avg_a < alpha_threshold {
                    return None;
                }

                if crisp_lines && num_valid >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut dark_sample = valid_samples[0];
                    for &s in &valid_samples[..num_valid] {
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
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if num_valid == 0 || transparent_count >= 6 {
                    return None;
                }

                let mut dark_sample_opt = None;
                if crisp_lines && num_valid >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut darkest = valid_samples[0];
                    for &s in &valid_samples[..num_valid] {
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
                    let mut votes: [([u8; 4], usize); 9] = [([0, 0, 0, 0], 0); 9];
                    let mut num_votes = 0usize;

                    for &p in &valid_samples[..num_valid] {
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
                        let mut found = false;
                        for entry in &mut votes[..num_votes] {
                            if entry.0 == snapped {
                                entry.1 += bonus;
                                found = true;
                                break;
                            }
                        }
                        if !found && num_votes < 9 {
                            votes[num_votes] = (snapped, bonus);
                            num_votes += 1;
                        }
                    }
                    votes[..num_votes].iter().max_by_key(|a| a.1).map(|(c, _)| *c)
                } else {
                    let mut clusters: [([u8; 4], usize); 9] = [([0, 0, 0, 0], 0); 9];
                    let mut num_clusters = 0usize;

                    for &p in &valid_samples[..num_valid] {
                        let bonus = if let Some(ds) = dark_sample_opt {
                            if (p[0] as i32 - ds[0] as i32).abs() + (p[1] as i32 - ds[1] as i32).abs() + (p[2] as i32 - ds[2] as i32).abs() < 40 {
                                2
                            } else {
                                1
                            }
                        } else {
                            1
                        };
                        let mut found = false;
                        for entry in &mut clusters[..num_clusters] {
                            let dr = (entry.0[0] as i32 - p[0] as i32).abs();
                            let dg = (entry.0[1] as i32 - p[1] as i32).abs();
                            let db = (entry.0[2] as i32 - p[2] as i32).abs();
                            if dr + dg + db <= 24 {
                                entry.1 += bonus;
                                found = true;
                                break;
                            }
                        }
                        if !found && num_clusters < 9 {
                            clusters[num_clusters] = ([p[0], p[1], p[2], 255], bonus);
                            num_clusters += 1;
                        }
                    }
                    clusters[..num_clusters].iter().max_by_key(|a| a.1).map(|(c, _)| *c)
                }
            }
            PixelPrecisionMode::Median => {
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if num_valid == 0 || transparent_count >= 6 {
                    return None;
                }

                let slice = &mut valid_samples[..num_valid];
                slice.sort_by(|a, b| {
                    let la = 0.299 * a[0] as f32 + 0.587 * a[1] as f32 + 0.114 * a[2] as f32;
                    let lb = 0.299 * b[0] as f32 + 0.587 * b[1] as f32 + 0.114 * b[2] as f32;
                    la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
                });

                let med = slice[num_valid / 2];
                let col = [med[0], med[1], med[2], 255];
                if let Some(pal) = palette {
                    Some(nearest_palette_color(col, pal))
                } else {
                    Some(col)
                }
            }
            PixelPrecisionMode::Contrast => {
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if num_valid == 0 || transparent_count >= 6 {
                    return None;
                }

                let mut min_luma = 255.0f32;
                let mut max_luma = 0.0f32;
                let mut darkest = valid_samples[0];
                for &s in &valid_samples[..num_valid] {
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
                    let slice = &mut valid_samples[..num_valid];
                    slice.sort_by(|a, b| {
                        let la = 0.299 * a[0] as f32 + 0.587 * a[1] as f32 + 0.114 * a[2] as f32;
                        let lb = 0.299 * b[0] as f32 + 0.587 * b[1] as f32 + 0.114 * b[2] as f32;
                        la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    let med = slice[num_valid / 2];
                    [med[0], med[1], med[2], 255]
                };

                if let Some(pal) = palette {
                    Some(nearest_palette_color(chosen, pal))
                } else {
                    Some(chosen)
                }
            }
            PixelPrecisionMode::MaxChroma => {
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;
                let mut transparent_count = 0usize;

                for &dy in &[-0.33, 0.0, 0.33] {
                    for &dx in &[-0.33, 0.0, 0.33] {
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
                            } else {
                                transparent_count += 1;
                            }
                        } else {
                            transparent_count += 1;
                        }
                    }
                }

                if num_valid == 0 || transparent_count >= 6 {
                    return None;
                }

                let mut max_chroma = -1i32;
                let mut best_sample = valid_samples[0];
                for &s in &valid_samples[..num_valid] {
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
                let mut valid_samples = [[0u8; 4]; 9];
                let mut num_valid = 0usize;

                for &(dy, wy) in &[(-0.35, 1.0), (0.0, 2.0), (0.35, 1.0)] {
                    for &(dx, wx) in &[(-0.35, 1.0), (0.0, 2.0), (0.35, 1.0)] {
                        let w = wx * wy;
                        if let Some(p) = ctx.sample_rgba(sketch_img, fx + dx, fy + dy) {
                            if p[3] >= alpha_threshold {
                                valid_samples[num_valid] = p;
                                num_valid += 1;
                                let alpha_weight = (p[3] as f32 / 255.0) * w;
                                sum_r += p[0] as f32 * alpha_weight;
                                sum_g += p[1] as f32 * alpha_weight;
                                sum_b += p[2] as f32 * alpha_weight;
                                total_w += alpha_weight;
                            }
                        }
                    }
                }

                if total_w < 1.0 || num_valid == 0 {
                    return None;
                }

                if crisp_lines && num_valid >= 2 {
                    let mut min_luma = 255.0f32;
                    let mut max_luma = 0.0f32;
                    let mut dark_sample = valid_samples[0];
                    for &s in &valid_samples[..num_valid] {
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

/// Adjusts brightness, contrast, and saturation in-place across a whole frame grid using a 256-entry precomputed LUT.
pub fn adjust_color_grid(
    grid: &mut [Option<[u8; 4]>],
    brightness: i32,
    contrast: i32,
    saturation: i32,
) {
    if brightness == 0 && contrast == 0 && saturation == 0 {
        return;
    }

    let mut bc_lut = [0u8; 256];
    let b_delta = (brightness as f32 / 100.0) * 255.0;
    let c = (contrast.clamp(-99, 99) as f32) * 2.55;
    let c_factor = if contrast != 0 {
        (259.0 * (c + 255.0)) / (255.0 * (259.0 - c))
    } else {
        1.0
    };

    for (i, slot) in bc_lut.iter_mut().enumerate() {
        let mut v = i as f32;
        if brightness != 0 {
            v += b_delta;
        }
        if contrast != 0 {
            v = c_factor * (v - 128.0) + 128.0;
        }
        *slot = v.clamp(0.0, 255.0) as u8;
    }

    let sat_factor = 1.0 + saturation as f32 / 100.0;
    let do_sat = saturation != 0;

    for col in grid.iter_mut().flatten() {
        let r = bc_lut[col[0] as usize];
        let g = bc_lut[col[1] as usize];
        let b = bc_lut[col[2] as usize];
        if do_sat {
            let (h, s, l) = crate::model::palette::rgb_to_hsl(r, g, b);
            let s_new = (s * sat_factor).clamp(0.0, 1.0);
            let (nr, ng, nb) = crate::model::palette::hsl_to_rgb(h, s_new, l);
            col[0] = nr;
            col[1] = ng;
            col[2] = nb;
        } else {
            col[0] = r;
            col[1] = g;
            col[2] = b;
        }
    }
}

/// Adjusts brightness (-100..=100), contrast (-100..=100), and saturation (-100..=100) of an RGBA pixel.
#[allow(dead_code)]
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
    let mut best_dist = i32::MAX;

    for &col in palette {
        let dr = col[0] as i32 - r0;
        let dg = col[1] as i32 - g0;
        let db = col[2] as i32 - b0;
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

    #[test]
    fn test_sketch_fit_transformations() {
        let mut state = SketchReferenceState::default();
        // Frame: 32x32, Sketch: 64x128
        state.fit_width(32, 32, 64, 128);
        let (_, _, hx_w, _) = state.half_extents(32, 32, 64, 128);
        assert!((2.0 * hx_w - 32.0).abs() < 1e-4, "Width should fit 32px exactly");
        assert_eq!(state.offset_x, 0.0);
        assert_eq!(state.offset_y, 0.0);

        state.fit_height(32, 32, 64, 128);
        let (_, _, _, hy_h) = state.half_extents(32, 32, 64, 128);
        assert!((2.0 * hy_h - 32.0).abs() < 1e-4, "Height should fit 32px exactly");
        assert_eq!(state.offset_x, 0.0);
        assert_eq!(state.offset_y, 0.0);

        // 1:1 pixel fit: Sketch 16x20 on 32x32 frame
        state.fit_to_pixels(32, 32, 16, 20);
        let (cx, cy, hx_p, hy_p) = state.half_extents(32, 32, 16, 20);
        assert!((2.0 * hx_p - 16.0).abs() < 1e-4, "Width should match original 16px");
        assert!((2.0 * hy_p - 20.0).abs() < 1e-4, "Height should match original 20px");
        assert!(!state.bilinear_filter, "Bilinear filter should be disabled for 1:1 pixel fit");

        // Top-left boundary must land exactly on an integer pixel
        let left = cx - hx_p;
        let top = cy - hy_p;
        assert!((left - left.round()).abs() < 1e-4, "Left boundary must land on integer pixel coordinate");
        assert!((top - top.round()).abs() < 1e-4, "Top boundary must land on integer pixel coordinate");
    }

    #[test]
    fn test_adjust_color_grid_matches_adjust_rgba_color() {
        let test_colors = [
            [0, 0, 0, 255],
            [255, 255, 255, 255],
            [128, 64, 32, 255],
            [200, 100, 50, 255],
            [15, 80, 220, 255],
        ];
        for brightness in [-50, 0, 30] {
            for contrast in [-40, 0, 50] {
                for saturation in [-60, 0, 40] {
                    let mut grid: Vec<Option<[u8; 4]>> = test_colors.iter().map(|&c| Some(c)).collect();
                    adjust_color_grid(&mut grid, brightness, contrast, saturation);
                    for (i, &col) in test_colors.iter().enumerate() {
                        let expected = adjust_rgba_color(col, brightness, contrast, saturation);
                        assert_eq!(grid[i], Some(expected), "Mismatch at b={brightness}, c={contrast}, s={saturation}");
                    }
                }
            }
        }
    }
}

use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};

use super::palette::PaletteColorMatcher;

/// Direction for Pixel Flow particle matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FlowDirection {
    /// Two-way matching: balances existing motion and expanding details.
    #[default]
    Bidirectional,
    /// Forward: Source to Target. Ideal for moving or contracting shapes.
    Forward,
    /// Reverse: Target to Source. Ideal for explosions, blooming, and expanding shapes.
    Reverse,
}

/// Algorithm used for generating inbetween animation frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InbetweenAlgorithm {
    /// Discrete particle vector matching with context scoring and hole-free expansion.
    #[default]
    PixelFlow,
    /// Connected component cluster motion: moves solid body parts/limbs as rigid pieces (best for characters).
    CoherentMotion,
    /// Authentic 8x8 Bayer matrix dithering transition with 0 color corruption or darkening.
    DitherMorph,
    /// Smooth cross-dissolve opacity blend snapped to the project's palette.
    CrossDissolve,
}

/// Scope of layers to apply the inbetween generation to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InbetweenScope {
    /// Processes each visible layer independently, preserving layer separation.
    #[default]
    AllLayers,
    /// Processes and updates only the currently active layer.
    ActiveLayer,
}

/// Mode of inbetween generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InbetweenMode {
    /// Single frame gap: between source frame and next frame.
    #[default]
    SinglePair,
    /// Entire animation: between all consecutive frames across the timeline.
    AllFrames,
}

/// Configuration parameters for inbetween frame generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InbetweenConfig {
    /// Number of intermediate frames to generate per gap (1..=8).
    pub frame_count: usize,
    /// Selected interpolation algorithm.
    pub algorithm: InbetweenAlgorithm,
    /// Direction strategy for Pixel Flow.
    pub flow_direction: FlowDirection,
    /// Maximum pixel search radius for movement matching (PixelFlow & CoherentMotion).
    pub search_radius: u32,
    /// Layer scope.
    pub scope: InbetweenScope,
    /// Whether to loop to frame 0 when generating on the last frame of the animation.
    pub loop_to_first: bool,
    /// Whether to subdivide the original frame's duration among the new frames.
    pub subdivide_duration: bool,
    /// Generation mode: single pair vs all frames.
    pub mode: InbetweenMode,
}

impl Default for InbetweenConfig {
    fn default() -> Self {
        Self {
            frame_count: 1,
            algorithm: InbetweenAlgorithm::PixelFlow,
            flow_direction: FlowDirection::Bidirectional,
            search_radius: 16,
            scope: InbetweenScope::AllLayers,
            loop_to_first: true,
            subdivide_duration: true,
            mode: InbetweenMode::SinglePair,
        }
    }
}

/// Maximum color distance allowed for particle / cluster matching.
/// Rejects matches between conflicting elements (e.g. yellow flame vs black outline).
const MAX_COLOR_DIST: u32 = 45_000;

/// Fast color distance (redmean integer approximation).
#[inline]
pub fn color_dist_sq(c1: [u8; 4], c2: [u8; 4]) -> u32 {
    let r_mean = (c1[0] as i32 + c2[0] as i32) / 2;
    let dr = c1[0] as i32 - c2[0] as i32;
    let dg = c1[1] as i32 - c2[1] as i32;
    let db = c1[2] as i32 - c2[2] as i32;
    let da = c1[3] as i32 - c2[3] as i32;

    let r_weight = 2 + (r_mean >> 8);
    let b_weight = 2 + ((255 - r_mean) >> 8);

    (r_weight * dr * dr + 4 * dg * dg + b_weight * db * db + 4 * da * da).max(0) as u32
}

/// Generates a sequence of intermediate frames between `source` and `target`.
pub fn generate_inbetween_sequence(
    source: &RgbaImage,
    target: &RgbaImage,
    count: usize,
    algorithm: InbetweenAlgorithm,
    flow_direction: FlowDirection,
    search_radius: u32,
    palette: Option<&[[u8; 4]]>,
) -> Vec<RgbaImage> {
    if count == 0 {
        return Vec::new();
    }
    let mut frames = Vec::with_capacity(count);
    for step in 1..=count {
        let t = (step as f32) / ((count + 1) as f32);
        frames.push(interpolate_frame(
            source,
            target,
            t,
            algorithm,
            flow_direction,
            search_radius,
            palette,
        ));
    }
    frames
}

/// Interpolates between `source` and `target` for normalized time `t` in (0.0..1.0).
pub fn interpolate_frame(
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
    algorithm: InbetweenAlgorithm,
    flow_direction: FlowDirection,
    search_radius: u32,
    palette: Option<&[[u8; 4]]>,
) -> RgbaImage {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.0 {
        return source.clone();
    }
    if t >= 1.0 {
        return target.clone();
    }

    let (w, h) = source.dimensions();
    if (w, h) != target.dimensions() || w == 0 || h == 0 {
        return source.clone();
    }

    match algorithm {
        InbetweenAlgorithm::PixelFlow => {
            interpolate_pixel_flow(source, target, t, flow_direction, search_radius)
        }
        InbetweenAlgorithm::CoherentMotion => {
            interpolate_coherent_motion(source, target, t, search_radius)
        }
        InbetweenAlgorithm::DitherMorph => interpolate_dither_morph(source, target, t),
        InbetweenAlgorithm::CrossDissolve => interpolate_cross_dissolve(source, target, t, palette),
    }
}

/// Builds a spatial grid for fast radius lookup of non-transparent pixels.
fn build_spatial_grid(
    img: &RgbaImage,
    cell_size: usize,
    grid_cols: usize,
    grid_rows: usize,
) -> Vec<Vec<(u16, u16, [u8; 4])>> {
    let (w, h) = img.dimensions();
    let mut grid = vec![Vec::new(); grid_cols * grid_rows];
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y).0;
            if p[3] > 0 {
                let col = (x as usize) / cell_size;
                let row = (y as usize) / cell_size;
                if col < grid_cols && row < grid_rows {
                    grid[row * grid_cols + col].push((x as u16, y as u16, p));
                }
            }
        }
    }
    grid
}

/// Computes neighborhood context difference between source(sx, sy) and target(tx, ty).
/// Compares 4 orthogonal neighbors (up, down, left, right) to prevent features (like eyes)
/// from matching dissimilar features (like whiskers or outer borders).
#[inline]
fn neighbor_context_diff(
    source: &RgbaImage,
    target: &RgbaImage,
    sx: i32,
    sy: i32,
    tx: i32,
    ty: i32,
) -> u32 {
    let (w, h) = source.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;

    let offsets = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut total_diff = 0u32;

    for (ox, oy) in offsets {
        let nsx = sx + ox;
        let nsy = sy + oy;
        let ntx = tx + ox;
        let nty = ty + oy;

        let sc = if nsx >= 0 && nsx < w_i && nsy >= 0 && nsy < h_i {
            source.get_pixel(nsx as u32, nsy as u32).0
        } else {
            [0, 0, 0, 0]
        };

        let tc = if ntx >= 0 && ntx < w_i && nty >= 0 && nty < h_i {
            target.get_pixel(ntx as u32, nty as u32).0
        } else {
            [0, 0, 0, 0]
        };

        if (sc[3] == 0) != (tc[3] == 0) {
            total_diff += 8_000;
        } else if sc[3] > 0 {
            total_diff += (color_dist_sq(sc, tc) / 8).min(10_000);
        }
    }

    total_diff
}

/// Finds the closest matching pixel within radius in a spatial grid.
/// Enforces `MAX_COLOR_DIST` so that disparate colors (e.g. fire vs black outline) never match.
#[allow(clippy::too_many_arguments)]
fn find_grid_match(
    grid: &[Vec<(u16, u16, [u8; 4])>],
    grid_cols: usize,
    grid_rows: usize,
    cell_size: usize,
    source: &RgbaImage,
    target: &RgbaImage,
    sx: i32,
    sy: i32,
    scolor: [u8; 4],
    radius: i32,
    radius_sq: u32,
) -> Option<(i32, i32, [u8; 4])> {
    let min_col = ((sx - radius).max(0) as usize) / cell_size;
    let max_col = (((sx + radius).max(0) as usize) / cell_size).min(grid_cols.saturating_sub(1));
    let min_row = ((sy - radius).max(0) as usize) / cell_size;
    let max_row = (((sy + radius).max(0) as usize) / cell_size).min(grid_rows.saturating_sub(1));

    let mut best_exact: Option<(i32, i32, [u8; 4], u32)> = None;
    let mut best_approx: Option<(i32, i32, [u8; 4], u32)> = None;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let cell = &grid[r * grid_cols + c];
            for &(tx, ty, tcolor) in cell {
                let dx = (tx as i32) - sx;
                let dy = (ty as i32) - sy;
                let dist_sq = (dx * dx + dy * dy) as u32;
                if dist_sq <= radius_sq {
                    if tcolor == scolor {
                        let score = dist_sq * 4 + neighbor_context_diff(source, target, sx, sy, tx as i32, ty as i32);
                        if best_exact.is_none_or(|(_, _, _, s)| score < s) {
                            best_exact = Some((tx as i32, ty as i32, tcolor, score));
                        }
                    } else {
                        let c_dist = color_dist_sq(scolor, tcolor);
                        // Strict color compatibility: never match radically different colors!
                        if c_dist <= MAX_COLOR_DIST {
                            let n_diff = neighbor_context_diff(source, target, sx, sy, tx as i32, ty as i32);
                            let score = dist_sq * 4 + c_dist + n_diff;
                            if best_approx.is_none_or(|(_, _, _, s)| score < s) {
                                best_approx = Some((tx as i32, ty as i32, tcolor, score));
                            }
                        }
                    }
                }
            }
        }
    }

    best_exact
        .map(|(x, y, c, _)| (x, y, c))
        .or_else(|| best_approx.map(|(x, y, c, _)| (x, y, c)))
}

/// Calculates the center of mass (centroid) of non-transparent pixels in an image.
fn calculate_centroid(img: &RgbaImage) -> (f32, f32) {
    let (w, h) = img.dimensions();
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    let mut count = 0usize;

    for y in 0..h {
        for x in 0..w {
            if img.get_pixel(x, y)[3] > 0 {
                sum_x += x as f64;
                sum_y += y as f64;
                count += 1;
            }
        }
    }

    if count > 0 {
        ((sum_x / (count as f64)) as f32, (sum_y / (count as f64)) as f32)
    } else {
        ((w as f32) * 0.5, (h as f32) * 0.5)
    }
}

/// Closes 1-pixel and 2-pixel cracks/holes caused by discrete integer coordinate stepping.
/// If an empty pixel is surrounded by non-empty pixels, fills it with the median/majority neighbor color.
fn close_cracks(img: &mut RgbaImage, passes: usize) {
    let (w, h) = img.dimensions();
    if w < 3 || h < 3 {
        return;
    }

    for _ in 0..passes {
        let copy = img.clone();
        for y in 1..(h - 1) {
            for x in 1..(w - 1) {
                if copy.get_pixel(x, y)[3] == 0 {
                    let n_up = copy.get_pixel(x, y - 1).0;
                    let n_dn = copy.get_pixel(x, y + 1).0;
                    let n_lt = copy.get_pixel(x - 1, y).0;
                    let n_rt = copy.get_pixel(x + 1, y).0;

                    let op_up = n_up[3] > 0;
                    let op_dn = n_dn[3] > 0;
                    let op_lt = n_lt[3] > 0;
                    let op_rt = n_rt[3] > 0;

                    let count = (op_up as u8) + (op_dn as u8) + (op_lt as u8) + (op_rt as u8);

                    // Opposite bridge (horizontal or vertical) or 3+ neighbors
                    if (op_up && op_dn) || (op_lt && op_rt) || count >= 3 {
                        let fill_color = if op_up && op_dn {
                            n_up
                        } else if op_lt && op_rt {
                            n_lt
                        } else if op_up {
                            n_up
                        } else if op_lt {
                            n_lt
                        } else {
                            n_dn
                        };
                        img.put_pixel(x, y, Rgba(fill_color));
                    }
                }
            }
        }
    }
}

/// Regularizes a 2D vector field using a 3x3 local median filter.
/// Keeps connected body parts, outlines, and facial features moving cohesively.
fn regularize_flow_field(
    flow_x: &mut [f32],
    flow_y: &mut [f32],
    has_flow: &[bool],
    w: usize,
    h: usize,
) {
    let orig_x = flow_x.to_vec();
    let orig_y = flow_y.to_vec();

    for y in 1..(h.saturating_sub(1)) {
        for x in 1..(w.saturating_sub(1)) {
            let idx = y * w + x;
            if !has_flow[idx] {
                continue;
            }

            let mut xs = Vec::with_capacity(9);
            let mut ys = Vec::with_capacity(9);

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nidx = ((y as i32 + dy) as usize) * w + ((x as i32 + dx) as usize);
                    if has_flow[nidx] {
                        xs.push(orig_x[nidx]);
                        ys.push(orig_y[nidx]);
                    }
                }
            }

            if xs.len() >= 3 {
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                flow_x[idx] = xs[xs.len() / 2];
                flow_y[idx] = ys[ys.len() / 2];
            }
        }
    }
}

/// Pixel Flow algorithm: discrete particle vector matching with context scoring,
/// vector field smoothing, and hole-free radial fallback.
fn interpolate_pixel_flow(
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
    flow_direction: FlowDirection,
    search_radius: u32,
) -> RgbaImage {
    let (w, h) = source.dimensions();
    let mut result = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));

    let radius = search_radius.clamp(2, 64) as i32;
    let radius_sq = (radius * radius) as u32;
    let cell_size = (radius.max(4) as usize).clamp(4, 32);
    let grid_cols = (w as usize).div_ceil(cell_size);
    let grid_rows = (h as usize).div_ceil(cell_size);

    let target_grid = build_spatial_grid(target, cell_size, grid_cols, grid_rows);
    let source_grid = build_spatial_grid(source, cell_size, grid_cols, grid_rows);

    let source_centroid = calculate_centroid(source);
    let _target_centroid = calculate_centroid(target);

    let w_usize = w as usize;
    let h_usize = h as usize;
    let total_pixels = w_usize * h_usize;

    let mut target_matched = vec![false; total_pixels];
    let mut source_matched = vec![false; total_pixels];

    match flow_direction {
        FlowDirection::Forward => {
            let mut flow_x = vec![0.0f32; total_pixels];
            let mut flow_y = vec![0.0f32; total_pixels];
            let mut has_flow = vec![false; total_pixels];
            let mut target_colors = vec![[0u8; 4]; total_pixels];

            for sy in 0..(h as i32) {
                for sx in 0..(w as i32) {
                    let sp = source.get_pixel(sx as u32, sy as u32).0;
                    if sp[3] == 0 {
                        continue;
                    }
                    let s_idx = (sy as usize) * w_usize + (sx as usize);

                    if let Some((tx, ty, tcolor)) = find_grid_match(
                        &target_grid,
                        grid_cols,
                        grid_rows,
                        cell_size,
                        source,
                        target,
                        sx,
                        sy,
                        sp,
                        radius,
                        radius_sq,
                    ) {
                        flow_x[s_idx] = (tx - sx) as f32;
                        flow_y[s_idx] = (ty - sy) as f32;
                        has_flow[s_idx] = true;
                        target_colors[s_idx] = tcolor;
                        target_matched[(ty as usize) * w_usize + (tx as usize)] = true;
                    }
                }
            }

            // Smooth vectors to prevent parts of the sprite from breaking apart
            regularize_flow_field(&mut flow_x, &mut flow_y, &has_flow, w_usize, h_usize);

            // Step source pixels
            for sy in 0..(h as i32) {
                for sx in 0..(w as i32) {
                    let sp = source.get_pixel(sx as u32, sy as u32).0;
                    if sp[3] == 0 {
                        continue;
                    }
                    let s_idx = (sy as usize) * w_usize + (sx as usize);

                    if has_flow[s_idx] {
                        let rx = ((sx as f32) + t * flow_x[s_idx]).round() as i32;
                        let ry = ((sy as f32) + t * flow_y[s_idx]).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            let tcol = target_colors[s_idx];
                            let chosen = if t < 0.5 || sp == tcol { sp } else { tcol };
                            result.put_pixel(rx as u32, ry as u32, Rgba(chosen));
                        }
                    } else if t < 0.5 {
                        result.put_pixel(sx as u32, sy as u32, Rgba(sp));
                    }
                }
            }

            // Target pixels without match appear smoothly at t >= 0.5
            if t >= 0.5 {
                for ty in 0..(h as i32) {
                    for tx in 0..(w as i32) {
                        let idx = (ty as usize) * w_usize + (tx as usize);
                        if !target_matched[idx] {
                            let tp = target.get_pixel(tx as u32, ty as u32).0;
                            if tp[3] > 0 {
                                result.put_pixel(tx as u32, ty as u32, Rgba(tp));
                            }
                        }
                    }
                }
            }
        }

        FlowDirection::Reverse => {
            // Target pixels trace back to source origins
            for ty in 0..(h as i32) {
                for tx in 0..(w as i32) {
                    let tp = target.get_pixel(tx as u32, ty as u32).0;
                    if tp[3] == 0 {
                        continue;
                    }

                    if let Some((sx, sy, scolor)) = find_grid_match(
                        &source_grid,
                        grid_cols,
                        grid_rows,
                        cell_size,
                        target,
                        source,
                        tx,
                        ty,
                        tp,
                        radius,
                        radius_sq,
                    ) {
                        source_matched[(sy as usize) * w_usize + (sx as usize)] = true;
                        let dx = (tx - sx) as f32;
                        let dy = (ty - sy) as f32;
                        let rx = ((sx as f32) + t * dx).round() as i32;
                        let ry = ((sy as f32) + t * dy).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            let chosen = if t < 0.5 || scolor == tp { scolor } else { tp };
                            result.put_pixel(rx as u32, ry as u32, Rgba(chosen));
                        }
                    } else {
                        // Expanding particle without direct match: expand outward from source centroid!
                        // This prevents giant empty holes in explosions and blooms!
                        let c_dx = (tx as f32) - source_centroid.0;
                        let c_dy = (ty as f32) - source_centroid.1;
                        let rx = (source_centroid.0 + t * c_dx).round() as i32;
                        let ry = (source_centroid.1 + t * c_dy).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            result.put_pixel(rx as u32, ry as u32, Rgba(tp));
                        }
                    }
                }
            }

            // Unmatched source pixels fade out at t < 0.5
            if t < 0.5 {
                for sy in 0..(h as i32) {
                    for sx in 0..(w as i32) {
                        let idx = (sy as usize) * w_usize + (sx as usize);
                        if !source_matched[idx] {
                            let sp = source.get_pixel(sx as u32, sy as u32).0;
                            if sp[3] > 0 {
                                result.put_pixel(sx as u32, sy as u32, Rgba(sp));
                            }
                        }
                    }
                }
            }
        }

        FlowDirection::Bidirectional => {
            // Forward pass for source pixels
            for sy in 0..(h as i32) {
                for sx in 0..(w as i32) {
                    let sp = source.get_pixel(sx as u32, sy as u32).0;
                    if sp[3] == 0 {
                        continue;
                    }

                    if let Some((tx, ty, tcolor)) = find_grid_match(
                        &target_grid,
                        grid_cols,
                        grid_rows,
                        cell_size,
                        source,
                        target,
                        sx,
                        sy,
                        sp,
                        radius,
                        radius_sq,
                    ) {
                        target_matched[(ty as usize) * w_usize + (tx as usize)] = true;
                        let dx = (tx - sx) as f32;
                        let dy = (ty - sy) as f32;
                        let rx = ((sx as f32) + t * dx).round() as i32;
                        let ry = ((sy as f32) + t * dy).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            let chosen = if t < 0.5 || sp == tcolor { sp } else { tcolor };
                            result.put_pixel(rx as u32, ry as u32, Rgba(chosen));
                        }
                    } else if t < 0.5 {
                        result.put_pixel(sx as u32, sy as u32, Rgba(sp));
                    }
                }
            }

            // Reverse pass for unmatched target pixels: expand from closest source particle / centroid
            for ty in 0..(h as i32) {
                for tx in 0..(w as i32) {
                    let idx = (ty as usize) * w_usize + (tx as usize);
                    if target_matched[idx] {
                        continue;
                    }

                    let tp = target.get_pixel(tx as u32, ty as u32).0;
                    if tp[3] == 0 {
                        continue;
                    }

                    if let Some((sx, sy, scolor)) = find_grid_match(
                        &source_grid,
                        grid_cols,
                        grid_rows,
                        cell_size,
                        target,
                        source,
                        tx,
                        ty,
                        tp,
                        radius,
                        radius_sq,
                    ) {
                        let dx = (tx - sx) as f32;
                        let dy = (ty - sy) as f32;
                        let rx = ((sx as f32) + t * dx).round() as i32;
                        let ry = ((sy as f32) + t * dy).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            let chosen = if t < 0.5 || scolor == tp { scolor } else { tp };
                            result.put_pixel(rx as u32, ry as u32, Rgba(chosen));
                        }
                    } else {
                        // Expanding particle: expand outward from source centroid to avoid empty holes
                        let c_dx = (tx as f32) - source_centroid.0;
                        let c_dy = (ty as f32) - source_centroid.1;
                        let rx = (source_centroid.0 + t * c_dx).round() as i32;
                        let ry = (source_centroid.1 + t * c_dy).round() as i32;

                        if rx >= 0 && rx < (w as i32) && ry >= 0 && ry < (h as i32) {
                            result.put_pixel(rx as u32, ry as u32, Rgba(tp));
                        } else if t >= 0.5 {
                            result.put_pixel(tx as u32, ty as u32, Rgba(tp));
                        }
                    }
                }
            }
        }
    }

    // Close any 1-pixel rounding cracks and Swiss-cheese holes
    close_cracks(&mut result, 2);

    result
}

/// 8x8 normalized Bayer dithering matrix in range [0.0, 63.0 / 64.0].
const BAYER_8X8: [[f32; 8]; 8] = [
    [ 0.0/64.0, 32.0/64.0,  8.0/64.0, 40.0/64.0,  2.0/64.0, 34.0/64.0, 10.0/64.0, 42.0/64.0],
    [48.0/64.0, 16.0/64.0, 56.0/64.0, 24.0/64.0, 50.0/64.0, 18.0/64.0, 58.0/64.0, 26.0/64.0],
    [12.0/64.0, 44.0/64.0,  4.0/64.0, 36.0/64.0, 14.0/64.0, 46.0/64.0,  6.0/64.0, 38.0/64.0],
    [60.0/64.0, 28.0/64.0, 52.0/64.0, 20.0/64.0, 62.0/64.0, 30.0/64.0, 54.0/64.0, 22.0/64.0],
    [ 3.0/64.0, 35.0/64.0, 11.0/64.0, 43.0/64.0,  1.0/64.0, 33.0/64.0,  9.0/64.0, 41.0/64.0],
    [51.0/64.0, 19.0/64.0, 59.0/64.0, 27.0/64.0, 49.0/64.0, 17.0/64.0, 57.0/64.0, 25.0/64.0],
    [15.0/64.0, 47.0/64.0,  7.0/64.0, 39.0/64.0, 13.0/64.0, 45.0/64.0,  5.0/64.0, 37.0/64.0],
    [63.0/64.0, 31.0/64.0, 55.0/64.0, 23.0/64.0, 61.0/64.0, 29.0/64.0, 53.0/64.0, 21.0/64.0],
];

/// Authentic pixel art Bayer Dither Morph:
/// Transitions pixels through an 8x8 Bayer matrix without creating muddy intermediate colors or black dots.
fn interpolate_dither_morph(
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
) -> RgbaImage {
    let (w, h) = source.dimensions();
    let mut result = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));

    for y in 0..h {
        let bayer_row = BAYER_8X8[(y as usize) % 8];
        for x in 0..w {
            let ca = source.get_pixel(x, y).0;
            let cb = target.get_pixel(x, y).0;

            if ca == cb {
                result.put_pixel(x, y, Rgba(ca));
                continue;
            }

            let threshold = bayer_row[(x as usize) % 8];

            if ca[3] > 0 && cb[3] > 0 {
                // Both frames have solid pixels: crisp Bayer threshold cut (0 color mush, 0 black dots)
                let chosen = if t >= threshold { cb } else { ca };
                result.put_pixel(x, y, Rgba(chosen));
            } else if ca[3] > 0 && cb[3] == 0 {
                // Disappearing pixel: stays visible with its true color until threshold is reached
                if t < (1.0 - threshold) {
                    result.put_pixel(x, y, Rgba(ca));
                }
            } else if ca[3] == 0 && cb[3] > 0 {
                // Appearing pixel: becomes visible with its true color once threshold is reached
                if t >= threshold {
                    result.put_pixel(x, y, Rgba(cb));
                }
            }
        }
    }

    result
}

/// Cross Dissolve algorithm: linear opacity blending with safe palette snapping and zero black fringing.
fn interpolate_cross_dissolve(
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
    palette: Option<&[[u8; 4]]>,
) -> RgbaImage {
    let (w, h) = source.dimensions();
    let mut result = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    let mut matcher = palette.map(PaletteColorMatcher::new);

    for y in 0..h {
        for x in 0..w {
            let sp = source.get_pixel(x, y).0;
            let tp = target.get_pixel(x, y).0;

            let sa = sp[3];
            let ta = tp[3];

            if sa == 0 && ta == 0 {
                continue;
            }

            if sa == 0 {
                // Fading in from transparent: preserve target's color, only ramp alpha
                let alpha = ((ta as f32) * t).round() as u8;
                if alpha >= 96 {
                    result.put_pixel(x, y, Rgba([tp[0], tp[1], tp[2], alpha]));
                }
            } else if ta == 0 {
                // Fading out to transparent: preserve source's color, only ramp alpha
                let alpha = ((sa as f32) * (1.0 - t)).round() as u8;
                if alpha >= 96 {
                    result.put_pixel(x, y, Rgba([sp[0], sp[1], sp[2], alpha]));
                }
            } else {
                // Both opaque: blend RGB and snap to palette
                let r = ((1.0 - t) * (sp[0] as f32) + t * (tp[0] as f32)).round().clamp(0.0, 255.0) as u8;
                let g = ((1.0 - t) * (sp[1] as f32) + t * (tp[1] as f32)).round().clamp(0.0, 255.0) as u8;
                let b = ((1.0 - t) * (sp[2] as f32) + t * (tp[2] as f32)).round().clamp(0.0, 255.0) as u8;
                let a = ((1.0 - t) * (sp[3] as f32) + t * (tp[3] as f32)).round().clamp(0.0, 255.0) as u8;

                let blended = [r, g, b, a];
                let final_color = if let Some(ref mut m) = matcher {
                    m.match_color(blended)
                } else {
                    blended
                };
                result.put_pixel(x, y, Rgba(final_color));
            }
        }
    }

    result
}

/// Coherent Motion algorithm:
/// Extracts connected component clusters from the source sprite and calculates their dominant displacement
/// vector against the target frame. Moves entire body parts, limbs, and facial features as solid rigid pieces,
/// completely eliminating tearing and smearing on characters.
fn interpolate_coherent_motion(
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
    search_radius: u32,
) -> RgbaImage {
    let (w, h) = source.dimensions();
    let mut result = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));

    let w_i = w as i32;
    let h_i = h as i32;
    let radius = search_radius.clamp(2, 32) as i32;

    // 1. Identify 8-connected clusters in source
    let mut visited = vec![false; (w as usize) * (h as usize)];
    let mut clusters: Vec<Vec<(i32, i32, [u8; 4])>> = Vec::new();

    for y in 0..h_i {
        for x in 0..w_i {
            let idx = (y as usize) * (w as usize) + (x as usize);
            if visited[idx] {
                continue;
            }
            let p = source.get_pixel(x as u32, y as u32).0;
            if p[3] == 0 {
                visited[idx] = true;
                continue;
            }

            // BFS flood fill for cluster
            let mut cluster = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((x, y));
            visited[idx] = true;

            while let Some((cx, cy)) = queue.pop_front() {
                let cp = source.get_pixel(cx as u32, cy as u32).0;
                cluster.push((cx, cy, cp));

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = cx + dx;
                        let ny = cy + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * (w as usize) + (nx as usize);
                            if !visited[nidx] {
                                let np = source.get_pixel(nx as u32, ny as u32).0;
                                if np[3] > 0 {
                                    visited[nidx] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }
                }
            }

            clusters.push(cluster);
        }
    }

    // 2. For each cluster, find the consensus translation vector (vx, vy)
    let mut target_covered = vec![false; (w as usize) * (h as usize)];

    for cluster in &clusters {
        let mut best_vx = 0i32;
        let mut best_vy = 0i32;
        let mut best_score = i64::MIN;

        // Test candidate translations within search radius
        for vy in -radius..=radius {
            for vx in -radius..=radius {
                let mut score = 0i64;
                let mut valid_matches = 0usize;

                for &(px, py, pc) in cluster {
                    let tx = px + vx;
                    let ty = py + vy;
                    if tx >= 0 && tx < w_i && ty >= 0 && ty < h_i {
                        let tp = target.get_pixel(tx as u32, ty as u32).0;
                        if tp[3] > 0 {
                            let dist = color_dist_sq(pc, tp);
                            if dist <= MAX_COLOR_DIST {
                                score += 20 - ((dist / 2000) as i64);
                                valid_matches += 1;
                            } else {
                                score -= 5;
                            }
                        } else {
                            score -= 8;
                        }
                    } else {
                        score -= 15;
                    }
                }

                // Penalize large displacements slightly to prefer smaller motions
                let dist_penalty = ((vx * vx + vy * vy) as i64) / 4;
                let final_score = score - dist_penalty;

                if valid_matches > (cluster.len() / 4) && final_score > best_score {
                    best_score = final_score;
                    best_vx = vx;
                    best_vy = vy;
                }
            }
        }

        // 3. Move cluster as a single solid rigid unit
        let shift_x = ((best_vx as f32) * t).round() as i32;
        let shift_y = ((best_vy as f32) * t).round() as i32;

        for &(px, py, pc) in cluster {
            let rx = px + shift_x;
            let ry = py + shift_y;
            if rx >= 0 && rx < w_i && ry >= 0 && ry < h_i {
                result.put_pixel(rx as u32, ry as u32, Rgba(pc));
            }

            // Mark destination in target as covered
            let dest_tx = px + best_vx;
            let dest_ty = py + best_vy;
            if dest_tx >= 0 && dest_tx < w_i && dest_ty >= 0 && dest_ty < h_i {
                target_covered[(dest_ty as usize) * (w as usize) + (dest_tx as usize)] = true;
            }
        }
    }

    // 4. Any newly emerging target pixels not covered by source clusters fade in smoothly
    if t >= 0.5 {
        for ty in 0..h_i {
            for tx in 0..w_i {
                let idx = (ty as usize) * (w as usize) + (tx as usize);
                if !target_covered[idx] {
                    let tp = target.get_pixel(tx as u32, ty as u32).0;
                    if tp[3] > 0 {
                        result.put_pixel(tx as u32, ty as u32, Rgba(tp));
                    }
                }
            }
        }
    }

    close_cracks(&mut result, 2);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_identical_frames() {
        let mut img = RgbaImage::new(8, 8);
        img.put_pixel(3, 3, Rgba([255, 0, 0, 255]));
        img.put_pixel(4, 4, Rgba([0, 255, 0, 255]));

        let res_flow = interpolate_frame(&img, &img, 0.5, InbetweenAlgorithm::PixelFlow, FlowDirection::Bidirectional, 16, None);
        assert_eq!(res_flow.get_pixel(3, 3).0, [255, 0, 0, 255]);
        assert_eq!(res_flow.get_pixel(4, 4).0, [0, 255, 0, 255]);

        let res_dither = interpolate_frame(&img, &img, 0.5, InbetweenAlgorithm::DitherMorph, FlowDirection::Bidirectional, 16, None);
        assert_eq!(res_dither.get_pixel(3, 3).0, [255, 0, 0, 255]);
        assert_eq!(res_dither.get_pixel(4, 4).0, [0, 255, 0, 255]);

        let res_dissolve = interpolate_frame(&img, &img, 0.5, InbetweenAlgorithm::CrossDissolve, FlowDirection::Bidirectional, 16, None);
        assert_eq!(res_dissolve.get_pixel(3, 3).0, [255, 0, 0, 255]);
        assert_eq!(res_dissolve.get_pixel(4, 4).0, [0, 255, 0, 255]);

        let res_cluster = interpolate_frame(&img, &img, 0.5, InbetweenAlgorithm::CoherentMotion, FlowDirection::Bidirectional, 16, None);
        assert_eq!(res_cluster.get_pixel(3, 3).0, [255, 0, 0, 255]);
        assert_eq!(res_cluster.get_pixel(4, 4).0, [0, 255, 0, 255]);
    }

    #[test]
    fn test_pixel_flow_shifts_pixel_along_vector() {
        let mut source = RgbaImage::new(10, 10);
        let mut target = RgbaImage::new(10, 10);

        // Move a red 2x2 square from (2, 2) to (6, 4) -> displacement is (+4, +2)
        for y in 0..2 {
            for x in 0..2 {
                source.put_pixel(2 + x, 2 + y, Rgba([255, 0, 0, 255]));
                target.put_pixel(6 + x, 4 + y, Rgba([255, 0, 0, 255]));
            }
        }

        // At t = 0.5, position should be exactly (4, 3)
        let mid = interpolate_frame(&source, &target, 0.5, InbetweenAlgorithm::PixelFlow, FlowDirection::Bidirectional, 16, None);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(
                    mid.get_pixel(4 + x, 3 + y).0,
                    [255, 0, 0, 255],
                    "Pixel at ({}, {}) should be red",
                    4 + x,
                    3 + y
                );
            }
        }
    }

    #[test]
    fn test_pixel_flow_reverse_explosion_expands_from_spark() {
        let mut source = RgbaImage::new(11, 11);
        let mut target = RgbaImage::new(11, 11);

        // Center spark in source at (5, 5)
        source.put_pixel(5, 5, Rgba([255, 200, 0, 255]));

        // Target has 4 explosion particles: at (1, 1), (9, 1), (1, 9), (9, 9)
        target.put_pixel(1, 1, Rgba([255, 200, 0, 255]));
        target.put_pixel(9, 1, Rgba([255, 200, 0, 255]));
        target.put_pixel(1, 9, Rgba([255, 200, 0, 255]));
        target.put_pixel(9, 9, Rgba([255, 200, 0, 255]));

        let mid = interpolate_frame(&source, &target, 0.5, InbetweenAlgorithm::PixelFlow, FlowDirection::Reverse, 16, None);
        assert_eq!(mid.get_pixel(3, 3).0, [255, 200, 0, 255]);
        assert_eq!(mid.get_pixel(7, 3).0, [255, 200, 0, 255]);
        assert_eq!(mid.get_pixel(3, 7).0, [255, 200, 0, 255]);
        assert_eq!(mid.get_pixel(7, 7).0, [255, 200, 0, 255]);
    }

    #[test]
    fn test_dither_morph_no_black_dots() {
        let mut source = RgbaImage::new(4, 4);
        let mut target = RgbaImage::new(4, 4);

        // Transparent background with an orange dot at (1, 1) moving to (2, 2)
        source.put_pixel(1, 1, Rgba([255, 128, 0, 255]));
        target.put_pixel(2, 2, Rgba([255, 128, 0, 255]));

        let mid = interpolate_frame(&source, &target, 0.5, InbetweenAlgorithm::DitherMorph, FlowDirection::Bidirectional, 16, None);

        // All non-transparent pixels must be [255, 128, 0, 255]. ZERO BLACK DOTS!
        for y in 0..4 {
            for x in 0..4 {
                let p = mid.get_pixel(x, y).0;
                if p[3] > 0 {
                    assert_eq!(p, [255, 128, 0, 255], "Pixel must never darken or become black!");
                }
            }
        }
    }

    #[test]
    fn test_coherent_motion_moves_cluster_as_unit() {
        let mut source = RgbaImage::new(12, 12);
        let mut target = RgbaImage::new(12, 12);

        // Character face cluster (eye, nose, fur)
        // Fur: [230, 140, 40, 255], Eye: [0, 0, 0, 255]
        for y in 0..3 {
            for x in 0..3 {
                source.put_pixel(2 + x, 2 + y, Rgba([230, 140, 40, 255]));
                target.put_pixel(4 + x, 2 + y, Rgba([230, 140, 40, 255]));
            }
        }
        // Eye inside the face
        source.put_pixel(3, 3, Rgba([0, 0, 0, 255]));
        target.put_pixel(5, 3, Rgba([0, 0, 0, 255]));

        // At t = 0.5, cluster should shift by exactly (+1, 0)
        let mid = interpolate_frame(&source, &target, 0.5, InbetweenAlgorithm::CoherentMotion, FlowDirection::Bidirectional, 16, None);

        // Face is at (3..=5, 2..=4), eye is at (4, 3)
        assert_eq!(mid.get_pixel(4, 3).0, [0, 0, 0, 255], "Eye must stay inside face at shifted position!");
        assert_eq!(mid.get_pixel(3, 2).0, [230, 140, 40, 255], "Fur must shift with face!");
    }

    #[test]
    fn test_generate_inbetween_sequence_count() {
        let source = RgbaImage::new(8, 8);
        let target = RgbaImage::new(8, 8);

        let seq = generate_inbetween_sequence(&source, &target, 3, InbetweenAlgorithm::PixelFlow, FlowDirection::Bidirectional, 16, None);
        assert_eq!(seq.len(), 3);
    }
}

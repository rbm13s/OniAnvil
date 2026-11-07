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

/// Counts the number of non-transparent 8-connected neighbors around `(x, y)`.
#[inline]
fn count_opaque_neighbors_8(img: &RgbaImage, x: i32, y: i32) -> u8 {
    let (w, h) = img.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    let mut count = 0u8;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < w_i
                && ny >= 0
                && ny < h_i
                && img.get_pixel(nx as u32, ny as u32)[3] > 0
            {
                count += 1;
            }
        }
    }
    count
}

/// Returns `true` if `(x, y)` is strictly inside a flat-colored region (all 8 neighbors are
/// in-bounds, opaque, and nearly identical in color). Such pixels suffer from the optical flow
/// aperture problem and should inherit their displacement from surrounding boundary/feature pixels.
#[inline]
fn is_flat_interior_pixel(img: &RgbaImage, x: i32, y: i32) -> bool {
    let (w, h) = img.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    if x <= 0 || x >= w_i - 1 || y <= 0 || y >= h_i - 1 {
        return false;
    }
    let center = img.get_pixel(x as u32, y as u32).0;
    if center[3] == 0 {
        return false;
    }
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let np = img.get_pixel((x + dx) as u32, (y + dy) as u32).0;
            if np[3] == 0 || color_dist_sq(center, np) > 500 {
                return false;
            }
        }
    }
    true
}

/// Precomputes a boolean mask indicating which pixels are strictly inside a flat-colored region.
fn compute_flat_interior_mask(img: &RgbaImage) -> Vec<bool> {
    let (w, h) = img.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;
    let mut mask = vec![false; w_usize * (h as usize)];
    for y in 1..(h_i - 1) {
        for x in 1..(w_i - 1) {
            if is_flat_interior_pixel(img, x, y) {
                mask[(y as usize) * w_usize + (x as usize)] = true;
            }
        }
    }
    mask
}

/// Computes 3x3 neighborhood context difference between `source(sx, sy)` and `target(tx, ty)`
/// with early-exit pruning when `total_diff >= limit`.
#[inline]
fn neighbor_context_diff(
    source: &RgbaImage,
    target: &RgbaImage,
    sx: i32,
    sy: i32,
    tx: i32,
    ty: i32,
    limit: u32,
) -> u32 {
    let (w, h) = source.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;

    let mut total_diff = 0u32;

    for oy in -1..=1 {
        for ox in -1..=1 {
            if ox == 0 && oy == 0 {
                continue;
            }
            let is_ortho = ox == 0 || oy == 0;
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
                total_diff += if is_ortho { 10_000 } else { 5_000 };
            } else if sc[3] > 0 {
                let cdiff = color_dist_sq(sc, tc);
                total_diff += if is_ortho {
                    (cdiff / 4).min(10_000)
                } else {
                    (cdiff / 8).min(5_000)
                };
            }

            if total_diff >= limit {
                return total_diff;
            }
        }
    }

    total_diff
}

/// Finds the best matching pixel within `radius` in a spatial grid using a unified cost
/// combining motion prior deviation, spatial distance, color similarity, 3x3 structural
/// context, and target occupancy, with branch-and-bound pruning.
#[allow(clippy::too_many_arguments)]
fn find_grid_match(
    grid: &[Vec<(u16, u16, [u8; 4])>],
    grid_cols: usize,
    grid_rows: usize,
    cell_size: usize,
    from_img: &RgbaImage,
    to_img: &RgbaImage,
    occupancy: &[u16],
    sx: i32,
    sy: i32,
    scolor: [u8; 4],
    prior_vx: i32,
    prior_vy: i32,
    radius: i32,
    radius_sq: u32,
) -> Option<(i32, i32, [u8; 4])> {
    let min_col = ((sx - radius).max(0) as usize) / cell_size;
    let max_col = (((sx + radius).max(0) as usize) / cell_size).min(grid_cols.saturating_sub(1));
    let min_row = ((sy - radius).max(0) as usize) / cell_size;
    let max_row = (((sy + radius).max(0) as usize) / cell_size).min(grid_rows.saturating_sub(1));

    let w_usize = to_img.width() as usize;
    let mut best_match: Option<(i32, i32, [u8; 4], u32)> = None;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let cell = &grid[r * grid_cols + c];
            for &(tx, ty, tcolor) in cell {
                let dx = (tx as i32) - sx;
                let dy = (ty as i32) - sy;
                let dist_sq = (dx * dx + dy * dy) as u32;
                if dist_sq <= radius_sq {
                    let c_dist = if tcolor == scolor {
                        0
                    } else {
                        color_dist_sq(scolor, tcolor)
                    };
                    if c_dist <= MAX_COLOR_DIST {
                        let dev_x = dx - prior_vx;
                        let dev_y = dy - prior_vy;
                        let dev_sq = (dev_x * dev_x + dev_y * dev_y) as u32;
                        let occ = occupancy[(ty as usize) * w_usize + (tx as usize)] as u32;
                        let base_score = dev_sq * 16 + dist_sq * 10 + c_dist + occ * 120;

                        let current_best = best_match.map_or(u32::MAX, |(_, _, _, s)| s);
                        if base_score >= current_best {
                            continue;
                        }

                        let n_diff = neighbor_context_diff(
                            from_img,
                            to_img,
                            sx,
                            sy,
                            tx as i32,
                            ty as i32,
                            current_best - base_score,
                        );
                        let score = base_score + n_diff;
                        if score < current_best {
                            best_match = Some((tx as i32, ty as i32, tcolor, score));
                        }
                    }
                }
            }
        }
    }

    best_match.map(|(x, y, c, _)| (x, y, c))
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

/// Picks the most frequent (mode) color among a slice of RGBA colors, falling back to the first.
#[inline]
fn majority_color(colors: &[[u8; 4]]) -> [u8; 4] {
    if colors.is_empty() {
        return [0, 0, 0, 0];
    }
    let mut best_color = colors[0];
    let mut best_count = 0usize;
    for &c in colors {
        let count = colors.iter().filter(|&&other| other == c).count();
        if count > best_count {
            best_count = count;
            best_color = c;
        }
    }
    best_color
}

/// Closes 1-pixel and 2-pixel cracks/holes caused by discrete integer coordinate stepping.
/// If an empty pixel is surrounded by non-empty pixels, fills it with the majority neighbor color.
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

                    // Opposite bridge (horizontal or vertical) or 3+ orthogonal neighbors
                    if (op_up && op_dn) || (op_lt && op_rt) || count >= 3 {
                        let mut neighbors = Vec::with_capacity(8);
                        if op_up {
                            neighbors.push(n_up);
                        }
                        if op_dn {
                            neighbors.push(n_dn);
                        }
                        if op_lt {
                            neighbors.push(n_lt);
                        }
                        if op_rt {
                            neighbors.push(n_rt);
                        }
                        let fill_color = if op_up && op_dn && n_up == n_dn {
                            n_up
                        } else if op_lt && op_rt && n_lt == n_rt {
                            n_lt
                        } else {
                            majority_color(&neighbors)
                        };
                        img.put_pixel(x, y, Rgba(fill_color));
                    }
                }
            }
        }
    }
}

/// Regularizes a 2D vector field using multi-pass local median filtering across all pixels
/// (including canvas borders). Suppresses outlier matches so connected parts move cohesively.
fn regularize_flow_field(
    flow_x: &mut [f32],
    flow_y: &mut [f32],
    has_flow: &[bool],
    w: usize,
    h: usize,
    passes: usize,
) {
    let w_i = w as i32;
    let h_i = h as i32;

    for _ in 0..passes {
        let orig_x = flow_x.to_vec();
        let orig_y = flow_y.to_vec();

        for y in 0..h_i {
            for x in 0..w_i {
                let idx = (y as usize) * w + (x as usize);
                if !has_flow[idx] {
                    continue;
                }

                let mut xs = Vec::with_capacity(25);
                let mut ys = Vec::with_capacity(25);

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * w + (nx as usize);
                            if has_flow[nidx] {
                                xs.push(orig_x[nidx]);
                                ys.push(orig_y[nidx]);
                            }
                        }
                    }
                }

                // If part of a connected stroke (at least 1 immediate neighbor) but < 3 flow samples
                // in 3x3, widen to 5x5 to suppress small outlier pairs.
                if xs.len() == 2 {
                    xs.clear();
                    ys.clear();
                    for dy in -2..=2 {
                        for dx in -2..=2 {
                            let nx = x + dx;
                            let ny = y + dy;
                            if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                                let nidx = (ny as usize) * w + (nx as usize);
                                if has_flow[nidx] {
                                    xs.push(orig_x[nidx]);
                                    ys.push(orig_y[nidx]);
                                }
                            }
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
}

struct ComponentPriors {
    comp_id: Vec<i32>,
    components: Vec<Vec<(i32, i32)>>,
    priors: Vec<(i32, i32)>,
}

/// Extracts 8-connected opaque components from `from_img` and estimates a coarse translation
/// prior `(prior_vx, prior_vy)` for each component by correlating its boundary/feature pixels
/// against `to_img` using a 2-stage coarse-to-fine search.
fn extract_components_and_priors(
    from_img: &RgbaImage,
    to_img: &RgbaImage,
    from_flat: &[bool],
    to_flat: &[bool],
    radius: i32,
) -> ComponentPriors {
    let (w, h) = from_img.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;
    let total = w_usize * (h as usize);

    let mut comp_id = vec![-1i32; total];
    let mut components: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut priors: Vec<(i32, i32)> = Vec::new();

    let search_r = radius.clamp(2, 24);

    for y in 0..h_i {
        for x in 0..w_i {
            let idx = (y as usize) * w_usize + (x as usize);
            if comp_id[idx] != -1 || from_img.get_pixel(x as u32, y as u32)[3] == 0 {
                continue;
            }

            let cid = components.len() as i32;
            let mut pixels = Vec::new();
            let mut boundary = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((x, y));
            comp_id[idx] = cid;

            while let Some((cx, cy)) = queue.pop_front() {
                pixels.push((cx, cy));
                let cidx = (cy as usize) * w_usize + (cx as usize);
                if !from_flat[cidx] {
                    let cp = from_img.get_pixel(cx as u32, cy as u32).0;
                    boundary.push((cx, cy, cp));
                }

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = cx + dx;
                        let ny = cy + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * w_usize + (nx as usize);
                            if comp_id[nidx] == -1
                                && from_img.get_pixel(nx as u32, ny as u32)[3] > 0
                            {
                                comp_id[nidx] = cid;
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
            }

            // Estimate coarse translation prior for components with >= 3 pixels (2-stage coarse-to-fine)
            let mut best_vx = 0i32;
            let mut best_vy = 0i32;
            if pixels.len() >= 3 && !boundary.is_empty() {
                let step = (boundary.len() / 32).max(1);
                let sampled: Vec<(i32, i32, [u8; 4])> =
                    boundary.iter().copied().step_by(step).collect();
                let mut best_score = i64::MIN;

                let eval_shift = |vx: i32, vy: i32| -> (usize, i64) {
                    let mut score = 0i64;
                    let mut valid_matches = 0usize;
                    for &(px, py, pc) in &sampled {
                        let tx = px + vx;
                        let ty = py + vy;
                        if tx >= 0 && tx < w_i && ty >= 0 && ty < h_i {
                            let tp = to_img.get_pixel(tx as u32, ty as u32).0;
                            if tp[3] > 0 {
                                let dist = color_dist_sq(pc, tp);
                                if dist <= MAX_COLOR_DIST {
                                    score += 24 - ((dist / 2000) as i64);
                                    if !to_flat[(ty as usize) * w_usize + (tx as usize)] {
                                        score += 10;
                                    }
                                    valid_matches += 1;
                                } else {
                                    score -= 6;
                                }
                            } else {
                                score -= 10;
                            }
                        } else {
                            score -= 15;
                        }
                    }
                    let dist_penalty = ((vx * vx + vy * vy) as i64) / 4;
                    (valid_matches, score - dist_penalty)
                };

                // Stage 1: Coarse step-2 scan
                for vy in (-search_r..=search_r).step_by(2) {
                    for vx in (-search_r..=search_r).step_by(2) {
                        let (valid, score) = eval_shift(vx, vy);
                        if valid > (sampled.len() / 4) && score > best_score {
                            best_score = score;
                            best_vx = vx;
                            best_vy = vy;
                        }
                    }
                }

                // Stage 2: Fine +-1 refinement around coarse winner
                let c_vx = best_vx;
                let c_vy = best_vy;
                for vy in (c_vy - 1).max(-search_r)..=(c_vy + 1).min(search_r) {
                    for vx in (c_vx - 1).max(-search_r)..=(c_vx + 1).min(search_r) {
                        let (valid, score) = eval_shift(vx, vy);
                        if valid > (sampled.len() / 4) && score > best_score {
                            best_score = score;
                            best_vx = vx;
                            best_vy = vy;
                        }
                    }
                }
            }

            components.push(pixels);
            priors.push((best_vx, best_vy));
        }
    }

    ComponentPriors {
        comp_id,
        components,
        priors,
    }
}

/// Dense regularized flow field from `from_img` to `to_img`.
struct FlowField {
    vx: Vec<f32>,
    vy: Vec<f32>,
    has_flow: Vec<bool>,
    matched_color: Vec<[u8; 4]>,
}

/// Computes a regularized, harmonically inpainted flow field from `from_img` to `to_img`.
/// 1. Matches boundary/feature pixels using 3x3 context, occupancy, and component motion priors.
/// 2. Regularizes boundary vectors with median filtering.
/// 3. Propagates flow vectors across flat interior and unmatched connected pixels via BFS wavefront
///    and harmonic (Laplacian) relaxation so entire shapes move cohesively without interior holes.
#[allow(clippy::too_many_arguments)]
fn compute_flow_field(
    from_img: &RgbaImage,
    to_img: &RgbaImage,
    to_grid: &[Vec<(u16, u16, [u8; 4])>],
    grid_cols: usize,
    grid_rows: usize,
    cell_size: usize,
    radius: i32,
    radius_sq: u32,
    allow_radial_fallback: bool,
    to_centroid: (f32, f32),
) -> FlowField {
    let (w, h) = from_img.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;
    let h_usize = h as usize;
    let total = w_usize * h_usize;

    let from_flat = compute_flat_interior_mask(from_img);
    let to_flat = compute_flat_interior_mask(to_img);

    let ComponentPriors {
        comp_id,
        components,
        priors,
    } = extract_components_and_priors(from_img, to_img, &from_flat, &to_flat, radius);

    let mut vx = vec![0.0f32; total];
    let mut vy = vec![0.0f32; total];
    let mut has_flow = vec![false; total];
    let mut is_inpainted = vec![false; total];
    let mut matched_color = vec![[0u8; 4]; total];
    let mut occupancy = vec![0u16; total];

    // Step 1: Match only Feature/Boundary pixels (Flat Interior pixels are inpainted in Step 3)
    for y in 0..h_i {
        for x in 0..w_i {
            let p = from_img.get_pixel(x as u32, y as u32).0;
            if p[3] == 0 {
                continue;
            }
            let idx = (y as usize) * w_usize + (x as usize);
            matched_color[idx] = p;

            if from_flat[idx] {
                continue;
            }

            let cid = comp_id[idx];
            let (prior_vx, prior_vy) = if cid >= 0 {
                priors[cid as usize]
            } else {
                (0, 0)
            };

            if let Some((tx, ty, tcolor)) = find_grid_match(
                to_grid,
                grid_cols,
                grid_rows,
                cell_size,
                from_img,
                to_img,
                &occupancy,
                x,
                y,
                p,
                prior_vx,
                prior_vy,
                radius,
                radius_sq,
            ) {
                vx[idx] = (tx - x) as f32;
                vy[idx] = (ty - y) as f32;
                has_flow[idx] = true;
                matched_color[idx] = tcolor;
                let tidx = (ty as usize) * w_usize + (tx as usize);
                occupancy[tidx] = occupancy[tidx].saturating_add(1);
            }
        }
    }

    // Step 2: Regularize boundary/feature vectors before propagating into interiors
    regularize_flow_field(&mut vx, &mut vy, &has_flow, w_usize, h_usize, 2);

    // Step 3: Wavefront BFS propagation into Flat Interior and unmatched connected pixels
    loop {
        let mut updates: Vec<(usize, f32, f32)> = Vec::new();
        for y in 0..h_i {
            for x in 0..w_i {
                let idx = (y as usize) * w_usize + (x as usize);
                if has_flow[idx] || from_img.get_pixel(x as u32, y as u32)[3] == 0 {
                    continue;
                }

                let mut sum_x = 0.0f32;
                let mut sum_y = 0.0f32;
                let mut weight_sum = 0.0f32;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * w_usize + (nx as usize);
                            if has_flow[nidx] {
                                let w_neighbor = if dx == 0 || dy == 0 { 1.5 } else { 1.0 };
                                sum_x += vx[nidx] * w_neighbor;
                                sum_y += vy[nidx] * w_neighbor;
                                weight_sum += w_neighbor;
                            }
                        }
                    }
                }

                if weight_sum > 0.0 {
                    updates.push((idx, sum_x / weight_sum, sum_y / weight_sum));
                }
            }
        }

        if updates.is_empty() {
            break;
        }

        for (idx, nx, ny) in updates {
            vx[idx] = nx;
            vy[idx] = ny;
            has_flow[idx] = true;
            is_inpainted[idx] = true;
        }
    }

    // Harmonic (Laplacian) relaxation on inpainted interior pixels so velocity gradients are smooth
    for _ in 0..6 {
        let prev_x = vx.clone();
        let prev_y = vy.clone();
        for y in 0..h_i {
            for x in 0..w_i {
                let idx = (y as usize) * w_usize + (x as usize);
                if !is_inpainted[idx] {
                    continue;
                }
                let mut sum_x = prev_x[idx];
                let mut sum_y = prev_y[idx];
                let mut count = 1.0f32;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * w_usize + (nx as usize);
                            if has_flow[nidx] {
                                sum_x += prev_x[nidx];
                                sum_y += prev_y[nidx];
                                count += 1.0;
                            }
                        }
                    }
                }
                vx[idx] = sum_x / count;
                vy[idx] = sum_y / count;
            }
        }
    }

    // Step 4: Handle completely unmatched components (where 0 pixels in the component matched)
    for (cid, comp_pixels) in components.iter().enumerate() {
        let any_matched = comp_pixels
            .iter()
            .any(|&(x, y)| has_flow[(y as usize) * w_usize + (x as usize)]);
        if any_matched {
            continue;
        }

        let (prior_vx, prior_vy) = priors[cid];
        if prior_vx != 0 || prior_vy != 0 {
            for &(x, y) in comp_pixels {
                let idx = (y as usize) * w_usize + (x as usize);
                vx[idx] = prior_vx as f32;
                vy[idx] = prior_vy as f32;
                has_flow[idx] = true;
            }
        } else if allow_radial_fallback && comp_pixels.len() <= 16 {
            // Coherent cluster fallback toward to_centroid for small expanding explosion particles
            let mut cx = 0.0f32;
            let mut cy = 0.0f32;
            for &(x, y) in comp_pixels {
                cx += x as f32;
                cy += y as f32;
            }
            cx /= comp_pixels.len() as f32;
            cy /= comp_pixels.len() as f32;
            let shift_x = to_centroid.0 - cx;
            let shift_y = to_centroid.1 - cy;
            for &(x, y) in comp_pixels {
                let idx = (y as usize) * w_usize + (x as usize);
                vx[idx] = shift_x;
                vy[idx] = shift_y;
                has_flow[idx] = true;
            }
        }
    }

    // Final median regularization pass
    regularize_flow_field(&mut vx, &mut vy, &has_flow, w_usize, h_usize, 1);

    // Smooth sharp internal velocity cliffs across color boundaries within the same connected
    // component (e.g. yellow core vs orange lobes vs smoke ring in explosions) so adjacent zones
    // expand cohesively instead of shearing apart.
    for _ in 0..4 {
        let prev_x = vx.clone();
        let prev_y = vy.clone();
        for y in 0..h_i {
            for x in 0..w_i {
                let idx = (y as usize) * w_usize + (x as usize);
                if !has_flow[idx] {
                    continue;
                }
                let cid = comp_id[idx];
                let mut sum_x = prev_x[idx] * 2.0;
                let mut sum_y = prev_y[idx] * 2.0;
                let mut weight = 2.0f32;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let nidx = (ny as usize) * w_usize + (nx as usize);
                            if has_flow[nidx] && comp_id[nidx] == cid {
                                sum_x += prev_x[nidx];
                                sum_y += prev_y[nidx];
                                weight += 1.0;
                            }
                        }
                    }
                }
                vx[idx] = sum_x / weight;
                vy[idx] = sum_y / weight;
            }
        }
    }

    // Refresh matched_color at the regularized destination so stale pre-regularization colors never leak
    for y in 0..h_i {
        for x in 0..w_i {
            let idx = (y as usize) * w_usize + (x as usize);
            if !has_flow[idx] {
                continue;
            }
            let sp = from_img.get_pixel(x as u32, y as u32).0;
            let tx = ((x as f32) + vx[idx]).round() as i32;
            let ty = ((y as f32) + vy[idx]).round() as i32;
            if tx >= 0 && tx < w_i && ty >= 0 && ty < h_i {
                let tp = to_img.get_pixel(tx as u32, ty as u32).0;
                if tp[3] > 0 && color_dist_sq(sp, tp) <= MAX_COLOR_DIST {
                    matched_color[idx] = tp;
                } else {
                    matched_color[idx] = sp;
                }
            } else {
                matched_color[idx] = sp;
            }
        }
    }

    FlowField {
        vx,
        vy,
        has_flow,
        matched_color,
    }
}

/// Fills interior gaps/holes in `result` using inverse (backward) flow lookup into `source` and
/// `target`. Detects both immediate 1px cracks and multi-pixel internal tears (up to 4px wide)
/// bridged by warped pixels on opposite sides.
#[allow(clippy::too_many_arguments)]
fn fill_flow_gaps(
    result: &mut RgbaImage,
    source: &RgbaImage,
    target: &RgbaImage,
    t: f32,
    mid_vx: &mut [f32],
    mid_vy: &mut [f32],
    has_mid: &mut [bool],
    origin_solid: &mut [bool],
    passes: usize,
) {
    let (w, h) = result.dimensions();
    if w < 3 || h < 3 {
        return;
    }
    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;

    for _ in 0..passes {
        let copy = result.clone();
        let copy_vx = mid_vx.to_vec();
        let copy_vy = mid_vy.to_vec();
        let copy_has = has_mid.to_vec();

        for y in 0..h_i {
            for x in 0..w_i {
                if copy.get_pixel(x as u32, y as u32)[3] > 0 {
                    continue;
                }

                let op = |dx: i32, dy: i32| -> bool {
                    let nx = x + dx;
                    let ny = y + dy;
                    nx >= 0
                        && nx < w_i
                        && ny >= 0
                        && ny < h_i
                        && copy.get_pixel(nx as u32, ny as u32)[3] > 0
                };

                let op_up = op(0, -1);
                let op_dn = op(0, 1);
                let op_lt = op(-1, 0);
                let op_rt = op(1, 0);
                let op_ul = op(-1, -1);
                let op_ur = op(1, -1);
                let op_dl = op(-1, 1);
                let op_dr = op(1, 1);

                let count_4 = (op_up as u8) + (op_dn as u8) + (op_lt as u8) + (op_rt as u8);
                let count_8 =
                    count_4 + (op_ul as u8) + (op_ur as u8) + (op_dl as u8) + (op_dr as u8);

                let ortho_bridge = (op_up && op_dn) || (op_lt && op_rt);
                let diag_bridge =
                    ((op_ul && op_dr) || (op_ur && op_dl)) && count_4 >= 1 && count_8 >= 4;

                // Also check wider bridges (up to 3px in opposite directions) when touching at least 1 pixel
                let wide_bridge = count_8 >= 2 && {
                    let ray = |dx: i32, dy: i32| -> bool {
                        (1..=3).any(|s| op(dx * s, dy * s))
                    };
                    (ray(-1, 0) && ray(1, 0) && (ray(0, -1) || ray(0, 1)))
                        || (ray(0, -1) && ray(0, 1) && (ray(-1, 0) || ray(1, 0)))
                };

                if !ortho_bridge && !diag_bridge && !wide_bridge && count_4 < 3 && count_8 < 5 {
                    continue;
                }

                let mut sum_vx = 0.0f32;
                let mut sum_vy = 0.0f32;
                let mut flow_count = 0.0f32;
                let mut neighbor_colors = Vec::with_capacity(8);

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w_i && ny >= 0 && ny < h_i {
                            let np = copy.get_pixel(nx as u32, ny as u32).0;
                            if np[3] > 0 {
                                neighbor_colors.push(np);
                                let nidx = (ny as usize) * w_usize + (nx as usize);
                                if copy_has[nidx] {
                                    sum_vx += copy_vx[nidx];
                                    sum_vy += copy_vy[nidx];
                                    flow_count += 1.0;
                                }
                            }
                        }
                    }
                }

                let idx = (y as usize) * w_usize + (x as usize);
                let chosen = if flow_count > 0.0 {
                    let avg_vx = sum_vx / flow_count;
                    let avg_vy = sum_vy / flow_count;

                    let sx = ((x as f32) - t * avg_vx).round() as i32;
                    let sy = ((y as f32) - t * avg_vy).round() as i32;
                    let tx = ((x as f32) + (1.0 - t) * avg_vx).round() as i32;
                    let ty = ((y as f32) + (1.0 - t) * avg_vy).round() as i32;

                    let sp = if sx >= 0 && sx < w_i && sy >= 0 && sy < h_i {
                        source.get_pixel(sx as u32, sy as u32).0
                    } else {
                        [0, 0, 0, 0]
                    };
                    let tp = if tx >= 0 && tx < w_i && ty >= 0 && ty < h_i {
                        target.get_pixel(tx as u32, ty as u32).0
                    } else {
                        [0, 0, 0, 0]
                    };

                    // For wide bridges (not immediate 1px cracks), require both source and target
                    // inverse lookups to be opaque so genuine concavities are never filled.
                    if wide_bridge && !ortho_bridge && !diag_bridge && count_4 < 3 && count_8 < 5 {
                        if sp[3] > 0 && tp[3] > 0 {
                            mid_vx[idx] = avg_vx;
                            mid_vy[idx] = avg_vy;
                            has_mid[idx] = true;
                            if t < 0.5 || sp == tp { sp } else { tp }
                        } else {
                            [0, 0, 0, 0]
                        }
                    } else {
                        mid_vx[idx] = avg_vx;
                        mid_vy[idx] = avg_vy;
                        has_mid[idx] = true;
                        if sp[3] > 0 && tp[3] > 0 {
                            if t < 0.5 || sp == tp {
                                sp
                            } else {
                                tp
                            }
                        } else if sp[3] > 0 {
                            sp
                        } else if tp[3] > 0 {
                            tp
                        } else {
                            majority_color(&neighbor_colors)
                        }
                    }
                } else if ortho_bridge || diag_bridge || count_4 >= 3 || count_8 >= 5 {
                    majority_color(&neighbor_colors)
                } else {
                    [0, 0, 0, 0]
                };

                if chosen[3] > 0 {
                    result.put_pixel(x as u32, y as u32, Rgba(chosen));
                    origin_solid[idx] = true;
                }
            }
        }
    }
}

/// Removes isolated 1-pixel stray artifacts that detached from a solid region (`origin_solid == true`)
/// and ended up with zero opaque 8-neighbors in the generated frame. Genuine 1-px particles
/// (`origin_solid == false`) are preserved untouched.
fn remove_stray_pixels(result: &mut RgbaImage, origin_solid: &[bool]) {
    let (w, h) = result.dimensions();
    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;
    let copy = result.clone();

    for y in 0..h_i {
        for x in 0..w_i {
            let idx = (y as usize) * w_usize + (x as usize);
            if origin_solid[idx]
                && copy.get_pixel(x as u32, y as u32)[3] > 0
                && count_opaque_neighbors_8(&copy, x, y) == 0
            {
                result.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
            }
        }
    }
}

/// Pixel Flow algorithm: boundary-guided vector matching with harmonic interior flow inpainting,
/// forward-backward consistency, watertight bilinear quad rasterization, and inverse-flow gap fill.
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
    let target_centroid = calculate_centroid(target);

    let w_i = w as i32;
    let h_i = h as i32;
    let w_usize = w as usize;
    let h_usize = h as usize;
    let total_pixels = w_usize * h_usize;

    let mut best_err = vec![f32::INFINITY; total_pixels];
    let mut mid_vx = vec![0.0f32; total_pixels];
    let mut mid_vy = vec![0.0f32; total_pixels];
    let mut has_mid = vec![false; total_pixels];
    let mut origin_solid = vec![false; total_pixels];

    #[allow(clippy::too_many_arguments)]
    let mut splat_pixel = |fx: f32,
                           fy: f32,
                           vx: f32,
                           vy: f32,
                           color: [u8; 4],
                           is_solid: bool,
                           priority_bias: f32,
                           result: &mut RgbaImage| {
        let rx = fx.round() as i32;
        let ry = fy.round() as i32;
        if rx >= 0 && rx < w_i && ry >= 0 && ry < h_i {
            let r_idx = (ry as usize) * w_usize + (rx as usize);
            let err =
                (fx - (rx as f32)).powi(2) + (fy - (ry as f32)).powi(2) + priority_bias;
            if err < best_err[r_idx] {
                best_err[r_idx] = err;
                result.put_pixel(rx as u32, ry as u32, Rgba(color));
                mid_vx[r_idx] = vx;
                mid_vy[r_idx] = vy;
                has_mid[r_idx] = true;
                origin_solid[r_idx] = is_solid;
            }
        }
    };

    #[allow(clippy::too_many_arguments)]
    let mut splat_flow_mesh = |from_img: &RgbaImage,
                               flow: &FlowField,
                               is_source: bool,
                               base_bias: f32,
                               result: &mut RgbaImage| {
        let step_t = if is_source { t } else { 1.0 - t };
        let fwd_sign = if is_source { 1.0f32 } else { -1.0f32 };

        let pixel_color = |idx: usize, p: [u8; 4]| -> [u8; 4] {
            let mcol = flow.matched_color[idx];
            if is_source {
                if t < 0.5 || p == mcol { p } else { mcol }
            } else if t < 0.5 || mcol == p {
                mcol
            } else {
                p
            }
        };

        // 1. Splat primary vertices
        for y in 0..h_i {
            for x in 0..w_i {
                let p = from_img.get_pixel(x as u32, y as u32).0;
                if p[3] == 0 {
                    continue;
                }
                let idx = (y as usize) * w_usize + (x as usize);
                let is_solid = count_opaque_neighbors_8(from_img, x, y) >= 2;

                if flow.has_flow[idx] {
                    let fx = (x as f32) + step_t * flow.vx[idx];
                    let fy = (y as f32) + step_t * flow.vy[idx];
                    let fwd_vx = fwd_sign * flow.vx[idx];
                    let fwd_vy = fwd_sign * flow.vy[idx];
                    let chosen = pixel_color(idx, p);
                    splat_pixel(fx, fy, fwd_vx, fwd_vy, chosen, is_solid, base_bias, result);
                } else if (is_source && t < 0.5) || (!is_source && t >= 0.5) {
                    splat_pixel(
                        x as f32,
                        y as f32,
                        0.0,
                        0.0,
                        p,
                        is_solid,
                        base_bias + 0.5,
                        result,
                    );
                }
            }
        }

        // 2. Watertight bilinear rasterization of stretched 2x2 quads and edges so expanding
        //    fireballs, rotating limbs, and shearing zones never tear open into internal holes.
        for y in 0..h_i {
            for x in 0..w_i {
                let i00 = (y as usize) * w_usize + (x as usize);
                if !flow.has_flow[i00] || from_img.get_pixel(x as u32, y as u32)[3] == 0 {
                    continue;
                }
                let p00 = from_img.get_pixel(x as u32, y as u32).0;
                let c00 = pixel_color(i00, p00);
                let fx00 = (x as f32) + step_t * flow.vx[i00];
                let fy00 = (y as f32) + step_t * flow.vy[i00];

                let has_10 = x + 1 < w_i
                    && flow.has_flow[(y as usize) * w_usize + ((x + 1) as usize)]
                    && from_img.get_pixel((x + 1) as u32, y as u32)[3] > 0;
                let has_01 = y + 1 < h_i
                    && flow.has_flow[((y + 1) as usize) * w_usize + (x as usize)]
                    && from_img.get_pixel(x as u32, (y + 1) as u32)[3] > 0;
                let has_11 = x + 1 < w_i
                    && y + 1 < h_i
                    && flow.has_flow[((y + 1) as usize) * w_usize + ((x + 1) as usize)]
                    && from_img.get_pixel((x + 1) as u32, (y + 1) as u32)[3] > 0;

                if has_10 && has_01 && has_11 {
                    let i10 = (y as usize) * w_usize + ((x + 1) as usize);
                    let i01 = ((y + 1) as usize) * w_usize + (x as usize);
                    let i11 = ((y + 1) as usize) * w_usize + ((x + 1) as usize);

                    let fx10 = ((x + 1) as f32) + step_t * flow.vx[i10];
                    let fy10 = (y as f32) + step_t * flow.vy[i10];
                    let fx01 = (x as f32) + step_t * flow.vx[i01];
                    let fy01 = ((y + 1) as f32) + step_t * flow.vy[i01];
                    let fx11 = ((x + 1) as f32) + step_t * flow.vx[i11];
                    let fy11 = ((y + 1) as f32) + step_t * flow.vy[i11];

                    let max_span = (fx10 - fx00)
                        .abs()
                        .max((fy10 - fy00).abs())
                        .max((fx01 - fx00).abs())
                        .max((fy01 - fy00).abs())
                        .max((fx11 - fx10).abs())
                        .max((fy11 - fy10).abs())
                        .max((fx11 - fx01).abs())
                        .max((fy11 - fy01).abs());

                    if max_span > 1.05 && max_span <= 8.0 {
                        let steps = (max_span.ceil() as i32).clamp(2, 8);
                        let inv_steps = 1.0 / (steps as f32);
                        let c10 = pixel_color(i10, from_img.get_pixel((x + 1) as u32, y as u32).0);
                        let c01 = pixel_color(i01, from_img.get_pixel(x as u32, (y + 1) as u32).0);
                        let c11 = pixel_color(
                            i11,
                            from_img.get_pixel((x + 1) as u32, (y + 1) as u32).0,
                        );

                        for iv in 0..=steps {
                            let v = (iv as f32) * inv_steps;
                            for iu in 0..=steps {
                                if (iu == 0 || iu == steps) && (iv == 0 || iv == steps) {
                                    continue;
                                }
                                let u = (iu as f32) * inv_steps;
                                let w00 = (1.0 - u) * (1.0 - v);
                                let w10 = u * (1.0 - v);
                                let w01 = (1.0 - u) * v;
                                let w11 = u * v;

                                let q_fx = w00 * fx00 + w10 * fx10 + w01 * fx01 + w11 * fx11;
                                let q_fy = w00 * fy00 + w10 * fy10 + w01 * fy01 + w11 * fy11;
                                let q_vx = fwd_sign
                                    * (w00 * flow.vx[i00]
                                        + w10 * flow.vx[i10]
                                        + w01 * flow.vx[i01]
                                        + w11 * flow.vx[i11]);
                                let q_vy = fwd_sign
                                    * (w00 * flow.vy[i00]
                                        + w10 * flow.vy[i10]
                                        + w01 * flow.vy[i01]
                                        + w11 * flow.vy[i11]);
                                let q_col = if v < 0.5 {
                                    if u < 0.5 { c00 } else { c10 }
                                } else if u < 0.5 {
                                    c01
                                } else {
                                    c11
                                };
                                splat_pixel(
                                    q_fx,
                                    q_fy,
                                    q_vx,
                                    q_vy,
                                    q_col,
                                    true,
                                    base_bias + 0.35,
                                    result,
                                );
                            }
                        }
                        continue;
                    }
                }

                // Fallback 1D edge subdivision for boundary pixels not part of a full 2x2 quad
                if has_10 {
                    let i10 = (y as usize) * w_usize + ((x + 1) as usize);
                    let fx10 = ((x + 1) as f32) + step_t * flow.vx[i10];
                    let fy10 = (y as f32) + step_t * flow.vy[i10];
                    let span = (fx10 - fx00).abs().max((fy10 - fy00).abs());
                    if span > 1.05 && span <= 6.0 {
                        let steps = (span.ceil() as i32).clamp(2, 6);
                        let c10 = pixel_color(i10, from_img.get_pixel((x + 1) as u32, y as u32).0);
                        for s in 1..steps {
                            let u = (s as f32) / (steps as f32);
                            splat_pixel(
                                fx00 + u * (fx10 - fx00),
                                fy00 + u * (fy10 - fy00),
                                fwd_sign * (flow.vx[i00] + u * (flow.vx[i10] - flow.vx[i00])),
                                fwd_sign * (flow.vy[i00] + u * (flow.vy[i10] - flow.vy[i00])),
                                if u < 0.5 { c00 } else { c10 },
                                true,
                                base_bias + 0.35,
                                result,
                            );
                        }
                    }
                }

                if has_01 {
                    let i01 = ((y + 1) as usize) * w_usize + (x as usize);
                    let fx01 = (x as f32) + step_t * flow.vx[i01];
                    let fy01 = ((y + 1) as f32) + step_t * flow.vy[i01];
                    let span = (fx01 - fx00).abs().max((fy01 - fy00).abs());
                    if span > 1.05 && span <= 6.0 {
                        let steps = (span.ceil() as i32).clamp(2, 6);
                        let c01 = pixel_color(i01, from_img.get_pixel(x as u32, (y + 1) as u32).0);
                        for s in 1..steps {
                            let v = (s as f32) / (steps as f32);
                            splat_pixel(
                                fx00 + v * (fx01 - fx00),
                                fy00 + v * (fy01 - fy00),
                                fwd_sign * (flow.vx[i00] + v * (flow.vx[i01] - flow.vx[i00])),
                                fwd_sign * (flow.vy[i00] + v * (flow.vy[i01] - flow.vy[i00])),
                                if v < 0.5 { c00 } else { c01 },
                                true,
                                base_bias + 0.35,
                                result,
                            );
                        }
                    }
                }
            }
        }
    };

    match flow_direction {
        FlowDirection::Forward => {
            let fwd = compute_flow_field(
                source,
                target,
                &target_grid,
                grid_cols,
                grid_rows,
                cell_size,
                radius,
                radius_sq,
                false,
                target_centroid,
            );
            splat_flow_mesh(source, &fwd, true, 0.0, &mut result);
        }

        FlowDirection::Reverse => {
            let rev = compute_flow_field(
                target,
                source,
                &source_grid,
                grid_cols,
                grid_rows,
                cell_size,
                radius,
                radius_sq,
                true,
                source_centroid,
            );
            splat_flow_mesh(target, &rev, false, 0.0, &mut result);
        }

        FlowDirection::Bidirectional => {
            let mut fwd = compute_flow_field(
                source,
                target,
                &target_grid,
                grid_cols,
                grid_rows,
                cell_size,
                radius,
                radius_sq,
                false,
                target_centroid,
            );
            let mut rev = compute_flow_field(
                target,
                source,
                &source_grid,
                grid_cols,
                grid_rows,
                cell_size,
                radius,
                radius_sq,
                true,
                source_centroid,
            );

            // Mutually align forward and reverse flow fields so trajectories never split into ghost copies
            for sy in 0..h_i {
                for sx in 0..w_i {
                    let s_idx = (sy as usize) * w_usize + (sx as usize);
                    if !fwd.has_flow[s_idx] {
                        continue;
                    }
                    let tx = ((sx as f32) + fwd.vx[s_idx]).round() as i32;
                    let ty = ((sy as f32) + fwd.vy[s_idx]).round() as i32;
                    if tx >= 0 && tx < w_i && ty >= 0 && ty < h_i {
                        let t_idx = (ty as usize) * w_usize + (tx as usize);
                        if rev.has_flow[t_idx] {
                            let r_vx = -rev.vx[t_idx];
                            let r_vy = -rev.vy[t_idx];
                            fwd.vx[s_idx] = 0.5 * (fwd.vx[s_idx] + r_vx);
                            fwd.vy[s_idx] = 0.5 * (fwd.vy[s_idx] + r_vy);
                        }
                    }
                }
            }
            regularize_flow_field(&mut fwd.vx, &mut fwd.vy, &fwd.has_flow, w_usize, h_usize, 1);

            for ty in 0..h_i {
                for tx in 0..w_i {
                    let t_idx = (ty as usize) * w_usize + (tx as usize);
                    if !rev.has_flow[t_idx] {
                        continue;
                    }
                    let sx = ((tx as f32) + rev.vx[t_idx]).round() as i32;
                    let sy = ((ty as f32) + rev.vy[t_idx]).round() as i32;
                    if sx >= 0 && sx < w_i && sy >= 0 && sy < h_i {
                        let s_idx = (sy as usize) * w_usize + (sx as usize);
                        if fwd.has_flow[s_idx] {
                            rev.vx[t_idx] = -fwd.vx[s_idx];
                            rev.vy[t_idx] = -fwd.vy[s_idx];
                        }
                    }
                }
            }
            regularize_flow_field(&mut rev.vx, &mut rev.vy, &rev.has_flow, w_usize, h_usize, 1);

            let fwd_bias = if t < 0.5 { 0.0 } else { 0.25 };
            let rev_bias = if t >= 0.5 { 0.0 } else { 0.25 };

            splat_flow_mesh(source, &fwd, true, fwd_bias, &mut result);
            splat_flow_mesh(target, &rev, false, rev_bias, &mut result);
        }
    }

    // Reconstruct interior gaps using inverse (backward) flow sampling into source and target
    fill_flow_gaps(
        &mut result,
        source,
        target,
        t,
        &mut mid_vx,
        &mut mid_vy,
        &mut has_mid,
        &mut origin_solid,
        3,
    );

    // Close any remaining 1-pixel rounding cracks and remove isolated detached stray pixels
    close_cracks(&mut result, 2);
    remove_stray_pixels(&mut result, &origin_solid);

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

    #[test]
    fn test_pixel_flow_overlapping_outlined_sprite_no_holes_or_stray_pixels() {
        let mut source = RgbaImage::new(20, 20);
        let mut target = RgbaImage::new(20, 20);

        let outline = Rgba([20, 20, 25, 255]);
        let fill = Rgba([240, 170, 120, 255]);

        // Draw a 7x7 outlined square at (2, 3) in source and shifted by (+4, +2) to (6, 5) in target.
        // Note that source (2..9, 3..10) and target (6..13, 5..12) overlap in (6..9, 5..10)!
        for dy in 0..7 {
            for dx in 0..7 {
                let is_border = dx == 0 || dx == 6 || dy == 0 || dy == 6;
                let color = if is_border { outline } else { fill };
                source.put_pixel(2 + dx, 3 + dy, color);
                target.put_pixel(6 + dx, 5 + dy, color);
            }
        }
        // Add an unmatched specular highlight in source and a new detail in target
        source.put_pixel(4, 5, Rgba([255, 255, 255, 255]));
        target.put_pixel(10, 9, Rgba([50, 120, 255, 255]));

        for dir in [
            FlowDirection::Forward,
            FlowDirection::Reverse,
            FlowDirection::Bidirectional,
        ] {
            let mid = interpolate_frame(
                &source,
                &target,
                0.5,
                InbetweenAlgorithm::PixelFlow,
                dir,
                16,
                None,
            );

            // At t = 0.5, the 7x7 square must be at (4..11, 4..11) with ZERO interior holes
            // and ZERO stray pixels anywhere outside (4..11, 4..11).
            for y in 0..20 {
                for x in 0..20 {
                    let inside = (4..11).contains(&x) && (4..11).contains(&y);
                    let alpha = mid.get_pixel(x, y)[3];
                    if inside {
                        assert!(
                            alpha > 0,
                            "Missing pixel (hole) at ({}, {}) for direction {:?}",
                            x,
                            y,
                            dir
                        );
                    } else {
                        assert_eq!(
                            alpha, 0,
                            "Stray pixel at ({}, {}) for direction {:?}",
                            x,
                            y,
                            dir
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_pixel_flow_scaling_and_deforming_no_interior_holes() {
        let mut small = RgbaImage::new(16, 16);
        let mut large = RgbaImage::new(16, 16);
        let color = Rgba([220, 80, 40, 255]);

        // 4x4 square at (6..10, 6..10) expanding to 8x8 square at (4..12, 4..12)
        for y in 6..10 {
            for x in 6..10 {
                small.put_pixel(x, y, color);
            }
        }
        for y in 4..12 {
            for x in 4..12 {
                large.put_pixel(x, y, color);
            }
        }

        for dir in [
            FlowDirection::Forward,
            FlowDirection::Reverse,
            FlowDirection::Bidirectional,
        ] {
            let mid_expand = interpolate_frame(
                &small,
                &large,
                0.5,
                InbetweenAlgorithm::PixelFlow,
                dir,
                16,
                None,
            );
            let mid_contract = interpolate_frame(
                &large,
                &small,
                0.5,
                InbetweenAlgorithm::PixelFlow,
                dir,
                16,
                None,
            );

            // Core 5x5 region (6..10, 6..10) must be 100% solid without holes, and outer margin < 3 or >= 13 must be empty
            for y in 6..10 {
                for x in 6..10 {
                    assert!(
                        mid_expand.get_pixel(x, y)[3] > 0,
                        "Hole during expansion at ({}, {}) in {:?}",
                        x,
                        y,
                        dir
                    );
                    assert!(
                        mid_contract.get_pixel(x, y)[3] > 0,
                        "Hole during contraction at ({}, {}) in {:?}",
                        x,
                        y,
                        dir
                    );
                }
            }
            for y in 0..16 {
                for x in 0..16 {
                    if !(3..13).contains(&x) || !(3..13).contains(&y) {
                        assert_eq!(
                            mid_expand.get_pixel(x, y)[3],
                            0,
                            "Stray pixel during expansion at ({}, {}) in {:?}",
                            x,
                            y,
                            dir
                        );
                        assert_eq!(
                            mid_contract.get_pixel(x, y)[3],
                            0,
                            "Stray pixel during contraction at ({}, {}) in {:?}",
                            x,
                            y,
                            dir
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_pixel_flow_explosion_fireball_multi_zone_no_holes() {
        let mut source = RgbaImage::new(32, 32);
        let mut target = RgbaImage::new(32, 32);

        let yellow = Rgba([255, 250, 140, 255]);
        let orange = Rgba([255, 130, 20, 255]);
        let smoke = Rgba([65, 35, 45, 255]);

        // Source fireball: radius 8 orange with a tiny radius 2 yellow hot-spot in center (16, 16)
        for y in 0..32 {
            for x in 0..32 {
                let dx = (x as i32) - 16;
                let dy = (y as i32) - 16;
                let d2 = dx * dx + dy * dy;
                if d2 <= 4 {
                    source.put_pixel(x, y, yellow);
                } else if d2 <= 64 {
                    source.put_pixel(x, y, orange);
                }

                // Target fireball: expanded yellow core (r <= 8), orange ring (r <= 11), dark smoke (r <= 13)
                if d2 <= 64 {
                    target.put_pixel(x, y, yellow);
                } else if d2 <= 121 {
                    target.put_pixel(x, y, orange);
                } else if d2 <= 169 {
                    target.put_pixel(x, y, smoke);
                }
            }
        }

        for dir in [
            FlowDirection::Reverse,
            FlowDirection::Bidirectional,
            FlowDirection::Forward,
        ] {
            let mid = interpolate_frame(
                &source,
                &target,
                0.5,
                InbetweenAlgorithm::PixelFlow,
                dir,
                22,
                None,
            );

            // Every pixel inside the intermediate core (r <= 8, i.e. d2 <= 64) must be 100% opaque with ZERO holes!
            for y in 0..32 {
                for x in 0..32 {
                    let dx = (x as i32) - 16;
                    let dy = (y as i32) - 16;
                    if dx * dx + dy * dy <= 64 {
                        assert!(
                            mid.get_pixel(x, y)[3] > 0,
                            "Explosion fireball has a hole at ({}, {}) in {:?}",
                            x,
                            y,
                            dir
                        );
                    }
                }
            }
        }
    }
}

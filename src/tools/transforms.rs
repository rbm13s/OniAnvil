use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub fn flip_region_horizontal(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
) {
    for y in 0..h {
        for x in 0..(w / 2) {
            let ax1 = origin_x + x;
            let ax2 = origin_x + (w - 1 - x);
            let ay = origin_y + y;
            if ax2 < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p1 = *atlas_rgba.get_pixel(ax1, ay);
                let p2 = *atlas_rgba.get_pixel(ax2, ay);
                atlas_rgba.put_pixel(ax1, ay, p2);
                atlas_rgba.put_pixel(ax2, ay, p1);
            }
        }
    }
}

pub fn flip_region_vertical(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
) {
    for y in 0..(h / 2) {
        let ay1 = origin_y + y;
        let ay2 = origin_y + (h - 1 - y);
        for x in 0..w {
            let ax = origin_x + x;
            if ax < atlas_rgba.width() && ay2 < atlas_rgba.height() {
                let p1 = *atlas_rgba.get_pixel(ax, ay1);
                let p2 = *atlas_rgba.get_pixel(ax, ay2);
                atlas_rgba.put_pixel(ax, ay1, p2);
                atlas_rgba.put_pixel(ax, ay2, p1);
            }
        }
    }
}

/// Rotates a frame region 90° clockwise around its center.
pub fn rotate_region_90_cw(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
) {
    let mut temp = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    for y in 0..h {
        for x in 0..w {
            let ax = origin_x + x;
            let ay = origin_y + y;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                temp.put_pixel(x, y, *atlas_rgba.get_pixel(ax, ay));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let ax = origin_x + x;
            let ay = origin_y + y;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
            }
        }
    }

    let cx = (w as f32 - 1.0) * 0.5;
    let cy = (h as f32 - 1.0) * 0.5;
    for y in 0..h {
        for x in 0..w {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let nx = (cx - dy).round() as i32;
            let ny = (cy + dx).round() as i32;
            if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                let ax = origin_x + nx as u32;
                let ay = origin_y + ny as u32;
                if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                    atlas_rgba.put_pixel(ax, ay, *temp.get_pixel(x, y));
                }
            }
        }
    }
}

/// Shifts pixels in a frame region by `(dx, dy)` with optional `wrap_around`.
#[allow(clippy::too_many_arguments)]
pub fn nudge_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    dx: i32,
    dy: i32,
    wrap_around: bool,
) {
    let mut temp = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    for y in 0..h {
        for x in 0..w {
            let ax = origin_x + x;
            let ay = origin_y + y;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                temp.put_pixel(x, y, *atlas_rgba.get_pixel(ax, ay));
            }
        }
    }

    let iw = w as i32;
    let ih = h as i32;
    for y in 0..ih {
        for x in 0..iw {
            let src_x = x - dx;
            let src_y = y - dy;
            let pixel = if wrap_around {
                let wx = src_x.rem_euclid(iw) as u32;
                let wy = src_y.rem_euclid(ih) as u32;
                *temp.get_pixel(wx, wy)
            } else if src_x >= 0 && src_x < iw && src_y >= 0 && src_y < ih {
                *temp.get_pixel(src_x as u32, src_y as u32)
            } else {
                Rgba([0, 0, 0, 0])
            };

            let ax = origin_x + x as u32;
            let ay = origin_y + y as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, pixel);
            }
        }
    }
}

/// Shifts only the pixels specified by `selected_pixels` by `(dx, dy)`.
#[allow(clippy::too_many_arguments)]
pub fn nudge_selection_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    dx: i32,
    dy: i32,
    wrap_around: bool,
    selected_pixels: &std::collections::HashSet<(i32, i32)>,
) {
    if selected_pixels.is_empty() {
        return;
    }
    let iw = w.max(1) as i32;
    let ih = h.max(1) as i32;

    // 1. Extract values of selected pixels
    let mut extracted = Vec::with_capacity(selected_pixels.len());
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p = *atlas_rgba.get_pixel(ax, ay);
                extracted.push(((lx, ly), p));
            }
        }
    }

    // 2. Clear old positions of selected pixels
    for &((lx, ly), _) in &extracted {
        let ax = origin_x + lx as u32;
        let ay = origin_y + ly as u32;
        if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
            atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
        }
    }

    // 3. Write them into new translated positions
    for ((lx, ly), p) in extracted {
        let nx = if wrap_around {
            (lx + dx).rem_euclid(iw)
        } else {
            lx + dx
        };
        let ny = if wrap_around {
            (ly + dy).rem_euclid(ih)
        } else {
            ly + dy
        };
        if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
            let ax = origin_x + nx as u32;
            let ay = origin_y + ny as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, p);
            }
        }
    }
}

/// Flips selected pixels horizontally within their bounding box.
/// Returns the updated set of selected pixel coordinates.
#[allow(clippy::too_many_arguments)]
pub fn flip_selection_horizontal(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    selected_pixels: &std::collections::HashSet<(i32, i32)>,
) -> std::collections::HashSet<(i32, i32)> {
    if selected_pixels.is_empty() {
        return std::collections::HashSet::new();
    }
    let iw = w.max(1) as i32;
    let ih = h.max(1) as i32;

    let min_x = selected_pixels.iter().map(|p| p.0).min().unwrap_or(0);
    let max_x = selected_pixels.iter().map(|p| p.0).max().unwrap_or(0);

    let mut extracted = Vec::with_capacity(selected_pixels.len());
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p = *atlas_rgba.get_pixel(ax, ay);
                extracted.push(((lx, ly), p));
            }
        }
    }

    for &((lx, ly), _) in &extracted {
        let ax = origin_x + lx as u32;
        let ay = origin_y + ly as u32;
        if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
            atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
        }
    }

    let mut new_pixels = std::collections::HashSet::with_capacity(extracted.len());
    for ((lx, ly), p) in extracted {
        let nx = min_x + (max_x - lx);
        let ny = ly;
        if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
            let ax = origin_x + nx as u32;
            let ay = origin_y + ny as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, p);
                new_pixels.insert((nx, ny));
            }
        }
    }

    new_pixels
}

/// Flips selected pixels vertically within their bounding box.
/// Returns the updated set of selected pixel coordinates.
#[allow(clippy::too_many_arguments)]
pub fn flip_selection_vertical(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    selected_pixels: &std::collections::HashSet<(i32, i32)>,
) -> std::collections::HashSet<(i32, i32)> {
    if selected_pixels.is_empty() {
        return std::collections::HashSet::new();
    }
    let iw = w.max(1) as i32;
    let ih = h.max(1) as i32;

    let min_y = selected_pixels.iter().map(|p| p.1).min().unwrap_or(0);
    let max_y = selected_pixels.iter().map(|p| p.1).max().unwrap_or(0);

    let mut extracted = Vec::with_capacity(selected_pixels.len());
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p = *atlas_rgba.get_pixel(ax, ay);
                extracted.push(((lx, ly), p));
            }
        }
    }

    for &((lx, ly), _) in &extracted {
        let ax = origin_x + lx as u32;
        let ay = origin_y + ly as u32;
        if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
            atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
        }
    }

    let mut new_pixels = std::collections::HashSet::with_capacity(extracted.len());
    for ((lx, ly), p) in extracted {
        let nx = lx;
        let ny = min_y + (max_y - ly);
        if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
            let ax = origin_x + nx as u32;
            let ay = origin_y + ny as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, p);
                new_pixels.insert((nx, ny));
            }
        }
    }

    new_pixels
}

/// Rotates selected pixels 90° clockwise within their bounding box.
/// Returns the updated set of selected pixel coordinates.
#[allow(clippy::too_many_arguments)]
pub fn rotate_selection_90_cw(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    selected_pixels: &std::collections::HashSet<(i32, i32)>,
) -> std::collections::HashSet<(i32, i32)> {
    if selected_pixels.is_empty() {
        return std::collections::HashSet::new();
    }
    let iw = w.max(1) as i32;
    let ih = h.max(1) as i32;

    let min_x = selected_pixels.iter().map(|p| p.0).min().unwrap_or(0);
    let max_x = selected_pixels.iter().map(|p| p.0).max().unwrap_or(0);
    let min_y = selected_pixels.iter().map(|p| p.1).min().unwrap_or(0);
    let max_y = selected_pixels.iter().map(|p| p.1).max().unwrap_or(0);

    let cx = (min_x + max_x) as f32 * 0.5;
    let cy = (min_y + max_y) as f32 * 0.5;

    let mut extracted = Vec::with_capacity(selected_pixels.len());
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p = *atlas_rgba.get_pixel(ax, ay);
                extracted.push(((lx, ly), p));
            }
        }
    }

    for &((lx, ly), _) in &extracted {
        let ax = origin_x + lx as u32;
        let ay = origin_y + ly as u32;
        if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
            atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
        }
    }

    let mut new_pixels = std::collections::HashSet::with_capacity(extracted.len());
    for ((lx, ly), p) in extracted {
        let dx = lx as f32 - cx;
        let dy = ly as f32 - cy;
        let nx = (cx - dy).round() as i32;
        let ny = (cy + dx).round() as i32;
        if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
            let ax = origin_x + nx as u32;
            let ay = origin_y + ny as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, p);
                new_pixels.insert((nx, ny));
            }
        }
    }

    new_pixels
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RotationAlgorithm {
    #[default]
    RotSprite,
    NearestNeighbor,
}

impl RotationAlgorithm {
    pub const ALL: [Self; 2] = [Self::RotSprite, Self::NearestNeighbor];
}

/// Scale2x pixel art upscaling algorithm (AdvMame2x).
/// Smooths diagonal edges without introducing new colors or blur.
pub fn scale2x(src: &RgbaImage) -> RgbaImage {
    let w = src.width();
    let h = src.height();
    let mut dst = RgbaImage::new(w * 2, h * 2);

    for y in 0..h {
        for x in 0..w {
            let p = *src.get_pixel(x, y);
            let a = if y > 0 { *src.get_pixel(x, y - 1) } else { p };
            let c = if y + 1 < h { *src.get_pixel(x, y + 1) } else { p };
            let d = if x > 0 { *src.get_pixel(x - 1, y) } else { p };
            let b = if x + 1 < w { *src.get_pixel(x + 1, y) } else { p };

            let mut e1 = p;
            let mut e2 = p;
            let mut e3 = p;
            let mut e4 = p;

            if c != a && d != b {
                if d == a { e1 = d; }
                if a == b { e2 = b; }
                if d == c { e3 = d; }
                if c == b { e4 = b; }
            }

            dst.put_pixel(2 * x,     2 * y,     e1);
            dst.put_pixel(2 * x + 1, 2 * y,     e2);
            dst.put_pixel(2 * x,     2 * y + 1, e3);
            dst.put_pixel(2 * x + 1, 2 * y + 1, e4);
        }
    }
    dst
}

/// Applies 3 passes of Scale2x to scale an image 8x.
pub fn scale8x(src: &RgbaImage) -> RgbaImage {
    let s2 = scale2x(src);
    let s4 = scale2x(&s2);
    scale2x(&s4)
}

/// Rotates an image using standard nearest-neighbor backward mapping.
pub fn rotate_image_nearest(src: &RgbaImage, angle_deg: f32) -> RgbaImage {
    let w = src.width();
    let h = src.height();
    if w == 0 || h == 0 {
        return src.clone();
    }
    let mut dst = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    let rad = -angle_deg.to_radians();
    let cos_a = rad.cos();
    let sin_a = rad.sin();
    let cx = (w as f32 - 1.0) * 0.5;
    let cy = (h as f32 - 1.0) * 0.5;

    for y in 0..h {
        for x in 0..w {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let sx = (cx + dx * cos_a - dy * sin_a).round() as i32;
            let sy = (cy + dx * sin_a + dy * cos_a).round() as i32;
            if sx >= 0 && sx < w as i32 && sy >= 0 && sy < h as i32 {
                dst.put_pixel(x, y, *src.get_pixel(sx as u32, sy as u32));
            }
        }
    }
    dst
}

/// Rotates an image using the authentic RotSprite algorithm:
/// 8x Scale2x upscale -> Nearest-neighbor rotation -> 8x Downscale with dominant color voting.
pub fn rotate_image_rotsprite(src: &RgbaImage, angle_deg: f32) -> RgbaImage {
    let w = src.width();
    let h = src.height();
    if w == 0 || h == 0 {
        return src.clone();
    }
    let upscaled = scale8x(src);
    let uw = upscaled.width();
    let uh = upscaled.height();

    let mut rotated_up = RgbaImage::from_pixel(uw, uh, Rgba([0, 0, 0, 0]));
    let rad = -angle_deg.to_radians();
    let cos_a = rad.cos();
    let sin_a = rad.sin();
    let ucx = (uw as f32 - 1.0) * 0.5;
    let ucy = (uh as f32 - 1.0) * 0.5;

    for y in 0..uh {
        for x in 0..uw {
            let dx = x as f32 - ucx;
            let dy = y as f32 - ucy;
            let sx = (ucx + dx * cos_a - dy * sin_a).round() as i32;
            let sy = (ucy + dx * sin_a + dy * cos_a).round() as i32;
            if sx >= 0 && sx < uw as i32 && sy >= 0 && sy < uh as i32 {
                rotated_up.put_pixel(x, y, *upscaled.get_pixel(sx as u32, sy as u32));
            }
        }
    }

    let mut dst = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    let mut color_counts: HashMap<[u8; 4], u32> = HashMap::with_capacity(16);

    for y in 0..h {
        for x in 0..w {
            color_counts.clear();
            let mut transparent_count = 0u32;
            let start_x = x * 8;
            let start_y = y * 8;

            for by in 0..8 {
                for bx in 0..8 {
                    let p = *rotated_up.get_pixel(start_x + bx, start_y + by);
                    if p[3] == 0 {
                        transparent_count += 1;
                    } else {
                        *color_counts.entry([p[0], p[1], p[2], p[3]]).or_insert(0) += 1;
                    }
                }
            }

            if transparent_count >= 32 || color_counts.is_empty() {
                dst.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            } else {
                let dominant = color_counts
                    .iter()
                    .max_by_key(|&(_, count)| *count)
                    .map(|(&c, _)| c)
                    .unwrap_or([0, 0, 0, 0]);
                dst.put_pixel(x, y, Rgba(dominant));
            }
        }
    }

    dst
}

/// Rotates an image using either RotSprite or Nearest-Neighbor algorithm.
pub fn rotate_image(src: &RgbaImage, angle_deg: f32, algorithm: RotationAlgorithm) -> RgbaImage {
    match algorithm {
        RotationAlgorithm::RotSprite => rotate_image_rotsprite(src, angle_deg),
        RotationAlgorithm::NearestNeighbor => rotate_image_nearest(src, angle_deg),
    }
}

/// Rotates an entire frame region by an arbitrary angle.
pub fn rotate_region_arbitrary(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    angle_deg: f32,
    algorithm: RotationAlgorithm,
) {
    if w == 0 || h == 0 {
        return;
    }
    let mut temp = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    for y in 0..h {
        for x in 0..w {
            let ax = origin_x + x;
            let ay = origin_y + y;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                temp.put_pixel(x, y, *atlas_rgba.get_pixel(ax, ay));
            }
        }
    }

    let rotated = rotate_image(&temp, angle_deg, algorithm);

    for y in 0..h {
        for x in 0..w {
            let ax = origin_x + x;
            let ay = origin_y + y;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, *rotated.get_pixel(x, y));
            }
        }
    }
}

/// Rotates selected pixels by an arbitrary angle within their bounding box.
/// Returns the updated set of selected pixel coordinates.
#[allow(clippy::too_many_arguments)]
pub fn rotate_selection_arbitrary(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    w: u32,
    h: u32,
    angle_deg: f32,
    algorithm: RotationAlgorithm,
    selected_pixels: &HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    if selected_pixels.is_empty() || w == 0 || h == 0 {
        return HashSet::new();
    }
    let iw = w as i32;
    let ih = h as i32;

    let min_x = selected_pixels.iter().map(|p| p.0).min().unwrap_or(0);
    let max_x = selected_pixels.iter().map(|p| p.0).max().unwrap_or(0);
    let min_y = selected_pixels.iter().map(|p| p.1).min().unwrap_or(0);
    let max_y = selected_pixels.iter().map(|p| p.1).max().unwrap_or(0);

    let cx = (min_x + max_x) as f32 * 0.5;
    let cy = (min_y + max_y) as f32 * 0.5;
    let half_w = (max_x - min_x) as f32 * 0.5 + 0.5;
    let half_h = (max_y - min_y) as f32 * 0.5 + 0.5;
    let r = (half_w * half_w + half_h * half_h).sqrt().ceil() as i32 + 1;
    let patch_dim = (2 * r + 1) as u32;

    let mut patch = RgbaImage::from_pixel(patch_dim, patch_dim, Rgba([0, 0, 0, 0]));

    // 1. Copy selected pixels to patch
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let p = *atlas_rgba.get_pixel(ax, ay);
                let px = lx - cx.round() as i32 + r;
                let py = ly - cy.round() as i32 + r;
                if px >= 0 && px < patch_dim as i32 && py >= 0 && py < patch_dim as i32 {
                    patch.put_pixel(px as u32, py as u32, p);
                }
            }
        }
    }

    // 2. Clear old positions in atlas
    for &(lx, ly) in selected_pixels {
        if lx >= 0 && ly >= 0 && lx < iw && ly < ih {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
            }
        }
    }

    // 3. Rotate the patch
    let rotated = rotate_image(&patch, angle_deg, algorithm);

    // 4. Write back rotated pixels and collect new selection coordinates
    let mut new_pixels = HashSet::with_capacity(selected_pixels.len());
    let base_x = cx.round() as i32 - r;
    let base_y = cy.round() as i32 - r;

    for py in 0..patch_dim {
        for px in 0..patch_dim {
            let p = *rotated.get_pixel(px, py);
            if p[3] > 0 {
                let nx = base_x + px as i32;
                let ny = base_y + py as i32;
                if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
                    let ax = origin_x + nx as u32;
                    let ay = origin_y + ny as u32;
                    if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                        atlas_rgba.put_pixel(ax, ay, p);
                        new_pixels.insert((nx, ny));
                    }
                }
            }
        }
    }

    new_pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_flip_and_rotate_selection() {
        let mut img = RgbaImage::from_pixel(8, 8, Rgba([0, 0, 0, 0]));
        img.put_pixel(1, 1, Rgba([255, 0, 0, 255]));
        img.put_pixel(2, 1, Rgba([0, 255, 0, 255]));

        let mut sel = HashSet::new();
        sel.insert((1, 1));
        sel.insert((2, 1));

        // Flip horizontal inside [1..2, 1..1]
        let flipped = flip_selection_horizontal(&mut img, 0, 0, 8, 8, &sel);
        assert!(flipped.contains(&(1, 1)));
        assert!(flipped.contains(&(2, 1)));
        assert_eq!(*img.get_pixel(1, 1), Rgba([0, 255, 0, 255]));
        assert_eq!(*img.get_pixel(2, 1), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_scale2x_and_rotsprite() {
        let mut img = RgbaImage::from_pixel(2, 2, Rgba([0, 0, 0, 0]));
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        img.put_pixel(1, 1, Rgba([255, 0, 0, 255]));

        let scaled = scale2x(&img);
        assert_eq!(scaled.width(), 4);
        assert_eq!(scaled.height(), 4);

        let rotated = rotate_image(&img, 90.0, RotationAlgorithm::RotSprite);
        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 2);
    }

    #[test]
    fn test_rotate_selection_arbitrary() {
        let mut img = RgbaImage::from_pixel(8, 8, Rgba([0, 0, 0, 0]));
        img.put_pixel(3, 3, Rgba([255, 0, 0, 255]));
        img.put_pixel(4, 3, Rgba([255, 0, 0, 255]));

        let mut sel = HashSet::new();
        sel.insert((3, 3));
        sel.insert((4, 3));

        let res = rotate_selection_arbitrary(&mut img, 0, 0, 8, 8, 90.0, RotationAlgorithm::NearestNeighbor, &sel);
        assert!(!res.is_empty());
    }
}

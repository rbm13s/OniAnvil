use crate::tools::{GradientDither, GradientKind};
use image::{Rgba, RgbaImage};
use std::collections::{HashSet, VecDeque};

/// Returns whether two RGBA colors are within `tolerance` (0..=255) Euclidean distance.
#[inline]
pub fn color_matches(c1: [u8; 4], c2: [u8; 4], tolerance: u8) -> bool {
    if tolerance == 0 {
        c1 == c2
    } else {
        let dr = c1[0] as i32 - c2[0] as i32;
        let dg = c1[1] as i32 - c2[1] as i32;
        let db = c1[2] as i32 - c2[2] as i32;
        let da = c1[3] as i32 - c2[3] as i32;
        let dist_sq = (dr * dr + dg * dg + db * db + da * da) as u32;
        let max_dist = tolerance as u32;
        dist_sq <= max_dist * max_dist * 4
    }
}

/// Performs a 4-connected Flood Fill inside the specified frame sub-rectangle of `atlas_rgba`.
/// Returns `true` if at least one pixel was modified.
#[allow(clippy::too_many_arguments)]
pub fn flood_fill_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    start_x: i32,
    start_y: i32,
    fill_color: [u8; 4],
    lock_alpha: bool,
    tolerance: u8,
) -> bool {
    if start_x < 0
        || start_y < 0
        || (start_x as u32) >= region_w
        || (start_y as u32) >= region_h
    {
        return false;
    }

    let ax0 = origin_x + start_x as u32;
    let ay0 = origin_y + start_y as u32;
    if ax0 >= atlas_rgba.width() || ay0 >= atlas_rgba.height() {
        return false;
    }

    let target_rgba = *atlas_rgba.get_pixel(ax0, ay0);
    if lock_alpha && target_rgba[3] == 0 {
        return false;
    }

    let replacement = Rgba(fill_color);
    if color_matches(target_rgba.0, fill_color, tolerance) && fill_color == target_rgba.0 {
        return false;
    }

    let w = region_w as usize;
    let h = region_h as usize;
    let mut visited = vec![false; w * h];
    let mut queue = VecDeque::new();

    let start_ux = start_x as usize;
    let start_uy = start_y as usize;
    visited[start_uy * w + start_ux] = true;
    queue.push_back((start_x, start_y));

    let mut changed = false;
    while let Some((cx, cy)) = queue.pop_front() {
        let px = origin_x + cx as u32;
        let py = origin_y + cy as u32;
        if px < atlas_rgba.width() && py < atlas_rgba.height() {
            atlas_rgba.put_pixel(px, py, replacement);
            changed = true;
        }

        for (nx, ny) in [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)] {
            if nx >= 0 && ny >= 0 && (nx as u32) < region_w && (ny as u32) < region_h {
                let idx = (ny as usize) * w + (nx as usize);
                if !visited[idx] {
                    let npx = origin_x + nx as u32;
                    let npy = origin_y + ny as u32;
                    if npx < atlas_rgba.width()
                        && npy < atlas_rgba.height()
                        && color_matches(atlas_rgba.get_pixel(npx, npy).0, target_rgba.0, tolerance)
                    {
                        visited[idx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    changed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutlineCornerStyle {
    #[default]
    Circle,
    Square,
    Diamond,
}

impl OutlineCornerStyle {
    #[inline]
    pub fn contains_offset(&self, dx: i32, dy: i32, thickness: i32) -> bool {
        match self {
            Self::Circle => {
                if thickness == 1 {
                    dx.abs() + dy.abs() <= 1
                } else {
                    dx * dx + dy * dy <= thickness * thickness
                }
            }
            Self::Square => dx.abs() <= thickness && dy.abs() <= thickness,
            Self::Diamond => dx.abs() + dy.abs() <= thickness,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutlinePlacement {
    #[default]
    Outside,
    Inside,
}

/// Generates an outer or inner outline contour of configurable thickness and corner geometry
/// around opaque pixel clusters, optionally constrained to a selection mask.
#[allow(clippy::too_many_arguments)]
pub fn generate_outline_in_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    outline_color: [u8; 4],
    thickness: i32,
    corner_style: OutlineCornerStyle,
    placement: OutlinePlacement,
    selection: Option<&HashSet<(i32, i32)>>,
) -> bool {
    if thickness <= 0 {
        return false;
    }

    let is_foreground = |lx: i32, ly: i32| -> bool {
        if lx < 0 || ly < 0 || (lx as u32) >= region_w || (ly as u32) >= region_h {
            return false;
        }
        if let Some(sel) = selection {
            if !sel.is_empty() && !sel.contains(&(lx, ly)) {
                return false;
            }
        }
        let ax = origin_x + lx as u32;
        let ay = origin_y + ly as u32;
        if ax >= atlas_rgba.width() || ay >= atlas_rgba.height() {
            return false;
        }
        atlas_rgba.get_pixel(ax, ay)[3] > 0
    };

    let mut to_paint = Vec::new();

    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            let fg = is_foreground(lx, ly);
            match placement {
                OutlinePlacement::Outside => {
                    if !fg {
                        let mut touches_fg = false;
                        for dy in -thickness..=thickness {
                            for dx in -thickness..=thickness {
                                if (dx != 0 || dy != 0)
                                    && corner_style.contains_offset(dx, dy, thickness)
                                    && is_foreground(lx + dx, ly + dy)
                                {
                                    touches_fg = true;
                                    break;
                                }
                            }
                            if touches_fg {
                                break;
                            }
                        }
                        if touches_fg {
                            to_paint.push((origin_x + lx as u32, origin_y + ly as u32));
                        }
                    }
                }
                OutlinePlacement::Inside => {
                    if fg {
                        let mut touches_bg = false;
                        for dy in -thickness..=thickness {
                            for dx in -thickness..=thickness {
                                if (dx != 0 || dy != 0)
                                    && corner_style.contains_offset(dx, dy, thickness)
                                    && !is_foreground(lx + dx, ly + dy)
                                {
                                    touches_bg = true;
                                    break;
                                }
                            }
                            if touches_bg {
                                break;
                            }
                        }
                        if touches_bg {
                            to_paint.push((origin_x + lx as u32, origin_y + ly as u32));
                        }
                    }
                }
            }
        }
    }

    if to_paint.is_empty() {
        return false;
    }

    let col = Rgba(outline_color);
    for (ax, ay) in to_paint {
        if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
            atlas_rgba.put_pixel(ax, ay, col);
        }
    }

    true
}

/// Draws a 1px outline in `outline_color` around all opaque (`alpha > 0`) pixel clusters
/// inside the specified frame region.
pub fn auto_outline_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    outline_color: [u8; 4],
) -> bool {
    generate_outline_in_region(
        atlas_rgba,
        origin_x,
        origin_y,
        region_w,
        region_h,
        outline_color,
        1,
        OutlineCornerStyle::Diamond,
        OutlinePlacement::Outside,
        None,
    )
}

/// Replaces all occurrences of `from_color` with `to_color` within the frame region,
/// optionally constrained to `selection`. Returns `true` if at least one pixel was modified.
#[allow(clippy::too_many_arguments)]
pub fn replace_color_in_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    from_color: [u8; 4],
    to_color: [u8; 4],
    selection: Option<&std::collections::HashSet<(i32, i32)>>,
    tolerance: u8,
) -> bool {
    if from_color == to_color && tolerance == 0 {
        return false;
    }
    let replacement = Rgba(to_color);
    let mut changed = false;

    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            if let Some(sel) = selection {
                if !sel.is_empty() && !sel.contains(&(lx, ly)) {
                    continue;
                }
            }
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width()
                && ay < atlas_rgba.height()
                && color_matches(atlas_rgba.get_pixel(ax, ay).0, from_color, tolerance)
            {
                atlas_rgba.put_pixel(ax, ay, replacement);
                changed = true;
            }
        }
    }

    changed
}

/// Adjusts Hue (-180..=180 deg), Saturation (-100..=100%), Value/Brightness (-100..=100%),
/// and Contrast (-100..=100%) within the frame region, optionally constrained to `selection`.
#[allow(clippy::too_many_arguments)]
pub fn adjust_hsv_in_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    hue_shift: f32,
    sat_shift: f32,
    val_shift: f32,
    contrast_shift: f32,
    selection: Option<&std::collections::HashSet<(i32, i32)>>,
) -> bool {
    if hue_shift.abs() < 0.001
        && sat_shift.abs() < 0.001
        && val_shift.abs() < 0.001
        && contrast_shift.abs() < 0.001
    {
        return false;
    }

    let mut changed = false;
    let contrast_factor = if contrast_shift.abs() > 0.001 {
        let c = contrast_shift.clamp(-99.0, 99.0) * 2.55;
        (259.0 * (c + 255.0)) / (255.0 * (259.0 - c))
    } else {
        1.0
    };

    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            if let Some(sel) = selection {
                if !sel.is_empty() && !sel.contains(&(lx, ly)) {
                    continue;
                }
            }
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let pixel = *atlas_rgba.get_pixel(ax, ay);
                if pixel[3] == 0 {
                    continue;
                }

                let col32 = egui::Color32::from_rgba_unmultiplied(
                    pixel[0], pixel[1], pixel[2], pixel[3],
                );
                let mut hsva = egui::ecolor::Hsva::from(col32);

                hsva.h = (hsva.h + hue_shift / 360.0).rem_euclid(1.0);
                hsva.s = (hsva.s + sat_shift / 100.0).clamp(0.0, 1.0);
                hsva.v = (hsva.v + val_shift / 100.0).clamp(0.0, 1.0);

                let adjusted_col = egui::Color32::from(hsva);
                let (mut r, mut g, mut b) = (
                    adjusted_col.r() as f32,
                    adjusted_col.g() as f32,
                    adjusted_col.b() as f32,
                );

                if (contrast_factor - 1.0).abs() > 0.001 {
                    r = (contrast_factor * (r - 128.0) + 128.0).clamp(0.0, 255.0);
                    g = (contrast_factor * (g - 128.0) + 128.0).clamp(0.0, 255.0);
                    b = (contrast_factor * (b - 128.0) + 128.0).clamp(0.0, 255.0);
                }

                let new_pixel = Rgba([r.round() as u8, g.round() as u8, b.round() as u8, pixel[3]]);
                if new_pixel != pixel {
                    atlas_rgba.put_pixel(ax, ay, new_pixel);
                    changed = true;
                }
            }
        }
    }

    changed
}

pub const BAYER_4X4: [[f32; 4]; 4] = [
    [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
    [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
    [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
    [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

pub const BAYER_8X8: [[f32; 8]; 8] = [
    [0.0 / 64.0, 32.0 / 64.0, 8.0 / 64.0, 40.0 / 64.0, 2.0 / 64.0, 34.0 / 64.0, 10.0 / 64.0, 42.0 / 64.0],
    [48.0 / 64.0, 16.0 / 64.0, 56.0 / 64.0, 24.0 / 64.0, 50.0 / 64.0, 18.0 / 64.0, 58.0 / 64.0, 26.0 / 64.0],
    [12.0 / 64.0, 44.0 / 64.0, 4.0 / 64.0, 36.0 / 64.0, 14.0 / 64.0, 46.0 / 64.0, 6.0 / 64.0, 38.0 / 64.0],
    [60.0 / 64.0, 28.0 / 64.0, 52.0 / 64.0, 20.0 / 64.0, 62.0 / 64.0, 30.0 / 64.0, 54.0 / 64.0, 22.0 / 64.0],
    [3.0 / 64.0, 35.0 / 64.0, 11.0 / 64.0, 43.0 / 64.0, 1.0 / 64.0, 33.0 / 64.0, 9.0 / 64.0, 41.0 / 64.0],
    [51.0 / 64.0, 19.0 / 64.0, 59.0 / 64.0, 27.0 / 64.0, 49.0 / 64.0, 17.0 / 64.0, 57.0 / 64.0, 25.0 / 64.0],
    [15.0 / 64.0, 47.0 / 64.0, 7.0 / 64.0, 39.0 / 64.0, 13.0 / 64.0, 45.0 / 64.0, 5.0 / 64.0, 37.0 / 64.0],
    [63.0 / 64.0, 31.0 / 64.0, 55.0 / 64.0, 23.0 / 64.0, 61.0 / 64.0, 29.0 / 64.0, 53.0 / 64.0, 21.0 / 64.0],
];

/// Fills the region or selection with a linear or radial gradient between `c0` and `c1`,
/// with optional dithering (Bayer 4×4, Bayer 8×8, or 50% checkerboard).
#[allow(clippy::too_many_arguments)]
pub fn apply_gradient_in_region(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    p0: (i32, i32),
    p1: (i32, i32),
    c0: [u8; 4],
    c1: [u8; 4],
    kind: GradientKind,
    dither: GradientDither,
    selection: Option<&HashSet<(i32, i32)>>,
    lock_alpha: bool,
) -> bool {
    let mut changed = false;

    let vx = (p1.0 - p0.0) as f32;
    let vy = (p1.1 - p0.1) as f32;
    let len_sq = vx * vx + vy * vy;
    let radius = len_sq.sqrt();

    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            if let Some(sel) = selection {
                if !sel.is_empty() && !sel.contains(&(lx, ly)) {
                    continue;
                }
            }
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                let existing = *atlas_rgba.get_pixel(ax, ay);
                if lock_alpha && existing[3] == 0 {
                    continue;
                }

                let ux = (lx - p0.0) as f32;
                let uy = (ly - p0.1) as f32;

                let t = match kind {
                    GradientKind::Linear => {
                        if len_sq > 0.001 {
                            ((ux * vx + uy * vy) / len_sq).clamp(0.0, 1.0)
                        } else {
                            0.0
                        }
                    }
                    GradientKind::Radial => {
                        if radius > 0.001 {
                            ((ux * ux + uy * uy).sqrt() / radius).clamp(0.0, 1.0)
                        } else {
                            0.0
                        }
                    }
                };

                let dither_offset = match dither {
                    GradientDither::None => 0.0,
                    GradientDither::Bayer4x4 => {
                        let bx = (lx.rem_euclid(4)) as usize;
                        let by = (ly.rem_euclid(4)) as usize;
                        (BAYER_4X4[by][bx] - 0.5) * 0.25
                    }
                    GradientDither::Bayer8x8 => {
                        let bx = (lx.rem_euclid(8)) as usize;
                        let by = (ly.rem_euclid(8)) as usize;
                        (BAYER_8X8[by][bx] - 0.5) * 0.25
                    }
                    GradientDither::Dither50 => {
                        if (lx + ly).rem_euclid(2) == 1 {
                            0.12
                        } else {
                            -0.12
                        }
                    }
                };

                let t_eff = (t + dither_offset).clamp(0.0, 1.0);
                let inv_t = 1.0 - t_eff;

                let r = (c0[0] as f32 * inv_t + c1[0] as f32 * t_eff).round() as u8;
                let g = (c0[1] as f32 * inv_t + c1[1] as f32 * t_eff).round() as u8;
                let b = (c0[2] as f32 * inv_t + c1[2] as f32 * t_eff).round() as u8;
                let a = (c0[3] as f32 * inv_t + c1[3] as f32 * t_eff).round() as u8;

                let new_pixel = Rgba([r, g, b, a]);
                if new_pixel != existing {
                    atlas_rgba.put_pixel(ax, ay, new_pixel);
                    changed = true;
                }
            }
        }
    }

    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_color_in_region() {
        let mut img = RgbaImage::from_pixel(4, 4, Rgba([255, 0, 0, 255]));
        img.put_pixel(1, 1, Rgba([0, 255, 0, 255]));

        // Replace red with blue
        assert!(replace_color_in_region(
            &mut img,
            0,
            0,
            4,
            4,
            [255, 0, 0, 255],
            [0, 0, 255, 255],
            None,
            0,
        ));

        assert_eq!(*img.get_pixel(0, 0), Rgba([0, 0, 255, 255]));
        // The green pixel at (1,1) is unchanged
        assert_eq!(*img.get_pixel(1, 1), Rgba([0, 255, 0, 255]));

        // With selection: only replace pixel inside selection
        let mut sel = std::collections::HashSet::new();
        sel.insert((1, 1));
        assert!(replace_color_in_region(
            &mut img,
            0,
            0,
            4,
            4,
            [0, 255, 0, 255],
            [255, 255, 0, 255],
            Some(&sel),
            0,
        ));
        assert_eq!(*img.get_pixel(1, 1), Rgba([255, 255, 0, 255]));

        // With tolerance: replace near-blue colors
        img.put_pixel(2, 2, Rgba([5, 5, 250, 255]));
        assert!(replace_color_in_region(
            &mut img,
            0,
            0,
            4,
            4,
            [0, 0, 255, 255],
            [200, 200, 200, 255],
            None,
            15,
        ));
        assert_eq!(*img.get_pixel(2, 2), Rgba([200, 200, 200, 255]));
    }

    #[test]
    fn test_adjust_hsv_in_region() {
        let mut img = RgbaImage::from_pixel(4, 4, Rgba([255, 0, 0, 255]));
        // Shift Hue by 180 degrees (Red -> Cyan)
        assert!(adjust_hsv_in_region(
            &mut img,
            0,
            0,
            4,
            4,
            180.0,
            0.0,
            0.0,
            0.0,
            None,
        ));
        let p = *img.get_pixel(0, 0);
        // In cyan, red is low and green/blue are high
        assert!(p[0] < 50);
        assert!(p[1] > 200);
        assert!(p[2] > 200);
    }

    #[test]
    fn test_apply_gradient_in_region_linear_and_radial() {
        let mut img = RgbaImage::from_pixel(16, 16, Rgba([0, 0, 0, 0]));

        // Linear gradient from (0, 0) to (15, 0) between Black and White
        assert!(apply_gradient_in_region(
            &mut img,
            0,
            0,
            16,
            16,
            (0, 0),
            (15, 0),
            [0, 0, 0, 255],
            [255, 255, 255, 255],
            GradientKind::Linear,
            GradientDither::None,
            None,
            false,
        ));

        // Start is pure black
        assert_eq!(*img.get_pixel(0, 0), Rgba([0, 0, 0, 255]));
        // End is pure white
        assert_eq!(*img.get_pixel(15, 0), Rgba([255, 255, 255, 255]));
        // Midpoint (7, 0) is roughly grey
        let mid = *img.get_pixel(7, 0);
        assert!(mid[0] > 100 && mid[0] < 140);

        // Radial gradient with selection
        let mut sel = HashSet::new();
        sel.insert((8, 8));
        assert!(apply_gradient_in_region(
            &mut img,
            0,
            0,
            16,
            16,
            (8, 8),
            (16, 8),
            [255, 0, 0, 255],
            [0, 0, 255, 255],
            GradientKind::Radial,
            GradientDither::Bayer4x4,
            Some(&sel),
            false,
        ));
        // Center pixel inside selection is red
        assert_eq!(*img.get_pixel(8, 8), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_generate_outline_in_region_outside_and_inside() {
        let mut img = RgbaImage::from_pixel(8, 8, Rgba([0, 0, 0, 0]));
        // 2x2 opaque sprite in the center at (3, 3)..(4, 4)
        for y in 3..=4 {
            for x in 3..=4 {
                img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }

        // Generate 1px outside outline with Diamond corner style (orthogonal neighbors)
        let mut out_img = img.clone();
        assert!(generate_outline_in_region(
            &mut out_img,
            0,
            0,
            8,
            8,
            [0, 255, 0, 255],
            1,
            OutlineCornerStyle::Diamond,
            OutlinePlacement::Outside,
            None,
        ));
        // Direct orthogonal neighbor (3, 2) is outline green
        assert_eq!(*out_img.get_pixel(3, 2), Rgba([0, 255, 0, 255]));
        // Diagonal neighbor (2, 2) is still transparent in Diamond mode
        assert_eq!(*out_img.get_pixel(2, 2), Rgba([0, 0, 0, 0]));

        // Generate 1px outside outline with Square corner style (includes diagonals)
        let mut sq_img = img.clone();
        assert!(generate_outline_in_region(
            &mut sq_img,
            0,
            0,
            8,
            8,
            [0, 255, 0, 255],
            1,
            OutlineCornerStyle::Square,
            OutlinePlacement::Outside,
            None,
        ));
        // Diagonal neighbor (2, 2) is outline green in Square mode
        assert_eq!(*sq_img.get_pixel(2, 2), Rgba([0, 255, 0, 255]));

        // Generate 1px inside outline: turns sprite border into outline color
        let mut in_img = img.clone();
        assert!(generate_outline_in_region(
            &mut in_img,
            0,
            0,
            8,
            8,
            [255, 255, 0, 255],
            1,
            OutlineCornerStyle::Circle,
            OutlinePlacement::Inside,
            None,
        ));
        // (3, 3) was foreground and touches background, so becomes yellow
        assert_eq!(*in_img.get_pixel(3, 3), Rgba([255, 255, 0, 255]));
    }
}

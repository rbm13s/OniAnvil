use crate::tools::{DitherPattern, SymmetryMode};

/// Generates a 4/8-connected Bresenham pixel path from `p0` to `p1` (inclusive).
pub fn bresenham_line(p0: (i32, i32), p1: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let (mut x0, mut y0) = p0;
    let (x1, y1) = p1;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    points
}

/// Removes 1px L-shaped corner doubles from a freehand stroke path.
pub fn filter_pixel_perfect(raw_points: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut dedup: Vec<(i32, i32)> = Vec::with_capacity(raw_points.len());
    for &pt in raw_points {
        if dedup.last().copied() != Some(pt) {
            dedup.push(pt);
        }
    }
    if dedup.len() < 3 {
        return dedup;
    }

    let mut result: Vec<(i32, i32)> = Vec::with_capacity(dedup.len());
    let mut i = 0;
    while i < dedup.len() {
        if i > 0 && i + 1 < dedup.len() {
            let prev = *result.last().unwrap_or(&dedup[i - 1]);
            let curr = dedup[i];
            let next = dedup[i + 1];

            let d_prev = (curr.0 - prev.0).abs() + (curr.1 - prev.1).abs();
            let d_next = (next.0 - curr.0).abs() + (next.1 - curr.1).abs();
            let diag_x = (next.0 - prev.0).abs();
            let diag_y = (next.1 - prev.1).abs();

            if d_prev == 1 && d_next == 1 && diag_x == 1 && diag_y == 1 {
                i += 1;
                continue;
            }
        }
        result.push(dedup[i]);
        i += 1;
    }
    result
}

/// Checks if a pixel coordinate `(x, y)` passes the active dither pattern mask.
pub fn dither_passes(pattern: DitherPattern, x: i32, y: i32) -> bool {
    let ux = x.rem_euclid(4);
    let uy = y.rem_euclid(4);
    match pattern {
        DitherPattern::Checker50 => (ux + uy) % 2 == 0,
        DitherPattern::Sparse25 => ux % 2 == 0 && uy % 2 == 0,
        DitherPattern::HorizontalLines => uy % 2 == 0,
        DitherPattern::VerticalLines => ux % 2 == 0,
    }
}

/// Expands a single pixel coordinate `(x, y)` by `brush_size` and mirrors across `symmetry`.
pub fn expand_stamp_points(
    center: (i32, i32),
    brush_size: u32,
    symmetry: SymmetryMode,
    sym_norm_x: f32,
    sym_norm_y: f32,
    frame_w: u32,
    frame_h: u32,
) -> Vec<(i32, i32)> {
    let size = brush_size.clamp(1, 64) as i32;
    let half_low = (size - 1) / 2;
    let half_high = size / 2;
    let radius_sq = (size as f32 * 0.5) * (size as f32 * 0.5);
    let use_circle = size >= 4;

    let mut base_stamp = Vec::new();
    for dy in -half_low..=half_high {
        for dx in -half_low..=half_high {
            if use_circle {
                let cx = dx as f32 - if size % 2 == 0 { 0.5 } else { 0.0 };
                let cy = dy as f32 - if size % 2 == 0 { 0.5 } else { 0.0 };
                if cx * cx + cy * cy > radius_sq + 0.25 {
                    continue;
                }
            }
            base_stamp.push((center.0 + dx, center.1 + dy));
        }
    }

    let axis_x2 = (sym_norm_x * frame_w as f32 * 2.0).round() as i32 - 1;
    let axis_y2 = (sym_norm_y * frame_h as f32 * 2.0).round() as i32 - 1;

    let mut out = Vec::with_capacity(base_stamp.len() * 4);
    for (px, py) in base_stamp {
        out.push((px, py));
        match symmetry {
            SymmetryMode::None => {}
            SymmetryMode::Horizontal => {
                out.push((axis_x2 - px, py));
            }
            SymmetryMode::Vertical => {
                out.push((px, axis_y2 - py));
            }
            SymmetryMode::Quad => {
                let mx = axis_x2 - px;
                let my = axis_y2 - py;
                out.push((mx, py));
                out.push((px, my));
                out.push((mx, my));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_perfect_removes_l_corner() {
        // (0,0) -> (1,0) -> (1,1) is an L-corner where (1,0) is redundant
        let raw = vec![(0, 0), (1, 0), (1, 1)];
        let filtered = filter_pixel_perfect(&raw);
        assert_eq!(filtered, vec![(0, 0), (1, 1)]);
    }

    #[test]
    fn test_bresenham_line_endpoints() {
        let line = bresenham_line((2, 3), (7, 5));
        assert_eq!(line.first().copied(), Some((2, 3)));
        assert_eq!(line.last().copied(), Some((7, 5)));
    }
}

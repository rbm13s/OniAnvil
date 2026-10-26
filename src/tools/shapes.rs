use crate::tools::stroke::bresenham_line;

/// Snaps `p1` relative to `p0` to 0°, 45°, or 90° increments when `shift_snap` is true.
pub fn snap_line_endpoint(p0: (i32, i32), p1: (i32, i32), shift_snap: bool) -> (i32, i32) {
    if !shift_snap {
        return p1;
    }
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let adx = dx.abs();
    let ady = dy.abs();

    if adx > ady * 2 {
        (p1.0, p0.1)
    } else if ady > adx * 2 {
        (p0.0, p1.1)
    } else {
        let len = adx.max(ady);
        (p0.0 + dx.signum() * len, p0.1 + dy.signum() * len)
    }
}

/// Constrains `p1` relative to `p0` to a 1:1 square/circle bounding box when `shift_square` is true.
pub fn snap_box_endpoint(p0: (i32, i32), p1: (i32, i32), shift_square: bool) -> (i32, i32) {
    if !shift_square {
        return p1;
    }
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let len = dx.abs().max(dy.abs());
    let sx = if dx >= 0 { 1 } else { -1 };
    let sy = if dy >= 0 { 1 } else { -1 };
    (p0.0 + sx * len, p0.1 + sy * len)
}

pub fn rasterize_line(p0: (i32, i32), p1: (i32, i32), shift_snap: bool) -> Vec<(i32, i32)> {
    let end = snap_line_endpoint(p0, p1, shift_snap);
    bresenham_line(p0, end)
}

pub fn rasterize_rectangle(
    p0: (i32, i32),
    p1: (i32, i32),
    filled: bool,
    shift_square: bool,
) -> Vec<(i32, i32)> {
    let end = snap_box_endpoint(p0, p1, shift_square);
    let min_x = p0.0.min(end.0);
    let max_x = p0.0.max(end.0);
    let min_y = p0.1.min(end.1);
    let max_y = p0.1.max(end.1);

    let mut pts = Vec::new();
    if filled {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                pts.push((x, y));
            }
        }
    } else {
        for x in min_x..=max_x {
            pts.push((x, min_y));
            if max_y != min_y {
                pts.push((x, max_y));
            }
        }
        for y in (min_y + 1)..max_y {
            pts.push((min_x, y));
            if max_x != min_x {
                pts.push((max_x, y));
            }
        }
    }
    pts
}

pub fn rasterize_ellipse(
    p0: (i32, i32),
    p1: (i32, i32),
    filled: bool,
    shift_circle: bool,
) -> Vec<(i32, i32)> {
    let end = snap_box_endpoint(p0, p1, shift_circle);
    let min_x = p0.0.min(end.0);
    let max_x = p0.0.max(end.0);
    let min_y = p0.1.min(end.1);
    let max_y = p0.1.max(end.1);

    let w = max_x - min_x;
    let h = max_y - min_y;
    if w == 0 && h == 0 {
        return vec![(min_x, min_y)];
    }

    // Center & radii in doubled coordinates so both even and odd dimensions are symmetric
    let cx2 = min_x + max_x;
    let cy2 = min_y + max_y;
    let rx = (w as f64) * 0.5 + 0.25;
    let ry = (h as f64) * 0.5 + 0.25;

    let mut pts = Vec::new();
    if filled {
        for y in min_y..=max_y {
            let dy = (2 * y - cy2) as f64 * 0.5;
            for x in min_x..=max_x {
                let dx = (2 * x - cx2) as f64 * 0.5;
                let norm = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry);
                if norm <= 1.0 {
                    pts.push((x, y));
                }
            }
        }
    } else {
        let inside = |x: i32, y: i32| -> bool {
            if x < min_x || x > max_x || y < min_y || y > max_y {
                return false;
            }
            let dx = (2 * x - cx2) as f64 * 0.5;
            let dy = (2 * y - cy2) as f64 * 0.5;
            (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
        };

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if inside(x, y) {
                    let is_border = !inside(x - 1, y)
                        || !inside(x + 1, y)
                        || !inside(x, y - 1)
                        || !inside(x, y + 1);
                    if is_border {
                        pts.push((x, y));
                    }
                }
            }
        }
    }
    pts
}

/// Rasterizes a 3-point quadratic Bezier curve from `p0` to `p2` controlled by `p1`.
/// Produces a connected, continuous sequence of pixel coordinates.
pub fn rasterize_quadratic_bezier(
    p0: (i32, i32),
    p1: (i32, i32),
    p2: (i32, i32),
) -> Vec<(i32, i32)> {
    if p0 == p1 && p1 == p2 {
        return vec![p0];
    }
    if p0 == p2 {
        let seg1 = bresenham_line(p0, p1);
        let mut seg2 = bresenham_line(p1, p0);
        let mut result = seg1;
        result.append(&mut seg2);
        result.dedup();
        return result;
    }

    let d01 = ((p1.0 - p0.0).abs() + (p1.1 - p0.1).abs()) as f32;
    let d12 = ((p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()) as f32;
    let approx_len = (d01 + d12).max(1.0);
    let steps = (approx_len * 2.0).ceil().max(4.0) as usize;

    let mut result = Vec::new();
    let mut prev_pt = p0;
    result.push(p0);

    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let u = 1.0 - t;
        let x = (u * u * p0.0 as f32 + 2.0 * u * t * p1.0 as f32 + t * t * p2.0 as f32).round() as i32;
        let y = (u * u * p0.1 as f32 + 2.0 * u * t * p1.1 as f32 + t * t * p2.1 as f32).round() as i32;
        let curr_pt = (x, y);
        if curr_pt != prev_pt {
            let segment = bresenham_line(prev_pt, curr_pt);
            for pt in segment.into_iter().skip(1) {
                if result.last() != Some(&pt) {
                    result.push(pt);
                }
            }
            prev_pt = curr_pt;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rasterize_quadratic_bezier() {
        // Straight line when control point is collinear midpoint
        let pts_straight = rasterize_quadratic_bezier((0, 0), (5, 0), (10, 0));
        assert_eq!(pts_straight.first(), Some(&(0, 0)));
        assert_eq!(pts_straight.last(), Some(&(10, 0)));
        assert_eq!(pts_straight.len(), 11);

        // Curved arc
        let pts_curved = rasterize_quadratic_bezier((0, 0), (5, 10), (10, 0));
        assert_eq!(pts_curved.first(), Some(&(0, 0)));
        assert_eq!(pts_curved.last(), Some(&(10, 0)));
        assert!(pts_curved.iter().any(|&(_, y)| y >= 4));
    }
}

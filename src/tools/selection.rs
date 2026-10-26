use crate::tools::fill::color_matches;
use crate::tools::stroke::bresenham_line;
use image::RgbaImage;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionModifyShape {
    #[default]
    Circle,
    Square,
    Diamond,
}

impl SelectionModifyShape {
    #[inline]
    pub fn contains_offset(&self, dx: i32, dy: i32, n: i32) -> bool {
        match self {
            Self::Circle => {
                if n == 1 {
                    dx.abs() + dy.abs() <= 1
                } else {
                    dx * dx + dy * dy <= n * n
                }
            }
            Self::Square => dx.abs() <= n && dy.abs() <= n,
            Self::Diamond => dx.abs() + dy.abs() <= n,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SelectionMask {
    /// Selected pixel coordinates `(x, y)` in local frame coordinates.
    pub pixels: HashSet<(i32, i32)>,
    /// Optional rectangular marquee bounds `(min_x, min_y, max_x, max_y)` in local frame coordinates.
    pub rect_bounds: Option<(i32, i32, i32, i32)>,
}

impl SelectionMask {
    pub fn clear(&mut self) {
        self.pixels.clear();
        self.rect_bounds = None;
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        if self.pixels.is_empty() {
            return (0, 0, 0, 0);
        }
        let min_x = self.pixels.iter().map(|p| p.0).min().unwrap_or(0);
        let max_x = self.pixels.iter().map(|p| p.0).max().unwrap_or(0);
        let min_y = self.pixels.iter().map(|p| p.1).min().unwrap_or(0);
        let max_y = self.pixels.iter().map(|p| p.1).max().unwrap_or(0);
        (min_x, min_y, max_x, max_y)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        if self.pixels.is_empty() {
            true
        } else {
            self.pixels.contains(&(x, y))
        }
    }

    pub fn set_rect(&mut self, p0: (i32, i32), p1: (i32, i32), max_w: u32, max_h: u32) {
        self.pixels.clear();
        let min_x = p0.0.min(p1.0).max(0);
        let max_x = p0.0.max(p1.0).min(max_w as i32 - 1);
        let min_y = p0.1.min(p1.1).max(0);
        let max_y = p0.1.max(p1.1).min(max_h as i32 - 1);

        if min_x <= max_x && min_y <= max_y {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    self.pixels.insert((x, y));
                }
            }
            self.rect_bounds = Some((min_x, min_y, max_x, max_y));
        } else {
            self.rect_bounds = None;
        }
    }

    pub fn select_all(&mut self, max_w: u32, max_h: u32) {
        self.set_rect(
            (0, 0),
            (max_w.saturating_sub(1) as i32, max_h.saturating_sub(1) as i32),
            max_w,
            max_h,
        );
    }

    pub fn invert(&mut self, max_w: u32, max_h: u32) {
        let mut new_pixels = HashSet::new();
        for y in 0..(max_h as i32) {
            for x in 0..(max_w as i32) {
                if !self.pixels.contains(&(x, y)) {
                    new_pixels.insert((x, y));
                }
            }
        }
        self.pixels = new_pixels;
        self.rect_bounds = None;
    }

    pub fn set_polygon(&mut self, polygon: &[(i32, i32)], max_w: u32, max_h: u32) {
        self.pixels.clear();
        if polygon.is_empty() {
            self.rect_bounds = None;
            return;
        }

        let iw = max_w.max(1) as i32;
        let ih = max_h.max(1) as i32;

        if polygon.len() < 3 {
            for &(px, py) in polygon {
                if px >= 0 && px < iw && py >= 0 && py < ih {
                    self.pixels.insert((px, py));
                }
            }
            let min_x = polygon.iter().map(|p| p.0).min().unwrap_or(0).clamp(0, iw - 1);
            let max_x = polygon.iter().map(|p| p.0).max().unwrap_or(0).clamp(0, iw - 1);
            let min_y = polygon.iter().map(|p| p.1).min().unwrap_or(0).clamp(0, ih - 1);
            let max_y = polygon.iter().map(|p| p.1).max().unwrap_or(0).clamp(0, ih - 1);
            self.rect_bounds = Some((min_x, min_y, max_x, max_y));
            return;
        }

        let raw_min_x = polygon.iter().map(|p| p.0).min().unwrap_or(0);
        let raw_max_x = polygon.iter().map(|p| p.0).max().unwrap_or(0);
        let raw_min_y = polygon.iter().map(|p| p.1).min().unwrap_or(0);
        let raw_max_y = polygon.iter().map(|p| p.1).max().unwrap_or(0);

        // Include 1px exterior margin outside polygon and canvas bounds
        let grid_min_x = raw_min_x.min(0) - 1;
        let grid_max_x = raw_max_x.max(iw - 1) + 1;
        let grid_min_y = raw_min_y.min(0) - 1;
        let grid_max_y = raw_max_y.max(ih - 1) + 1;

        let w = (grid_max_x - grid_min_x + 1) as usize;
        let h = (grid_max_y - grid_min_y + 1) as usize;

        if w == 0 || h == 0 || w.checked_mul(h).is_none_or(|size| size > 16_000_000) {
            return;
        }

        let mut is_boundary = vec![false; w * h];
        let n = polygon.len();
        for i in 0..n {
            let p0 = polygon[i];
            let p1 = polygon[(i + 1) % n];
            for (bx, by) in bresenham_line(p0, p1) {
                if bx >= grid_min_x && bx <= grid_max_x && by >= grid_min_y && by <= grid_max_y {
                    let gx = (bx - grid_min_x) as usize;
                    let gy = (by - grid_min_y) as usize;
                    is_boundary[gy * w + gx] = true;
                }
            }
        }

        // 4-connected flood-fill from exterior boundary of the grid.
        // Because Bresenham is 8-connected, 4-connected flood fill cannot penetrate
        // diagonal or orthogonal boundaries, leaving all interior enclosed pixels unvisited.
        let mut visited = vec![false; w * h];
        let mut stack = Vec::with_capacity(2 * (w + h));

        for x in 0..w {
            let top_idx = x;
            if !is_boundary[top_idx] && !visited[top_idx] {
                visited[top_idx] = true;
                stack.push((x as i32, 0));
            }
            let bot_idx = (h - 1) * w + x;
            if !is_boundary[bot_idx] && !visited[bot_idx] {
                visited[bot_idx] = true;
                stack.push((x as i32, (h - 1) as i32));
            }
        }
        for y in 0..h {
            let left_idx = y * w;
            if !is_boundary[left_idx] && !visited[left_idx] {
                visited[left_idx] = true;
                stack.push((0, y as i32));
            }
            let right_idx = y * w + (w - 1);
            if !is_boundary[right_idx] && !visited[right_idx] {
                visited[right_idx] = true;
                stack.push(((w - 1) as i32, y as i32));
            }
        }

        while let Some((cx, cy)) = stack.pop() {
            let neighbors = [
                (cx + 1, cy),
                (cx - 1, cy),
                (cx, cy + 1),
                (cx, cy - 1),
            ];
            for (nx, ny) in neighbors {
                if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                    let nidx = ny as usize * w + nx as usize;
                    if !visited[nidx] && !is_boundary[nidx] {
                        visited[nidx] = true;
                        stack.push((nx, ny));
                    }
                }
            }
        }

        let mut min_sel_x = i32::MAX;
        let mut max_sel_x = i32::MIN;
        let mut min_sel_y = i32::MAX;
        let mut max_sel_y = i32::MIN;

        for gy in 0..h {
            let y = gy as i32 + grid_min_y;
            if y < 0 || y >= ih {
                continue;
            }
            for gx in 0..w {
                let x = gx as i32 + grid_min_x;
                if x < 0 || x >= iw {
                    continue;
                }
                let idx = gy * w + gx;
                if !visited[idx] {
                    self.pixels.insert((x, y));
                    min_sel_x = min_sel_x.min(x);
                    max_sel_x = max_sel_x.max(x);
                    min_sel_y = min_sel_y.min(y);
                    max_sel_y = max_sel_y.max(y);
                }
            }
        }

        if self.pixels.is_empty() {
            self.rect_bounds = None;
        } else {
            self.rect_bounds = Some((min_sel_x, min_sel_y, max_sel_x, max_sel_y));
        }
    }

    pub fn translate(&mut self, dx: i32, dy: i32, max_w: u32, max_h: u32, wrap_around: bool) {
        let iw = max_w.max(1) as i32;
        let ih = max_h.max(1) as i32;
        let mut new_pixels = HashSet::new();
        for &(x, y) in &self.pixels {
            let nx = if wrap_around {
                (x + dx).rem_euclid(iw)
            } else {
                x + dx
            };
            let ny = if wrap_around {
                (y + dy).rem_euclid(ih)
            } else {
                y + dy
            };
            if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
                new_pixels.insert((nx, ny));
            }
        }
        self.pixels = new_pixels;
        if let Some((min_x, min_y, max_x, max_y)) = self.rect_bounds {
            if wrap_around {
                self.rect_bounds = None;
            } else {
                self.rect_bounds = Some((
                    (min_x + dx).clamp(0, iw - 1),
                    (min_y + dy).clamp(0, ih - 1),
                    (max_x + dx).clamp(0, iw - 1),
                    (max_y + dy).clamp(0, ih - 1),
                ));
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn select_magic_wand(
        &mut self,
        atlas_rgba: &RgbaImage,
        origin_x: u32,
        origin_y: u32,
        region_w: u32,
        region_h: u32,
        start_x: i32,
        start_y: i32,
        global_match: bool,
        tolerance: u8,
    ) {
        self.clear();
        if start_x < 0
            || start_y < 0
            || (start_x as u32) >= region_w
            || (start_y as u32) >= region_h
        {
            return;
        }

        let ax0 = origin_x + start_x as u32;
        let ay0 = origin_y + start_y as u32;
        if ax0 >= atlas_rgba.width() || ay0 >= atlas_rgba.height() {
            return;
        }

        let target = *atlas_rgba.get_pixel(ax0, ay0);

        if global_match {
            for y in 0..(region_h as i32) {
                for x in 0..(region_w as i32) {
                    let px = origin_x + x as u32;
                    let py = origin_y + y as u32;
                    if px < atlas_rgba.width()
                        && py < atlas_rgba.height()
                        && color_matches(atlas_rgba.get_pixel(px, py).0, target.0, tolerance)
                    {
                        self.pixels.insert((x, y));
                    }
                }
            }
        } else {
            let mut queue = VecDeque::new();
            queue.push_back((start_x, start_y));
            self.pixels.insert((start_x, start_y));

            while let Some((cx, cy)) = queue.pop_front() {
                for (nx, ny) in [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)] {
                    if nx >= 0
                        && ny >= 0
                        && (nx as u32) < region_w
                        && (ny as u32) < region_h
                        && !self.pixels.contains(&(nx, ny))
                    {
                        let px = origin_x + nx as u32;
                        let py = origin_y + ny as u32;
                        if px < atlas_rgba.width()
                            && py < atlas_rgba.height()
                            && color_matches(atlas_rgba.get_pixel(px, py).0, target.0, tolerance)
                        {
                            self.pixels.insert((nx, ny));
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
        self.update_rect_bounds_from_pixels();
    }

    pub fn update_rect_bounds_from_pixels(&mut self) {
        if self.pixels.is_empty() {
            self.rect_bounds = None;
        } else {
            let min_x = self.pixels.iter().map(|p| p.0).min().unwrap_or(0);
            let max_x = self.pixels.iter().map(|p| p.0).max().unwrap_or(0);
            let min_y = self.pixels.iter().map(|p| p.1).min().unwrap_or(0);
            let max_y = self.pixels.iter().map(|p| p.1).max().unwrap_or(0);
            self.rect_bounds = Some((min_x, min_y, max_x, max_y));
        }
    }

    /// Expands the current selection outwards by `n` pixels using the specified geometric structuring element.
    pub fn expand(&mut self, n: i32, shape: SelectionModifyShape, max_w: u32, max_h: u32) {
        if n <= 0 || self.pixels.is_empty() {
            return;
        }
        let iw = max_w.max(1) as i32;
        let ih = max_h.max(1) as i32;
        let mut expanded = self.pixels.clone();
        for &(x, y) in &self.pixels {
            for dy in -n..=n {
                for dx in -n..=n {
                    if shape.contains_offset(dx, dy, n) {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < iw && ny >= 0 && ny < ih {
                            expanded.insert((nx, ny));
                        }
                    }
                }
            }
        }
        self.pixels = expanded;
        self.update_rect_bounds_from_pixels();
    }

    /// Contracts the current selection inwards by `n` pixels using the specified geometric structuring element.
    pub fn contract(&mut self, n: i32, shape: SelectionModifyShape) {
        if n <= 0 || self.pixels.is_empty() {
            return;
        }
        let mut contracted = HashSet::new();
        for &(x, y) in &self.pixels {
            let mut keep = true;
            for dy in -n..=n {
                for dx in -n..=n {
                    if shape.contains_offset(dx, dy, n) && !self.pixels.contains(&(x + dx, y + dy)) {
                        keep = false;
                        break;
                    }
                }
                if !keep {
                    break;
                }
            }
            if keep {
                contracted.insert((x, y));
            }
        }
        self.pixels = contracted;
        self.update_rect_bounds_from_pixels();
    }

    /// Transforms the selection into an inner border contour of thickness `n` pixels.
    pub fn border(&mut self, n: i32, shape: SelectionModifyShape) {
        if n <= 0 || self.pixels.is_empty() {
            return;
        }
        let mut inner_rim = HashSet::new();
        for &(x, y) in &self.pixels {
            let mut touches_outside = false;
            for dy in -n..=n {
                for dx in -n..=n {
                    if shape.contains_offset(dx, dy, n) && !self.pixels.contains(&(x + dx, y + dy)) {
                        touches_outside = true;
                        break;
                    }
                }
                if touches_outside {
                    break;
                }
            }
            if touches_outside {
                inner_rim.insert((x, y));
            }
        }
        self.pixels = inner_rim;
        self.update_rect_bounds_from_pixels();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_select_all_and_invert() {
        let mut mask = SelectionMask::default();
        mask.select_all(4, 4);
        assert_eq!(mask.pixels.len(), 16);
        assert!(mask.contains(0, 0));
        assert!(mask.contains(3, 3));

        // Inverting a full selection produces an empty set of pixels
        mask.invert(4, 4);
        assert!(mask.pixels.is_empty());

        // Inverting empty selection selects all 16
        mask.invert(4, 4);
        assert_eq!(mask.pixels.len(), 16);
    }

    #[test]
    fn test_selection_translate() {
        let mut mask = SelectionMask::default();
        mask.set_rect((1, 1), (2, 2), 4, 4);
        assert_eq!(mask.pixels.len(), 4);

        mask.translate(1, 0, 4, 4, false);
        assert!(mask.contains(2, 1));
        assert!(mask.contains(3, 2));
        assert!(!mask.contains(1, 1));
    }

    #[test]
    fn test_selection_polygon() {
        let mut mask = SelectionMask::default();
        let tri = vec![(0, 0), (10, 0), (5, 10)];
        mask.set_polygon(&tri, 32, 32);

        assert!(mask.contains(0, 0));
        assert!(mask.contains(10, 0));
        assert!(mask.contains(5, 10));
        // Interior point
        assert!(mask.contains(5, 3));
        // Far outside point
        assert!(!mask.contains(20, 20));
    }

    #[test]
    fn test_selection_polygon_self_intersecting_loops() {
        let mut mask = SelectionMask::default();
        // A loop with self-intersection / internal loop
        let looped_poly = vec![
            (2, 2), (10, 2), (10, 10), (2, 10), // outer box
            (2, 2), (6, 6), (8, 6), (8, 8), (6, 8), (6, 6), // internal coil/loop
            (2, 2),
        ];
        mask.set_polygon(&looped_poly, 16, 16);

        // Center of outer box
        assert!(mask.contains(4, 4));
        // Inside internal coil
        assert!(mask.contains(7, 7));
        // Outside
        assert!(!mask.contains(0, 0));
        assert!(!mask.contains(12, 12));
    }

    #[test]
    fn test_selection_expand_contract_border() {
        let mut mask = SelectionMask::default();
        // 3x3 square at (2, 2)..(4, 4)
        mask.set_rect((2, 2), (4, 4), 16, 16);
        assert_eq!(mask.pixels.len(), 9);

        // Border with Diamond, 1px: interior (3, 3) removed, 8 outer perimeter pixels remain
        let mut border_mask = mask.clone();
        border_mask.border(1, SelectionModifyShape::Diamond);
        assert_eq!(border_mask.pixels.len(), 8);
        assert!(!border_mask.contains(3, 3));
        assert!(border_mask.contains(2, 2));

        // Contract with Diamond, 1px: only the interior pixel (3, 3) survives
        let mut contract_mask = mask.clone();
        contract_mask.contract(1, SelectionModifyShape::Diamond);
        assert_eq!(contract_mask.pixels.len(), 1);
        assert!(contract_mask.contains(3, 3));

        // Expand with Square, 1px: 3x3 expands to 5x5 = 25 pixels
        let mut expand_mask = mask.clone();
        expand_mask.expand(1, SelectionModifyShape::Square, 16, 16);
        assert_eq!(expand_mask.pixels.len(), 25);
        assert!(expand_mask.contains(1, 1));
        assert!(expand_mask.contains(5, 5));
    }
}

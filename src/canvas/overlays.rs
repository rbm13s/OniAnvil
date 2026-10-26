use crate::canvas::viewport::{BackgroundMode, CanvasViewMode, ViewportState};
use crate::model::metadata::ProjectMetadata;
use crate::tools::selection::SelectionMask;
use crate::tools::SymmetryMode;
use egui::{Color32, FontId, Painter, Pos2, Rect, Stroke, Vec2};

pub fn draw_canvas_background(
    painter: &Painter,
    viewport_rect: Rect,
    target_rect: Rect,
    viewport: &ViewportState,
) {
    let clipped = target_rect.intersect(viewport_rect);
    if !clipped.is_positive() {
        return;
    }

    match viewport.background_mode {
        BackgroundMode::SolidColor => {
            let [r, g, b, a] = viewport.custom_bg_color;
            painter.rect_filled(clipped, 0.0, Color32::from_rgba_unmultiplied(r, g, b, a));
        }
        BackgroundMode::DarkCheckerboard | BackgroundMode::LightCheckerboard => {
            let (c1, c2) = match viewport.background_mode {
                BackgroundMode::DarkCheckerboard => (
                    Color32::from_rgb(34, 34, 34),
                    Color32::from_rgb(46, 46, 46),
                ),
                _ => (
                    Color32::from_rgb(185, 185, 185),
                    Color32::from_rgb(220, 220, 220),
                ),
            };

            painter.rect_filled(clipped, 0.0, c1);

            let tile_size = (viewport.zoom * 4.0).clamp(8.0, 32.0);
            let start_col = ((clipped.min.x - target_rect.min.x) / tile_size).floor() as i32;
            let end_col = ((clipped.max.x - target_rect.min.x) / tile_size).ceil() as i32;
            let start_row = ((clipped.min.y - target_rect.min.y) / tile_size).floor() as i32;
            let end_row = ((clipped.max.y - target_rect.min.y) / tile_size).ceil() as i32;

            for row in start_row..end_row {
                for col in start_col..end_col {
                    if (row + col).rem_euclid(2) == 1 {
                        let min = Pos2::new(
                            target_rect.min.x + col as f32 * tile_size,
                            target_rect.min.y + row as f32 * tile_size,
                        );
                        let cell = Rect::from_min_size(min, Vec2::splat(tile_size))
                            .intersect(clipped);
                        if cell.is_positive() {
                            painter.rect_filled(cell, 0.0, c2);
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_pixel_grid(
    painter: &Painter,
    viewport_rect: Rect,
    canvas_rect: Rect,
    content_w: u32,
    content_h: u32,
    zoom: f32,
) {
    if zoom < 6.0 {
        return;
    }

    // Pale, non-intrusive pixel grid that fades in smoothly
    let alpha = ((zoom - 6.0) / 14.0).clamp(0.05, 0.18);
    let stroke = Stroke::new(
        1.0,
        Color32::from_rgba_unmultiplied(130, 130, 130, (alpha * 255.0) as u8),
    );

    let clipped = canvas_rect.intersect(viewport_rect);
    if !clipped.is_positive() {
        return;
    }

    let min_px_x = ((clipped.min.x - canvas_rect.min.x) / zoom).floor().max(0.0) as u32;
    let max_px_x = ((clipped.max.x - canvas_rect.min.x) / zoom).ceil().min(content_w as f32) as u32;
    let min_px_y = ((clipped.min.y - canvas_rect.min.y) / zoom).floor().max(0.0) as u32;
    let max_px_y = ((clipped.max.y - canvas_rect.min.y) / zoom).ceil().min(content_h as f32) as u32;

    for x in min_px_x..=max_px_x {
        let sx = canvas_rect.min.x + x as f32 * zoom;
        painter.line_segment(
            [Pos2::new(sx, clipped.min.y), Pos2::new(sx, clipped.max.y)],
            stroke,
        );
    }

    for y in min_px_y..=max_px_y {
        let sy = canvas_rect.min.y + y as f32 * zoom;
        painter.line_segment(
            [Pos2::new(clipped.min.x, sy), Pos2::new(clipped.max.x, sy)],
            stroke,
        );
    }
}

pub fn draw_frame_boundary_grid(
    painter: &Painter,
    canvas_rect: Rect,
    metadata: &ProjectMetadata,
    active_frame: usize,
    view_mode: CanvasViewMode,
    zoom: f32,
) {
    if view_mode != CanvasViewMode::Atlas {
        painter.rect_stroke(
            canvas_rect,
            0.0,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(130, 130, 130, 100)),
            egui::StrokeKind::Outside,
        );
        return;
    }

    let fw = metadata.frame_width as f32 * zoom;
    let fh = metadata.frame_height as f32 * zoom;
    let cell_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(140, 140, 140, 60));

    for idx in 0..(metadata.frame_count as usize) {
        let (ox, oy) = metadata.frame_origin(idx);
        let min = Pos2::new(
            canvas_rect.min.x + ox as f32 * zoom,
            canvas_rect.min.y + oy as f32 * zoom,
        );
        let rect = Rect::from_min_size(min, Vec2::new(fw, fh));
        painter.rect_stroke(rect, 0.0, cell_stroke, egui::StrokeKind::Inside);
    }

    // Highlight active frame in Atlas Mode
    let (ax, ay) = metadata.frame_origin(active_frame);
    let active_min = Pos2::new(
        canvas_rect.min.x + ax as f32 * zoom,
        canvas_rect.min.y + ay as f32 * zoom,
    );
    let active_rect = Rect::from_min_size(active_min, Vec2::new(fw, fh));
    painter.rect_stroke(
        active_rect,
        0.0,
        Stroke::new(1.5, Color32::from_rgba_unmultiplied(225, 225, 225, 200)),
        egui::StrokeKind::Outside,
    );
}

pub fn draw_symmetry_axes(
    painter: &Painter,
    active_frame_rect: Rect,
    symmetry: SymmetryMode,
    norm_x: f32,
    norm_y: f32,
) {
    let stroke = Stroke::new(1.5, Color32::from_rgba_unmultiplied(210, 180, 90, 210));
    let handle_col = Color32::from_rgb(235, 200, 95);
    let cx = active_frame_rect.min.x + active_frame_rect.width() * norm_x;
    let cy = active_frame_rect.min.y + active_frame_rect.height() * norm_y;

    match symmetry {
        SymmetryMode::None => {}
        SymmetryMode::Horizontal => {
            painter.line_segment(
                [
                    Pos2::new(cx, active_frame_rect.min.y),
                    Pos2::new(cx, active_frame_rect.max.y),
                ],
                stroke,
            );
            painter.circle_filled(Pos2::new(cx, active_frame_rect.min.y), 4.5, handle_col);
        }
        SymmetryMode::Vertical => {
            painter.line_segment(
                [
                    Pos2::new(active_frame_rect.min.x, cy),
                    Pos2::new(active_frame_rect.max.x, cy),
                ],
                stroke,
            );
            painter.circle_filled(Pos2::new(active_frame_rect.min.x, cy), 4.5, handle_col);
        }
        SymmetryMode::Quad => {
            painter.line_segment(
                [
                    Pos2::new(cx, active_frame_rect.min.y),
                    Pos2::new(cx, active_frame_rect.max.y),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(active_frame_rect.min.x, cy),
                    Pos2::new(active_frame_rect.max.x, cy),
                ],
                stroke,
            );
            painter.circle_filled(Pos2::new(cx, active_frame_rect.min.y), 4.5, handle_col);
            painter.circle_filled(Pos2::new(active_frame_rect.min.x, cy), 4.5, handle_col);
        }
    }
}

pub fn draw_marching_ants_segment(painter: &Painter, p_start: Pos2, p_end: Pos2, phase: f32) {
    let (a, b) = if p_start.x + p_start.y <= p_end.x + p_end.y {
        (p_start, p_end)
    } else {
        (p_end, p_start)
    };
    let dir = b - a;
    let len = dir.length();
    if len < 0.25 {
        return;
    }
    let stroke_dark = Stroke::new(1.5, Color32::from_rgb(15, 15, 15));
    let stroke_light = Stroke::new(1.5, Color32::from_rgb(250, 250, 250));
    painter.line_segment([a, b], stroke_dark);

    let unit = dir / len;
    let period = 8.0_f32;
    let dash = 4.0_f32;
    let u0 = a.x + a.y - phase;
    let k_start = (u0 / period).floor() as i32;
    let k_end = ((u0 + len) / period).ceil() as i32;

    for k in k_start..=k_end {
        let t0 = (k as f32 * period - u0).clamp(0.0, len);
        let t1 = (k as f32 * period + dash - u0).clamp(0.0, len);
        if t1 > t0 + 0.15 {
            painter.line_segment([a + unit * t0, a + unit * t1], stroke_light);
        }
    }
}

pub fn draw_selection_overlay(
    painter: &Painter,
    active_frame_rect: Rect,
    selection: &SelectionMask,
    lasso_preview: Option<&[(i32, i32)]>,
    zoom: f32,
) {
    let has_lasso = lasso_preview.map(|pts| pts.len() >= 2).unwrap_or(false);
    if selection.is_empty() && !has_lasso {
        return;
    }

    let ctx = painter.ctx();
    ctx.request_repaint_after(std::time::Duration::from_millis(50));
    let phase = (ctx.input(|i| i.time) * 18.0).rem_euclid(8.0) as f32;

    for &(x, y) in &selection.pixels {
        let min = Pos2::new(
            active_frame_rect.min.x + x as f32 * zoom,
            active_frame_rect.min.y + y as f32 * zoom,
        );
        let r = Rect::from_min_size(min, Vec2::splat(zoom));
        if !selection.pixels.contains(&(x - 1, y)) {
            draw_marching_ants_segment(painter, r.left_top(), r.left_bottom(), phase);
        }
        if !selection.pixels.contains(&(x + 1, y)) {
            draw_marching_ants_segment(painter, r.right_top(), r.right_bottom(), phase);
        }
        if !selection.pixels.contains(&(x, y - 1)) {
            draw_marching_ants_segment(painter, r.left_top(), r.right_top(), phase);
        }
        if !selection.pixels.contains(&(x, y + 1)) {
            draw_marching_ants_segment(painter, r.left_bottom(), r.right_bottom(), phase);
        }
    }

    if let Some(pts) = lasso_preview {
        if pts.len() >= 2 {
            for pair in pts.windows(2) {
                let p0 = Pos2::new(
                    active_frame_rect.min.x + (pair[0].0 as f32 + 0.5) * zoom,
                    active_frame_rect.min.y + (pair[0].1 as f32 + 0.5) * zoom,
                );
                let p1 = Pos2::new(
                    active_frame_rect.min.x + (pair[1].0 as f32 + 0.5) * zoom,
                    active_frame_rect.min.y + (pair[1].1 as f32 + 0.5) * zoom,
                );
                draw_marching_ants_segment(painter, p0, p1, phase);
            }
        }
    }
}

pub fn draw_preview_pixels_and_cursor(
    painter: &Painter,
    active_frame_rect: Rect,
    preview_pixels: &[(i32, i32)],
    preview_color: [u8; 4],
    hover_local_px: Option<(i32, i32)>,
    brush_size: u32,
    zoom: f32,
) {
    if !preview_pixels.is_empty() {
        let [r, g, b, a] = preview_color;
        let fill_col = if a == 0 {
            Color32::from_rgba_unmultiplied(240, 90, 90, 120)
        } else {
            Color32::from_rgba_unmultiplied(r, g, b, a)
        };
        for &(px, py) in preview_pixels {
            let min = Pos2::new(
                active_frame_rect.min.x + px as f32 * zoom,
                active_frame_rect.min.y + py as f32 * zoom,
            );
            let cell = Rect::from_min_size(min, Vec2::splat(zoom));
            painter.rect_filled(cell, 0.0, fill_col);
        }
    }

    if let Some((hx, hy)) = hover_local_px {
        let size = brush_size.clamp(1, 64) as i32;
        let half_low = (size - 1) / 2;
        let min = Pos2::new(
            active_frame_rect.min.x + (hx - half_low) as f32 * zoom,
            active_frame_rect.min.y + (hy - half_low) as f32 * zoom,
        );
        let cursor_rect = Rect::from_min_size(min, Vec2::splat(size as f32 * zoom));
        painter.rect_stroke(
            cursor_rect,
            0.0,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(235, 235, 235, 190)),
            egui::StrokeKind::Outside,
        );
    }
}

pub fn draw_pivot_gizmos(
    painter: &Painter,
    active_frame_rect: Rect,
    metadata: &ProjectMetadata,
    zoom: f32,
) {
    if !metadata.pivot.show_gizmos {
        return;
    }

    let origin_pos = Pos2::new(
        active_frame_rect.min.x + metadata.pivot.origin_x as f32 * zoom,
        active_frame_rect.min.y + metadata.pivot.origin_y as f32 * zoom,
    );
    let arm = 6.0;
    let origin_stroke = Stroke::new(1.5, Color32::from_rgb(240, 190, 70));
    painter.line_segment(
        [
            Pos2::new(origin_pos.x - arm, origin_pos.y),
            Pos2::new(origin_pos.x + arm, origin_pos.y),
        ],
        origin_stroke,
    );
    painter.line_segment(
        [
            Pos2::new(origin_pos.x, origin_pos.y - arm),
            Pos2::new(origin_pos.x, origin_pos.y + arm),
        ],
        origin_stroke,
    );
    painter.circle_stroke(origin_pos, 3.5, origin_stroke);

    for pt in &metadata.pivot.action_points {
        let pos = Pos2::new(
            active_frame_rect.min.x + pt.x as f32 * zoom,
            active_frame_rect.min.y + pt.y as f32 * zoom,
        );
        let [r, g, b, a] = pt.color;
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);
        let stroke = Stroke::new(1.5, color);
        painter.line_segment(
            [Pos2::new(pos.x - 4.0, pos.y), Pos2::new(pos.x + 4.0, pos.y)],
            stroke,
        );
        painter.line_segment(
            [Pos2::new(pos.x, pos.y - 4.0), Pos2::new(pos.x + 4.0, pos.y)],
            stroke,
        );
        painter.text(
            Pos2::new(pos.x + 5.0, pos.y - 5.0),
            egui::Align2::LEFT_BOTTOM,
            &pt.name,
            FontId::proportional(11.0),
            color,
        );
    }
}

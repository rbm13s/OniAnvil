use crate::canvas::viewport::{BackgroundMode, CanvasViewMode, ViewportState};
use crate::model::metadata::ProjectMetadata;
use crate::tools::selection::SelectionMask;
use crate::tools::SymmetryMode;
use std::collections::HashSet;
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

pub fn draw_marching_ants_segment_colored(
    painter: &Painter,
    p_start: Pos2,
    p_end: Pos2,
    phase: f32,
    stroke_dark: Stroke,
    stroke_light: Stroke,
) {
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

#[allow(dead_code)]
pub fn draw_marching_ants_segment(painter: &Painter, p_start: Pos2, p_end: Pos2, phase: f32) {
    let stroke_dark = Stroke::new(1.5, Color32::from_rgb(15, 15, 15));
    let stroke_light = Stroke::new(1.5, Color32::from_rgb(250, 250, 250));
    draw_marching_ants_segment_colored(painter, p_start, p_end, phase, stroke_dark, stroke_light);
}

fn draw_mask_marching_ants(
    painter: &Painter,
    active_frame_rect: Rect,
    pixels: &HashSet<(i32, i32)>,
    zoom: f32,
    phase: f32,
    stroke_dark: Stroke,
    stroke_light: Stroke,
) {
    for &(x, y) in pixels {
        let min = Pos2::new(
            active_frame_rect.min.x + x as f32 * zoom,
            active_frame_rect.min.y + y as f32 * zoom,
        );
        let r = Rect::from_min_size(min, Vec2::splat(zoom));
        if !pixels.contains(&(x - 1, y)) {
            draw_marching_ants_segment_colored(painter, r.left_top(), r.left_bottom(), phase, stroke_dark, stroke_light);
        }
        if !pixels.contains(&(x + 1, y)) {
            draw_marching_ants_segment_colored(painter, r.right_top(), r.right_bottom(), phase, stroke_dark, stroke_light);
        }
        if !pixels.contains(&(x, y - 1)) {
            draw_marching_ants_segment_colored(painter, r.left_top(), r.right_top(), phase, stroke_dark, stroke_light);
        }
        if !pixels.contains(&(x, y + 1)) {
            draw_marching_ants_segment_colored(painter, r.left_bottom(), r.right_bottom(), phase, stroke_dark, stroke_light);
        }
    }
}

pub fn draw_selection_overlay(
    painter: &Painter,
    active_frame_rect: Rect,
    selection: &SelectionMask,
    intersect_candidate: Option<&SelectionMask>,
    zoom: f32,
) {
    let has_main = !selection.is_empty();
    let has_cand = intersect_candidate.map(|c| !c.is_empty()).unwrap_or(false);
    if !has_main && !has_cand {
        return;
    }

    let ctx = painter.ctx();
    ctx.request_repaint_after(std::time::Duration::from_millis(50));
    let phase = (ctx.input(|i| i.time) * 18.0).rem_euclid(8.0) as f32;

    if has_main {
        let stroke_dark = Stroke::new(1.5, Color32::from_rgb(15, 15, 15));
        let stroke_light = Stroke::new(1.5, Color32::from_rgb(250, 250, 250));
        draw_mask_marching_ants(
            painter,
            active_frame_rect,
            &selection.pixels,
            zoom,
            phase,
            stroke_dark,
            stroke_light,
        );
    }

    if let Some(cand) = intersect_candidate {
        if !cand.is_empty() {
            let stroke_dark = Stroke::new(1.5, Color32::from_rgb(10, 35, 60));
            let stroke_light = Stroke::new(1.5, Color32::from_rgb(0, 230, 255));
            draw_mask_marching_ants(
                painter,
                active_frame_rect,
                &cand.pixels,
                zoom,
                phase,
                stroke_dark,
                stroke_light,
            );
        }
    }
}
pub fn brush_cursor_geometry(
    origin: Pos2,
    hover_px: (i32, i32),
    brush_size: u32,
    zoom: f32,
) -> (Rect, Pos2, f32) {
    let size = brush_size.clamp(1, 64) as i32;
    let half_low = (size - 1) / 2;
    let min = Pos2::new(
        origin.x + (hover_px.0 - half_low) as f32 * zoom,
        origin.y + (hover_px.1 - half_low) as f32 * zoom,
    );
    let rect = Rect::from_min_size(min, Vec2::splat(size as f32 * zoom));
    let center = rect.center();
    let radius = (size as f32 * 0.5) * zoom;
    (rect, center, radius)
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

    if let Some(hover_px) = hover_local_px {
        let (cursor_rect, center, radius) =
            brush_cursor_geometry(active_frame_rect.min, hover_px, brush_size, zoom);
        if brush_size > 1 {
            // High-contrast dual stroke for round brush cursor:
            // Dark outer shadow ring ensures visibility against bright canvases,
            // while bright inner ring ensures visibility against dark canvases.
            painter.circle_stroke(
                center,
                radius + 0.5,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(20, 20, 20, 160)),
            );
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(240, 240, 240, 220)),
            );
        } else {
            painter.rect_stroke(
                cursor_rect,
                0.0,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(235, 235, 235, 190)),
                egui::StrokeKind::Outside,
            );
        }
    }
}

pub fn draw_pivot_gizmos(
    painter: &Painter,
    active_frame_rect: Rect,
    metadata: &ProjectMetadata,
    active_frame: usize,
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

    let onion = &metadata.onion_skin;
    let total_frames = metadata.frame_count as usize;

    for (ap_idx, pt) in metadata.pivot.action_points.iter().enumerate() {
        let (cur_x, cur_y) = metadata
            .get_frame_action_point(active_frame, ap_idx)
            .unwrap_or((pt.x, pt.y));
        let cur_pos = Pos2::new(
            active_frame_rect.min.x + cur_x as f32 * zoom,
            active_frame_rect.min.y + cur_y as f32 * zoom,
        );
        let [r, g, b, a] = pt.color;

        // Draw onion skin ghost action points and trajectory lines if Onion Skin is enabled
        if onion.enabled && total_frames > 1 {
            let base_opacity = onion.opacity.clamp(0.15, 0.95);

            // Previous frames (red tint if color_tint is enabled, otherwise point's own color)
            let mut prev_chain_pos = cur_pos;
            for step in 1..=(onion.prev_frames as usize) {
                if let Some(prev_f) = active_frame.checked_sub(step) {
                    let (px, py) = metadata
                        .get_frame_action_point(prev_f, ap_idx)
                        .unwrap_or((pt.x, pt.y));
                    let ghost_pos = Pos2::new(
                        active_frame_rect.min.x + px as f32 * zoom,
                        active_frame_rect.min.y + py as f32 * zoom,
                    );
                    let falloff = (base_opacity / (step as f32)).clamp(0.12, 0.85);
                    let alpha = (falloff * 255.0) as u8;
                    let ghost_col = if onion.color_tint {
                        Color32::from_rgba_unmultiplied(255, 95, 95, alpha)
                    } else {
                        Color32::from_rgba_unmultiplied(r, g, b, alpha)
                    };
                    let line_col = Color32::from_rgba_unmultiplied(
                        ghost_col.r(),
                        ghost_col.g(),
                        ghost_col.b(),
                        alpha / 2,
                    );
                    if (ghost_pos - prev_chain_pos).length_sq() > 1.0 {
                        painter.line_segment([prev_chain_pos, ghost_pos], Stroke::new(1.0, line_col));
                    }
                    let g_stroke = Stroke::new(1.2, ghost_col);
                    painter.line_segment(
                        [
                            Pos2::new(ghost_pos.x - 3.0, ghost_pos.y),
                            Pos2::new(ghost_pos.x + 3.0, ghost_pos.y),
                        ],
                        g_stroke,
                    );
                    painter.line_segment(
                        [
                            Pos2::new(ghost_pos.x, ghost_pos.y - 3.0),
                            Pos2::new(ghost_pos.x, ghost_pos.y + 3.0),
                        ],
                        g_stroke,
                    );
                    prev_chain_pos = ghost_pos;
                }
            }

            // Next frames (green tint if color_tint is enabled, otherwise point's own color)
            let mut next_chain_pos = cur_pos;
            for step in 1..=(onion.next_frames as usize) {
                let next_f = active_frame + step;
                if next_f < total_frames {
                    let (nx, ny) = metadata
                        .get_frame_action_point(next_f, ap_idx)
                        .unwrap_or((pt.x, pt.y));
                    let ghost_pos = Pos2::new(
                        active_frame_rect.min.x + nx as f32 * zoom,
                        active_frame_rect.min.y + ny as f32 * zoom,
                    );
                    let falloff = (base_opacity / (step as f32)).clamp(0.12, 0.85);
                    let alpha = (falloff * 255.0) as u8;
                    let ghost_col = if onion.color_tint {
                        Color32::from_rgba_unmultiplied(95, 240, 145, alpha)
                    } else {
                        Color32::from_rgba_unmultiplied(r, g, b, alpha)
                    };
                    let line_col = Color32::from_rgba_unmultiplied(
                        ghost_col.r(),
                        ghost_col.g(),
                        ghost_col.b(),
                        alpha / 2,
                    );
                    if (ghost_pos - next_chain_pos).length_sq() > 1.0 {
                        painter.line_segment([next_chain_pos, ghost_pos], Stroke::new(1.0, line_col));
                    }
                    let g_stroke = Stroke::new(1.2, ghost_col);
                    painter.line_segment(
                        [
                            Pos2::new(ghost_pos.x - 3.0, ghost_pos.y),
                            Pos2::new(ghost_pos.x + 3.0, ghost_pos.y),
                        ],
                        g_stroke,
                    );
                    painter.line_segment(
                        [
                            Pos2::new(ghost_pos.x, ghost_pos.y - 3.0),
                            Pos2::new(ghost_pos.x, ghost_pos.y + 3.0),
                        ],
                        g_stroke,
                    );
                    next_chain_pos = ghost_pos;
                }
            }
        }

        let color = Color32::from_rgba_unmultiplied(r, g, b, a);
        let stroke = Stroke::new(1.5, color);
        painter.line_segment(
            [
                Pos2::new(cur_pos.x - 4.0, cur_pos.y),
                Pos2::new(cur_pos.x + 4.0, cur_pos.y),
            ],
            stroke,
        );
        painter.line_segment(
            [
                Pos2::new(cur_pos.x, cur_pos.y - 4.0),
                Pos2::new(cur_pos.x, cur_pos.y + 4.0),
            ],
            stroke,
        );
        painter.text(
            Pos2::new(cur_pos.x + 5.0, cur_pos.y - 5.0),
            egui::Align2::LEFT_BOTTOM,
            &pt.name,
            FontId::proportional(11.0),
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_cursor_geometry_size_1() {
        let (rect, center, radius) = brush_cursor_geometry(Pos2::new(10.0, 20.0), (3, 5), 1, 16.0);
        // Size 1: pixel rect is 16x16
        assert_eq!(rect.min, Pos2::new(10.0 + 3.0 * 16.0, 20.0 + 5.0 * 16.0));
        assert_eq!(rect.width(), 16.0);
        assert_eq!(rect.height(), 16.0);
        assert_eq!(center, Pos2::new(rect.min.x + 8.0, rect.min.y + 8.0));
        assert_eq!(radius, 8.0);
    }

    #[test]
    fn test_brush_cursor_geometry_size_even() {
        // Size 4: half_low = 1, half_high = 2. Min pixel is (3 - 1, 5 - 1) = (2, 4)
        let (rect, center, radius) = brush_cursor_geometry(Pos2::ZERO, (3, 5), 4, 10.0);
        assert_eq!(rect.min, Pos2::new(20.0, 40.0));
        assert_eq!(rect.width(), 40.0);
        assert_eq!(rect.height(), 40.0);
        assert_eq!(center, Pos2::new(40.0, 60.0));
        assert_eq!(radius, 20.0);
    }

    #[test]
    fn test_brush_cursor_geometry_size_odd() {
        // Size 5: half_low = 2, half_high = 2. Min pixel is (3 - 2, 5 - 2) = (1, 3)
        let (rect, center, radius) = brush_cursor_geometry(Pos2::ZERO, (3, 5), 5, 10.0);
        assert_eq!(rect.min, Pos2::new(10.0, 30.0));
        assert_eq!(rect.width(), 50.0);
        assert_eq!(rect.height(), 50.0);
        assert_eq!(center, Pos2::new(35.0, 55.0));
        assert_eq!(radius, 25.0);
    }
}

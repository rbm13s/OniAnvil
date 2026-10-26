use crate::canvas::overlays::{
    draw_canvas_background, draw_frame_boundary_grid, draw_pivot_gizmos, draw_pixel_grid,
    draw_preview_pixels_and_cursor, draw_selection_overlay, draw_symmetry_axes,
};
use crate::canvas::viewport::{CanvasViewMode, ViewportState};
use crate::history::command::{HistorySnapshot, HistoryStack};
use crate::model::project::Project;
use crate::model::sketch::{nearest_palette_color, QuickPeekMode, SketchLayerMode};
use crate::tools::fill::flood_fill_region;
use crate::tools::selection::SelectionMask;
use crate::tools::shapes::{
    rasterize_ellipse, rasterize_line, rasterize_quadratic_bezier, rasterize_rectangle,
};
use crate::tools::stroke::{
    bresenham_line, dither_passes, expand_stamp_points, filter_pixel_perfect,
};
use crate::tools::transforms::nudge_selection_region;
use crate::tools::{ToolKind, ToolSettings};
use egui::epaint::{Mesh, Vertex};
use egui::{
    Color32, Context, Painter, PointerButton, Pos2, Rect, Sense, TextureHandle, TextureOptions, Ui,
    Vec2,
};
use image::{Rgba, RgbaImage};

struct ActiveStroke {
    tool: ToolKind,
    button: PointerButton,
    frame_index: usize,
    start_local: (i32, i32),
    last_local: (i32, i32),
    raw_points: Vec<(i32, i32)>,
    base_atlas: RgbaImage,
    moving_selection: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveState {
    /// After 1st click: start point `p0` is placed; moving mouse previews straight line to cursor; 2nd click sets `p2`.
    WaitingForEnd {
        p0: (i32, i32),
        frame_index: usize,
        button: PointerButton,
    },
    /// After 2nd click: start `p0` and end `p2` are fixed; moving mouse adjusts control point `p1`; 3rd click commits curve.
    WaitingForControl {
        p0: (i32, i32),
        p2: (i32, i32),
        frame_index: usize,
        button: PointerButton,
    },
}

#[derive(Default)]
pub struct CanvasRenderer {
    atlas_texture: Option<TextureHandle>,
    layer_textures: Vec<TextureHandle>,
    sketch_texture: Option<TextureHandle>,
    sketch_bilinear: bool,
    active_stroke: Option<ActiveStroke>,
    pub curve_state: Option<CurveState>,
    dragging_sym_x: bool,
    dragging_sym_y: bool,
    pub selection: SelectionMask,
    pub active_pivot_target: Option<usize>,
}

impl CanvasRenderer {
    pub fn ensure_texture(&mut self, ctx: &Context, project: &mut Project) -> &TextureHandle {
        if self.atlas_texture.is_none()
            || project.canvas_dirty
            || self.layer_textures.len() != project.metadata.layers.len()
        {
            project.ensure_layers_initialized();
            project.sync_linked_frames(project.active_frame);
            project.sync_active_layer();
            let color_image = project.to_color_image();
            match &mut self.atlas_texture {
                Some(handle) => handle.set(color_image, TextureOptions::NEAREST),
                None => {
                    self.atlas_texture = Some(ctx.load_texture(
                        "onianvil_atlas_texture",
                        color_image,
                        TextureOptions::NEAREST,
                    ));
                }
            }

            let layer_count = project.metadata.layers.len();
            if self.layer_textures.len() > layer_count {
                self.layer_textures.truncate(layer_count);
            }
            for (idx, layer) in project.metadata.layers.iter().enumerate() {
                let l_img = layer.layer_rgba.as_ref().unwrap_or(&project.atlas_rgba);
                let l_size = [l_img.width() as usize, l_img.height() as usize];
                let l_color = egui::ColorImage::from_rgba_unmultiplied(l_size, l_img.as_raw());
                if idx < self.layer_textures.len() {
                    self.layer_textures[idx].set(l_color, TextureOptions::NEAREST);
                } else {
                    let handle = ctx.load_texture(
                        format!("onianvil_layer_texture_{}", idx),
                        l_color,
                        TextureOptions::NEAREST,
                    );
                    self.layer_textures.push(handle);
                }
            }

            project.canvas_dirty = false;
        }

        let want_bilinear = project.metadata.sketch.bilinear_filter;
        if project.sketch_dirty || (self.sketch_texture.is_some() && self.sketch_bilinear != want_bilinear) {
            let tex_opts = if want_bilinear {
                TextureOptions::LINEAR
            } else {
                TextureOptions::NEAREST
            };
            if let Some(sketch_color_img) = project.sketch_to_color_image() {
                match &mut self.sketch_texture {
                    Some(handle) => handle.set(sketch_color_img, tex_opts),
                    None => {
                        self.sketch_texture = Some(ctx.load_texture(
                            "onianvil_sketch_reference_texture",
                            sketch_color_img,
                            tex_opts,
                        ));
                    }
                }
            } else {
                self.sketch_texture = None;
            }
            self.sketch_bilinear = want_bilinear;
            project.sketch_dirty = false;
        }

        self.atlas_texture.as_ref().unwrap()
    }

    pub fn atlas_texture(&self) -> Option<&TextureHandle> {
        self.atlas_texture.as_ref()
    }

    pub fn layer_texture(&self, layer_index: usize) -> Option<&TextureHandle> {
        self.layer_textures.get(layer_index)
    }

    pub fn frame_uv(project: &Project, frame_index: usize) -> Rect {
        let (atlas_w, atlas_h) = project.metadata.atlas_dimensions();
        let aw = (atlas_w.max(1)) as f32;
        let ah = (atlas_h.max(1)) as f32;
        let (ox, oy) = project.metadata.frame_origin(frame_index);
        let fw = project.metadata.frame_width as f32;
        let fh = project.metadata.frame_height as f32;

        Rect::from_min_max(
            Pos2::new(ox as f32 / aw, oy as f32 / ah),
            Pos2::new((ox as f32 + fw) / aw, (oy as f32 + fh) / ah),
        )
    }

    fn draw_sketch_mesh(
        &self,
        painter: &Painter,
        active_frame_rect: Rect,
        project: &Project,
        zoom: f32,
        opacity_override: Option<f32>,
    ) {
        let (sketch_tex, sketch_img) = match (&self.sketch_texture, &project.sketch_rgba) {
            (Some(tex), Some(img)) => (tex, img),
            _ => return,
        };

        let alpha = opacity_override
            .unwrap_or(project.metadata.sketch.opacity)
            .clamp(0.0, 1.0);
        if alpha <= 0.01 {
            return;
        }

        let quad_f = project.metadata.sketch.transformed_frame_quad(
            project.metadata.frame_width,
            project.metadata.frame_height,
            sketch_img.width(),
            sketch_img.height(),
        );

        let mut screen_pts = [Pos2::ZERO; 4];
        for (i, (fx, fy)) in quad_f.iter().copied().enumerate() {
            screen_pts[i] = Pos2::new(
                active_frame_rect.min.x + fx * zoom,
                active_frame_rect.min.y + fy * zoom,
            );
        }

        let (u0, u1) = if project.metadata.sketch.flip_h {
            (1.0, 0.0)
        } else {
            (0.0, 1.0)
        };
        let (v0, v1) = if project.metadata.sketch.flip_v {
            (1.0, 0.0)
        } else {
            (0.0, 1.0)
        };

        let uvs = [
            Pos2::new(u0, v0),
            Pos2::new(u1, v0),
            Pos2::new(u1, v1),
            Pos2::new(u0, v1),
        ];

        let col = Color32::from_rgba_unmultiplied(255, 255, 255, (alpha * 255.0) as u8);
        let mut mesh = Mesh::with_texture(sketch_tex.id());
        for i in 0..4 {
            mesh.vertices.push(Vertex {
                pos: screen_pts[i],
                uv: uvs[i],
                color: col,
            });
        }
        mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        painter.add(egui::Shape::mesh(mesh));
    }

    fn draw_onion_skin_layers(
        &self,
        painter: &Painter,
        active_frame_rect: Rect,
        project: &Project,
    ) {
        let onion = &project.metadata.onion_skin;
        if !onion.enabled || project.metadata.frame_count <= 1 {
            return;
        }
        let tex = match &self.atlas_texture {
            Some(t) => t,
            None => return,
        };

        let cur = project.active_frame;
        let total = project.metadata.frame_count as usize;
        let base_opacity = onion.opacity.clamp(0.05, 0.95);

        // Draw previous frames (furthest first)
        for step in (1..=onion.prev_frames as usize).rev() {
            if let Some(prev_idx) = cur.checked_sub(step) {
                let falloff = base_opacity / (step as f32);
                let a = (falloff * 255.0).clamp(10.0, 240.0) as u8;
                let tint = if onion.color_tint {
                    Color32::from_rgba_unmultiplied(255, 95, 95, a)
                } else {
                    Color32::from_rgba_unmultiplied(255, 255, 255, a)
                };
                let uv = Self::frame_uv(project, prev_idx);
                painter.image(tex.id(), active_frame_rect, uv, tint);
            }
        }

        // Draw next frames (furthest first)
        for step in (1..=onion.next_frames as usize).rev() {
            let next_idx = cur + step;
            if next_idx < total {
                let falloff = base_opacity / (step as f32);
                let a = (falloff * 255.0).clamp(10.0, 240.0) as u8;
                let tint = if onion.color_tint {
                    Color32::from_rgba_unmultiplied(95, 240, 145, a)
                } else {
                    Color32::from_rgba_unmultiplied(255, 255, 255, a)
                };
                let uv = Self::frame_uv(project, next_idx);
                painter.image(tex.id(), active_frame_rect, uv, tint);
            }
        }
    }

    /// Maps a raw content pixel `(cx, cy)` to `(frame_index, local_x, local_y)` if valid.
    fn resolve_target_frame_and_local(
        project: &Project,
        viewport: &ViewportState,
        cx: i32,
        cy: i32,
    ) -> Option<(usize, i32, i32)> {
        let fw = project.metadata.frame_width.max(1) as i32;
        let fh = project.metadata.frame_height.max(1) as i32;

        match viewport.view_mode {
            CanvasViewMode::Frame => {
                if viewport.tiled_preview_3x3 {
                    if cx >= -fw && cx < fw * 2 && cy >= -fh && cy < fh * 2 {
                        return Some((
                            project.active_frame,
                            cx.rem_euclid(fw),
                            cy.rem_euclid(fh),
                        ));
                    }
                } else if cx >= 0 && cx < fw && cy >= 0 && cy < fh {
                    return Some((project.active_frame, cx, cy));
                }
                None
            }
            CanvasViewMode::Atlas => {
                let (aw, ah) = project.metadata.atlas_dimensions();
                if cx < 0 || cy < 0 || cx >= aw as i32 || cy >= ah as i32 {
                    return None;
                }
                let col = (cx / fw) as u32;
                let row = (cy / fh) as u32;
                let idx = (row * project.metadata.columns.max(1) + col) as usize;
                if idx < project.metadata.frame_count as usize {
                    let lx = cx % fw;
                    let ly = cy % fh;
                    Some((idx, lx, ly))
                } else {
                    None
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_point_stamp(
        atlas_rgba: &mut RgbaImage,
        project_meta: &crate::model::metadata::ProjectMetadata,
        selection: &SelectionMask,
        viewport: &ViewportState,
        tools: &ToolSettings,
        tool: ToolKind,
        button: PointerButton,
        target_frame: usize,
        center_local: (i32, i32),
    ) {
        let fw = project_meta.frame_width.max(1);
        let fh = project_meta.frame_height.max(1);

        let frames_to_edit: Vec<usize> = if tools.batch_mode {
            (0..project_meta.frame_count as usize).collect()
        } else {
            vec![target_frame]
        };

        if tool == ToolKind::Pencil {
            if let Some(custom) = &tools.custom_brush {
                let off_x = center_local.0 - (custom.width as i32 / 2);
                let off_y = center_local.1 - (custom.height as i32 / 2);
                for f_idx in frames_to_edit {
                    let (ox, oy) = project_meta.frame_origin(f_idx);
                    for &((rx, ry), color) in &custom.pixels {
                        if color[3] == 0 {
                            continue;
                        }
                        let lx = off_x + rx;
                        let ly = off_y + ry;
                        if lx < 0 || ly < 0 || (lx as u32) >= fw || (ly as u32) >= fh {
                            continue;
                        }
                        if !selection.contains(lx, ly) {
                            continue;
                        }
                        let ax = ox + lx as u32;
                        let ay = oy + ly as u32;
                        if ax >= atlas_rgba.width() || ay >= atlas_rgba.height() {
                            continue;
                        }
                        let existing = *atlas_rgba.get_pixel(ax, ay);
                        if tools.lock_alpha && existing[3] == 0 {
                            continue;
                        }
                        if button == PointerButton::Secondary {
                            atlas_rgba.put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
                        } else {
                            atlas_rgba.put_pixel(ax, ay, Rgba(color));
                        }
                    }
                }
                return;
            }
        }

        let brush_size = tool.effective_brush_size(tools.brush_size);

        let stamped = expand_stamp_points(
            center_local,
            brush_size,
            tools.symmetry,
            viewport.symmetry_norm_x,
            viewport.symmetry_norm_y,
            fw,
            fh,
        );

        let base_draw_color = match (tool, button) {
            (ToolKind::Eraser, _) => [0, 0, 0, 0],
            (_, PointerButton::Secondary) => tools.bg_color,
            _ => tools.fg_color,
        };

        for f_idx in frames_to_edit {
            let (ox, oy) = project_meta.frame_origin(f_idx);
            for &(lx, ly) in &stamped {
                if lx < 0 || ly < 0 || (lx as u32) >= fw || (ly as u32) >= fh {
                    continue;
                }
                if !selection.contains(lx, ly) {
                    continue;
                }
                if tool == ToolKind::DitherBrush && !dither_passes(tools.dither_pattern, lx, ly) {
                    continue;
                }

                let ax = ox + lx as u32;
                let ay = oy + ly as u32;
                if ax >= atlas_rgba.width() || ay >= atlas_rgba.height() {
                    continue;
                }

                let existing = *atlas_rgba.get_pixel(ax, ay);
                if tools.lock_alpha && existing[3] == 0 {
                    continue;
                }

                if tool == ToolKind::Shading {
                    if existing[3] == 0 {
                        continue;
                    }
                    let cur_rgba = [existing[0], existing[1], existing[2], existing[3]];
                    if let Some(pos) = project_meta
                        .palette
                        .colors
                        .iter()
                        .position(|c| *c == cur_rgba)
                    {
                        let new_pos = if button == PointerButton::Secondary {
                            (pos + 1).min(project_meta.palette.colors.len().saturating_sub(1))
                        } else {
                            pos.saturating_sub(1)
                        };
                        let shaded = project_meta.palette.colors[new_pos];
                        atlas_rgba.put_pixel(ax, ay, Rgba(shaded));
                    }
                } else {
                    atlas_rgba.put_pixel(ax, ay, Rgba(base_draw_color));
                }
            }
        }
    }

    fn compute_shape_preview_points(
        project_meta: &crate::model::metadata::ProjectMetadata,
        viewport: &ViewportState,
        tools: &ToolSettings,
        tool: ToolKind,
        start_local: (i32, i32),
        end_local: (i32, i32),
        shift_down: bool,
    ) -> Vec<(i32, i32)> {
        let base_pts = match tool {
            ToolKind::Line | ToolKind::Curve => rasterize_line(start_local, end_local, shift_down),
            ToolKind::Rectangle => {
                rasterize_rectangle(start_local, end_local, tools.filled_shape, shift_down)
            }
            ToolKind::Ellipse => {
                rasterize_ellipse(start_local, end_local, tools.filled_shape, shift_down)
            }
            _ => Vec::new(),
        };

        let fw = project_meta.frame_width.max(1);
        let fh = project_meta.frame_height.max(1);
        let mut expanded = Vec::new();

        let brush_size = tool.effective_brush_size(tools.brush_size);

        for pt in base_pts {
            let stamp = expand_stamp_points(
                pt,
                brush_size,
                tools.symmetry,
                viewport.symmetry_norm_x,
                viewport.symmetry_norm_y,
                fw,
                fh,
            );
            for (lx, ly) in stamp {
                if lx >= 0 && ly >= 0 && (lx as u32) < fw && (ly as u32) < fh {
                    expanded.push((lx, ly));
                }
            }
        }
        expanded
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &mut Project,
        viewport: &mut ViewportState,
        tools: &mut ToolSettings,
        history: &mut HistoryStack,
    ) {
        let ctx = ui.ctx().clone();
        let avail_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(avail_rect, Sense::click_and_drag());

        let (content_w, content_h) = match viewport.view_mode {
            CanvasViewMode::Frame => (project.metadata.frame_width, project.metadata.frame_height),
            CanvasViewMode::Atlas => project.metadata.atlas_dimensions(),
        };

        if viewport.needs_fit_to_view && avail_rect.width() > 32.0 && avail_rect.height() > 32.0 {
            viewport.fit_to_rect(avail_rect, content_w, content_h);
        }

        // Smooth cursor-centered Mouse Wheel Zoom
        if response.hovered() {
            let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_y.abs() > 0.05 {
                let anchor_pos = response.hover_pos().unwrap_or_else(|| avail_rect.center());
                viewport.zoom_by_wheel_at_cursor(avail_rect, anchor_pos, scroll_y);
            }
        }

        let (space_down, alt_down, shift_down) = ui.input(|i| {
            (
                i.key_down(egui::Key::Space),
                i.modifiers.alt,
                i.modifiers.shift,
            )
        });

        let effective_tool = if alt_down {
            ToolKind::Eyedropper
        } else {
            tools.active_tool
        };

        let middle_drag = response.dragged_by(PointerButton::Middle);
        let space_left_drag = space_down && response.dragged_by(PointerButton::Primary);
        let quick_peek_active = space_down && !space_left_drag && !middle_drag;

        if middle_drag || space_left_drag {
            viewport.pan_offset += response.drag_delta();
        }

        let canvas_rect = viewport.canvas_screen_rect(avail_rect, content_w, content_h);
        let active_frame_rect_for_input = match viewport.view_mode {
            CanvasViewMode::Frame => canvas_rect,
            CanvasViewMode::Atlas => {
                let (ox, oy) = project.metadata.frame_origin(project.active_frame);
                let min = Pos2::new(
                    canvas_rect.min.x + ox as f32 * viewport.zoom,
                    canvas_rect.min.y + oy as f32 * viewport.zoom,
                );
                let size = Vec2::new(
                    project.metadata.frame_width as f32 * viewport.zoom,
                    project.metadata.frame_height as f32 * viewport.zoom,
                );
                Rect::from_min_size(min, size)
            }
        };

        // Check draggable Symmetry Axis handles
        if tools.symmetry != crate::tools::SymmetryMode::None {
            let primary_down = ui.input(|i| i.pointer.button_down(PointerButton::Primary));
            if !primary_down {
                self.dragging_sym_x = false;
                self.dragging_sym_y = false;
            } else if let Some(pos) = response.hover_pos() {
                if !self.dragging_sym_x && !self.dragging_sym_y && self.active_stroke.is_none() {
                    let sym_cx = active_frame_rect_for_input.min.x
                        + active_frame_rect_for_input.width() * viewport.symmetry_norm_x;
                    let sym_cy = active_frame_rect_for_input.min.y
                        + active_frame_rect_for_input.height() * viewport.symmetry_norm_y;
                    let top_handle = Pos2::new(sym_cx, active_frame_rect_for_input.min.y);
                    let left_handle = Pos2::new(active_frame_rect_for_input.min.x, sym_cy);

                    if matches!(
                        tools.symmetry,
                        crate::tools::SymmetryMode::Horizontal | crate::tools::SymmetryMode::Quad
                    ) && pos.distance(top_handle) <= 9.0
                    {
                        self.dragging_sym_x = true;
                    } else if matches!(
                        tools.symmetry,
                        crate::tools::SymmetryMode::Vertical | crate::tools::SymmetryMode::Quad
                    ) && pos.distance(left_handle) <= 9.0
                    {
                        self.dragging_sym_y = true;
                    }
                }

                if self.dragging_sym_x {
                    let raw_nx = ((pos.x - active_frame_rect_for_input.min.x)
                        / active_frame_rect_for_input.width().max(1.0))
                    .clamp(0.05, 0.95);
                    let steps = (project.metadata.frame_width.max(1) * 2) as f32;
                    viewport.symmetry_norm_x = (raw_nx * steps).round() / steps;
                }
                if self.dragging_sym_y {
                    let raw_ny = ((pos.y - active_frame_rect_for_input.min.y)
                        / active_frame_rect_for_input.height().max(1.0))
                    .clamp(0.05, 0.95);
                    let steps = (project.metadata.frame_height.max(1) * 2) as f32;
                    viewport.symmetry_norm_y = (raw_ny * steps).round() / steps;
                }
            }
        } else {
            self.dragging_sym_x = false;
            self.dragging_sym_y = false;
        }

        let mut hover_resolved: Option<(usize, i32, i32)> = None;
        if let Some(hover_pos) = response.hover_pos() {
            let (cx, cy) = viewport.screen_to_pixel(canvas_rect, hover_pos);
            if let Some((f_idx, lx, ly)) =
                Self::resolve_target_frame_and_local(project, viewport, cx, cy)
            {
                viewport.hover_pixel = Some((lx, ly));
                hover_resolved = Some((f_idx, lx, ly));
            } else {
                viewport.hover_pixel = None;
            }
        } else {
            viewport.hover_pixel = None;
        }

        // Handle drawing / tool interactions when not panning with Space
        let layer_locked = project.active_layer_locked();
        if !space_down && !middle_drag && !self.dragging_sym_x && !self.dragging_sym_y {
            if effective_tool != ToolKind::Curve
                || ui.input(|i| i.key_pressed(egui::Key::Escape))
            {
                self.curve_state = None;
            }
            let primary_pressed = ui.input(|i| i.pointer.button_pressed(PointerButton::Primary));
            let secondary_pressed =
                ui.input(|i| i.pointer.button_pressed(PointerButton::Secondary));
            let pressed_btn = if primary_pressed {
                Some(PointerButton::Primary)
            } else if secondary_pressed {
                Some(PointerButton::Secondary)
            } else {
                None
            };

            let primary_down = ui.input(|i| i.pointer.button_down(PointerButton::Primary));
            let secondary_down = ui.input(|i| i.pointer.button_down(PointerButton::Secondary));
            let active_btn = if primary_down {
                Some(PointerButton::Primary)
            } else if secondary_down {
                Some(PointerButton::Secondary)
            } else {
                None
            };

            if response.hovered() && effective_tool == ToolKind::Curve {
                if let (Some(btn), Some((f_idx, lx, ly))) = (pressed_btn, hover_resolved) {
                    if !layer_locked {
                        match self.curve_state.take() {
                            None => {
                                project.active_frame = f_idx;
                                self.curve_state = Some(CurveState::WaitingForEnd {
                                    p0: (lx, ly),
                                    frame_index: f_idx,
                                    button: btn,
                                });
                            }
                            Some(CurveState::WaitingForEnd {
                                p0,
                                frame_index,
                                button,
                            }) => {
                                let p2 = crate::tools::shapes::snap_line_endpoint(
                                    p0,
                                    (lx, ly),
                                    shift_down,
                                );
                                self.curve_state = Some(CurveState::WaitingForControl {
                                    p0,
                                    p2,
                                    frame_index,
                                    button,
                                });
                            }
                            Some(CurveState::WaitingForControl {
                                p0,
                                p2,
                                frame_index,
                                button,
                            }) => {
                                project.sync_active_layer();
                                history.push(HistorySnapshot {
                                    atlas_rgba: project.atlas_rgba.clone(),
                                    metadata: project.metadata.clone(),
                                    active_frame: project.active_frame,
                                });
                                let raw_pts = rasterize_quadratic_bezier(p0, (lx, ly), p2);
                                let base_pts = if tools.pixel_perfect {
                                    filter_pixel_perfect(&raw_pts)
                                } else {
                                    raw_pts
                                };
                                for pt in base_pts {
                                    Self::apply_point_stamp(
                                        &mut project.atlas_rgba,
                                        &project.metadata,
                                        &self.selection,
                                        viewport,
                                        tools,
                                        ToolKind::Curve,
                                        button,
                                        frame_index,
                                        pt,
                                    );
                                }
                                project.canvas_dirty = true;
                                project.has_unsaved_changes = true;
                                self.curve_state = None;
                            }
                        }
                    }
                }
            } else if response.hovered() {
                if let (Some(btn), Some((f_idx, lx, ly))) = (active_btn, hover_resolved) {
                    if self.active_stroke.is_none() {
                        project.active_frame = f_idx;
                        match effective_tool {
                            ToolKind::Eyedropper => {
                                let (ox, oy) = project.metadata.frame_origin(f_idx);
                                let ax = ox + lx as u32;
                                let ay = oy + ly as u32;
                                let mut picked_color: Option<[u8; 4]> = None;

                                let sketch_on_top = project.metadata.sketch.visible
                                    && project.metadata.sketch.layer_mode
                                        == SketchLayerMode::Overlay;

                                if !sketch_on_top
                                    && ax < project.atlas_rgba.width()
                                    && ay < project.atlas_rgba.height()
                                {
                                    let p = *project.atlas_rgba.get_pixel(ax, ay);
                                    if p[3] > 0 {
                                        picked_color = Some([p[0], p[1], p[2], p[3]]);
                                    }
                                }

                                // Smart Sketch Sampling if canvas pixel is transparent or sketch is on top
                                if picked_color.is_none() && project.metadata.sketch.visible {
                                    if let Some(sketch_img) = &project.sketch_rgba {
                                        if let Some(sampled) =
                                            project.metadata.sketch.sample_rgba_at_frame_pos(
                                                sketch_img,
                                                project.metadata.frame_width,
                                                project.metadata.frame_height,
                                                lx as f32 + 0.5,
                                                ly as f32 + 0.5,
                                            )
                                        {
                                            let final_col =
                                                if project.metadata.sketch.snap_sample_to_palette {
                                                    nearest_palette_color(
                                                        sampled,
                                                        &project.metadata.palette.colors,
                                                    )
                                                } else {
                                                    let exact =
                                                        [sampled[0], sampled[1], sampled[2], 255];
                                                    if !project
                                                        .metadata
                                                        .palette
                                                        .colors
                                                        .contains(&exact)
                                                    {
                                                        project.metadata.palette.colors.push(exact);
                                                    }
                                                    exact
                                                };
                                            picked_color = Some(final_col);
                                        }
                                    }
                                }

                                if picked_color.is_none()
                                    && ax < project.atlas_rgba.width()
                                    && ay < project.atlas_rgba.height()
                                {
                                    let p = *project.atlas_rgba.get_pixel(ax, ay);
                                    if p[3] > 0 {
                                        picked_color = Some([p[0], p[1], p[2], p[3]]);
                                    }
                                }

                                if let Some(col) = picked_color {
                                    if btn == PointerButton::Secondary {
                                        tools.bg_color = col;
                                    } else {
                                        tools.fg_color = col;
                                    }
                                }
                            }
                            ToolKind::FloodFill => {
                                if !layer_locked {
                                    project.sync_active_layer();
                                    let snap = HistorySnapshot {
                                        atlas_rgba: project.atlas_rgba.clone(),
                                        metadata: project.metadata.clone(),
                                        active_frame: project.active_frame,
                                    };
                                    let fill_col = if btn == PointerButton::Secondary {
                                        tools.bg_color
                                    } else {
                                        tools.fg_color
                                    };
                                    let fw = project.metadata.frame_width;
                                    let fh = project.metadata.frame_height;
                                    let frames: Vec<usize> = if tools.batch_mode {
                                        (0..project.metadata.frame_count as usize).collect()
                                    } else {
                                        vec![f_idx]
                                    };
                                    let mut any_changed = false;
                                    for target_f in frames {
                                        let (ox, oy) = project.metadata.frame_origin(target_f);
                                        if flood_fill_region(
                                            &mut project.atlas_rgba,
                                            ox,
                                            oy,
                                            fw,
                                            fh,
                                            lx,
                                            ly,
                                            fill_col,
                                            tools.lock_alpha,
                                            tools.tolerance,
                                        ) {
                                            any_changed = true;
                                        }
                                    }
                                    if any_changed {
                                        history.push(snap);
                                        project.canvas_dirty = true;
                                        project.has_unsaved_changes = true;
                                    }
                                }
                            }
                            ToolKind::MagicWand => {
                                let (ox, oy) = project.metadata.frame_origin(f_idx);
                                self.selection.select_magic_wand(
                                    &project.atlas_rgba,
                                    ox,
                                    oy,
                                    project.metadata.frame_width,
                                    project.metadata.frame_height,
                                    lx,
                                    ly,
                                    tools.wand_global || shift_down,
                                    tools.tolerance,
                                );
                            }
                            ToolKind::PivotPoint => {
                                project.metadata.pivot.show_gizmos = true;
                                if let Some(ap_idx) = self.active_pivot_target {
                                    if let Some(ap) =
                                        project.metadata.pivot.action_points.get_mut(ap_idx)
                                    {
                                        ap.x = lx;
                                        ap.y = ly;
                                    } else {
                                        project.metadata.pivot.origin_x = lx;
                                        project.metadata.pivot.origin_y = ly;
                                    }
                                } else {
                                    project.metadata.pivot.origin_x = lx;
                                    project.metadata.pivot.origin_y = ly;
                                }
                            }
                            ToolKind::Curve => {}
                            ToolKind::Pencil
                            | ToolKind::Eraser
                            | ToolKind::DitherBrush
                            | ToolKind::Shading
                            | ToolKind::Line
                            | ToolKind::Rectangle
                            | ToolKind::Ellipse
                            | ToolKind::RectSelect
                            | ToolKind::Lasso
                            | ToolKind::Gradient => {
                                let is_sel_tool = matches!(
                                    effective_tool,
                                    ToolKind::RectSelect | ToolKind::Lasso
                                );
                                if !layer_locked || is_sel_tool {
                                    let moving_sel = is_sel_tool
                                        && !layer_locked
                                        && !self.selection.is_empty()
                                        && self.selection.contains(lx, ly);

                                    if moving_sel {
                                        project.sync_active_layer();
                                        history.push(HistorySnapshot {
                                            atlas_rgba: project.atlas_rgba.clone(),
                                            metadata: project.metadata.clone(),
                                            active_frame: project.active_frame,
                                        });
                                    } else if matches!(
                                        effective_tool,
                                        ToolKind::Pencil
                                            | ToolKind::Eraser
                                            | ToolKind::DitherBrush
                                            | ToolKind::Shading
                                    ) {
                                        project.sync_active_layer();
                                        history.push(HistorySnapshot {
                                            atlas_rgba: project.atlas_rgba.clone(),
                                            metadata: project.metadata.clone(),
                                            active_frame: project.active_frame,
                                        });
                                        Self::apply_point_stamp(
                                            &mut project.atlas_rgba,
                                            &project.metadata,
                                            &self.selection,
                                            viewport,
                                            tools,
                                            effective_tool,
                                            btn,
                                            f_idx,
                                            (lx, ly),
                                        );
                                        project.canvas_dirty = true;
                                        project.has_unsaved_changes = true;
                                    }
                                    self.active_stroke = Some(ActiveStroke {
                                        tool: effective_tool,
                                        button: btn,
                                        frame_index: f_idx,
                                        start_local: (lx, ly),
                                        last_local: (lx, ly),
                                        raw_points: vec![(lx, ly)],
                                        base_atlas: project.atlas_rgba.clone(),
                                        moving_selection: moving_sel,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Update active stroke if mouse button is still held
            if let Some(btn) = active_btn {
                if let Some(hover_pos) = response.hover_pos() {
                    let (cx, cy) = viewport.screen_to_pixel(canvas_rect, hover_pos);
                    if let Some(stroke) = &mut self.active_stroke {
                        let fw = project.metadata.frame_width.max(1) as i32;
                        let fh = project.metadata.frame_height.max(1) as i32;
                        let (lx, ly) = match viewport.view_mode {
                            CanvasViewMode::Frame => {
                                if viewport.tiled_preview_3x3 {
                                    (cx.rem_euclid(fw), cy.rem_euclid(fh))
                                } else {
                                    (cx.clamp(0, fw - 1), cy.clamp(0, fh - 1))
                                }
                            }
                            CanvasViewMode::Atlas => {
                                let (ox, oy) = project.metadata.frame_origin(stroke.frame_index);
                                (
                                    (cx - ox as i32).clamp(0, fw - 1),
                                    (cy - oy as i32).clamp(0, fh - 1),
                                )
                            }
                        };

                        if (lx, ly) != stroke.last_local {
                            if stroke.moving_selection {
                                let dx = lx - stroke.last_local.0;
                                let dy = ly - stroke.last_local.1;
                                if dx != 0 || dy != 0 {
                                    let (ox, oy) =
                                        project.metadata.frame_origin(stroke.frame_index);
                                    let fw = project.metadata.frame_width;
                                    let fh = project.metadata.frame_height;
                                    nudge_selection_region(
                                        &mut project.atlas_rgba,
                                        ox,
                                        oy,
                                        fw,
                                        fh,
                                        dx,
                                        dy,
                                        tools.wrap_around,
                                        &self.selection.pixels,
                                    );
                                    self.selection.translate(dx, dy, fw, fh, tools.wrap_around);
                                    project.canvas_dirty = true;
                                    project.has_unsaved_changes = true;
                                    stroke.last_local = (lx, ly);
                                }
                            } else {
                                match stroke.tool {
                                    ToolKind::Pencil
                                    | ToolKind::Eraser
                                    | ToolKind::DitherBrush
                                    | ToolKind::Shading => {
                                        let seg = bresenham_line(stroke.last_local, (lx, ly));
                                        let eff_brush_size =
                                            stroke.tool.effective_brush_size(tools.brush_size);
                                        let use_pixel_perfect = tools.pixel_perfect
                                            && eff_brush_size == 1
                                            && matches!(
                                                stroke.tool,
                                                ToolKind::Pencil | ToolKind::Eraser
                                            );

                                        if use_pixel_perfect {
                                            for pt in seg {
                                                if stroke.raw_points.last().copied() != Some(pt) {
                                                    stroke.raw_points.push(pt);
                                                }
                                            }
                                            let filtered =
                                                filter_pixel_perfect(&stroke.raw_points);
                                            project.atlas_rgba = stroke.base_atlas.clone();
                                            for pt in filtered {
                                                Self::apply_point_stamp(
                                                    &mut project.atlas_rgba,
                                                    &project.metadata,
                                                    &self.selection,
                                                    viewport,
                                                    tools,
                                                    stroke.tool,
                                                    btn,
                                                    stroke.frame_index,
                                                    pt,
                                                );
                                            }
                                        } else {
                                            for pt in seg {
                                                Self::apply_point_stamp(
                                                    &mut project.atlas_rgba,
                                                    &project.metadata,
                                                    &self.selection,
                                                    viewport,
                                                    tools,
                                                    stroke.tool,
                                                    btn,
                                                    stroke.frame_index,
                                                    pt,
                                                );
                                            }
                                        }
                                        project.canvas_dirty = true;
                                        project.has_unsaved_changes = true;
                                    }
                                    ToolKind::RectSelect => {
                                        self.selection.set_rect(
                                            stroke.start_local,
                                            (lx, ly),
                                            project.metadata.frame_width,
                                            project.metadata.frame_height,
                                        );
                                    }
                                    ToolKind::Lasso => {
                                        if stroke.raw_points.last() != Some(&(lx, ly)) {
                                            stroke.raw_points.push((lx, ly));
                                            if stroke.raw_points.len() >= 3 {
                                                self.selection.set_polygon(
                                                    &stroke.raw_points,
                                                    project.metadata.frame_width,
                                                    project.metadata.frame_height,
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                stroke.last_local = (lx, ly);
                            }
                        }
                    }
                }
            } else if let Some(finished) = self.active_stroke.take() {
                if finished.moving_selection {
                    // Selection movement finished
                } else if finished.tool == ToolKind::RectSelect {
                    if finished.start_local == finished.last_local {
                        self.selection.clear();
                    }
                } else if finished.tool == ToolKind::Lasso {
                    if finished.raw_points.len() <= 2 {
                        self.selection.clear();
                    } else {
                        self.selection.set_polygon(
                            &finished.raw_points,
                            project.metadata.frame_width,
                            project.metadata.frame_height,
                        );
                    }
                } else if matches!(
                    finished.tool,
                    ToolKind::Line | ToolKind::Rectangle | ToolKind::Ellipse
                ) {
                    project.sync_active_layer();
                    history.push(HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    });
                    let base_pts = match finished.tool {
                        ToolKind::Line => {
                            rasterize_line(finished.start_local, finished.last_local, shift_down)
                        }
                        ToolKind::Rectangle => rasterize_rectangle(
                            finished.start_local,
                            finished.last_local,
                            tools.filled_shape,
                            shift_down,
                        ),
                        ToolKind::Ellipse => rasterize_ellipse(
                            finished.start_local,
                            finished.last_local,
                            tools.filled_shape,
                            shift_down,
                        ),
                        _ => Vec::new(),
                    };
                    for pt in base_pts {
                        Self::apply_point_stamp(
                            &mut project.atlas_rgba,
                            &project.metadata,
                            &self.selection,
                            viewport,
                            tools,
                            finished.tool,
                            finished.button,
                            finished.frame_index,
                            pt,
                        );
                    }
                    project.canvas_dirty = true;
                    project.has_unsaved_changes = true;
                } else if finished.tool == ToolKind::Gradient {
                    project.sync_active_layer();
                    history.push(HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    });
                    let (c1, c2) = if finished.button == PointerButton::Secondary {
                        (tools.bg_color, tools.fg_color)
                    } else {
                        (tools.fg_color, tools.bg_color)
                    };
                    let (fw, fh) = (project.metadata.frame_width, project.metadata.frame_height);
                    let (ox, oy) = project.metadata.frame_origin(finished.frame_index);
                    let end_pt = crate::tools::shapes::snap_line_endpoint(
                        finished.start_local,
                        finished.last_local,
                        shift_down,
                    );
                    crate::tools::fill::apply_gradient_in_region(
                        &mut project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        finished.start_local,
                        end_pt,
                        c1,
                        c2,
                        tools.gradient_kind,
                        tools.gradient_dither,
                        if self.selection.is_empty() {
                            None
                        } else {
                            Some(&self.selection.pixels)
                        },
                        tools.lock_alpha,
                    );
                    project.canvas_dirty = true;
                    project.has_unsaved_changes = true;
                }
            }
        }

        // Upload textures if modified
        let _ = self.ensure_texture(&ctx, project);

        let painter = ui.painter().with_clip_rect(avail_rect);
        painter.rect_filled(avail_rect, 0.0, ui.visuals().extreme_bg_color);

        let background_target =
            if viewport.view_mode == CanvasViewMode::Frame && viewport.tiled_preview_3x3 {
                Rect::from_center_size(canvas_rect.center(), canvas_rect.size() * 3.0)
            } else {
                canvas_rect
            };

        // Layer 1: Checkerboard / Solid Background
        draw_canvas_background(&painter, avail_rect, background_target, viewport);

        let active_frame_rect = match viewport.view_mode {
            CanvasViewMode::Frame => canvas_rect,
            CanvasViewMode::Atlas => {
                let (ox, oy) = project.metadata.frame_origin(project.active_frame);
                let min = Pos2::new(
                    canvas_rect.min.x + ox as f32 * viewport.zoom,
                    canvas_rect.min.y + oy as f32 * viewport.zoom,
                );
                let size = Vec2::new(
                    project.metadata.frame_width as f32 * viewport.zoom,
                    project.metadata.frame_height as f32 * viewport.zoom,
                );
                Rect::from_min_size(min, size)
            }
        };

        // Layer 2: High-Res Sketch Underlay (if visible and set to Underlay)
        if project.metadata.sketch.visible
            && project.metadata.sketch.layer_mode == SketchLayerMode::Underlay
        {
            self.draw_sketch_mesh(&painter, active_frame_rect, project, viewport.zoom, None);
        }

        // Layer 3: Multi-Frame Onion Skinning
        self.draw_onion_skin_layers(&painter, active_frame_rect, project);

        // Layer 4 & 5: Tiled 3x3 Preview + Active Frame / Full Atlas Pixel Layer
        if let Some(tex) = &self.atlas_texture {
            match viewport.view_mode {
                CanvasViewMode::Frame => {
                    let uv = Self::frame_uv(project, project.active_frame);

                    if viewport.tiled_preview_3x3 {
                        let size = canvas_rect.size();
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                let offset = Vec2::new(dx as f32 * size.x, dy as f32 * size.y);
                                let tile_rect = canvas_rect.translate(offset);
                                painter.image(
                                    tex.id(),
                                    tile_rect,
                                    uv,
                                    Color32::from_rgba_unmultiplied(255, 255, 255, 190),
                                );
                            }
                        }
                    }

                    painter.image(tex.id(), canvas_rect, uv, Color32::WHITE);
                }
                CanvasViewMode::Atlas => {
                    let full_uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
                    painter.image(tex.id(), canvas_rect, full_uv, Color32::WHITE);
                }
            }
        }

        // Layer 6: High-Res Sketch Overlay (if visible and set to Overlay)
        if project.metadata.sketch.visible
            && project.metadata.sketch.layer_mode == SketchLayerMode::Overlay
        {
            self.draw_sketch_mesh(&painter, active_frame_rect, project, viewport.zoom, None);
        }

        // Layer 6b: Spacebar Quick Peek (PreviousFrame / SketchOverlay / Both)
        if quick_peek_active {
            let mode = project.metadata.onion_skin.quick_peek_mode;
            if matches!(mode, QuickPeekMode::PreviousFrame | QuickPeekMode::Both)
                && project.metadata.frame_count > 1
            {
                if let Some(tex) = &self.atlas_texture {
                    let prev_idx = if project.active_frame > 0 {
                        project.active_frame - 1
                    } else {
                        (project.metadata.frame_count as usize).saturating_sub(1)
                    };
                    let uv = Self::frame_uv(project, prev_idx);
                    let peek_alpha = if mode == QuickPeekMode::Both {
                        170
                    } else {
                        215
                    };
                    painter.image(
                        tex.id(),
                        active_frame_rect,
                        uv,
                        Color32::from_rgba_unmultiplied(255, 255, 255, peek_alpha),
                    );
                }
            }

            if matches!(mode, QuickPeekMode::SketchOverlay | QuickPeekMode::Both) {
                let peek_opacity = if mode == QuickPeekMode::Both {
                    0.70
                } else {
                    0.85
                };
                self.draw_sketch_mesh(
                    &painter,
                    active_frame_rect,
                    project,
                    viewport.zoom,
                    Some(peek_opacity),
                );
            }
        }

        // Compute shape preview pixels if dragging a Line / Curve / Rectangle / Ellipse
        let (preview_pixels, preview_color) = if let Some(stroke) = &self.active_stroke {
            if matches!(
                stroke.tool,
                ToolKind::Line | ToolKind::Rectangle | ToolKind::Ellipse
            ) {
                let pts = Self::compute_shape_preview_points(
                    &project.metadata,
                    viewport,
                    tools,
                    stroke.tool,
                    stroke.start_local,
                    stroke.last_local,
                    shift_down,
                );
                let col = if stroke.button == PointerButton::Secondary {
                    tools.bg_color
                } else {
                    tools.fg_color
                };
                (pts, col)
            } else {
                (Vec::new(), tools.fg_color)
            }
        } else if let Some(curve) = &self.curve_state {
            let (raw_pts, btn) = match curve {
                CurveState::WaitingForEnd { p0, button, .. } => {
                    let p2 = viewport.hover_pixel.unwrap_or(*p0);
                    (rasterize_line(*p0, p2, shift_down), *button)
                }
                CurveState::WaitingForControl { p0, p2, button, .. } => {
                    let p1 = viewport.hover_pixel.unwrap_or(*p2);
                    (rasterize_quadratic_bezier(*p0, p1, *p2), *button)
                }
            };
            let base_pts = if tools.pixel_perfect {
                filter_pixel_perfect(&raw_pts)
            } else {
                raw_pts
            };
            let brush_size = ToolKind::Curve.effective_brush_size(tools.brush_size);
            let fw = project.metadata.frame_width.max(1);
            let fh = project.metadata.frame_height.max(1);
            let mut expanded = Vec::new();
            for pt in base_pts {
                let stamp = expand_stamp_points(
                    pt,
                    brush_size,
                    tools.symmetry,
                    viewport.symmetry_norm_x,
                    viewport.symmetry_norm_y,
                    fw,
                    fh,
                );
                for (lx, ly) in stamp {
                    if lx >= 0 && ly >= 0 && (lx as u32) < fw && (ly as u32) < fh {
                        expanded.push((lx, ly));
                    }
                }
            }
            let col = if btn == PointerButton::Secondary {
                tools.bg_color
            } else {
                tools.fg_color
            };
            (expanded, col)
        } else {
            (Vec::new(), tools.fg_color)
        };

        let (hover_for_cursor, cursor_brush_size) = if alt_down {
            (viewport.hover_pixel, 1)
        } else if tools.active_tool == ToolKind::Pencil && tools.custom_brush.is_some() {
            if let Some(custom) = &tools.custom_brush {
                if let Some((hx, hy)) = viewport.hover_pixel {
                    let off_x = hx - (custom.width as i32 / 2);
                    let off_y = hy - (custom.height as i32 / 2);
                    for &((rx, ry), color) in &custom.pixels {
                        if color[3] == 0 {
                            continue;
                        }
                        let px = off_x + rx;
                        let py = off_y + ry;
                        let min = egui::Pos2::new(
                            active_frame_rect.min.x + px as f32 * viewport.zoom,
                            active_frame_rect.min.y + py as f32 * viewport.zoom,
                        );
                        let cell = egui::Rect::from_min_size(min, egui::Vec2::splat(viewport.zoom));
                        let alpha = (color[3] as f32 * 0.7) as u8;
                        painter.rect_filled(
                            cell,
                            0.0,
                            egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], alpha),
                        );
                    }
                    let min_box = egui::Pos2::new(
                        active_frame_rect.min.x + off_x as f32 * viewport.zoom,
                        active_frame_rect.min.y + off_y as f32 * viewport.zoom,
                    );
                    let brush_box = egui::Rect::from_min_size(
                        min_box,
                        egui::vec2(
                            custom.width as f32 * viewport.zoom,
                            custom.height as f32 * viewport.zoom,
                        ),
                    );
                    painter.rect_stroke(
                        brush_box,
                        0.0,
                        egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgba_unmultiplied(200, 220, 255, 180),
                        ),
                        egui::StrokeKind::Outside,
                    );
                }
            }
            (None, 1)
        } else {
            (viewport.hover_pixel, tools.effective_brush_size())
        };

        draw_preview_pixels_and_cursor(
            &painter,
            active_frame_rect,
            &preview_pixels,
            preview_color,
            hover_for_cursor,
            cursor_brush_size,
            viewport.zoom,
        );

        if viewport.show_pixel_grid {
            draw_pixel_grid(
                &painter,
                avail_rect,
                canvas_rect,
                content_w,
                content_h,
                viewport.zoom,
            );
        }

        if viewport.show_frame_grid {
            draw_frame_boundary_grid(
                &painter,
                canvas_rect,
                &project.metadata,
                project.active_frame,
                viewport.view_mode,
                viewport.zoom,
            );
        }

        let lasso_preview = self.active_stroke.as_ref().and_then(|s| {
            if s.tool == ToolKind::Lasso && !s.moving_selection {
                Some(s.raw_points.as_slice())
            } else {
                None
            }
        });
        draw_selection_overlay(
            &painter,
            active_frame_rect,
            &self.selection,
            lasso_preview,
            viewport.zoom,
        );

        // Curve 3-click handle markers
        if let Some(curve) = &self.curve_state {
            match curve {
                CurveState::WaitingForEnd { p0, .. } => {
                    let s0 = Pos2::new(
                        active_frame_rect.min.x + (p0.0 as f32 + 0.5) * viewport.zoom,
                        active_frame_rect.min.y + (p0.1 as f32 + 0.5) * viewport.zoom,
                    );
                    painter.circle_filled(s0, 4.0, Color32::WHITE);
                    painter.circle_stroke(
                        s0,
                        4.0,
                        egui::Stroke::new(1.5, Color32::from_rgb(60, 170, 255)),
                    );
                }
                CurveState::WaitingForControl { p0, p2, .. } => {
                    let p1 = viewport.hover_pixel.unwrap_or(*p2);
                    let s0 = Pos2::new(
                        active_frame_rect.min.x + (p0.0 as f32 + 0.5) * viewport.zoom,
                        active_frame_rect.min.y + (p0.1 as f32 + 0.5) * viewport.zoom,
                    );
                    let s1 = Pos2::new(
                        active_frame_rect.min.x + (p1.0 as f32 + 0.5) * viewport.zoom,
                        active_frame_rect.min.y + (p1.1 as f32 + 0.5) * viewport.zoom,
                    );
                    let s2 = Pos2::new(
                        active_frame_rect.min.x + (p2.0 as f32 + 0.5) * viewport.zoom,
                        active_frame_rect.min.y + (p2.1 as f32 + 0.5) * viewport.zoom,
                    );
                    painter.line_segment(
                        [s0, s1],
                        egui::Stroke::new(
                            1.0,
                            Color32::from_rgba_unmultiplied(120, 200, 255, 130),
                        ),
                    );
                    painter.line_segment(
                        [s2, s1],
                        egui::Stroke::new(
                            1.0,
                            Color32::from_rgba_unmultiplied(120, 200, 255, 130),
                        ),
                    );
                    painter.circle_filled(s0, 4.0, Color32::WHITE);
                    painter.circle_stroke(
                        s0,
                        4.0,
                        egui::Stroke::new(1.5, Color32::from_rgb(60, 170, 255)),
                    );
                    painter.circle_filled(s2, 4.0, Color32::WHITE);
                    painter.circle_stroke(
                        s2,
                        4.0,
                        egui::Stroke::new(1.5, Color32::from_rgb(60, 170, 255)),
                    );
                    painter.circle_filled(s1, 3.5, Color32::from_rgb(255, 190, 60));
                }
            }
        }

        // Gradient vector live preview guide
        if let Some(stroke) = &self.active_stroke {
            if stroke.tool == ToolKind::Gradient {
                let end_pt = crate::tools::shapes::snap_line_endpoint(
                    stroke.start_local,
                    stroke.last_local,
                    shift_down,
                );
                let p0 = Pos2::new(
                    active_frame_rect.min.x + (stroke.start_local.0 as f32 + 0.5) * viewport.zoom,
                    active_frame_rect.min.y + (stroke.start_local.1 as f32 + 0.5) * viewport.zoom,
                );
                let p1 = Pos2::new(
                    active_frame_rect.min.x + (end_pt.0 as f32 + 0.5) * viewport.zoom,
                    active_frame_rect.min.y + (end_pt.1 as f32 + 0.5) * viewport.zoom,
                );
                painter.line_segment([p0, p1], egui::Stroke::new(2.5, egui::Color32::from_black_alpha(180)));
                painter.line_segment([p0, p1], egui::Stroke::new(1.5, egui::Color32::from_rgb(60, 170, 255)));

                painter.circle_filled(p0, 4.5, egui::Color32::WHITE);
                painter.circle_stroke(p0, 4.5, egui::Stroke::new(1.5, egui::Color32::from_rgb(60, 170, 255)));

                painter.circle_filled(p1, 4.5, egui::Color32::from_rgb(255, 90, 90));
                painter.circle_stroke(p1, 4.5, egui::Stroke::new(1.5, egui::Color32::WHITE));
            }
        }
        draw_symmetry_axes(
            &painter,
            active_frame_rect,
            tools.symmetry,
            viewport.symmetry_norm_x,
            viewport.symmetry_norm_y,
        );
        draw_pivot_gizmos(&painter, active_frame_rect, &project.metadata, viewport.zoom);
    }
}

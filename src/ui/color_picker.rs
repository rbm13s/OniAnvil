use egui::ecolor::{Color32, Hsva};
use egui::{
    pos2, vec2, DragValue, Id, Mesh, Painter, Rect, Response, Sense, Stroke,
    StrokeKind, TextEdit, Ui, Vec2,
};
use crate::i18n::I18n;
use crate::io::settings::{default_recent_colors, AppSettings};
use crate::theme::{FA_CHECK, FA_COPY, FA_PLUS};

/// Parses a hex color string in `#RGB`, `RGB`, `#RGBA`, `RGBA`, `#RRGGBB`, `RRGGBB`, `#RRGGBBAA`, `RRGGBBAA`.
pub fn parse_hex(s: &str) -> Option<[u8; 4]> {
    let s = s.trim().trim_start_matches('#');
    match s.len() {
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()?;
            let g = u8::from_str_radix(&s[1..2], 16).ok()?;
            let b = u8::from_str_radix(&s[2..3], 16).ok()?;
            Some([r * 17, g * 17, b * 17, 255])
        }
        4 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()?;
            let g = u8::from_str_radix(&s[1..2], 16).ok()?;
            let b = u8::from_str_radix(&s[2..3], 16).ok()?;
            let a = u8::from_str_radix(&s[3..4], 16).ok()?;
            Some([r * 17, g * 17, b * 17, a * 17])
        }
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

/// Formats a color as `#RRGGBB` (or `#RRGGBBAA` if not fully opaque).
pub fn format_hex(col: [u8; 4]) -> String {
    if col[3] == 255 {
        format!("#{:02X}{:02X}{:02X}", col[0], col[1], col[2])
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", col[0], col[1], col[2], col[3])
    }
}

/// Internal state stored per color picker instance across frames.
#[derive(Clone, Debug)]
pub struct ColorPickerInternalState {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
    pub initial_color: [u8; 4],
    pub hex_buffer: String,
    pub hex_invalid: bool,
    pub copied_timer: f32,
    pub initialized: bool,
}

impl ColorPickerInternalState {
    pub fn new(current: [u8; 4]) -> Self {
        let hsva = Hsva::from(Color32::from_rgba_unmultiplied(
            current[0], current[1], current[2], current[3],
        ));
        Self {
            h: hsva.h,
            s: hsva.s,
            v: hsva.v,
            a: hsva.a,
            initial_color: current,
            hex_buffer: format_hex(current),
            hex_invalid: false,
            copied_timer: 0.0,
            initialized: true,
        }
    }

    pub fn sync_from_color(&mut self, current: [u8; 4]) {
        let hsva = Hsva::from(Color32::from_rgba_unmultiplied(
            current[0], current[1], current[2], current[3],
        ));
        // Keep current hue if saturation or value is zero to prevent slider jump
        if hsva.s > 0.001 && hsva.v > 0.001 {
            self.h = hsva.h;
        }
        self.s = hsva.s;
        self.v = hsva.v;
        self.a = hsva.a;
        self.hex_buffer = format_hex(current);
        self.hex_invalid = false;
    }
}

/// Draws a transparent checkerboard pattern on the given rect.
pub fn draw_checkers(painter: &Painter, rect: Rect, cell_size: f32) {
    if !rect.is_positive() {
        return;
    }
    let dark = Color32::from_gray(38);
    let light = Color32::from_gray(65);
    painter.rect_filled(rect, 0.0, dark);

    let mut mesh = Mesh::default();
    let cols = (rect.width() / cell_size).ceil() as usize;
    let rows = (rect.height() / cell_size).ceil() as usize;

    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                let min_x = rect.left() + x as f32 * cell_size;
                let min_y = rect.top() + y as f32 * cell_size;
                let max_x = (min_x + cell_size).min(rect.right());
                let max_y = (min_y + cell_size).min(rect.bottom());
                let cell_rect = Rect::from_min_max(pos2(min_x, min_y), pos2(max_x, max_y));
                mesh.add_colored_rect(cell_rect, light);
            }
        }
    }
    painter.add(egui::Shape::mesh(mesh));
}

fn draw_sv_canvas(painter: &Painter, rect: Rect, hue: f32) {
    const NX: u32 = 24;
    const NY: u32 = 16;
    let mut mesh = Mesh::default();

    for yi in 0..=NY {
        let v = 1.0 - (yi as f32 / NY as f32);
        let y = egui::lerp(rect.top()..=rect.bottom(), yi as f32 / NY as f32);
        for xi in 0..=NX {
            let s = xi as f32 / NX as f32;
            let x = egui::lerp(rect.left()..=rect.right(), s);
            let col = Color32::from(Hsva {
                h: hue,
                s,
                v,
                a: 1.0,
            });
            mesh.colored_vertex(pos2(x, y), col);
        }
    }

    for yi in 0..NY {
        for xi in 0..NX {
            let tl = yi * (NX + 1) + xi;
            let tr = tl + 1;
            let bl = tl + (NX + 1);
            let br = bl + 1;
            mesh.add_triangle(tl, bl, tr);
            mesh.add_triangle(tr, bl, br);
        }
    }

    painter.add(egui::Shape::mesh(mesh));
}

fn draw_hue_slider(painter: &Painter, rect: Rect) {
    const N: u32 = 36;
    let mut mesh = Mesh::default();
    for i in 0..=N {
        let t = i as f32 / N as f32;
        let x = egui::lerp(rect.left()..=rect.right(), t);
        let col = Color32::from(Hsva {
            h: t,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        });
        mesh.colored_vertex(pos2(x, rect.top()), col);
        mesh.colored_vertex(pos2(x, rect.bottom()), col);
        if i < N {
            let v0 = 2 * i;
            let v1 = 2 * i + 1;
            let v2 = 2 * i + 2;
            let v3 = 2 * i + 3;
            mesh.add_triangle(v0, v1, v2);
            mesh.add_triangle(v1, v3, v2);
        }
    }
    painter.add(egui::Shape::mesh(mesh));
}

fn draw_alpha_slider(painter: &Painter, rect: Rect, rgb: [u8; 3]) {
    draw_checkers(painter, rect, rect.height() / 2.0);
    let mut mesh = Mesh::default();
    let col_left = Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], 0);
    let col_right = Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], 255);

    mesh.colored_vertex(pos2(rect.left(), rect.top()), col_left);
    mesh.colored_vertex(pos2(rect.left(), rect.bottom()), col_left);
    mesh.colored_vertex(pos2(rect.right(), rect.top()), col_right);
    mesh.colored_vertex(pos2(rect.right(), rect.bottom()), col_right);

    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(1, 3, 2);
    painter.add(egui::Shape::mesh(mesh));
}

/// Commits a color into recent colors (capped at 10, deduplicated).
pub fn commit_recent_color(
    ctx: &egui::Context,
    settings: Option<&mut AppSettings>,
    color: [u8; 4],
) {
    let recent_id = Id::new("onianvil_recent_colors");
    if let Some(s) = settings {
        s.add_recent_color(color);
        ctx.data_mut(|d| d.insert_temp(recent_id, s.recent_colors.clone()));
    } else {
        let mut list: Vec<[u8; 4]> = ctx
            .data(|d| d.get_temp(recent_id))
            .unwrap_or_else(default_recent_colors);
        list.retain(|c| c != &color);
        list.insert(0, color);
        list.truncate(10);
        ctx.data_mut(|d| d.insert_temp(recent_id, list));
    }
}

/// Gets the current 10 recent colors.
pub fn get_recent_colors(
    ctx: &egui::Context,
    settings: Option<&AppSettings>,
) -> Vec<[u8; 4]> {
    if let Some(s) = settings {
        return s.recent_colors.clone();
    }
    let recent_id = Id::new("onianvil_recent_colors");
    ctx.data(|d| d.get_temp(recent_id))
        .unwrap_or_else(default_recent_colors)
}

/// Renders the complete modern color picker content (header, split comparison,
/// 2D SV canvas, Hue slider, Alpha slider, Hex input with copy, RGB 0..255 inputs, 10 recent colors).
pub fn show_color_picker_content(
    ui: &mut Ui,
    state_id: Id,
    color: &mut [u8; 4],
    i18n: &I18n,
    mut settings: Option<&mut AppSettings>,
) -> bool {
    let mut state: ColorPickerInternalState = ui
        .ctx()
        .data_mut(|d| d.get_temp(state_id))
        .unwrap_or_else(|| ColorPickerInternalState::new(*color));

    if !state.initialized {
        state = ColorPickerInternalState::new(*color);
    }

    if state.copied_timer > 0.0 {
        state.copied_timer = (state.copied_timer - ui.input(|i| i.stable_dt)).max(0.0);
        ui.ctx().request_repaint();
    }

    let mut changed = false;
    let canvas_width = 252.0;

    ui.set_width(canvas_width);

    // 1. Split Preview: Old vs New
    let preview_height = 24.0;
    let (prev_rect, prev_resp) =
        ui.allocate_exact_size(vec2(canvas_width, preview_height), Sense::click());

    if ui.is_rect_visible(prev_rect) {
        let left_rect = Rect::from_min_max(prev_rect.left_top(), pos2(prev_rect.center().x, prev_rect.bottom()));
        let right_rect = Rect::from_min_max(pos2(prev_rect.center().x, prev_rect.top()), prev_rect.right_bottom());

        // Draw checkers
        draw_checkers(ui.painter(), prev_rect, 6.0);

        // Fill initial color on left
        let old_col = Color32::from_rgba_unmultiplied(
            state.initial_color[0],
            state.initial_color[1],
            state.initial_color[2],
            state.initial_color[3],
        );
        ui.painter().rect_filled(left_rect, 0.0, old_col);

        // Fill current color on right
        let new_col = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        ui.painter().rect_filled(right_rect, 0.0, new_col);

        // Center separator line
        ui.painter().line_segment(
            [pos2(prev_rect.center().x, prev_rect.top()), pos2(prev_rect.center().x, prev_rect.bottom())],
            Stroke::new(1.0, Color32::from_black_alpha(150)),
        );

        // Outline
        ui.painter().rect_stroke(
            prev_rect,
            4.0,
            Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            StrokeKind::Inside,
        );
    }

    // Handle click on left half to revert
    if prev_resp.clicked() {
        if let Some(pos) = prev_resp.interact_pointer_pos() {
            if pos.x < prev_rect.center().x {
                *color = state.initial_color;
                state.sync_from_color(*color);
                changed = true;
            }
        }
    }
    prev_resp.on_hover_ui(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} ({}) ⇄ {} ({})",
                    format_hex(state.initial_color),
                    i18n.tr("color_picker.old"),
                    format_hex(*color),
                    i18n.tr("color_picker.new")
                ))
                .small(),
            );
        });
        ui.small(i18n.tr("color_picker.revert"));
    });

    ui.add_space(6.0);

    // 2. 2D SV Canvas
    let sv_height = 140.0;
    let (sv_rect, sv_resp) =
        ui.allocate_exact_size(vec2(canvas_width, sv_height), Sense::click_and_drag());

    if sv_resp.dragged() || sv_resp.clicked() {
        if let Some(pos) = sv_resp.interact_pointer_pos() {
            let s = egui::remap_clamp(pos.x, sv_rect.left()..=sv_rect.right(), 0.0..=1.0);
            let v = egui::remap_clamp(pos.y, sv_rect.bottom()..=sv_rect.top(), 0.0..=1.0);
            state.s = s;
            state.v = v;
            let c = Color32::from(Hsva {
                h: state.h,
                s: state.s,
                v: state.v,
                a: state.a,
            });
            *color = [c.r(), c.g(), c.b(), c.a()];
            state.hex_buffer = format_hex(*color);
            state.hex_invalid = false;
            changed = true;
        }
    }

    if ui.is_rect_visible(sv_rect) {
        draw_sv_canvas(ui.painter(), sv_rect, state.h);
        ui.painter().rect_stroke(
            sv_rect,
            4.0,
            Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            StrokeKind::Inside,
        );

        // Draw modern cursor
        let cx = egui::lerp(sv_rect.left()..=sv_rect.right(), state.s);
        let cy = egui::lerp(sv_rect.top()..=sv_rect.bottom(), 1.0 - state.v);
        let center = pos2(cx, cy);
        let cur_opaque = Color32::from(Hsva {
            h: state.h,
            s: state.s,
            v: state.v,
            a: 1.0,
        });

        ui.painter().circle_stroke(center, 6.0, Stroke::new(2.0, Color32::WHITE));
        ui.painter().circle_stroke(center, 4.5, Stroke::new(1.0, Color32::from_black_alpha(180)));
        ui.painter().circle_filled(center, 3.5, cur_opaque);
    }

    ui.add_space(6.0);

    // 3. Hue Slider
    let slider_height = 14.0;
    let (h_rect, h_resp) =
        ui.allocate_exact_size(vec2(canvas_width, slider_height), Sense::click_and_drag());

    if h_resp.dragged() || h_resp.clicked() {
        if let Some(pos) = h_resp.interact_pointer_pos() {
            state.h = egui::remap_clamp(pos.x, h_rect.left()..=h_rect.right(), 0.0..=1.0);
            let c = Color32::from(Hsva {
                h: state.h,
                s: state.s,
                v: state.v,
                a: state.a,
            });
            *color = [c.r(), c.g(), c.b(), c.a()];
            state.hex_buffer = format_hex(*color);
            state.hex_invalid = false;
            changed = true;
        }
    }

    if ui.is_rect_visible(h_rect) {
        draw_hue_slider(ui.painter(), h_rect);
        ui.painter().rect_stroke(
            h_rect,
            3.0,
            Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            StrokeKind::Inside,
        );

        let hx = egui::lerp(h_rect.left()..=h_rect.right(), state.h);
        let thumb_rect = Rect::from_center_size(
            pos2(hx, h_rect.center().y),
            vec2(7.0, h_rect.height() + 4.0),
        );
        ui.painter().rect_filled(thumb_rect, 3.0, Color32::WHITE);
        ui.painter().rect_stroke(
            thumb_rect,
            3.0,
            Stroke::new(1.5, Color32::from_gray(30)),
            StrokeKind::Inside,
        );
    }

    ui.add_space(4.0);

    // 4. Alpha Slider
    let (a_rect, a_resp) =
        ui.allocate_exact_size(vec2(canvas_width, slider_height), Sense::click_and_drag());

    if a_resp.dragged() || a_resp.clicked() {
        if let Some(pos) = a_resp.interact_pointer_pos() {
            state.a = egui::remap_clamp(pos.x, a_rect.left()..=a_rect.right(), 0.0..=1.0);
            let c = Color32::from(Hsva {
                h: state.h,
                s: state.s,
                v: state.v,
                a: state.a,
            });
            *color = [c.r(), c.g(), c.b(), c.a()];
            state.hex_buffer = format_hex(*color);
            state.hex_invalid = false;
            changed = true;
        }
    }

    if ui.is_rect_visible(a_rect) {
        draw_alpha_slider(ui.painter(), a_rect, [color[0], color[1], color[2]]);
        ui.painter().rect_stroke(
            a_rect,
            3.0,
            Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            StrokeKind::Inside,
        );

        let ax = egui::lerp(a_rect.left()..=a_rect.right(), state.a);
        let thumb_rect = Rect::from_center_size(
            pos2(ax, a_rect.center().y),
            vec2(7.0, a_rect.height() + 4.0),
        );
        ui.painter().rect_filled(thumb_rect, 3.0, Color32::WHITE);
        ui.painter().rect_stroke(
            thumb_rect,
            3.0,
            Stroke::new(1.5, Color32::from_gray(30)),
            StrokeKind::Inside,
        );
    }

    ui.add_space(8.0);

    // 5. HTML / HEX Input and Quick Copy
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.label(egui::RichText::new("HEX:").strong().small());

        let hex_edit = TextEdit::singleline(&mut state.hex_buffer)
            .desired_width(88.0)
            .hint_text("#RRGGBB")
            .font(egui::TextStyle::Monospace);

        let edit_resp = ui.add(hex_edit);

        if edit_resp.changed() {
            if let Some(parsed) = parse_hex(&state.hex_buffer) {
                *color = parsed;
                state.sync_from_color(*color);
                state.hex_invalid = false;
                changed = true;
            } else {
                state.hex_invalid = true;
            }
        }

        if edit_resp.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            if let Some(parsed) = parse_hex(&state.hex_buffer) {
                *color = parsed;
                state.sync_from_color(*color);
                commit_recent_color(ui.ctx(), settings.as_deref_mut(), *color);
            } else {
                state.hex_buffer = format_hex(*color);
                state.hex_invalid = false;
            }
        }

        if state.hex_invalid {
            ui.label(egui::RichText::new("!").color(Color32::from_rgb(240, 80, 80)).strong().small())
                .on_hover_text(i18n.tr("color_picker.invalid_hex"));
        }

        // Copy button
        let copy_label = if state.copied_timer > 0.0 {
            egui::RichText::new(FA_CHECK).color(Color32::from_rgb(90, 220, 110)).size(11.0)
        } else {
            egui::RichText::new(FA_COPY).size(11.0)
        };

        if ui
            .small_button(copy_label)
            .on_hover_text(i18n.tr("color_picker.copy_hex"))
            .clicked()
        {
            ui.ctx().copy_text(format_hex(*color));
            state.copied_timer = 1.6;
        }

        if state.copied_timer > 0.0 {
            ui.label(egui::RichText::new(i18n.tr("color_picker.copied")).color(Color32::from_rgb(90, 220, 110)).small());
        }
    });

    ui.add_space(4.0);

    // 6. RGB 0..255 Numeric DragValues
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let mut r = color[0];
        let mut g = color[1];
        let mut b = color[2];
        let mut a = color[3];

        let r_resp = ui.add(DragValue::new(&mut r).range(0..=255).speed(0.5).prefix("R:"));
        let g_resp = ui.add(DragValue::new(&mut g).range(0..=255).speed(0.5).prefix("G:"));
        let b_resp = ui.add(DragValue::new(&mut b).range(0..=255).speed(0.5).prefix("B:"));
        let a_resp = ui.add(DragValue::new(&mut a).range(0..=255).speed(0.5).prefix("A:"));

        if r_resp.changed() || g_resp.changed() || b_resp.changed() || a_resp.changed() {
            *color = [r, g, b, a];
            state.sync_from_color(*color);
            changed = true;
        }

        if (r_resp.drag_stopped() || g_resp.drag_stopped() || b_resp.drag_stopped() || a_resp.drag_stopped())
            && *color != state.initial_color
        {
            commit_recent_color(ui.ctx(), settings.as_deref_mut(), *color);
        }
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    // 7. 10 Recent Colors Row
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(i18n.tr("color_picker.recent")).strong().small());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button(egui::RichText::new(FA_PLUS).size(10.0))
                .on_hover_text(i18n.tr("color_picker.add_recent"))
                .clicked()
            {
                commit_recent_color(ui.ctx(), settings.as_deref_mut(), *color);
            }
        });
    });

    ui.add_space(4.0);

    let recents = get_recent_colors(ui.ctx(), settings.as_deref());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let swatch_size = vec2(21.0, 21.0);

        for rcol in recents.iter().take(10) {
            let (rect, resp) = ui.allocate_exact_size(swatch_size, Sense::click());
            let is_current = *rcol == *color;
            let is_hovered = resp.hovered();

            draw_checkers(ui.painter(), rect, 4.0);
            let col32 = Color32::from_rgba_unmultiplied(rcol[0], rcol[1], rcol[2], rcol[3]);
            ui.painter().rect_filled(rect, 3.0, col32);

            let stroke = if is_current {
                Stroke::new(2.0, Color32::WHITE)
            } else if is_hovered {
                Stroke::new(1.5, ui.visuals().widgets.hovered.bg_stroke.color)
            } else {
                Stroke::new(1.0, Color32::from_black_alpha(120))
            };
            ui.painter().rect_stroke(rect, 3.0, stroke, StrokeKind::Inside);

            if is_current {
                ui.painter().rect_stroke(
                    rect.shrink(1.5),
                    2.0,
                    Stroke::new(1.0, Color32::from_black_alpha(180)),
                    StrokeKind::Inside,
                );
            }

            if resp.clicked() {
                *color = *rcol;
                state.sync_from_color(*color);
                changed = true;
            }

            resp.on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                    ui.painter().rect_filled(r, 2.0, col32);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format_hex(*rcol)).strong().monospace().small());
                        ui.label(egui::RichText::new(format!(
                            "RGBA: {}, {}, {}, {}",
                            rcol[0], rcol[1], rcol[2], rcol[3]
                        )).small());
                    });
                });
            });
        }
    });

    // Auto-commit to recents on pointer release if dragging occurred
    if (sv_resp.drag_stopped() || h_resp.drag_stopped() || a_resp.drag_stopped())
        && *color != state.initial_color
    {
        commit_recent_color(ui.ctx(), settings, *color);
    }

    ui.ctx().data_mut(|d| d.insert_temp(state_id, state));

    changed
}

/// Sized modern color picker button with custom button rect dimensions.
pub fn modern_color_picker_button_sized(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash,
    color: &mut [u8; 4],
    i18n: &I18n,
    settings: Option<&mut AppSettings>,
    size: Vec2,
) -> Response {
    let popup_id = ui.make_persistent_id(id_salt);
    let state_id = popup_id.with("state");
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());

    let is_open = ui.memory(|mem| mem.is_popup_open(popup_id));

    if response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        if !is_open {
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, ColorPickerInternalState::new(*color));
            });
        }
    }

    if ui.is_rect_visible(rect) {
        draw_checkers(ui.painter(), rect, 5.0);
        let col32 = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        ui.painter().rect_filled(rect, 4.0, col32);

        let stroke = if is_open || response.hovered() {
            Stroke::new(1.5, ui.visuals().widgets.hovered.bg_stroke.color)
        } else {
            Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
        };
        ui.painter().rect_stroke(rect, 4.0, stroke, StrokeKind::Inside);
    }

    response = response.on_hover_ui(|ui| {
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
            let col32 = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
            ui.painter().rect_filled(r, 2.0, col32);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format_hex(*color)).strong().monospace().small());
                ui.label(egui::RichText::new(format!(
                    "RGBA: {}, {}, {}, {}",
                    color[0], color[1], color[2], color[3]
                )).small());
            });
        });
    });

    if is_open {
        let mut content_changed = false;
        egui::popup_below_widget(
            ui,
            popup_id,
            &response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                egui::Frame::popup(ui.style())
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(8)
                    .show(ui, |ui| {
                        if show_color_picker_content(
                            ui,
                            state_id,
                            color,
                            i18n,
                            settings,
                        ) {
                            content_changed = true;
                        }
                    });
            },
        );
        if content_changed {
            response.mark_changed();
        }
    }

    response
}

/// Standard modern color picker button (`24.0 × 24.0` px).
pub fn modern_color_picker_button(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash,
    color: &mut [u8; 4],
    i18n: &I18n,
    settings: Option<&mut AppSettings>,
) -> Response {
    modern_color_picker_button_sized(ui, id_salt, color, i18n, settings, vec2(24.0, 24.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_various_formats() {
        // #RGB
        assert_eq!(parse_hex("#f0a"), Some([255, 0, 170, 255]));
        // RGB
        assert_eq!(parse_hex("f0a"), Some([255, 0, 170, 255]));
        // #RGBA
        assert_eq!(parse_hex("#f0a8"), Some([255, 0, 170, 136]));
        // RGBA
        assert_eq!(parse_hex("f0a8"), Some([255, 0, 170, 136]));
        // #RRGGBB
        assert_eq!(parse_hex("#112233"), Some([17, 34, 51, 255]));
        // RRGGBB
        assert_eq!(parse_hex("112233"), Some([17, 34, 51, 255]));
        // #RRGGBBAA
        assert_eq!(parse_hex("#11223388"), Some([17, 34, 51, 136]));
        // RRGGBBAA
        assert_eq!(parse_hex("11223388"), Some([17, 34, 51, 136]));

        // Black and White
        assert_eq!(parse_hex("#000000"), Some([0, 0, 0, 255]));
        assert_eq!(parse_hex("#ffffff"), Some([255, 255, 255, 255]));

        // Invalid
        assert_eq!(parse_hex("xyz"), None);
        assert_eq!(parse_hex("#12"), None);
        assert_eq!(parse_hex(""), None);
        assert_eq!(parse_hex("#12345"), None);
    }

    #[test]
    fn test_format_hex_output() {
        assert_eq!(format_hex([255, 0, 128, 255]), "#FF0080");
        assert_eq!(format_hex([255, 0, 128, 120]), "#FF008078");
        assert_eq!(format_hex([0, 0, 0, 255]), "#000000");
        assert_eq!(format_hex([0, 0, 0, 0]), "#00000000");
    }

    #[test]
    fn test_color_picker_render_and_interactions() {
        let ctx = egui::Context::default();
        let i18n = I18n::new(crate::i18n::Language::Russian);
        let mut settings = AppSettings::default();
        let mut color = [255, 128, 64, 255];
        let state_id = Id::new("test_picker_render");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let changed = show_color_picker_content(
                    ui,
                    state_id,
                    &mut color,
                    &i18n,
                    Some(&mut settings),
                );
                assert!(!changed);
            });
        });

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _resp = modern_color_picker_button(
                    ui,
                    "btn_test",
                    &mut color,
                    &i18n,
                    Some(&mut settings),
                );
            });
        });
    }
}

use crate::canvas::renderer::CanvasRenderer;
use crate::i18n::I18n;
use crate::io::settings::AppSettings;
use crate::model::frame::ActionPoint;
use crate::model::palette::{
    apply_palette_swap_to_atlas, generate_color_ramp, replace_color_in_atlas, PaletteSortCriterion,
    PaletteSwap, PREDEFINED_PALETTES,
};
use crate::model::project::Project;
use crate::model::sketch::{QuickPeekMode, SketchLayerMode};
use crate::theme::*;
use crate::tools::ToolSettings;
use crate::ui::color_picker::modern_color_picker_button;
use egui::{Color32, Context, Pos2, Rect, ScrollArea, SidePanel, Stroke, Ui, Vec2};
use std::time::Duration;

#[derive(Default)]
pub struct RightDockActions {
    pub trigger_load_sketch: bool,
    pub trigger_clear_sketch: bool,
    pub trigger_bake_sketch: bool,
    pub trigger_import_palette: bool,
    pub trigger_export_palette_png: bool,
    pub trigger_export_palette_hex: bool,
    pub trigger_export_palette_gpl: bool,
    pub trigger_sort_palette: Option<PaletteSortCriterion>,
    pub trigger_remap_palette_modal: bool,
    pub request_delete_skin: Option<(usize, String)>,
    pub request_delete_palette_slot: Option<(usize, [u8; 4])>,
    pub request_delete_pivot_point: Option<(usize, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum PlaybackMode {
    #[default]
    Loop,
    PingPong,
    Once,
}


pub struct PreviewState {
    pub is_playing: bool,
    pub current_frame: usize,
    pub ping_pong_reverse: bool,
    pub accumulator_sec: f64,
    pub active_tag_index: Option<usize>,
    pub mode: PlaybackMode,
    pub speed_multiplier: f32,
    /// `None` = Fit to preview box, `Some(z)` = exact `1x`, `2x`, `4x` scale.
    pub preview_zoom: Option<f32>,
    pub motion_test: bool,
    pub scroll_speed_x: f32,
    pub scroll_speed_y: f32,
    pub scroll_offset: Vec2,
    pub selected_palette_slot: usize,
    pub selected_preset_index: usize,
    pub ramp_start_slot: usize,
    pub ramp_end_slot: usize,
    pub ramp_steps: u32,
    pub ramp_hue_shift: f32,
    /// `None` = clicking with Pivot tool sets Origin, `Some(idx)` = sets `action_points[idx]`.
    pub active_pivot_target: Option<usize>,
}

impl Default for PreviewState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_frame: 0,
            ping_pong_reverse: false,
            accumulator_sec: 0.0,
            active_tag_index: None,
            mode: PlaybackMode::Loop,
            speed_multiplier: 1.0,
            preview_zoom: None,
            motion_test: false,
            scroll_speed_x: -32.0,
            scroll_speed_y: 0.0,
            scroll_offset: Vec2::ZERO,
            selected_palette_slot: 0,
            selected_preset_index: 0,
            ramp_start_slot: 0,
            ramp_end_slot: 5,
            ramp_steps: 3,
            ramp_hue_shift: 12.0,
            active_pivot_target: None,
        }
    }
}

impl PreviewState {
    pub fn playback_range(&self, project: &Project) -> (usize, usize) {
        let max_idx = (project.metadata.frame_count.max(1) as usize).saturating_sub(1);
        if let Some(tag_idx) = self.active_tag_index {
            if let Some(tag) = project.metadata.tags.get(tag_idx) {
                let start = tag.start_frame.min(max_idx);
                let end = tag.end_frame.min(max_idx).max(start);
                return (start, end);
            }
        }
        (0, max_idx)
    }

    pub fn tick(&mut self, ctx: &Context, project: &mut Project) {
        if let Some(tag_idx) = self.active_tag_index {
            if tag_idx >= project.metadata.tags.len() {
                self.active_tag_index = None;
            }
        }

        let (start_f, end_f) = self.playback_range(project);
        let dt = ctx.input(|i| i.stable_dt).clamp(0.0, 0.1);

        if self.motion_test {
            self.scroll_offset.x = (self.scroll_offset.x + self.scroll_speed_x * dt) % 1024.0;
            self.scroll_offset.y = (self.scroll_offset.y + self.scroll_speed_y * dt) % 1024.0;
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        if !self.is_playing {
            self.current_frame = project.active_frame.clamp(start_f, end_f);
            self.accumulator_sec = 0.0;
            return;
        }

        let fps = project.metadata.fps.max(1) as f32;
        let spd = self.speed_multiplier.max(0.05);
        let repaint_ms = ((1000.0 / (fps * spd)).floor() as u64).clamp(4, 16);
        ctx.request_repaint_after(Duration::from_millis(repaint_ms));

        if self.current_frame < start_f || self.current_frame > end_f {
            self.current_frame = start_f;
            self.ping_pong_reverse = false;
        }

        if start_f == end_f {
            self.current_frame = start_f;
            project.active_frame = start_f;
            return;
        }

        self.accumulator_sec += dt as f64;
        let fps = project.metadata.fps.max(1) as f64;
        let spd = (self.speed_multiplier as f64).max(0.05);

        let mut safety = 0;
        while safety < 16 {
            safety += 1;
            let hold = project
                .metadata
                .frames
                .get(self.current_frame)
                .map(|f| f.hold_multiplier.max(1))
                .unwrap_or(1) as f64;
            let frame_duration = hold / (fps * spd);

            if self.accumulator_sec < frame_duration {
                break;
            }
            self.accumulator_sec -= frame_duration;

            let tag_dir = self
                .active_tag_index
                .and_then(|idx| project.metadata.tags.get(idx))
                .map(|t| t.direction);

            let effective_mode = match tag_dir {
                Some(crate::model::frame::TagDirection::PingPong) => PlaybackMode::PingPong,
                _ => self.mode,
            };
            let is_reverse_tag =
                tag_dir == Some(crate::model::frame::TagDirection::Reverse);

            if is_reverse_tag {
                if self.current_frame <= start_f {
                    self.current_frame = end_f;
                } else {
                    self.current_frame -= 1;
                }
            } else {
                match effective_mode {
                    PlaybackMode::Loop => {
                        if self.current_frame >= end_f {
                            self.current_frame = start_f;
                        } else {
                            self.current_frame += 1;
                        }
                    }
                    PlaybackMode::PingPong => {
                        if !self.ping_pong_reverse {
                            if self.current_frame >= end_f {
                                self.ping_pong_reverse = true;
                                self.current_frame = end_f.saturating_sub(1).max(start_f);
                            } else {
                                self.current_frame += 1;
                            }
                        } else if self.current_frame <= start_f {
                            self.ping_pong_reverse = false;
                            self.current_frame = (start_f + 1).min(end_f);
                        } else {
                            self.current_frame -= 1;
                        }
                    }
                    PlaybackMode::Once => {
                        if self.current_frame >= end_f {
                            self.is_playing = false;
                            self.accumulator_sec = 0.0;
                            break;
                        } else {
                            self.current_frame += 1;
                        }
                    }
                }
            }
        }
        project.active_frame = self.current_frame;
    }

    pub fn toggle_play(&mut self, project: &mut Project) {
        if !self.is_playing {
            let (start_f, end_f) = self.playback_range(project);
            let tag_dir = self
                .active_tag_index
                .and_then(|idx| project.metadata.tags.get(idx))
                .map(|t| t.direction);

            if tag_dir == Some(crate::model::frame::TagDirection::Reverse) {
                if project.active_frame <= start_f || project.active_frame > end_f {
                    self.current_frame = end_f;
                } else {
                    self.current_frame = project.active_frame;
                }
            } else if (self.mode == PlaybackMode::Once && self.current_frame >= end_f)
                || project.active_frame < start_f
                || project.active_frame > end_f
            {
                self.current_frame = start_f;
            } else {
                self.current_frame = project.active_frame;
            }
            self.accumulator_sec = 0.0;
            self.ping_pong_reverse = false;
            self.is_playing = true;
            project.active_frame = self.current_frame;
        } else {
            self.is_playing = false;
        }
    }

    pub fn step_prev(&mut self, project: &mut Project) {
        self.is_playing = false;
        let (start_f, end_f) = self.playback_range(project);
        let cur = project.active_frame.clamp(start_f, end_f);
        let prev = if cur <= start_f { end_f } else { cur - 1 };
        project.active_frame = prev;
        self.current_frame = prev;
    }

    pub fn step_next(&mut self, project: &mut Project) {
        self.is_playing = false;
        let (start_f, end_f) = self.playback_range(project);
        let cur = project.active_frame.clamp(start_f, end_f);
        let next = if cur >= end_f { start_f } else { cur + 1 };
        project.active_frame = next;
        self.current_frame = next;
    }
}

fn draw_scrolling_motion_backdrop(painter: &egui::Painter, rect: Rect, scroll_offset: Vec2) {
    let c1 = Color32::from_rgb(24, 24, 24);
    let c2 = Color32::from_rgb(33, 33, 33);
    let grid_line = Stroke::new(1.0, Color32::from_rgb(48, 48, 48));

    painter.rect_filled(rect, 3.0, c1);

    let cell = 16.0_f32;
    let ox = scroll_offset.x.rem_euclid(cell * 2.0);
    let oy = scroll_offset.y.rem_euclid(cell * 2.0);

    let cols = (rect.width() / cell).ceil() as i32 + 3;
    let rows = (rect.height() / cell).ceil() as i32 + 3;

    for r in -2..rows {
        for c in -2..cols {
            if (r + c).rem_euclid(2) == 0 {
                let min = Pos2::new(
                    rect.min.x + c as f32 * cell + ox - cell * 2.0,
                    rect.min.y + r as f32 * cell + oy - cell * 2.0,
                );
                let tile = Rect::from_min_size(min, Vec2::splat(cell)).intersect(rect);
                if tile.is_positive() {
                    painter.rect_filled(tile, 0.0, c2);
                }
            }
        }
    }

    // Draw scrolling reference grid lines
    for c in -2..cols {
        let sx = rect.min.x + c as f32 * cell + ox.rem_euclid(cell) - cell;
        if sx >= rect.min.x && sx <= rect.max.x {
            painter.line_segment([Pos2::new(sx, rect.min.y), Pos2::new(sx, rect.max.y)], grid_line);
        }
    }
    for r in -2..rows {
        let sy = rect.min.y + r as f32 * cell + oy.rem_euclid(cell) - cell;
        if sy >= rect.min.y && sy <= rect.max.y {
            painter.line_segment([Pos2::new(rect.min.x, sy), Pos2::new(rect.max.x, sy)], grid_line);
        }
    }
}

pub fn show_right_dock(
    ctx: &Context,
    i18n: &I18n,
    project: &mut Project,
    tools: &mut ToolSettings,
    preview: &mut PreviewState,
    renderer: &CanvasRenderer,
    mut settings: Option<&mut AppSettings>,
) -> RightDockActions {
    let mut actions = RightDockActions::default();

    SidePanel::right("onianvil_right_utility_dock")
        .resizable(true)
        .default_width(275.0)
        .width_range(235.0..=380.0)
        .show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // 1. Live Animation Preview Panel
                egui::CollapsingHeader::new(format!("{FA_PLAY} {}", i18n.tr("panel.preview.title")))
                    .default_open(true)
                    .show(ui, |ui| {
                        let preview_box_h = 128.0;
                        let (rect, _) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), preview_box_h),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter().with_clip_rect(rect);

                        if preview.motion_test {
                            draw_scrolling_motion_backdrop(&painter, rect, preview.scroll_offset);
                        } else {
                            painter.rect_filled(rect, 3.0, Color32::from_rgb(20, 20, 20));
                        }

                        painter.rect_stroke(
                            rect,
                            3.0,
                            Stroke::new(1.0, Color32::from_rgb(52, 52, 52)),
                            egui::StrokeKind::Inside,
                        );

                        let display_frame = if preview.is_playing {
                            preview.current_frame
                        } else {
                            project.active_frame
                        };

                        if let Some(tex) = renderer.atlas_texture() {
                            let fw = project.metadata.frame_width.max(1) as f32;
                            let fh = project.metadata.frame_height.max(1) as f32;
                            let fit_scale = ((rect.width() - 12.0) / fw)
                                .min((rect.height() - 12.0) / fh)
                                .max(0.5);
                            let scale = match preview.preview_zoom {
                                Some(exact) => exact,
                                None => fit_scale,
                            };
                            let draw_size = Vec2::new(fw * scale, fh * scale);
                            let draw_rect = Rect::from_center_size(rect.center(), draw_size);
                            let uv = CanvasRenderer::frame_uv(project, display_frame);
                            painter.image(tex.id(), draw_rect, uv, Color32::WHITE);
                        }

                        // Frame stepping & Play/Pause controls
                        ui.horizontal(|ui| {
                            if ui
                                .button(FA_STEP_PREV)
                                .on_hover_text(i18n.tr("tooltip.preview_prev"))
                                .clicked()
                            {
                                preview.step_prev(project);
                            }
                            let play_label = if preview.is_playing {
                                FA_PAUSE
                            } else {
                                FA_PLAY
                            };
                            if ui
                                .button(play_label)
                                .on_hover_text(i18n.tr("tooltip.preview_play"))
                                .clicked()
                            {
                                preview.toggle_play(project);
                            }
                            if ui
                                .button(FA_STEP_NEXT)
                                .on_hover_text(i18n.tr("tooltip.preview_next"))
                                .clicked()
                            {
                                preview.step_next(project);
                            }

                            ui.separator();
                            ui.small(format!(
                                "#{}/{}",
                                display_frame + 1,
                                project.metadata.frame_count
                            ));
                        });

                        // Independent Preview Zoom (Fit, 1x, 2x, 4x)
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.preview.zoom"));
                            if ui
                                .selectable_label(
                                    preview.preview_zoom.is_none(),
                                    format!("{FA_FIT} {}", i18n.tr("panel.preview.fit")),
                                )
                                .on_hover_text(i18n.tr("tooltip.preview_zoom"))
                                .clicked()
                            {
                                preview.preview_zoom = None;
                            }
                            for z in [1.0_f32, 2.0, 4.0] {
                                let selected = preview
                                    .preview_zoom
                                    .map(|cur| (cur - z).abs() < 0.01)
                                    .unwrap_or(false);
                                if ui
                                    .selectable_label(selected, format!("{}x", z as u32))
                                    .on_hover_text(i18n.tr("tooltip.preview_zoom"))
                                    .clicked()
                                {
                                    preview.preview_zoom = Some(z);
                                }
                            }
                        });

                        // Playback Mode & Speed Multiplier
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.preview.mode"));
                            egui::ComboBox::from_id_salt("preview_playback_mode_combo")
                                .width(85.0)
                                .selected_text(match preview.mode {
                                    PlaybackMode::Loop => i18n.tr("panel.preview.loop"),
                                    PlaybackMode::PingPong => i18n.tr("panel.preview.ping_pong"),
                                    PlaybackMode::Once => i18n.tr("panel.preview.once"),
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut preview.mode,
                                        PlaybackMode::Loop,
                                        i18n.tr("panel.preview.loop"),
                                    );
                                    ui.selectable_value(
                                        &mut preview.mode,
                                        PlaybackMode::PingPong,
                                        i18n.tr("panel.preview.ping_pong"),
                                    );
                                    ui.selectable_value(
                                        &mut preview.mode,
                                        PlaybackMode::Once,
                                        i18n.tr("panel.preview.once"),
                                    );
                                })
                                .response
                                .on_hover_text(i18n.tr("tooltip.preview_mode"));

                            ui.label(i18n.tr("panel.preview.speed"));
                            egui::ComboBox::from_id_salt("preview_speed_combo")
                                .width(58.0)
                                .selected_text(format!("{}x", preview.speed_multiplier))
                                .show_ui(ui, |ui| {
                                    for spd in [0.1_f32, 0.25, 0.5, 1.0, 2.0] {
                                        ui.selectable_value(
                                            &mut preview.speed_multiplier,
                                            spd,
                                            format!("{}x", spd),
                                        );
                                    }
                                });
                        });

                        // Active Tag Filter for Preview
                        if !project.metadata.tags.is_empty() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{FA_TAG} {}", i18n.tr("panel.preview.tag_filter")));
                                let selected_tag_label = match preview.active_tag_index {
                                    Some(idx) => project
                                        .metadata
                                        .tags
                                        .get(idx)
                                        .map(|t| {
                                            format!(
                                                "{} ({}-{})",
                                                t.name,
                                                t.start_frame + 1,
                                                t.end_frame + 1
                                            )
                                        })
                                        .unwrap_or_else(|| {
                                            i18n.tr("panel.preview.all_frames").to_string()
                                        }),
                                    None => i18n.tr("panel.preview.all_frames").to_string(),
                                };
                                egui::ComboBox::from_id_salt("preview_active_tag_combo")
                                    .width(145.0)
                                    .selected_text(selected_tag_label)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut preview.active_tag_index,
                                            None,
                                            i18n.tr("panel.preview.all_frames"),
                                        );
                                        for (idx, tag) in project.metadata.tags.iter().enumerate() {
                                            let label = format!(
                                                "{} ({}-{})",
                                                tag.name,
                                                tag.start_frame + 1,
                                                tag.end_frame + 1
                                            );
                                            ui.selectable_value(
                                                &mut preview.active_tag_index,
                                                Some(idx),
                                                label,
                                            );
                                        }
                                    });
                            });
                        }

                        ui.checkbox(&mut preview.motion_test, i18n.tr("panel.preview.motion_test"))
                            .on_hover_text(i18n.tr("tooltip.preview_motion"));
                        if preview.motion_test {
                            ui.horizontal(|ui| {
                                ui.small(i18n.tr("panel.preview.scroll_x"));
                                ui.add(
                                    egui::DragValue::new(&mut preview.scroll_speed_x)
                                        .range(-256.0..=256.0),
                                );
                                ui.small(i18n.tr("panel.preview.scroll_y"));
                                ui.add(
                                    egui::DragValue::new(&mut preview.scroll_speed_y)
                                        .range(-256.0..=256.0),
                                );
                            });
                        }
                    });

                ui.separator();

                // 2. Palette & Swaps Panel
                egui::CollapsingHeader::new(format!("{FA_PALETTE} {}", i18n.tr("panel.palette.title")))
                    .default_open(true)
                    .show(ui, |ui| {
                        // Predefined Palette Selector
                        let preset_idx = preview
                            .selected_preset_index
                            .min(PREDEFINED_PALETTES.len().saturating_sub(1));
                        ui.horizontal(|ui| {
                            ui.small(i18n.tr("panel.palette.preset"));
                            let combo_w = (ui.available_width() - 85.0).max(90.0);
                            egui::ComboBox::from_id_salt("palette_preset_combo")
                                .width(combo_w)
                                .height(260.0)
                                .selected_text(PREDEFINED_PALETTES[preset_idx].name)
                                .show_ui(ui, |ui| {
                                    for (idx, p) in PREDEFINED_PALETTES.iter().enumerate() {
                                        ui.selectable_value(
                                            &mut preview.selected_preset_index,
                                            idx,
                                            p.name,
                                        );
                                    }
                                })
                                .response
                                .on_hover_text(i18n.tr("tooltip.palette_preset"));

                            if ui
                                .small_button(format!("{FA_PALETTE} {}", i18n.tr("panel.palette.apply_preset")))
                                .on_hover_text(i18n.tr("tooltip.palette_preset"))
                                .clicked()
                            {
                                let chosen = &PREDEFINED_PALETTES[preview
                                    .selected_preset_index
                                    .min(PREDEFINED_PALETTES.len().saturating_sub(1))];
                                let cols = chosen.to_rgba_vec();
                                if !cols.is_empty() {
                                    project.metadata.palette.colors = cols.clone();
                                    let active_s = project.metadata.palette.active_swap_index;
                                    if let Some(swap) =
                                        project.metadata.palette.swaps.get_mut(active_s)
                                    {
                                        swap.colors = cols;
                                    }
                                    tools.fg_color = project.metadata.palette.colors[0];
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            let count = project.metadata.palette.colors.len();
                            let limit_str = match project.metadata.target_color_limit {
                                Some(lim) => lim.to_string(),
                                None => i18n.tr("panel.palette.unlimited").to_string(),
                            };
                            ui.small(format!(
                                "{} {}  •  {} {}",
                                i18n.tr("panel.palette.colors_count"),
                                count,
                                i18n.tr("panel.palette.limit"),
                                limit_str
                            ));
                        });

                        ui.add_space(2.0);

                        // Actions: Symmetrical 2x2 grid without horizontal blowout
                        ui.columns(2, |cols| {
                            if cols[0]
                                .add_sized(
                                    [cols[0].available_width(), 20.0],
                                    egui::Button::new(format!("{FA_PLUS} {}", i18n.tr("panel.palette.add_color"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_add_fg"))
                                .clicked()
                                && !project.metadata.palette.colors.contains(&tools.fg_color)
                            {
                                project.metadata.palette.colors.push(tools.fg_color);
                                for swap in &mut project.metadata.palette.swaps {
                                    swap.colors.push(tools.fg_color);
                                }
                            }

                            if cols[1]
                                .add_sized(
                                    [cols[1].available_width(), 20.0],
                                    egui::Button::new(format!("{FA_MAGIC_WAND} {}", i18n.tr("panel.palette.extract"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_extract"))
                                .clicked()
                            {
                                let extracted = project.extract_unique_colors(64);
                                if !extracted.is_empty() {
                                    project.metadata.palette.colors = extracted.clone();
                                    for swap in &mut project.metadata.palette.swaps {
                                        swap.colors = extracted.clone();
                                    }
                                }
                            }
                        });

                        ui.columns(2, |cols| {
                            if cols[0]
                                .add_sized(
                                    [cols[0].available_width(), 20.0],
                                    egui::Button::new(format!("{FA_IMPORT} {}", i18n.tr("panel.palette.import_btn"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_import"))
                                .clicked()
                            {
                                actions.trigger_import_palette = true;
                            }

                            let export_btn = cols[1]
                                .add_sized(
                                    [cols[1].available_width(), 20.0],
                                    egui::Button::new(format!(
                                        "{FA_EXPORT} {}",
                                        i18n.tr("panel.palette.export_btn")
                                    )),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_export_hex"));
                            let popup_id = cols[1].make_persistent_id("palette_export_menu_popup");
                            if export_btn.clicked() {
                                cols[1].memory_mut(|mem| mem.toggle_popup(popup_id));
                            }
                            egui::popup_below_widget(
                                &cols[1],
                                popup_id,
                                &export_btn,
                                egui::PopupCloseBehavior::CloseOnClickOutside,
                                |ui| {
                                    ui.set_min_width(130.0);
                                    if ui
                                        .button(format!(
                                            "{FA_VIEW_FRAME} {}",
                                            i18n.tr("panel.palette.export_png")
                                        ))
                                        .on_hover_text(i18n.tr("tooltip.palette_export_png"))
                                        .clicked()
                                    {
                                        actions.trigger_export_palette_png = true;
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                    if ui
                                        .button(format!(
                                            "{FA_PALETTE} {}",
                                            i18n.tr("panel.palette.export")
                                        ))
                                        .on_hover_text(i18n.tr("tooltip.palette_export_hex"))
                                        .clicked()
                                    {
                                        actions.trigger_export_palette_hex = true;
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                    if ui
                                        .button(format!(
                                            "{FA_TAG} GPL ({})",
                                            i18n.tr("menu.file.export_palette_gpl")
                                        ))
                                        .clicked()
                                    {
                                        actions.trigger_export_palette_gpl = true;
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                },
                            );
                        });

                        ui.columns(2, |cols| {
                            let sort_btn = cols[0]
                                .add_sized(
                                    [cols[0].available_width(), 20.0],
                                    egui::Button::new(format!("{FA_SORT} {}", i18n.tr("panel.palette.sort_btn"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_sort"));
                            let sort_popup_id = cols[0].make_persistent_id("palette_sort_menu_popup");
                            if sort_btn.clicked() {
                                cols[0].memory_mut(|mem| mem.toggle_popup(sort_popup_id));
                            }
                            egui::popup_below_widget(
                                &cols[0],
                                sort_popup_id,
                                &sort_btn,
                                egui::PopupCloseBehavior::CloseOnClickOutside,
                                |ui| {
                                    ui.set_min_width(140.0);
                                    if ui.button(i18n.tr("panel.palette.sort_hue")).clicked() {
                                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Hue);
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                    if ui.button(i18n.tr("panel.palette.sort_luminance")).clicked() {
                                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Luminance);
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                    if ui.button(i18n.tr("panel.palette.sort_saturation")).clicked() {
                                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Saturation);
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                    ui.separator();
                                    if ui.button(i18n.tr("panel.palette.sort_reverse")).clicked() {
                                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Reverse);
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                },
                            );

                            if cols[1]
                                .add_sized(
                                    [cols[1].available_width(), 20.0],
                                    egui::Button::new(format!("{FA_PALETTE} {}", i18n.tr("panel.palette.remap_btn"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.palette_remap"))
                                .clicked()
                            {
                                actions.trigger_remap_palette_modal = true;
                            }
                        });

                        ui.add_space(3.0);
                        let pal_len = project.metadata.palette.colors.len();
                        if pal_len > 0 {
                            preview.selected_palette_slot =
                                preview.selected_palette_slot.min(pal_len - 1);
                        }

                        // Sunken well for swatches with vertical scrolling if many colors
                        egui::Frame::canvas(ui.style())
                            .fill(ui.visuals().extreme_bg_color)
                            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                            .corner_radius(4.0)
                            .inner_margin(egui::Margin::same(5))
                            .show(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .max_height(140.0)
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.spacing_mut().item_spacing = Vec2::splat(3.0);
                                            for (idx, color) in project.metadata.palette.colors.iter().enumerate() {
                                                let [r, g, b, a] = *color;
                                                let c32 = Color32::from_rgba_unmultiplied(r, g, b, a);
                                                let (rect, resp) = ui
                                                    .allocate_exact_size(Vec2::splat(19.0), egui::Sense::click());
                                                ui.painter().rect_filled(rect, 2.0, c32);

                                                let is_selected = preview.selected_palette_slot == idx;
                                                let stroke_col = if is_selected {
                                                    ui.visuals().widgets.active.bg_fill
                                                } else {
                                                    ui.visuals().widgets.noninteractive.bg_stroke.color
                                                };
                                                ui.painter().rect_stroke(
                                                    rect,
                                                    2.0,
                                                    Stroke::new(if is_selected { 2.0 } else { 1.0 }, stroke_col),
                                                    egui::StrokeKind::Inside,
                                                );

                                                if is_selected {
                                                    let lum = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
                                                    let inner_col = if lum > 140 { Color32::BLACK } else { Color32::WHITE };
                                                    ui.painter().rect_stroke(
                                                        rect.shrink(1.5),
                                                        1.0,
                                                        Stroke::new(1.0, inner_col),
                                                        egui::StrokeKind::Inside,
                                                    );
                                                }

                                                let resp = resp.on_hover_text(format!(
                                                    "#{:02X}{:02X}{:02X}\n{}",
                                                    r,
                                                    g,
                                                    b,
                                                    i18n.tr("tooltip.palette_slot")
                                                ));
                                                if resp.clicked() {
                                                    preview.selected_palette_slot = idx;
                                                    tools.fg_color = *color;
                                                } else if resp.secondary_clicked() {
                                                    preview.selected_palette_slot = idx;
                                                    tools.bg_color = *color;
                                                }
                                            }
                                        });
                                    });
                            });

                        // Active Slot Editor + Global Color Replace
                        if !project.metadata.palette.colors.is_empty() {
                            let slot_idx = preview
                                .selected_palette_slot
                                .min(project.metadata.palette.colors.len() - 1);
                            let old_col = project.metadata.palette.colors[slot_idx];
                            let mut edit_col = old_col;

                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                ui.small(format!(
                                    "{} #{}",
                                    i18n.tr("panel.palette.edit_slot"),
                                    slot_idx + 1
                                ));
                                if modern_color_picker_button(
                                    ui,
                                    ("palette_slot_col", slot_idx),
                                    &mut edit_col,
                                    i18n,
                                    settings.as_deref_mut(),
                                )
                                .changed()
                                {
                                    project.metadata.palette.colors[slot_idx] = edit_col;
                                    tools.fg_color = edit_col;
                                    let active_s = project.metadata.palette.active_swap_index;
                                    if let Some(swap) =
                                        project.metadata.palette.swaps.get_mut(active_s)
                                    {
                                        if slot_idx < swap.colors.len() {
                                            swap.colors[slot_idx] = edit_col;
                                        }
                                    }
                                    if project.metadata.palette.global_color_replace
                                        && replace_color_in_atlas(
                                            &mut project.atlas_rgba,
                                            old_col,
                                            edit_col,
                                        )
                                    {
                                        project.canvas_dirty = true;
                                    }
                                }
                                if project.metadata.palette.colors.len() > 1
                                    && ui
                                        .small_button(format!("{FA_TRASH} {}", i18n.tr("panel.palette.del_slot")))
                                        .on_hover_text(i18n.tr("tooltip.palette_del_slot"))
                                        .clicked()
                                {
                                    actions.request_delete_palette_slot = Some((slot_idx, old_col));
                                }
                            });
                        }

                        ui.checkbox(
                            &mut project.metadata.palette.global_color_replace,
                            i18n.tr("panel.palette.global_replace"),
                        )
                        .on_hover_text(i18n.tr("tooltip.palette_global_replace"));

                        ui.separator();

                        // Color Ramp Generator (Collapsible)
                        egui::CollapsingHeader::new(format!("{FA_SLIDERS} {}", i18n.tr("panel.palette.ramp_title")))
                            .default_open(false)
                            .show(ui, |ui| {
                                let max_slot_1 = project.metadata.palette.colors.len().max(1);
                                let mut from_1 = (preview.ramp_start_slot + 1).clamp(1, max_slot_1);
                                let mut to_1 = (preview.ramp_end_slot + 1).clamp(1, max_slot_1);
                                ui.horizontal(|ui| {
                                    ui.small(i18n.tr("panel.palette.ramp_from"));
                                    if ui
                                        .add(egui::DragValue::new(&mut from_1).range(1..=max_slot_1))
                                        .changed()
                                    {
                                        preview.ramp_start_slot = from_1 - 1;
                                    }
                                    ui.small(i18n.tr("panel.palette.ramp_to"));
                                    if ui
                                        .add(egui::DragValue::new(&mut to_1).range(1..=max_slot_1))
                                        .changed()
                                    {
                                        preview.ramp_end_slot = to_1 - 1;
                                    }
                                    ui.small(i18n.tr("panel.palette.ramp_steps"));
                                    ui.add(egui::DragValue::new(&mut preview.ramp_steps).range(1..=16));
                                });
                                ui.horizontal(|ui| {
                                    ui.small(i18n.tr("panel.palette.ramp_hue"));
                                    ui.add(
                                        egui::DragValue::new(&mut preview.ramp_hue_shift)
                                            .range(-90.0..=90.0)
                                            .suffix("°"),
                                    );
                                    if ui
                                        .small_button(format!("{FA_SLIDERS} {}", i18n.tr("panel.palette.ramp")))
                                        .on_hover_text(i18n.tr("tooltip.palette_ramp"))
                                        .clicked()
                                        && project.metadata.palette.colors.len() >= 2
                                    {
                                        let c_start = project.metadata.palette.colors[preview
                                            .ramp_start_slot
                                            .min(project.metadata.palette.colors.len() - 1)];
                                        let c_end = project.metadata.palette.colors[preview
                                            .ramp_end_slot
                                            .min(project.metadata.palette.colors.len() - 1)];
                                        let ramp = generate_color_ramp(
                                            c_start,
                                            c_end,
                                            preview.ramp_steps,
                                            preview.ramp_hue_shift,
                                        );
                                        for rc in ramp {
                                            if !project.metadata.palette.colors.contains(&rc) {
                                                project.metadata.palette.colors.push(rc);
                                                for swap in &mut project.metadata.palette.swaps {
                                                    swap.colors.push(rc);
                                                }
                                            }
                                        }
                                    }
                                });
                            });

                        ui.separator();

                        // Palette Swaps (Skins) (Collapsible)
                        egui::CollapsingHeader::new(format!("{FA_SWAP} {}", i18n.tr("panel.palette.swaps_title")))
                            .default_open(true)
                            .show(ui, |ui| {
                                let prev_idx = project
                                    .metadata
                                    .palette
                                    .active_swap_index
                                    .min(project.metadata.palette.swaps.len().saturating_sub(1));
                                let mut next_idx = prev_idx;
                                let active_name = project
                                    .metadata
                                    .palette
                                    .swaps
                                    .get(prev_idx)
                                    .map(|s| s.name.clone())
                                    .unwrap_or_else(|| {
                                        i18n.tr("panel.palette.default_skin").to_string()
                                    });

                                ui.horizontal(|ui| {
                                    ui.small(i18n.tr("panel.palette.skin_select"));
                                    let btn_w = if project.metadata.palette.swaps.len() > 1 {
                                        56.0
                                    } else {
                                        28.0
                                    };
                                    let combo_w = (ui.available_width() - btn_w).max(70.0);
                                    egui::ComboBox::from_id_salt("palette_swaps_combo")
                                        .width(combo_w)
                                        .height(260.0)
                                        .selected_text(&active_name)
                                        .show_ui(ui, |ui| {
                                            for (idx, swap) in
                                                project.metadata.palette.swaps.iter().enumerate()
                                            {
                                                ui.selectable_value(&mut next_idx, idx, &swap.name);
                                            }
                                        });

                                    if ui
                                        .small_button(FA_PLUS)
                                        .on_hover_text(i18n.tr("tooltip.palette_new_skin"))
                                        .clicked()
                                    {
                                        let next_num = project.metadata.palette.swaps.len() + 1;
                                        project.metadata.palette.swaps.push(PaletteSwap {
                                            name: format!(
                                                "{} {}",
                                                i18n.tr("panel.palette.new_skin"),
                                                next_num
                                            ),
                                            colors: project.metadata.palette.colors.clone(),
                                        });
                                        project.metadata.palette.active_swap_index =
                                            project.metadata.palette.swaps.len() - 1;
                                    }

                                    if project.metadata.palette.swaps.len() > 1
                                        && ui
                                            .small_button(FA_TRASH)
                                            .on_hover_text(i18n.tr("tooltip.palette_del_skin"))
                                            .clicked()
                                    {
                                        actions.request_delete_skin =
                                            Some((prev_idx, active_name.clone()));
                                    }
                                });

                                if next_idx != prev_idx {
                                    let old_colors = project.metadata.palette.colors.clone();
                                    if let Some(target_swap) =
                                        project.metadata.palette.swaps.get(next_idx)
                                    {
                                        let new_colors = target_swap.colors.clone();
                                        if apply_palette_swap_to_atlas(
                                            &mut project.atlas_rgba,
                                            &old_colors,
                                            &new_colors,
                                        ) {
                                            project.canvas_dirty = true;
                                        }
                                        project.metadata.palette.colors = new_colors;
                                        project.metadata.palette.active_swap_index = next_idx;
                                    }
                                }

                                // Skin Renaming Row
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    ui.small(i18n.tr("panel.palette.skin_name"));
                                    let cur_s = project
                                        .metadata
                                        .palette
                                        .active_swap_index
                                        .min(project.metadata.palette.swaps.len().saturating_sub(1));
                                    if let Some(swap) =
                                        project.metadata.palette.swaps.get_mut(cur_s)
                                    {
                                        ui.add_sized(
                                            [ui.available_width(), 19.0],
                                            egui::TextEdit::singleline(&mut swap.name)
                                                .hint_text(i18n.tr("panel.palette.skin_name_hint")),
                                        );
                                    }
                                });
                            });
                    });

                ui.separator();

                // 3. Sketch Reference & Onion Skin Panel
                egui::CollapsingHeader::new(format!("{FA_EYE} {}", i18n.tr("panel.sketch.title")))
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .button(format!("{FA_FOLDER} {}", i18n.tr("panel.sketch.load")))
                                .on_hover_text(i18n.tr("tooltip.sketch_load"))
                                .clicked()
                            {
                                actions.trigger_load_sketch = true;
                            }
                            if ui
                                .add_enabled(
                                    project.sketch_rgba.is_some(),
                                    egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("panel.sketch.clear"))),
                                )
                                .on_hover_text(i18n.tr("tooltip.sketch_clear"))
                                .clicked()
                            {
                                actions.trigger_clear_sketch = true;
                            }
                        });

                        ui.checkbox(
                            &mut project.metadata.sketch.visible,
                            i18n.tr("panel.sketch.visible"),
                        )
                        .on_hover_text(i18n.tr("tooltip.sketch_visible"));

                        ui.checkbox(
                            &mut project.metadata.sketch.bilinear_filter,
                            i18n.tr("panel.sketch.bilinear"),
                        )
                        .on_hover_text(i18n.tr("tooltip.sketch_bilinear"));

                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.sketch.layer_mode"));
                            ui.selectable_value(
                                &mut project.metadata.sketch.layer_mode,
                                SketchLayerMode::Underlay,
                                i18n.tr("panel.sketch.underlay"),
                            )
                            .on_hover_text(i18n.tr("tooltip.sketch_underlay"));
                            ui.selectable_value(
                                &mut project.metadata.sketch.layer_mode,
                                SketchLayerMode::Overlay,
                                i18n.tr("panel.sketch.overlay"),
                            )
                            .on_hover_text(i18n.tr("tooltip.sketch_overlay"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.sketch.opacity"));
                            ui.add(egui::Slider::new(
                                &mut project.metadata.sketch.opacity,
                                0.0..=1.0,
                            ));
                        });
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.sketch.scale"));
                            ui.add(
                                egui::Slider::new(&mut project.metadata.sketch.scale, 0.1..=10.0)
                                    .logarithmic(true),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.sketch.rotation"));
                            ui.add(
                                egui::Slider::new(
                                    &mut project.metadata.sketch.rotation_deg,
                                    -180.0..=180.0,
                                )
                                .suffix("°"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.small(i18n.tr("panel.sketch.offset_x"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.sketch.offset_x)
                                    .speed(0.5)
                                    .range(-1024.0..=1024.0),
                            );
                            ui.small(i18n.tr("panel.sketch.offset_y"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.sketch.offset_y)
                                    .speed(0.5)
                                    .range(-1024.0..=1024.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.toggle_value(
                                &mut project.metadata.sketch.flip_h,
                                format!("{FA_FLIP_H} {}", i18n.tr("panel.sketch.flip_h")),
                            )
                            .on_hover_text(i18n.tr("tooltip.sketch_flip_h"));
                            ui.toggle_value(
                                &mut project.metadata.sketch.flip_v,
                                format!("{FA_FLIP_V} {}", i18n.tr("panel.sketch.flip_v")),
                            )
                            .on_hover_text(i18n.tr("tooltip.sketch_flip_v"));
                            if ui
                                .small_button(format!("{FA_RESET} {}", i18n.tr("panel.sketch.reset")))
                                .on_hover_text(i18n.tr("tooltip.sketch_reset"))
                                .clicked()
                            {
                                project.metadata.sketch.reset_transform();
                            }
                        });

                        ui.checkbox(
                            &mut project.metadata.sketch.snap_sample_to_palette,
                            i18n.tr("panel.sketch.snap_palette"),
                        )
                        .on_hover_text(i18n.tr("tooltip.sketch_snap"));
                        ui.checkbox(
                            &mut project.metadata.sketch.embed_in_json,
                            i18n.tr("panel.sketch.embed_base64"),
                        )
                        .on_hover_text(i18n.tr("tooltip.sketch_embed"));

                        if ui
                            .add_enabled(
                                project.sketch_rgba.is_some(),
                                egui::Button::new(format!("{FA_BAKE} {}", i18n.tr("panel.sketch.bake"))),
                            )
                            .on_hover_text(i18n.tr("tooltip.sketch_bake"))
                            .clicked()
                        {
                            actions.trigger_bake_sketch = true;
                        }

                        ui.separator();
                        ui.checkbox(
                            &mut project.metadata.onion_skin.enabled,
                            i18n.tr("panel.onion.enabled"),
                        )
                        .on_hover_text(i18n.tr("tooltip.onion_enable"));
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.onion.prev_frames"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.onion_skin.prev_frames)
                                    .range(0..=5),
                            );
                            ui.label(i18n.tr("panel.onion.next_frames"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.onion_skin.next_frames)
                                    .range(0..=5),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.onion.opacity"));
                            ui.add(egui::Slider::new(
                                &mut project.metadata.onion_skin.opacity,
                                0.05..=0.9,
                            ));
                        });
                        ui.checkbox(
                            &mut project.metadata.onion_skin.color_tint,
                            i18n.tr("panel.onion.tint"),
                        )
                        .on_hover_text(i18n.tr("tooltip.onion_tint"));
                        ui.horizontal(|ui| {
                            ui.small(i18n.tr("context.quick_peek"));
                            egui::ComboBox::from_id_salt("panel_quick_peek_combo")
                                .width(120.0)
                                .selected_text(match project.metadata.onion_skin.quick_peek_mode {
                                    QuickPeekMode::PreviousFrame => {
                                        i18n.tr("context.peek_prev_frame")
                                    }
                                    QuickPeekMode::SketchOverlay => i18n.tr("context.peek_sketch"),
                                    QuickPeekMode::Both => i18n.tr("context.peek_both"),
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut project.metadata.onion_skin.quick_peek_mode,
                                        QuickPeekMode::PreviousFrame,
                                        i18n.tr("context.peek_prev_frame"),
                                    );
                                    ui.selectable_value(
                                        &mut project.metadata.onion_skin.quick_peek_mode,
                                        QuickPeekMode::SketchOverlay,
                                        i18n.tr("context.peek_sketch"),
                                    );
                                    ui.selectable_value(
                                        &mut project.metadata.onion_skin.quick_peek_mode,
                                        QuickPeekMode::Both,
                                        i18n.tr("context.peek_both"),
                                    );
                                })
                                .response
                                .on_hover_text(i18n.tr("tooltip.quick_peek"));
                        });
                    });

                ui.separator();

                // 4. Pivot & Action Points Panel
                egui::CollapsingHeader::new(format!("{FA_PIVOT} {}", i18n.tr("panel.pivots.title")))
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.checkbox(
                            &mut project.metadata.pivot.show_gizmos,
                            i18n.tr("panel.pivots.show"),
                        )
                        .on_hover_text(i18n.tr("tooltip.pivot_show"));
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(
                                    preview.active_pivot_target.is_none(),
                                    i18n.tr("panel.pivots.target_origin"),
                                )
                                .on_hover_text(i18n.tr("tooltip.pivot_target"))
                                .clicked()
                            {
                                preview.active_pivot_target = None;
                            }
                            if ui
                                .small_button(i18n.tr("panel.pivots.center_bottom"))
                                .on_hover_text(i18n.tr("tooltip.pivot_bottom_center"))
                                .clicked()
                            {
                                project.metadata.pivot.origin_x =
                                    (project.metadata.frame_width / 2) as i32;
                                project.metadata.pivot.origin_y =
                                    project.metadata.frame_height as i32;
                            }
                            if ui
                                .small_button(i18n.tr("panel.pivots.center"))
                                .on_hover_text(i18n.tr("tooltip.pivot_center"))
                                .clicked()
                            {
                                project.metadata.pivot.origin_x =
                                    (project.metadata.frame_width / 2) as i32;
                                project.metadata.pivot.origin_y =
                                    (project.metadata.frame_height / 2) as i32;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label(i18n.tr("panel.pivots.origin_x"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.pivot.origin_x)
                                    .range(-512..=2048),
                            );
                            ui.label(i18n.tr("panel.pivots.origin_y"));
                            ui.add(
                                egui::DragValue::new(&mut project.metadata.pivot.origin_y)
                                    .range(-512..=2048),
                            );
                        });

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.small(i18n.tr("panel.pivots.action_points"));
                            if ui
                                .small_button(format!("{FA_PLUS} {}", i18n.tr("panel.pivots.add_point")))
                                .on_hover_text(i18n.tr("tooltip.pivot_add_point"))
                                .clicked()
                            {
                                project.metadata.pivot.action_points.push(ActionPoint {
                                    name: i18n.tr("panel.pivots.default_point_name").to_string(),
                                    x: (project.metadata.frame_width / 2) as i32,
                                    y: (project.metadata.frame_height / 2) as i32,
                                    color: [0, 220, 255, 255],
                                });
                                preview.active_pivot_target =
                                    Some(project.metadata.pivot.action_points.len() - 1);
                            }
                        });

                        for (idx, pt) in project.metadata.pivot.action_points.iter_mut().enumerate()
                        {
                            ui.horizontal(|ui| {
                                let is_target = preview.active_pivot_target == Some(idx);
                                if ui
                                    .selectable_label(is_target, FA_PIVOT)
                                    .on_hover_text(i18n.tr("tooltip.pivot_target"))
                                    .clicked()
                                {
                                    preview.active_pivot_target = if is_target {
                                        None
                                     } else {
                                        Some(idx)
                                    };
                                }
                                modern_color_picker_button(
                                    ui,
                                    ("attach_pt_col", idx),
                                    &mut pt.color,
                                    i18n,
                                    settings.as_deref_mut(),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(&mut pt.name).desired_width(72.0),
                                );
                                ui.add(egui::DragValue::new(&mut pt.x).range(-512..=2048));
                                ui.add(egui::DragValue::new(&mut pt.y).range(-512..=2048));
                                if ui
                                    .small_button(FA_TRASH)
                                    .on_hover_text(i18n.tr("tooltip.pivot_remove_point"))
                                    .clicked()
                                {
                                    actions.request_delete_pivot_point = Some((idx, pt.name.clone()));
                                }
                            });
                        }
                    });
            });
        });
    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::metadata::ProjectMetadata;
    use crate::model::project::Project;

    #[test]
    fn test_toggle_play_and_tick_advances_active_frame() {
        let meta = ProjectMetadata::new_config("anim".to_string(), 8, 8, 4, 4, None);
        let mut proj = Project::new(meta);
        assert_eq!(proj.metadata.frame_count, 4);
        assert_eq!(proj.active_frame, 0);

        let mut preview = PreviewState::default();
        let ctx = egui::Context::default();

        // 1. Initially stopped
        assert!(!preview.is_playing);
        preview.tick(&ctx, &mut proj);
        assert_eq!(proj.active_frame, 0);

        // 2. Start playback
        preview.toggle_play(&mut proj);
        assert!(preview.is_playing);
        assert_eq!(preview.current_frame, 0);
        assert_eq!(proj.active_frame, 0);

        // 3. Accumulate time past frame duration and tick
        proj.metadata.fps = 10;
        preview.accumulator_sec = 0.15;
        preview.tick(&ctx, &mut proj);
        assert_eq!(preview.current_frame, 1);
        assert_eq!(proj.active_frame, 1);

        // 4. Step prev / next while playing stops playback and updates active_frame
        preview.step_next(&mut proj);
        assert!(!preview.is_playing);
        assert_eq!(preview.current_frame, 2);
        assert_eq!(proj.active_frame, 2);

        preview.step_prev(&mut proj);
        assert!(!preview.is_playing);
        assert_eq!(preview.current_frame, 1);
        assert_eq!(proj.active_frame, 1);

        // 5. Toggle play resumes from active_frame
        preview.toggle_play(&mut proj);
        assert!(preview.is_playing);
        assert_eq!(preview.current_frame, 1);
        assert_eq!(proj.active_frame, 1);

        // 6. Toggle play again pauses
        preview.toggle_play(&mut proj);
        assert!(!preview.is_playing);
        assert_eq!(proj.active_frame, 1);
    }

    #[test]
    fn test_tag_direction_reverse_and_pingpong_playback() {
        let mut meta = ProjectMetadata::new_config("anim".to_string(), 8, 8, 4, 4, None);
        meta.tags[0] = crate::model::frame::FrameTag {
            name: "Backwards".to_string(),
            start_frame: 0,
            end_frame: 2,
            color: [255, 0, 0, 255],
            direction: crate::model::frame::TagDirection::Reverse,
        };
        let mut proj = Project::new(meta);
        let mut preview = PreviewState::default();
        let ctx = egui::Context::default();

        preview.active_tag_index = Some(0);
        preview.toggle_play(&mut proj);
        assert!(preview.is_playing);
        // Reverse tag begins at end_frame
        assert_eq!(proj.active_frame, 2);

        // Tick 1: 2 -> 1
        proj.metadata.fps = 10;
        preview.accumulator_sec = 0.15;
        preview.tick(&ctx, &mut proj);
        assert_eq!(proj.active_frame, 1);

        // Tick 2: 1 -> 0
        preview.accumulator_sec = 0.15;
        preview.tick(&ctx, &mut proj);
        assert_eq!(proj.active_frame, 0);

        // Tick 3: 0 -> 2 (loop to end_frame)
        preview.accumulator_sec = 0.15;
        preview.tick(&ctx, &mut proj);
        assert_eq!(proj.active_frame, 2);
    }
}


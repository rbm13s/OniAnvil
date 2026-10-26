use crate::canvas::renderer::CanvasRenderer;
use crate::history::command::{HistorySnapshot, HistoryStack};
use crate::i18n::I18n;
use crate::model::frame::{FrameTag, TagDirection};
use crate::model::project::Project;
use crate::theme::*;
use crate::ui::color_picker::modern_color_picker_button;
use crate::ui::right_dock::PreviewState;
use egui::{Color32, Context, ScrollArea, Stroke, TopBottomPanel, Ui, Vec2};

#[derive(Default)]
pub struct BottomDockActions {
    pub trigger_import_single_frame: Option<usize>,
    pub trigger_export_single_frame: Option<usize>,
    pub request_delete_frame: Option<usize>,
    pub request_delete_tag: Option<(usize, String)>,
    pub request_delete_layer: Option<(usize, String)>,
}

pub fn show_bottom_dock(
    ctx: &Context,
    i18n: &I18n,
    project: &mut Project,
    history: &mut HistoryStack,
    preview: &mut PreviewState,
    renderer: &CanvasRenderer,
) -> BottomDockActions {
    let mut actions = BottomDockActions::default();
    project.ensure_layers_initialized();

    TopBottomPanel::bottom("onianvil_bottom_timeline_dock")
        .resizable(true)
        .default_height(235.0)
        .height_range(135.0..=720.0)
        .show(ctx, |ui: &mut Ui| {
            let mut deferred_add_layer = false;

            // Row 1: Playback transport, FPS, Frame & Layer operations, and Hold multiplier
            ui.horizontal(|ui| {
                ui.strong(i18n.tr("timeline.title"));
                ui.separator();

                if ui
                    .small_button(FA_STEP_PREV)
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
                    .small_button(play_label)
                    .on_hover_text(i18n.tr("tooltip.preview_play"))
                    .clicked()
                {
                    preview.toggle_play(project);
                }
                if ui
                    .small_button(FA_STEP_NEXT)
                    .on_hover_text(i18n.tr("tooltip.preview_next"))
                    .clicked()
                {
                    preview.step_next(project);
                }

                ui.separator();
                ui.label(i18n.tr("timeline.fps"));
                ui.add(egui::DragValue::new(&mut project.metadata.fps).range(1..=120));

                ui.separator();
                let fps = project.metadata.fps.max(1) as f64;
                let (start_f, end_f) = preview.playback_range(project);
                let mut total_duration = 0.0;
                let mut current_time = 0.0;
                for f_idx in start_f..=end_f {
                    let hold = project
                        .metadata
                        .frames
                        .get(f_idx)
                        .map(|f| f.hold_multiplier.max(1))
                        .unwrap_or(1) as f64;
                    let dur = hold / fps;
                    if f_idx < project.active_frame {
                        current_time += dur;
                    }
                    total_duration += dur;
                }
                ui.label(
                    egui::RichText::new(format!(
                        "{}: {:.2}s / {:.2}s",
                        i18n.tr("timeline.current_time"),
                        current_time,
                        total_duration
                    ))
                    .weak(),
                );

                if !project.metadata.tags.is_empty() {
                    ui.separator();
                    let tag_loop = preview.active_tag_index.is_some();
                    if ui
                        .selectable_label(tag_loop, format!("{FA_TAG} {}", i18n.tr("timeline.loop_tag")))
                        .on_hover_text(i18n.tr("timeline.loop_tag"))
                        .clicked()
                    {
                        if tag_loop {
                            preview.active_tag_index = None;
                        } else {
                            preview.active_tag_index = Some(0);
                        }
                    }
                }

                ui.separator();

                if ui
                    .button(format!("{FA_PLUS} {}", i18n.tr("timeline.add_frame")))
                    .on_hover_text(i18n.tr("tooltip.timeline_add"))
                    .clicked()
                {
                    project.sync_active_layer();
                    history.push(HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    });
                    let cur = project.active_frame;
                    project.insert_blank_frame_after(cur);
                }

                if ui
                    .button(format!("{FA_COPY} {}", i18n.tr("timeline.dup_frame")))
                    .on_hover_text(i18n.tr("tooltip.timeline_dup"))
                    .clicked()
                {
                    project.sync_active_layer();
                    history.push(HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    });
                    let cur = project.active_frame;
                    project.duplicate_frame_at(cur);
                }

                let can_delete = project.metadata.frame_count > 1;
                if ui
                    .add_enabled(
                        can_delete,
                        egui::Button::new(format!("{FA_TRASH} {}", i18n.tr("timeline.del_frame"))),
                    )
                    .on_hover_text(i18n.tr("tooltip.timeline_del"))
                    .clicked()
                {
                    actions.request_delete_frame = Some(project.active_frame);
                }

                let can_move_left = project.active_frame > 0;
                if ui
                    .add_enabled(
                        can_move_left,
                        egui::Button::new(format!("{FA_STEP_PREV} {}", i18n.tr("timeline.move_left"))),
                    )
                    .on_hover_text(i18n.tr("tooltip.timeline_move_left"))
                    .clicked()
                {
                    project.sync_active_layer();
                    let snap = HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    };
                    let cur = project.active_frame;
                    if project.swap_frames(cur, cur - 1) {
                        history.push(snap);
                    }
                }

                let max_idx = (project.metadata.frame_count as usize).saturating_sub(1);
                let can_move_right = project.active_frame < max_idx;
                if ui
                    .add_enabled(
                        can_move_right,
                        egui::Button::new(format!("{FA_STEP_NEXT} {}", i18n.tr("timeline.move_right"))),
                    )
                    .on_hover_text(i18n.tr("tooltip.timeline_move_right"))
                    .clicked()
                {
                    project.sync_active_layer();
                    let snap = HistorySnapshot {
                        atlas_rgba: project.atlas_rgba.clone(),
                        metadata: project.metadata.clone(),
                        active_frame: project.active_frame,
                    };
                    let cur = project.active_frame;
                    if project.swap_frames(cur, cur + 1) {
                        history.push(snap);
                    }
                }

                ui.separator();

                if ui
                    .button(format!("{FA_LAYER_GROUP} {}", i18n.tr("timeline.add_layer")))
                    .on_hover_text(i18n.tr("tooltip.timeline_add_layer"))
                    .clicked()
                {
                    deferred_add_layer = true;
                }

                ui.separator();

                let active = project
                    .active_frame
                    .min(project.metadata.frames.len().saturating_sub(1));
                if let Some(frame_meta) = project.metadata.frames.get_mut(active) {
                    ui.label(i18n.tr("timeline.hold"));
                    ui.add(
                        egui::DragValue::new(&mut frame_meta.hold_multiplier)
                            .range(1..=32)
                            .suffix("x"),
                    );
                }
            });

            // Row 2: Frame Tags bar (inline editing + active tag filter selection)
            // IMPORTANT: auto_shrink([false, true]) prevents the horizontal tags scroll area from
            // consuming all vertical height in the resizable bottom dock.
            ScrollArea::horizontal()
                .id_salt("bottom_timeline_tags_scroll")
                .auto_shrink([false, true])
                .max_height(28.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.small(format!("{FA_TAG} {}", i18n.tr("timeline.tags")));
                        if ui
                            .selectable_label(
                                preview.active_tag_index.is_none(),
                                i18n.tr("panel.preview.all_frames"),
                            )
                            .clicked()
                        {
                            preview.active_tag_index = None;
                        }

                        let max_frame_1based = project.metadata.frame_count.max(1) as usize;

                        for (t_idx, tag) in project.metadata.tags.iter_mut().enumerate() {
                            ui.separator();
                            let is_active_tag = preview.active_tag_index == Some(t_idx);
                            if ui.selectable_label(is_active_tag, FA_PLAY).clicked() {
                                if is_active_tag {
                                    preview.active_tag_index = None;
                                } else {
                                    preview.active_tag_index = Some(t_idx);
                                }
                            }
                            modern_color_picker_button(
                                ui,
                                ("tag_col", t_idx),
                                &mut tag.color,
                                i18n,
                                None,
                            );
                            ui.add(egui::TextEdit::singleline(&mut tag.name).desired_width(64.0));

                            let mut start_1 = (tag.start_frame + 1).min(max_frame_1based);
                            let mut end_1 = (tag.end_frame + 1).min(max_frame_1based).max(start_1);

                            if ui
                                .add(egui::DragValue::new(&mut start_1).range(1..=max_frame_1based))
                                .changed()
                            {
                                tag.start_frame = start_1.saturating_sub(1);
                                if tag.end_frame < tag.start_frame {
                                    tag.end_frame = tag.start_frame;
                                }
                            }
                            ui.small("..");
                            if ui
                                .add(egui::DragValue::new(&mut end_1).range(start_1..=max_frame_1based))
                                .changed()
                            {
                                tag.end_frame = end_1.saturating_sub(1).max(tag.start_frame);
                            }

                            egui::ComboBox::from_id_salt(("tag_dir", t_idx))
                                .selected_text(match tag.direction {
                                    TagDirection::Forward => i18n.tr("tag.direction.forward"),
                                    TagDirection::Reverse => i18n.tr("tag.direction.reverse"),
                                    TagDirection::PingPong => i18n.tr("tag.direction.pingpong"),
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut tag.direction,
                                        TagDirection::Forward,
                                        i18n.tr("tag.direction.forward"),
                                    );
                                    ui.selectable_value(
                                        &mut tag.direction,
                                        TagDirection::Reverse,
                                        i18n.tr("tag.direction.reverse"),
                                    );
                                    ui.selectable_value(
                                        &mut tag.direction,
                                        TagDirection::PingPong,
                                        i18n.tr("tag.direction.pingpong"),
                                    );
                                });

                            if ui
                                .small_button(FA_TRASH)
                                .on_hover_text(i18n.tr("start.remove_recent"))
                                .clicked()
                            {
                                actions.request_delete_tag = Some((t_idx, tag.name.clone()));
                            }
                        }

                        if ui
                            .small_button(format!("{FA_PLUS} {}", i18n.tr("timeline.add_tag")))
                            .on_hover_text(i18n.tr("tooltip.timeline_add_tag"))
                            .clicked()
                        {
                            let last = (project.metadata.frame_count as usize).saturating_sub(1);
                            let palette_cols: [[u8; 4]; 4] = [
                                [110, 175, 235, 255],
                                [130, 200, 140, 255],
                                [230, 165, 95, 255],
                                [205, 120, 195, 255],
                            ];
                            let color =
                                palette_cols[project.metadata.tags.len() % palette_cols.len()];
                            project.metadata.tags.push(FrameTag {
                                name: format!(
                                    "{}_{}",
                                    i18n.tr("timeline.default_tag"),
                                    project.metadata.tags.len() + 1
                                ),
                                start_frame: project.active_frame.min(last),
                                end_frame: last,
                                color,
                                direction: TagDirection::Forward,
                            });
                        }
                    });
                });

            ui.separator();

            // Row 3: Multi-Layer Timeline (vertical & horizontal scrolling)
            let mut deferred_dup: Option<usize> = None;
            let mut deferred_create_linked: Option<usize> = None;
            let mut deferred_unlink: Option<usize> = None;
            let mut deferred_link_to: Option<(usize, usize)> = None;
            let mut deferred_insert: Option<usize> = None;
            let mut deferred_swap: Option<(usize, usize)> = None;
            let mut deferred_select_layer: Option<usize> = None;
            let mut deferred_dup_layer: Option<usize> = None;
            let mut deferred_merge_layer: Option<usize> = None;
            let mut deferred_swap_layers: Option<(usize, usize)> = None;

            let active_track_id = ui.make_persistent_id("timeline_active_frame_track");
            let prev_active: Option<usize> = ui.ctx().data(|d| d.get_temp(active_track_id));
            let should_scroll_to_active = prev_active != Some(project.active_frame);
            if should_scroll_to_active {
                ui.ctx().data_mut(|d| d.insert_temp(active_track_id, project.active_frame));
            }

            const TRACK_HEADER_W: f32 = 244.0;
            const CLIP_W: f32 = 64.0;
            const RULER_H: f32 = 24.0;
            const TRACK_H: f32 = 58.0;

            let scroll_output = ScrollArea::both()
                .id_salt("bottom_timeline_frames_scroll")
                .auto_shrink([false, false])
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    let total_frames = project.metadata.frame_count.max(1) as usize;
                    let layer_count = project.metadata.layers.len().max(1);
                    let playhead_frame = if preview.is_playing {
                        preview.current_frame
                    } else {
                        project.active_frame
                    };

                    ui.spacing_mut().item_spacing = Vec2::new(3.0, 3.0);

                    let is_dark = ui.visuals().dark_mode;

                    // 1. Timecode Ruler Row
                    ui.horizontal(|ui| {
                        let (hdr_rect, _) = ui.allocate_exact_size(
                            Vec2::new(TRACK_HEADER_W, RULER_H),
                            egui::Sense::hover(),
                        );
                        let (ruler_hdr_bg, ruler_hdr_stroke) = if is_dark {
                            (Color32::from_rgb(24, 26, 31), Color32::from_rgb(48, 52, 62))
                        } else {
                            (Color32::from_rgb(230, 233, 240), Color32::from_rgb(195, 202, 215))
                        };
                        ui.painter().rect_filled(hdr_rect, 2.0, ruler_hdr_bg);
                        ui.painter().rect_stroke(
                            hdr_rect,
                            2.0,
                            Stroke::new(1.0, ruler_hdr_stroke),
                            egui::StrokeKind::Inside,
                        );
                        let mut hdr_child = ui.new_child(
                            egui::UiBuilder::new()
                                .max_rect(hdr_rect.shrink2(Vec2::new(6.0, 2.0)))
                                .layout(egui::Layout::left_to_right(egui::Align::Center)),
                        );
                        hdr_child.label(
                            egui::RichText::new(format!(
                                "{FA_LAYER_GROUP} {} ({})",
                                i18n.tr("timeline.layers"),
                                layer_count
                            ))
                            .small()
                            .strong(),
                        );
                        hdr_child.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui
                                    .small_button(format!("{FA_PLUS} {}", i18n.tr("timeline.add_layer_short")))
                                    .on_hover_text(i18n.tr("tooltip.timeline_add_layer"))
                                    .clicked()
                                {
                                    deferred_add_layer = true;
                                }
                            },
                        );

                        for idx in 0..total_frames {
                            let is_active = project.active_frame == idx;
                            let is_playhead = playhead_frame == idx;
                            let hold = project
                                .metadata
                                .frames
                                .get(idx)
                                .map(|f| f.hold_multiplier)
                                .unwrap_or(1);
                            let is_linked = project.is_frame_linked(idx);

                            let tag_color = project
                                .metadata
                                .tags
                                .iter()
                                .find(|t| idx >= t.start_frame && idx <= t.end_frame)
                                .map(|t| {
                                    let [r, g, b, _] = t.color;
                                    Color32::from_rgb(r, g, b)
                                });

                            let (r_rect, r_resp) = ui.allocate_exact_size(
                                Vec2::new(CLIP_W, RULER_H),
                                egui::Sense::click(),
                            );
                            let r_bg = if is_playhead {
                                if is_dark {
                                    Color32::from_rgb(58, 34, 34)
                                } else {
                                    Color32::from_rgb(255, 215, 215)
                                }
                            } else if is_active {
                                if is_dark {
                                    Color32::from_rgb(42, 46, 56)
                                } else {
                                    Color32::from_rgb(210, 222, 240)
                                }
                            } else {
                                if is_dark {
                                    Color32::from_rgb(24, 26, 31)
                                } else {
                                    Color32::from_rgb(232, 236, 244)
                                }
                            };
                            ui.painter().rect_filled(r_rect, 2.0, r_bg);

                            // Tag bar at top of ruler cell
                            let tag_bar = egui::Rect::from_min_max(
                                r_rect.min,
                                egui::pos2(r_rect.max.x, r_rect.min.y + 4.0),
                            );
                            if let Some(tc) = tag_color {
                                ui.painter().rect_filled(tag_bar, 1.0, tc);
                            }

                            // Playhead marker cap
                            if is_playhead {
                                let cap_w = 12.0;
                                let cx = r_rect.center().x;
                                let cap_rect = egui::Rect::from_min_max(
                                    egui::pos2(cx - cap_w * 0.5, r_rect.max.y - 5.0),
                                    egui::pos2(cx + cap_w * 0.5, r_rect.max.y),
                                );
                                ui.painter()
                                    .rect_filled(cap_rect, 1.5, Color32::from_rgb(235, 65, 55));
                            }

                            // Ruler tick lines
                            let tick_stroke = if is_dark {
                                Color32::from_rgb(48, 52, 60)
                            } else {
                                Color32::from_rgb(205, 210, 220)
                            };
                            ui.painter().line_segment(
                                [r_rect.left_bottom(), r_rect.left_top()],
                                Stroke::new(1.0, tick_stroke),
                            );

                            let label_str = match (hold > 1, is_linked) {
                                (true, true) => format!("#{} {}x🔗", idx + 1, hold),
                                (true, false) => format!("#{} ({}x)", idx + 1, hold),
                                (false, true) => format!("#{} 🔗", idx + 1),
                                (false, false) => format!("#{}", idx + 1),
                            };
                            let ruler_text_col = if is_playhead || is_active {
                                if is_dark {
                                    Color32::WHITE
                                } else {
                                    Color32::from_rgb(25, 30, 42)
                                }
                            } else {
                                if is_dark {
                                    Color32::from_rgb(175, 180, 190)
                                } else {
                                    Color32::from_rgb(95, 105, 120)
                                }
                            };
                            ui.painter().text(
                                r_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                label_str,
                                egui::FontId::proportional(11.0),
                                ruler_text_col,
                            );

                            if r_resp.clicked() {
                                project.active_frame = idx;
                                preview.current_frame = idx;
                            } else if r_resp.secondary_clicked() {
                                project.active_frame = idx;
                                preview.current_frame = idx;
                                let popup_id = ui.make_persistent_id((
                                    "frame_popup_menu",
                                    project.metadata.active_layer,
                                    idx,
                                ));
                                ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                            }
                        }
                    });

                    // 2. Layer Track Rows (V1, V2, ... going downwards and scrolling vertically)
                    for layer_idx in 0..layer_count {
                        let is_active_layer = project.metadata.active_layer == layer_idx;
                        let layer_visible = project
                            .metadata
                            .layers
                            .get(layer_idx)
                            .map(|l| l.visible)
                            .unwrap_or(true);
                        let layer_locked = project
                            .metadata
                            .layers
                            .get(layer_idx)
                            .map(|l| l.locked)
                            .unwrap_or(false);

                        ui.horizontal(|ui| {
                            // Left Track Header
                            let (track_hdr_rect, track_hdr_resp) = ui.allocate_exact_size(
                                Vec2::new(TRACK_HEADER_W, TRACK_H),
                                egui::Sense::click(),
                            );
                            if track_hdr_resp.clicked() {
                                deferred_select_layer = Some(layer_idx);
                            }

                            let hdr_bg = if is_active_layer {
                                if is_dark {
                                    Color32::from_rgb(38, 44, 56)
                                } else {
                                    Color32::from_rgb(218, 228, 244)
                                }
                            } else {
                                if is_dark {
                                    Color32::from_rgb(26, 29, 35)
                                } else {
                                    Color32::from_rgb(238, 242, 248)
                                }
                            };
                            ui.painter().rect_filled(track_hdr_rect, 3.0, hdr_bg);

                            // Left active track indicator stripe
                            let stripe_rect = egui::Rect::from_min_max(
                                track_hdr_rect.min,
                                egui::pos2(track_hdr_rect.min.x + 4.0, track_hdr_rect.max.y),
                            );
                            let stripe_col = if is_active_layer {
                                Color32::from_rgb(235, 155, 50)
                            } else {
                                if is_dark {
                                    Color32::from_rgb(65, 75, 90)
                                } else {
                                    Color32::from_rgb(180, 188, 200)
                                }
                            };
                            ui.painter().rect_filled(
                                stripe_rect,
                                2.0,
                                stripe_col,
                            );
                            let track_border_col = if is_active_layer {
                                if is_dark {
                                    Color32::from_rgb(95, 120, 160)
                                } else {
                                    Color32::from_rgb(140, 170, 220)
                                }
                            } else {
                                if is_dark {
                                    Color32::from_rgb(48, 52, 62)
                                } else {
                                    Color32::from_rgb(205, 212, 224)
                                }
                            };
                            ui.painter().rect_stroke(
                                track_hdr_rect,
                                3.0,
                                Stroke::new(
                                    1.0,
                                    track_border_col,
                                ),
                                egui::StrokeKind::Inside,
                            );

                            let inner_hdr = egui::Rect::from_min_max(
                                egui::pos2(track_hdr_rect.min.x + 8.0, track_hdr_rect.min.y + 3.0),
                                egui::pos2(track_hdr_rect.max.x - 4.0, track_hdr_rect.max.y - 3.0),
                            );
                            let mut hdr_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(inner_hdr)
                                    .layout(egui::Layout::top_down(egui::Align::Min)),
                            );
                            hdr_ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);

                            // Track header top row: V-badge, layer name, Eye, Lock
                            hdr_ui.horizontal(|ui| {
                                let v_badge = format!("V{}", layer_idx + 1);
                                if ui
                                    .selectable_label(
                                        is_active_layer,
                                        egui::RichText::new(v_badge).small().strong(),
                                    )
                                    .clicked()
                                {
                                    deferred_select_layer = Some(layer_idx);
                                }

                                if let Some(layer_meta) = project.metadata.layers.get_mut(layer_idx)
                                {
                                    let name_resp = ui.add(
                                        egui::TextEdit::singleline(&mut layer_meta.name)
                                            .desired_width(118.0)
                                            .font(egui::TextStyle::Small),
                                    );
                                    if name_resp.clicked() {
                                        deferred_select_layer = Some(layer_idx);
                                    }

                                    let eye_icon = if layer_meta.visible {
                                        FA_EYE
                                    } else {
                                        FA_EYE_SLASH
                                    };
                                    if ui
                                        .small_button(eye_icon)
                                        .on_hover_text(i18n.tr("tooltip.layer_visibility"))
                                        .clicked()
                                    {
                                        layer_meta.visible = !layer_meta.visible;
                                        project.canvas_dirty = true;
                                        project.has_unsaved_changes = true;
                                    }

                                    let lock_icon = if layer_meta.locked {
                                        FA_LOCK
                                    } else {
                                        FA_LOCK_OPEN
                                    };
                                    if ui
                                        .small_button(lock_icon)
                                        .on_hover_text(i18n.tr("tooltip.layer_lock"))
                                        .clicked()
                                    {
                                        layer_meta.locked = !layer_meta.locked;
                                        project.has_unsaved_changes = true;
                                    }
                                }
                            });

                            // Track header bottom row: Opacity % + Up/Down/Dup/Merge/Del
                            hdr_ui.horizontal(|ui| {
                                if let Some(layer_meta) = project.metadata.layers.get_mut(layer_idx)
                                {
                                    let mut pct = (layer_meta.opacity * 100.0).round() as i32;
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut pct)
                                                .range(0..=100)
                                                .suffix("%"),
                                        )
                                        .on_hover_text(i18n.tr("tooltip.layer_opacity"))
                                        .changed()
                                    {
                                        layer_meta.opacity = (pct as f32 / 100.0).clamp(0.0, 1.0);
                                        project.canvas_dirty = true;
                                        project.has_unsaved_changes = true;
                                    }
                                }

                                if ui
                                    .add_enabled(
                                        layer_idx > 0,
                                        egui::Button::new(egui::RichText::new(FA_ARROW_UP).small()),
                                    )
                                    .on_hover_text(i18n.tr("tooltip.layer_move_up"))
                                    .clicked()
                                {
                                    deferred_swap_layers = Some((layer_idx, layer_idx - 1));
                                }
                                if ui
                                    .add_enabled(
                                        layer_idx + 1 < layer_count,
                                        egui::Button::new(
                                            egui::RichText::new(FA_ARROW_DOWN).small(),
                                        ),
                                    )
                                    .on_hover_text(i18n.tr("tooltip.layer_move_down"))
                                    .clicked()
                                {
                                    deferred_swap_layers = Some((layer_idx, layer_idx + 1));
                                }
                                if ui
                                    .small_button(FA_COPY)
                                    .on_hover_text(i18n.tr("tooltip.layer_duplicate"))
                                    .clicked()
                                {
                                    deferred_dup_layer = Some(layer_idx);
                                }
                                if ui
                                    .add_enabled(
                                        layer_count > 1,
                                        egui::Button::new(
                                            egui::RichText::new(FA_LAYER_GROUP).small(),
                                        ),
                                    )
                                    .on_hover_text(i18n.tr("tooltip.layer_merge_down"))
                                    .clicked()
                                {
                                    deferred_merge_layer = Some(layer_idx);
                                }
                                if ui
                                    .add_enabled(
                                        layer_count > 1,
                                        egui::Button::new(egui::RichText::new(FA_TRASH).small()),
                                    )
                                    .on_hover_text(i18n.tr("tooltip.layer_delete"))
                                    .clicked()
                                {
                                    let name = project
                                        .metadata
                                        .layers
                                        .get(layer_idx)
                                        .map(|l| l.name.clone())
                                        .unwrap_or_else(|| {
                                            format!(
                                                "{} {}",
                                                i18n.tr("timeline.layer_default"),
                                                layer_idx + 1
                                            )
                                        });
                                    actions.request_delete_layer = Some((layer_idx, name));
                                }
                            });

                            // Right Frame Clip Strip for this layer track
                            for idx in 0..total_frames {
                                let is_active_frame = project.active_frame == idx;
                                let is_active_cell = is_active_layer && is_active_frame;
                                let is_playhead = playhead_frame == idx;
                                let is_linked = project.is_frame_linked(idx);
                                let hold = project
                                    .metadata
                                    .frames
                                    .get(idx)
                                    .map(|f| f.hold_multiplier)
                                    .unwrap_or(1);

                                let (rect, resp) = ui.allocate_exact_size(
                                    Vec2::new(CLIP_W, TRACK_H),
                                    egui::Sense::click(),
                                );

                                // Track clip body & header bar
                                let clip_bg = if !layer_visible {
                                    if is_dark {
                                        Color32::from_rgb(20, 20, 24)
                                    } else {
                                        Color32::from_rgb(230, 233, 238)
                                    }
                                } else if is_active_cell {
                                    if is_dark {
                                        Color32::from_rgb(48, 54, 66)
                                    } else {
                                        Color32::from_rgb(212, 226, 248)
                                    }
                                } else if is_active_frame {
                                    if is_dark {
                                        Color32::from_rgb(34, 38, 46)
                                    } else {
                                        Color32::from_rgb(224, 232, 244)
                                    }
                                } else {
                                    if is_dark {
                                        Color32::from_rgb(24, 27, 33)
                                    } else {
                                        Color32::from_rgb(242, 245, 250)
                                    }
                                };
                                ui.painter().rect_filled(rect, 3.0, clip_bg);

                                let clip_hdr_rect = egui::Rect::from_min_max(
                                    rect.min,
                                    egui::pos2(rect.max.x, rect.min.y + 11.0),
                                );
                                let clip_hdr_col = if !layer_visible {
                                    if is_dark {
                                        Color32::from_rgb(45, 45, 52)
                                    } else {
                                        Color32::from_rgb(180, 185, 195)
                                    }
                                } else if is_active_cell {
                                    Color32::from_rgb(215, 140, 45)
                                } else if is_active_layer {
                                    if is_dark {
                                        Color32::from_rgb(62, 112, 168)
                                    } else {
                                        Color32::from_rgb(72, 130, 200)
                                    }
                                } else {
                                    if is_dark {
                                        Color32::from_rgb(46, 76, 114)
                                    } else {
                                        Color32::from_rgb(112, 146, 186)
                                    }
                                };
                                ui.painter().rect_filled(clip_hdr_rect, 2.0, clip_hdr_col);

                                let hdr_label = if is_linked {
                                    format!("V{}:#{} 🔗", layer_idx + 1, idx + 1)
                                } else if hold > 1 {
                                    format!("V{}:#{} {}x", layer_idx + 1, idx + 1, hold)
                                } else {
                                    format!("V{}:#{}", layer_idx + 1, idx + 1)
                                };
                                ui.painter().text(
                                    clip_hdr_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    hdr_label,
                                    egui::FontId::proportional(9.5),
                                    Color32::from_rgb(240, 244, 250),
                                );

                                // Thumbnail area inside the clip
                                let thumb_area = egui::Rect::from_min_max(
                                    egui::pos2(rect.min.x + 3.0, rect.min.y + 13.0),
                                    egui::pos2(rect.max.x - 3.0, rect.max.y - 3.0),
                                );
                                let thumb_bg = if is_dark {
                                    Color32::from_rgb(18, 19, 23)
                                } else {
                                    Color32::from_rgb(255, 255, 255)
                                };
                                ui.painter().rect_filled(
                                    thumb_area,
                                    2.0,
                                    thumb_bg,
                                );
                                if !is_dark {
                                    ui.painter().rect_stroke(
                                        thumb_area,
                                        2.0,
                                        Stroke::new(1.0, Color32::from_rgb(218, 222, 230)),
                                        egui::StrokeKind::Inside,
                                    );
                                }

                                let tex_opt = renderer
                                    .layer_texture(layer_idx)
                                    .or_else(|| renderer.atlas_texture());
                                if let Some(tex) = tex_opt {
                                    let uv = CanvasRenderer::frame_uv(project, idx);
                                    let fw = project.metadata.frame_width.max(1) as f32;
                                    let fh = project.metadata.frame_height.max(1) as f32;
                                    let scale = (thumb_area.width() / fw)
                                        .min(thumb_area.height() / fh)
                                        .max(0.01);
                                    let img_size = Vec2::new(fw * scale, fh * scale);
                                    let img_rect =
                                        egui::Rect::from_center_size(thumb_area.center(), img_size);
                                    let tint = if layer_visible {
                                        Color32::WHITE
                                    } else {
                                        Color32::from_rgba_unmultiplied(255, 255, 255, 90)
                                    };
                                    ui.painter().image(tex.id(), img_rect, uv, tint);
                                }

                                // Playhead vertical line across all tracks
                                if is_playhead {
                                    let px = rect.center().x;
                                    ui.painter().line_segment(
                                        [
                                            egui::pos2(px, rect.min.y),
                                            egui::pos2(px, rect.max.y),
                                        ],
                                        Stroke::new(
                                            1.5,
                                            Color32::from_rgba_unmultiplied(235, 65, 55, 190),
                                        ),
                                    );
                                }

                                // Clip border stroke
                                let border_color = if is_active_cell {
                                    if is_dark {
                                        Color32::from_rgb(250, 195, 80)
                                    } else {
                                        Color32::from_rgb(225, 135, 25)
                                    }
                                } else if is_active_frame {
                                    if is_dark {
                                        Color32::from_rgb(190, 200, 215)
                                    } else {
                                        Color32::from_rgb(95, 140, 205)
                                    }
                                } else {
                                    if is_dark {
                                        Color32::from_rgb(52, 58, 68)
                                    } else {
                                        Color32::from_rgb(205, 212, 224)
                                    }
                                };
                                ui.painter().rect_stroke(
                                    rect,
                                    3.0,
                                    Stroke::new(
                                        if is_active_cell { 2.0 } else { 1.0 },
                                        border_color,
                                    ),
                                    egui::StrokeKind::Inside,
                                );

                                if layer_locked {
                                    ui.painter().text(
                                        egui::pos2(rect.max.x - 4.0, rect.max.y - 3.0),
                                        egui::Align2::RIGHT_BOTTOM,
                                        FA_LOCK,
                                        egui::FontId::proportional(9.0),
                                        Color32::from_rgba_unmultiplied(240, 180, 90, 180),
                                    );
                                }

                                let popup_id =
                                    ui.make_persistent_id(("frame_popup_menu", layer_idx, idx));
                                if is_active_cell && should_scroll_to_active {
                                    resp.scroll_to_me(Some(egui::Align::Center));
                                }
                                let resp =
                                    resp.on_hover_text(i18n.tr("tooltip.timeline_frame_menu"));
                                if resp.clicked() {
                                    deferred_select_layer = Some(layer_idx);
                                    project.active_frame = idx;
                                    preview.current_frame = idx;
                                } else if resp.secondary_clicked() {
                                    deferred_select_layer = Some(layer_idx);
                                    project.active_frame = idx;
                                    preview.current_frame = idx;
                                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                                }

                                egui::popup_below_widget(
                                    ui,
                                    popup_id,
                                    &resp,
                                    egui::PopupCloseBehavior::CloseOnClickOutside,
                                    |ui| {
                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Extend);
                                        ui.strong(format!(
                                            "V{} — {} #{}",
                                            layer_idx + 1,
                                            i18n.tr("timeline.frame"),
                                            idx + 1
                                        ));
                                        ui.separator();
                                        if ui
                                            .button(format!(
                                                "{FA_IMPORT} {}",
                                                i18n.tr("timeline.import_frame")
                                            ))
                                            .clicked()
                                        {
                                            actions.trigger_import_single_frame = Some(idx);
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if ui
                                            .button(format!(
                                                "{FA_EXPORT} {}",
                                                i18n.tr("timeline.export_frame")
                                            ))
                                            .clicked()
                                        {
                                            actions.trigger_export_single_frame = Some(idx);
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        ui.separator();
                                        if ui
                                            .button(format!(
                                                "{FA_COPY} {}",
                                                i18n.tr("timeline.dup_frame")
                                            ))
                                            .clicked()
                                        {
                                            deferred_dup = Some(idx);
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if ui
                                            .button(format!(
                                                "{FA_PLUS} {}",
                                                i18n.tr("timeline.add_frame")
                                            ))
                                            .clicked()
                                        {
                                            deferred_insert = Some(idx);
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if ui
                                            .button(format!(
                                                "{FA_COPY} {}",
                                                i18n.tr("timeline.create_linked_frame")
                                            ))
                                            .clicked()
                                        {
                                            deferred_create_linked = Some(idx);
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if is_linked {
                                            if ui
                                                .button(format!(
                                                    "{FA_CLOSE} {}",
                                                    i18n.tr("timeline.unlink_frame")
                                                ))
                                                .clicked()
                                            {
                                                deferred_unlink = Some(idx);
                                                ui.memory_mut(|mem| mem.close_popup());
                                            }
                                        } else if total_frames > 1 {
                                            ui.menu_button(
                                                format!(
                                                    "{FA_TAG} {}",
                                                    i18n.tr("timeline.link_to_frame")
                                                ),
                                                |ui| {
                                                    for o in 0..total_frames {
                                                        if o != idx
                                                            && ui
                                                                .button(format!(
                                                                    "{} #{}",
                                                                    i18n.tr("timeline.frame"),
                                                                    o + 1
                                                                ))
                                                                .clicked()
                                                        {
                                                            deferred_link_to = Some((idx, o));
                                                            ui.memory_mut(|mem| mem.close_popup());
                                                        }
                                                    }
                                                },
                                            );
                                        }
                                        if idx > 0
                                            && ui
                                                .button(format!(
                                                    "{FA_STEP_PREV} {}",
                                                    i18n.tr("timeline.move_earlier")
                                                ))
                                                .on_hover_text(
                                                    i18n.tr("tooltip.timeline_move_left"),
                                                )
                                                .clicked()
                                        {
                                            deferred_swap = Some((idx, idx - 1));
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if idx + 1 < total_frames
                                            && ui
                                                .button(format!(
                                                    "{FA_STEP_NEXT} {}",
                                                    i18n.tr("timeline.move_later")
                                                ))
                                                .on_hover_text(
                                                    i18n.tr("tooltip.timeline_move_right"),
                                                )
                                                .clicked()
                                        {
                                            deferred_swap = Some((idx, idx + 1));
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                        if total_frames > 1 {
                                            ui.separator();
                                            if ui
                                                .button(
                                                    egui::RichText::new(format!(
                                                        "{FA_TRASH} {}",
                                                        i18n.tr("timeline.del_frame")
                                                    ))
                                                    .color(Color32::from_rgb(235, 110, 110)),
                                                )
                                                .clicked()
                                            {
                                                actions.request_delete_frame = Some(idx);
                                                ui.memory_mut(|mem| mem.close_popup());
                                            }
                                        }
                                    },
                                );
                            }
                        });
                    }
                });

            // Smart mouse-wheel scrolling: if vertical tracks fit inside the dock (or Shift is held),
            // mouse wheel scrolls horizontally across frames; otherwise vertical wheel scrolls layer tracks.
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if scroll_output.inner_rect.contains(hover_pos) {
                    let wheel_delta = ui.input(|i| i.raw_scroll_delta);
                    let shift_held = ui.input(|i| i.modifiers.shift);
                    let can_scroll_v =
                        scroll_output.content_size.y > scroll_output.inner_rect.height() + 2.0;
                    if wheel_delta.y != 0.0 && (!can_scroll_v || shift_held) {
                        let max_scroll_x = (scroll_output.content_size.x
                            - scroll_output.inner_rect.width())
                        .max(0.0);
                        let mut s = scroll_output.state;
                        s.offset.x = (s.offset.x - wheel_delta.y).clamp(0.0, max_scroll_x);
                        s.store(ui.ctx(), scroll_output.id);
                        ui.ctx().request_repaint();
                    }
                }
            }

            if let Some(target_layer) = deferred_select_layer {
                project.set_active_layer(target_layer);
            }
            if deferred_add_layer {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                let new_num = project.metadata.layers.len() + 1;
                project.add_layer(format!("{} {}", i18n.tr("timeline.layer_default"), new_num));
            }
            if let Some(l_idx) = deferred_dup_layer {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.duplicate_layer(l_idx);
            }
            if let Some(l_idx) = deferred_merge_layer {
                project.sync_active_layer();
                let snap = HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                };
                if project.merge_layer_down(l_idx) {
                    history.push(snap);
                }
            }
            if let Some((la, lb)) = deferred_swap_layers {
                project.sync_active_layer();
                let snap = HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                };
                if project.swap_layers(la, lb) {
                    history.push(snap);
                }
            }

            if let Some(idx) = deferred_dup {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.duplicate_frame_at(idx);
            }
            if let Some(idx) = deferred_insert {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.insert_blank_frame_after(idx);
            }
            if let Some(idx) = deferred_create_linked {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.create_linked_frame_after(idx);
            }
            if let Some(idx) = deferred_unlink {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.unlink_frame(idx);
            }
            if let Some((src, tgt)) = deferred_link_to {
                project.sync_active_layer();
                history.push(HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                });
                project.link_frame_to(src, tgt);
            }
            if let Some((a, b)) = deferred_swap {
                project.sync_active_layer();
                let snap = HistorySnapshot {
                    atlas_rgba: project.atlas_rgba.clone(),
                    metadata: project.metadata.clone(),
                    active_frame: project.active_frame,
                };
                if project.swap_frames(a, b) {
                    history.push(snap);
                }
            }
        });

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::renderer::CanvasRenderer;
    use crate::history::command::HistoryStack;
    use crate::model::metadata::ProjectMetadata;
    use crate::model::project::Project;

    #[test]
    fn test_bottom_dock_horizontal_scroll_render() {
        let ctx = egui::Context::default();
        let i18n = I18n::new(crate::i18n::Language::Russian);
        let meta = ProjectMetadata::new_config("scroll_test".to_string(), 16, 16, 20, 1, None);
        let mut proj = Project::new(meta);
        let mut history = HistoryStack::default();
        let mut preview = PreviewState::default();
        let renderer = CanvasRenderer::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            let actions = show_bottom_dock(
                ctx,
                &i18n,
                &mut proj,
                &mut history,
                &mut preview,
                &renderer,
            );
            assert!(actions.request_delete_frame.is_none());
        });
    }

    #[test]
    fn test_bottom_dock_light_theme_render() {
        let ctx = egui::Context::default();
        ctx.set_visuals(egui::Visuals::light());
        let i18n = I18n::new(crate::i18n::Language::English);
        let meta = ProjectMetadata::new_config("light_test".to_string(), 16, 16, 5, 2, None);
        let mut proj = Project::new(meta);
        let mut history = HistoryStack::default();
        let mut preview = PreviewState::default();
        let renderer = CanvasRenderer::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            let actions = show_bottom_dock(
                ctx,
                &i18n,
                &mut proj,
                &mut history,
                &mut preview,
                &renderer,
            );
            assert!(actions.request_delete_frame.is_none());
        });
    }
}

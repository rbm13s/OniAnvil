use crate::i18n::I18n;
use crate::io::packer::{pack_project_atlas, PackerSettings};
use crate::io::settings::AppSettings;
use crate::io::updater::GITHUB_REPO_URL;
use crate::model::metadata::ProjectMetadata;
use crate::model::palette::{PaletteDitherMode, PREDEFINED_PALETTES};
use crate::model::project::Project;
use crate::theme::*;
use crate::tools::fill::{OutlineCornerStyle, OutlinePlacement};
use crate::tools::filters::ConvolutionPreset;
use crate::tools::selection::SelectionModifyShape;
use crate::tools::transforms::RotationAlgorithm;
use egui::{vec2, Color32, Context, RichText, Sense, Stroke, StrokeKind, Ui, Vec2, Window};
use image::RgbaImage;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardProjectType {
    StaticSprite,
    AnimationAtlas,
}

pub struct NewProjectWizardState {
    pub open: bool,
    pub project_name: String,
    pub project_type: WizardProjectType,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_count: u32,
    pub columns: u32,
    pub color_limit_index: usize,
    pub selected_palette_preset: usize,
}

impl Default for NewProjectWizardState {
    fn default() -> Self {
        Self {
            open: false,
            project_name: String::from("sprite"),
            project_type: WizardProjectType::AnimationAtlas,
            frame_width: 32,
            frame_height: 32,
            frame_count: 4,
            columns: 4,
            color_limit_index: 2,
            selected_palette_preset: 0,
        }
    }
}

impl NewProjectWizardState {
    pub const FRAME_PRESETS: [(u32, u32, &'static str); 7] = [
        (8, 8, "8×8"),
        (16, 16, "16×16"),
        (24, 24, "24×24"),
        (32, 32, "32×32"),
        (48, 48, "48×48"),
        (64, 64, "64×64"),
        (128, 128, "128×128"),
    ];

    pub const COLOR_LIMIT_OPTIONS: [Option<u32>; 7] = [
        Some(4),
        Some(8),
        Some(16),
        Some(32),
        Some(64),
        Some(256),
        None,
    ];

    pub fn build_metadata(&self) -> ProjectMetadata {
        let (frames, cols) = match self.project_type {
            WizardProjectType::StaticSprite => (1, 1),
            WizardProjectType::AnimationAtlas => (
                self.frame_count.max(1),
                self.columns.max(1).min(self.frame_count.max(1)),
            ),
        };

        let target_limit = Self::COLOR_LIMIT_OPTIONS
            .get(self.color_limit_index)
            .copied()
            .flatten();

        let clean_name = if self.project_name.trim().is_empty() {
            String::from("sprite")
        } else {
            self.project_name.trim().to_string()
        };

        let mut meta = ProjectMetadata::new_config(
            clean_name,
            self.frame_width.max(1),
            self.frame_height.max(1),
            frames,
            cols,
            target_limit,
        );
        let preset_idx = self
            .selected_palette_preset
            .min(PREDEFINED_PALETTES.len().saturating_sub(1));
        meta.palette = PREDEFINED_PALETTES[preset_idx].to_palette_data();
        meta
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<ProjectMetadata> {
        if !self.open {
            return None;
        }

        let mut created_meta = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.new_project.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui: &mut Ui| {
                ui.set_min_width(380.0);

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.new_project.name"));
                    ui.text_edit_singleline(&mut self.project_name);
                });

                ui.separator();

                ui.label(i18n.tr("modal.new_project.type"));
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.project_type,
                        WizardProjectType::StaticSprite,
                        i18n.tr("modal.new_project.type_static"),
                    );
                    ui.selectable_value(
                        &mut self.project_type,
                        WizardProjectType::AnimationAtlas,
                        i18n.tr("modal.new_project.type_atlas"),
                    );
                });

                ui.add_space(4.0);
                ui.label(i18n.tr("modal.new_project.presets"));
                ui.horizontal_wrapped(|ui| {
                    for (pw, ph, label) in Self::FRAME_PRESETS {
                        let active = self.frame_width == pw && self.frame_height == ph;
                        if ui.selectable_label(active, label).clicked() {
                            self.frame_width = pw;
                            self.frame_height = ph;
                        }
                    }
                });

                ui.add_space(4.0);
                egui::Grid::new("new_project_dimensions_grid")
                    .num_columns(2)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.new_project.width"));
                        ui.add(egui::DragValue::new(&mut self.frame_width).range(1..=2048));
                        ui.end_row();

                        ui.label(i18n.tr("modal.new_project.height"));
                        ui.add(egui::DragValue::new(&mut self.frame_height).range(1..=2048));
                        ui.end_row();

                        if self.project_type == WizardProjectType::AnimationAtlas {
                            ui.label(i18n.tr("modal.new_project.frame_count"));
                            if ui
                                .add(egui::DragValue::new(&mut self.frame_count).range(1..=512))
                                .changed()
                            {
                                self.columns = self.columns.min(self.frame_count.max(1));
                            }
                            ui.end_row();

                            ui.label(i18n.tr("modal.new_project.columns"));
                            ui.add(
                                egui::DragValue::new(&mut self.columns)
                                    .range(1..=self.frame_count.max(1)),
                            );
                            ui.end_row();
                        }

                        ui.label(i18n.tr("modal.new_project.palette_preset"));
                        let preset_idx = self
                            .selected_palette_preset
                            .min(PREDEFINED_PALETTES.len().saturating_sub(1));
                        egui::ComboBox::from_id_salt("wizard_palette_preset_combo")
                            .width(180.0)
                            .selected_text(PREDEFINED_PALETTES[preset_idx].name)
                            .show_ui(ui, |ui| {
                                for (idx, p) in PREDEFINED_PALETTES.iter().enumerate() {
                                    ui.selectable_value(
                                        &mut self.selected_palette_preset,
                                        idx,
                                        p.name,
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label(i18n.tr("modal.new_project.color_limit"));
                        egui::ComboBox::from_id_salt("wizard_color_limit_combo")
                            .selected_text(match Self::COLOR_LIMIT_OPTIONS[self.color_limit_index] {
                                Some(n) => n.to_string(),
                                None => i18n.tr("panel.palette.unlimited").to_string(),
                            })
                            .show_ui(ui, |ui| {
                                for (idx, opt) in Self::COLOR_LIMIT_OPTIONS.iter().enumerate() {
                                    let text = match opt {
                                        Some(n) => n.to_string(),
                                        None => i18n.tr("panel.palette.unlimited").to_string(),
                                    };
                                    ui.selectable_value(&mut self.color_limit_index, idx, text);
                                }
                            });
                        ui.end_row();
                    });

                // Preview strip of the selected predefined palette
                let preset_colors = PREDEFINED_PALETTES[self
                    .selected_palette_preset
                    .min(PREDEFINED_PALETTES.len().saturating_sub(1))]
                .to_rgba_vec();
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::splat(2.0);
                    for [r, g, b, a] in preset_colors.iter().copied().take(32) {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
                        ui.painter().rect_filled(
                            rect,
                            1.5,
                            Color32::from_rgba_unmultiplied(r, g, b, a),
                        );
                    }
                });

                ui.separator();

                let preview_meta = self.build_metadata();
                let (atlas_w, atlas_h) = preview_meta.atlas_dimensions();
                let atlas_too_large = atlas_w > 8192 || atlas_h > 8192;
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.new_project.atlas_summary"));
                    ui.strong(format!(
                        "{}×{} px ({}×{})",
                        atlas_w, atlas_h, preview_meta.columns, preview_meta.rows
                    ));
                });
                if atlas_too_large {
                    ui.colored_label(
                        Color32::from_rgb(235, 80, 80),
                        format!("⚠ {} (max 8192×8192)", i18n.tr("modal.new_project.atlas_summary")),
                    );
                }

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(
                            !atlas_too_large,
                            egui::Button::new(format!("{FA_PLUS} {}", i18n.tr("modal.new_project.create"))),
                        )
                        .clicked()
                    {
                        created_meta = Some(preview_meta);
                        close_requested = true;
                    }
                    if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel"))).clicked() {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        created_meta
    }
}

#[derive(Default)]
pub struct ReGridModalState {
    pub open: bool,
    pub new_columns: u32,
}

impl ReGridModalState {
    pub fn open_for(&mut self, project: &Project) {
        self.new_columns = project.metadata.columns.max(1);
        self.open = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n, project: &Project) -> Option<u32> {
        if !self.open {
            return None;
        }
        let mut applied_cols = None;
        let mut window_open = self.open;
        let mut close_requested = false;
        let total = project.metadata.frame_count.max(1);
        self.new_columns = self.new_columns.clamp(1, total);

        Window::new(i18n.tr("modal.regrid.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(300.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.regrid.current"));
                    ui.strong(format!(
                        "{}×{} ({} frames)",
                        project.metadata.columns, project.metadata.rows, total
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.regrid.columns"));
                    ui.add(egui::DragValue::new(&mut self.new_columns).range(1..=total));
                });
                self.new_columns = self.new_columns.clamp(1, total.max(1));
                let new_rows = total.div_ceil(self.new_columns);
                let aw = self.new_columns * project.metadata.frame_width;
                let ah = new_rows * project.metadata.frame_height;
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.regrid.resulting"));
                    ui.strong(format!("{}×{} px ({}×{})", aw, ah, self.new_columns, new_rows));
                });

                ui.separator();
                ui.label(i18n.tr("modal.regrid.preview"));

                let preview_w = 260.0;
                let aspect = (self.new_columns as f32 * project.metadata.frame_width as f32)
                    / (new_rows as f32 * project.metadata.frame_height as f32).max(1.0);
                let preview_h = (preview_w / aspect).clamp(40.0, 150.0);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(preview_w, preview_h), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect_filled(rect, 4.0, Color32::from_black_alpha(100));
                let cell_w = rect.width() / self.new_columns as f32;
                let cell_h = rect.height() / new_rows as f32;

                painter.rect_stroke(
                    rect,
                    4.0,
                    egui::Stroke::new(1.0, Color32::from_white_alpha(40)),
                    egui::StrokeKind::Inside,
                );

                for i in 0..total {
                    let c = i % self.new_columns;
                    let r = i / self.new_columns;
                    let cell_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + c as f32 * cell_w, rect.min.y + r as f32 * cell_h),
                        egui::vec2(cell_w, cell_h),
                    );
                    painter.rect_filled(
                        cell_rect.shrink(1.0),
                        2.0,
                        Color32::from_rgba_unmultiplied(46, 120, 200, 150),
                    );
                    painter.rect_stroke(
                        cell_rect.shrink(1.0),
                        2.0,
                        egui::Stroke::new(1.0, Color32::from_white_alpha(70)),
                        egui::StrokeKind::Inside,
                    );
                    if cell_w >= 14.0 && cell_h >= 12.0 {
                        painter.text(
                            cell_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", i + 1),
                            egui::FontId::proportional(10.0),
                            Color32::WHITE,
                        );
                    }
                }

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button(format!("{FA_CHECK} {}", i18n.tr("modal.regrid.apply"))).clicked() {
                        applied_cols = Some(self.new_columns);
                        close_requested = true;
                    }
                    if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel"))).clicked() {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        applied_cols
    }
}

pub struct Resize9GridModalState {
    pub open: bool,
    pub new_width: u32,
    pub new_height: u32,
    pub anchor_col: u8,
    pub anchor_row: u8,
}

impl Default for Resize9GridModalState {
    fn default() -> Self {
        Self {
            open: false,
            new_width: 32,
            new_height: 32,
            anchor_col: 1,
            anchor_row: 2, // Bottom-Center default for character sprites
        }
    }
}

impl Resize9GridModalState {
    pub fn open_for(&mut self, project: &Project) {
        self.new_width = project.metadata.frame_width.max(1);
        self.new_height = project.metadata.frame_height.max(1);
        self.open = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<(u32, u32, u8, u8)> {
        if !self.open {
            return None;
        }
        let mut applied = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.resize9.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(310.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.resize9.new_w"));
                    ui.add(egui::DragValue::new(&mut self.new_width).range(1..=2048));
                });
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.resize9.new_h"));
                    ui.add(egui::DragValue::new(&mut self.new_height).range(1..=2048));
                });

                ui.add_space(6.0);
                ui.label(i18n.tr("modal.resize9.anchor"));
                let arrows = [
                    ["↖", "↑", "↗"],
                    ["←", "•", "→"],
                    ["↙", "↓", "↘"],
                ];
                egui::Grid::new("resize_9grid_selector")
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for r in 0..3_u8 {
                            for c in 0..3_u8 {
                                let selected = self.anchor_col == c && self.anchor_row == r;
                                let btn = egui::Button::new(arrows[r as usize][c as usize])
                                    .selected(selected)
                                    .min_size(Vec2::splat(30.0));
                                if ui.add(btn).clicked() {
                                    self.anchor_col = c;
                                    self.anchor_row = r;
                                }
                            }
                            ui.end_row();
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button(format!("{FA_CHECK} {}", i18n.tr("modal.resize9.apply"))).clicked() {
                        applied = Some((
                            self.new_width,
                            self.new_height,
                            self.anchor_col,
                            self.anchor_row,
                        ));
                        close_requested = true;
                    }
                    if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel"))).clicked() {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        applied
    }
}

pub struct SheetSlicerModalState {
    pub open: bool,
    pub sheet_name: String,
    pub source_rgba: Option<RgbaImage>,
    pub frame_w: u32,
    pub frame_h: u32,
    pub margin_x: u32,
    pub margin_y: u32,
    pub spacing_x: u32,
    pub spacing_y: u32,
    pub target_columns: u32,
    pub selected_frames: Vec<bool>,
    pub preview_texture: Option<egui::TextureHandle>,
    pub preview_zoom: f32,
}

impl Default for SheetSlicerModalState {
    fn default() -> Self {
        Self {
            open: false,
            sheet_name: String::from("sliced_sprite"),
            source_rgba: None,
            frame_w: 32,
            frame_h: 32,
            margin_x: 0,
            margin_y: 0,
            spacing_x: 0,
            spacing_y: 0,
            target_columns: 4,
            selected_frames: Vec::new(),
            preview_texture: None,
            preview_zoom: 0.0,
        }
    }
}

impl SheetSlicerModalState {
    pub fn open_with_sheet(&mut self, name: String, rgba: RgbaImage) {
        self.sheet_name = name;
        self.source_rgba = Some(rgba);
        self.selected_frames.clear();
        self.preview_texture = None;
        self.preview_zoom = 0.0;
        self.auto_detect_grid();
        self.open = true;
    }

    pub fn count_detected_frames(&self) -> (u32, u32) {
        let src = match &self.source_rgba {
            Some(img) => img,
            None => return (0, 0),
        };
        let fw = self.frame_w.max(1);
        let fh = self.frame_h.max(1);
        let step_x = fw.saturating_add(self.spacing_x).max(1);
        let step_y = fh.saturating_add(self.spacing_y).max(1);

        let mut cols = 0;
        let mut x = self.margin_x;
        while x.saturating_add(fw) <= src.width() {
            cols += 1;
            x = x.saturating_add(step_x);
        }

        let mut rows = 0;
        let mut y = self.margin_y;
        while y.saturating_add(fh) <= src.height() {
            rows += 1;
            y = y.saturating_add(step_y);
        }
        (cols, rows)
    }

    pub fn sync_selected_frames(&mut self) {
        let (cols, rows) = self.count_detected_frames();
        let total = (cols * rows) as usize;
        if self.selected_frames.len() != total {
            self.selected_frames = vec![true; total];
            if let Some(src) = &self.source_rgba {
                let fw = self.frame_w.max(1);
                let fh = self.frame_h.max(1);
                let step_x = fw.saturating_add(self.spacing_x).max(1);
                let step_y = fh.saturating_add(self.spacing_y).max(1);
                for idx in 0..total {
                    let r = (idx as u32) / cols.max(1);
                    let c = (idx as u32) % cols.max(1);
                    let ox = self.margin_x + c * step_x;
                    let oy = self.margin_y + r * step_y;
                    let mut has_pixel = false;
                    for ly in 0..fh {
                        for lx in 0..fw {
                            let px = ox + lx;
                            let py = oy + ly;
                            if px < src.width() && py < src.height() && src.get_pixel(px, py)[3] > 0 {
                                has_pixel = true;
                                break;
                            }
                        }
                        if has_pixel {
                            break;
                        }
                    }
                    if !has_pixel {
                        self.selected_frames[idx] = false;
                    }
                }
            }
        }
    }

    pub fn auto_detect_grid(&mut self) {
        let src = match &self.source_rgba {
            Some(s) => s,
            None => return,
        };
        let w = src.width();
        let h = src.height();
        if w == 0 || h == 0 {
            return;
        }

        let mut col_has = vec![false; w as usize];
        let mut row_has = vec![false; h as usize];
        for y in 0..h {
            for x in 0..w {
                if src.get_pixel(x, y)[3] > 10 {
                    col_has[x as usize] = true;
                    row_has[y as usize] = true;
                }
            }
        }

        let extract_spans = |has: &[bool]| {
            let mut spans = Vec::new();
            let mut in_span = false;
            let mut start = 0;
            for (i, &v) in has.iter().enumerate() {
                if v && !in_span {
                    in_span = true;
                    start = i;
                } else if !v && in_span {
                    in_span = false;
                    spans.push((start as u32, (i - 1) as u32));
                }
            }
            if in_span {
                spans.push((start as u32, (has.len() - 1) as u32));
            }
            spans
        };

        let x_spans = extract_spans(&col_has);
        let y_spans = extract_spans(&row_has);

        let solve_axis = |spans: &[(u32, u32)], dim: u32| -> (u32, u32, u32) {
            if spans.is_empty() {
                return (dim.clamp(1, 32), 0, 0);
            }
            let max_span_len = spans.iter().map(|&(s, e)| e - s + 1).max().unwrap_or(1);

            if spans.len() == 1 {
                for sz in [dim, 128, 96, 80, 64, 48, 32, 24, 16] {
                    if sz >= max_span_len && sz <= dim && dim.is_multiple_of(sz) {
                        return (sz, 0, 0);
                    }
                }
                return (dim, 0, 0);
            }

            let centers: Vec<f64> = spans.iter().map(|&(s, e)| (s + e) as f64 / 2.0).collect();
            let mut deltas: Vec<f64> = centers.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
            deltas.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median_delta = deltas[deltas.len() / 2].round() as u32;

            if median_delta >= max_span_len {
                if dim.is_multiple_of(median_delta) {
                    let fits = spans.iter().all(|&(s, e)| {
                        let k = s / median_delta;
                        e < (k + 1) * median_delta
                    });
                    if fits {
                        return (median_delta, 0, 0);
                    }
                }

                let first_s = spans.first().map(|&(s, _)| s).unwrap_or(0);
                for m in (0..=first_s).rev() {
                    if (dim.saturating_sub(m)) % median_delta == 0 || (dim.saturating_sub(2 * m)).is_multiple_of(median_delta) {
                        let fits = spans.iter().all(|&(s, e)| {
                            if s < m { return false; }
                            let k = (s - m) / median_delta;
                            e < m + (k + 1) * median_delta
                        });
                        if fits {
                            return (median_delta, m, 0);
                        }
                    }
                }

                for candidate_cell in [median_delta, median_delta.next_power_of_two()] {
                    if candidate_cell >= max_span_len && candidate_cell <= dim && dim.is_multiple_of(candidate_cell) {
                        return (candidate_cell, 0, 0);
                    }
                }
            }

            for sz in [128, 96, 80, 64, 48, 32, 24, 16] {
                if sz >= max_span_len && dim.is_multiple_of(sz) {
                    return (sz, 0, 0);
                }
            }
            (dim, 0, 0)
        };

        let (fw, mx, sx) = solve_axis(&x_spans, w);
        let (fh, my, sy) = solve_axis(&y_spans, h);

        self.margin_x = mx;
        self.margin_y = my;
        self.spacing_x = sx;
        self.spacing_y = sy;
        self.frame_w = fw.min(w).max(1);
        self.frame_h = fh.min(h).max(1);
        let step_x = self.frame_w.saturating_add(self.spacing_x).max(1);
        self.target_columns = (w.saturating_sub(self.margin_x) / step_x).max(1);

        self.selected_frames.clear();
        self.sync_selected_frames();
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<Project> {
        if !self.open || self.source_rgba.is_none() {
            return None;
        }
        let mut imported_project = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        let (src_w, src_h) = self
            .source_rgba
            .as_ref()
            .map(|i| i.dimensions())
            .unwrap_or((0, 0));

        self.sync_selected_frames();

        Window::new(i18n.tr("modal.slicer.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(520.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.slicer.source_size"));
                    ui.strong(format!("{}×{} px", src_w, src_h));
                });

                egui::Grid::new("sheet_slicer_grid")
                    .num_columns(2)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.slicer.frame_w"));
                        ui.add(egui::DragValue::new(&mut self.frame_w).range(1..=src_w.max(1)));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.frame_h"));
                        ui.add(egui::DragValue::new(&mut self.frame_h).range(1..=src_h.max(1)));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.margin_x"));
                        ui.add(egui::DragValue::new(&mut self.margin_x).range(0..=512));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.margin_y"));
                        ui.add(egui::DragValue::new(&mut self.margin_y).range(0..=512));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.spacing_x"));
                        ui.add(egui::DragValue::new(&mut self.spacing_x).range(0..=512));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.spacing_y"));
                        ui.add(egui::DragValue::new(&mut self.spacing_y).range(0..=512));
                        ui.end_row();

                        ui.label(i18n.tr("modal.slicer.out_cols"));
                        ui.add(egui::DragValue::new(&mut self.target_columns).range(1..=128));
                        ui.end_row();
                    });

                let (det_cols, det_rows) = self.count_detected_frames();
                let total_det = (det_cols * det_rows).max(1);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.slicer.detected"));
                    ui.strong(format!("{} ({}×{})", total_det, det_cols, det_rows));
                });

                ui.horizontal(|ui| {
                    if ui.button(format!("{FA_SLIDERS} {}", i18n.tr("modal.slicer.auto_detect"))).clicked() {
                        self.auto_detect_grid();
                    }
                    if ui.button(format!("{FA_CHECK} {}", i18n.tr("modal.slicer.select_all"))).clicked() {
                        for v in &mut self.selected_frames {
                            *v = true;
                        }
                    }
                    if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.slicer.deselect_all"))).clicked() {
                        for v in &mut self.selected_frames {
                            *v = false;
                        }
                    }
                });

                let selected_count = self.selected_frames.iter().filter(|&&v| v).count();
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.slicer.selected_count"));
                    ui.strong(format!("{} / {}", selected_count, total_det));
                });

                // Preview with interactive selection & zoom
                if let Some(src) = &self.source_rgba {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(i18n.tr("modal.slicer.preview")).strong());
                        ui.add_space(8.0);
                        if ui.selectable_label(self.preview_zoom <= 0.0, i18n.tr("zoom.fit")).clicked() {
                            self.preview_zoom = 0.0;
                        }
                        if ui.selectable_label(self.preview_zoom == 1.0, "100%").clicked() {
                            self.preview_zoom = 1.0;
                        }
                        if ui.selectable_label(self.preview_zoom == 2.0, "200%").clicked() {
                            self.preview_zoom = 2.0;
                        }
                        if ui.selectable_label(self.preview_zoom == 3.0, "300%").clicked() {
                            self.preview_zoom = 3.0;
                        }
                        if ui.selectable_label(self.preview_zoom == 4.0, "400%").clicked() {
                            self.preview_zoom = 4.0;
                        }
                        if ui.small_button(FA_MINUS).on_hover_text(i18n.tr("action.zoom_out")).clicked() {
                            if self.preview_zoom <= 1.0 {
                                self.preview_zoom = 0.0;
                            } else {
                                self.preview_zoom = (self.preview_zoom - 1.0).max(1.0);
                            }
                        }
                        if ui.small_button(FA_PLUS).on_hover_text(i18n.tr("action.zoom_in")).clicked() {
                            if self.preview_zoom <= 0.0 {
                                self.preview_zoom = 2.0;
                            } else {
                                self.preview_zoom = (self.preview_zoom + 1.0).min(8.0);
                            }
                        }
                    });

                    let tex = self.preview_texture.get_or_insert_with(|| {
                        ctx.load_texture(
                            "slicer_preview_sheet",
                            egui::ColorImage::from_rgba_unmultiplied(
                                [src.width() as usize, src.height() as usize],
                                src.as_raw(),
                            ),
                            egui::TextureOptions::NEAREST,
                        )
                    });

                    let view_w = 500.0;
                    let view_h = 240.0;

                    egui::ScrollArea::both()
                        .id_salt("slicer_preview_scroll_area")
                        .max_width(view_w)
                        .max_height(view_h)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let (pw, ph) = if self.preview_zoom <= 0.0 {
                                let max_w = view_w - 14.0;
                                let max_h = view_h - 14.0;
                                let sheet_aspect = src.width() as f32 / (src.height() as f32).max(1.0);
                                if sheet_aspect > (max_w / max_h) {
                                    (max_w, max_w / sheet_aspect)
                                } else {
                                    (max_h * sheet_aspect, max_h)
                                }
                            } else {
                                (src.width() as f32 * self.preview_zoom, src.height() as f32 * self.preview_zoom)
                            };

                            let (rect, response) = ui.allocate_exact_size(egui::vec2(pw, ph), egui::Sense::click());
                            let painter = ui.painter_at(rect);

                            // Checkerboard background
                            painter.rect_filled(rect, 2.0, Color32::from_rgb(24, 26, 32));

                            // Sheet texture
                            painter.image(
                                tex.id(),
                                rect,
                                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );

                            let scale_x = rect.width() / src.width() as f32;
                            let scale_y = rect.height() / src.height() as f32;

                            let fw = self.frame_w.max(1);
                            let fh = self.frame_h.max(1);
                            let step_x = fw.saturating_add(self.spacing_x).max(1);
                            let step_y = fh.saturating_add(self.spacing_y).max(1);

                            let mut clicked_idx = None;
                            let click_pos = if response.clicked() {
                                response.interact_pointer_pos()
                            } else {
                                None
                            };

                            for idx in 0..(total_det as usize) {
                                let r = (idx as u32) / det_cols.max(1);
                                let c = (idx as u32) % det_cols.max(1);
                                let ox = self.margin_x + c * step_x;
                                let oy = self.margin_y + r * step_y;

                                let cell_rect = egui::Rect::from_min_size(
                                    egui::pos2(rect.min.x + ox as f32 * scale_x, rect.min.y + oy as f32 * scale_y),
                                    egui::vec2(fw as f32 * scale_x, fh as f32 * scale_y),
                                );

                                if let Some(cp) = click_pos {
                                    if cell_rect.contains(cp) {
                                        clicked_idx = Some(idx);
                                    }
                                }

                                let is_selected = self.selected_frames.get(idx).copied().unwrap_or(true);
                                if is_selected {
                                    painter.rect_stroke(
                                        cell_rect,
                                        0.0,
                                        egui::Stroke::new(1.5, Color32::from_rgb(0, 230, 120)),
                                        egui::StrokeKind::Inside,
                                    );
                                    painter.rect_filled(
                                        cell_rect,
                                        0.0,
                                        Color32::from_rgba_unmultiplied(0, 230, 120, 25),
                                    );
                                } else {
                                    painter.rect_filled(cell_rect, 0.0, Color32::from_black_alpha(170));
                                    painter.rect_stroke(
                                        cell_rect,
                                        0.0,
                                        egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(220, 60, 60, 140)),
                                        egui::StrokeKind::Inside,
                                    );
                                }
                            }

                            if let Some(idx) = clicked_idx {
                                if let Some(val) = self.selected_frames.get_mut(idx) {
                                    *val = !*val;
                                }
                            }
                        });
                }

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button(format!("{FA_IMPORT} {}", i18n.tr("modal.slicer.import"))).clicked() {
                        if let Some(src) = &self.source_rgba {
                            imported_project = Some(Project::from_sliced_sheet_selected(
                                self.sheet_name.clone(),
                                src,
                                self.frame_w,
                                self.frame_h,
                                self.margin_x,
                                self.margin_y,
                                self.spacing_x,
                                self.spacing_y,
                                self.target_columns,
                                Some(&self.selected_frames),
                            ));
                        }
                        close_requested = true;
                    }
                    if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel"))).clicked() {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        if !self.open {
            self.source_rgba = None;
            self.preview_texture = None;
        }
        imported_project
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsavedAction {
    None,
    SaveProject,
    DiscardAndProceed,
    Cancel,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsavedContext {
    NewProject,
    CloseApp,
}

#[allow(dead_code)]
pub struct UnsavedChangesModalState {
    pub open: bool,
    pub context: UnsavedContext,
}

impl Default for UnsavedChangesModalState {
    fn default() -> Self {
        Self {
            open: false,
            context: UnsavedContext::NewProject,
        }
    }
}

#[allow(dead_code)]
impl UnsavedChangesModalState {
    pub fn open_for(&mut self, context: UnsavedContext) {
        self.context = context;
        self.open = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> UnsavedAction {
        if !self.open {
            return UnsavedAction::None;
        }
        let mut action = UnsavedAction::None;
        let mut window_open = self.open;
        let mut close_requested = false;

        let prompt = match self.context {
            UnsavedContext::NewProject => i18n.tr("modal.unsaved.new_prompt"),
            UnsavedContext::CloseApp => i18n.tr("modal.unsaved.close_prompt"),
        };

        Window::new(i18n.tr("modal.unsaved.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(390.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    ui.label(prompt);
                    ui.add_space(14.0);
                });

                let btn_size = egui::vec2(118.0, 26.0);
                let spacing = 8.0;
                let total_btn_width = btn_size.x * 3.0 + spacing * 2.0;
                let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.add_space(offset);
                    if ui
                        .add_sized(
                            btn_size,
                            egui::Button::new(format!("{FA_SAVE} {}", i18n.tr("modal.unsaved.save"))),
                        )
                        .clicked()
                    {
                        action = UnsavedAction::SaveProject;
                        close_requested = true;
                    }
                    if ui
                        .add_sized(
                            btn_size,
                            egui::Button::new(
                                egui::RichText::new(format!(
                                    "{FA_TRASH} {}",
                                    i18n.tr("modal.unsaved.discard")
                                ))
                                .color(Color32::from_rgb(240, 110, 110)),
                            ),
                        )
                        .clicked()
                    {
                        action = UnsavedAction::DiscardAndProceed;
                        close_requested = true;
                    }
                    if ui
                        .add_sized(
                            btn_size,
                            egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("modal.unsaved.cancel"))),
                        )
                        .clicked()
                    {
                        action = UnsavedAction::Cancel;
                        close_requested = true;
                    }
                });
                ui.add_space(4.0);

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    action = UnsavedAction::Cancel;
                    close_requested = true;
                }
            });

        self.open = window_open && !close_requested;
        action
    }
}

pub struct ExportScaledPngModalState {
    pub open: bool,
    pub scale_factor: u32,
    pub extrude_px: u32,
}

impl Default for ExportScaledPngModalState {
    fn default() -> Self {
        Self {
            open: false,
            scale_factor: 2,
            extrude_px: 0,
        }
    }
}

impl ExportScaledPngModalState {
    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
        project: &Project,
    ) -> Option<(u32, u32)> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.export_png.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(320.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_png.scale"));
                    for s in [1_u32, 2, 4, 8] {
                        if ui
                            .selectable_label(self.scale_factor == s, format!("{}x", s))
                            .clicked()
                        {
                            self.scale_factor = s;
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_png.extrude"));
                    for p in [0_u32, 1, 2] {
                        if ui
                            .selectable_label(self.extrude_px == p, format!("{}px", p))
                            .clicked()
                        {
                            self.extrude_px = p;
                        }
                    }
                });

                let cell_w =
                    project.metadata.frame_width * self.scale_factor + self.extrude_px * 2;
                let cell_h =
                    project.metadata.frame_height * self.scale_factor + self.extrude_px * 2;
                let out_w = cell_w * project.metadata.columns.max(1);
                let out_h = cell_h * project.metadata.rows.max(1);

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_png.output_size"));
                    ui.strong(format!("{}×{} px", out_w, out_h));
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        let btn_w = 120.0;
                        let gap = 12.0;
                        let total_w = btn_w * 2.0 + gap;
                        let pad = ((ui.available_width() - total_w) * 0.5).max(0.0);
                        ui.add_space(pad);
                        if ui
                            .add_sized(
                                [btn_w, 28.0],
                                egui::Button::new(
                                    RichText::new(format!("{FA_EXPORT} {}", i18n.tr("modal.export_png.export")))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            triggered = Some((self.scale_factor, self.extrude_px));
                            close_requested = true;
                        }
                        ui.add_space(gap);
                        if ui
                            .add_sized(
                                [btn_w, 28.0],
                                egui::Button::new(
                                    RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel")))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            close_requested = true;
                        }
                    });
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

pub struct ExportGifModalState {
    pub open: bool,
    pub scale_factor: u32,
    pub tag_index: Option<usize>,
}

impl Default for ExportGifModalState {
    fn default() -> Self {
        Self {
            open: false,
            scale_factor: 2,
            tag_index: None,
        }
    }
}

impl ExportGifModalState {
    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
        project: &Project,
    ) -> Option<(u32, usize, usize)> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        if let Some(idx) = self.tag_index {
            if idx >= project.metadata.tags.len() {
                self.tag_index = None;
            }
        }

        Window::new(i18n.tr("modal.export_gif.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(320.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_gif.scale"));
                    for s in [1_u32, 2, 4, 8] {
                        if ui
                            .selectable_label(self.scale_factor == s, format!("{}x", s))
                            .clicked()
                        {
                            self.scale_factor = s;
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_gif.range"));
                    let selected_label = match self.tag_index {
                        Some(idx) => project
                            .metadata
                            .tags
                            .get(idx)
                            .map(|t| format!("{} ({}-{})", t.name, t.start_frame + 1, t.end_frame + 1))
                            .unwrap_or_else(|| i18n.tr("panel.preview.all_frames").to_string()),
                        None => i18n.tr("panel.preview.all_frames").to_string(),
                    };
                    egui::ComboBox::from_id_salt("gif_export_tag_combo")
                        .selected_text(selected_label)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.tag_index,
                                None,
                                i18n.tr("panel.preview.all_frames"),
                            );
                            for (idx, tag) in project.metadata.tags.iter().enumerate() {
                                ui.selectable_value(
                                    &mut self.tag_index,
                                    Some(idx),
                                    format!("{} ({}-{})", tag.name, tag.start_frame + 1, tag.end_frame + 1),
                                );
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.export_gif.fps"));
                    ui.strong(format!("{} FPS", project.metadata.fps));
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        let btn_w = 120.0;
                        let gap = 12.0;
                        let total_w = btn_w * 2.0 + gap;
                        let pad = ((ui.available_width() - total_w) * 0.5).max(0.0);
                        ui.add_space(pad);
                        if ui
                            .add_sized(
                                [btn_w, 28.0],
                                egui::Button::new(
                                    RichText::new(format!("{FA_PLAY} {}", i18n.tr("modal.export_gif.export")))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            let max_idx =
                                (project.metadata.frame_count.max(1) as usize).saturating_sub(1);
                            let (start_f, end_f) = match self
                                .tag_index
                                .and_then(|idx| project.metadata.tags.get(idx))
                            {
                                Some(t) => (
                                    t.start_frame.min(max_idx),
                                    t.end_frame.min(max_idx).max(t.start_frame.min(max_idx)),
                                ),
                                None => (0, max_idx),
                            };
                            triggered = Some((self.scale_factor, start_f, end_f));
                            close_requested = true;
                        }
                        ui.add_space(gap);
                        if ui
                            .add_sized(
                                [btn_w, 28.0],
                                egui::Button::new(
                                    RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel")))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            close_requested = true;
                        }
                    });
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportPackedAtlasAction {
    pub settings: PackerSettings,
}

pub struct ExportPackedAtlasModalState {
    pub open: bool,
    pub settings: PackerSettings,
    pub estimated_dims: (u32, u32),
    pub estimated_efficiency: f32,
}

impl Default for ExportPackedAtlasModalState {
    fn default() -> Self {
        Self {
            open: false,
            settings: PackerSettings::default(),
            estimated_dims: (0, 0),
            estimated_efficiency: 0.0,
        }
    }
}

impl ExportPackedAtlasModalState {
    pub fn open_for(&mut self, project: &Project) {
        self.open = true;
        if let Ok(res) = pack_project_atlas(project, &self.settings) {
            self.estimated_dims = (res.width, res.height);
            self.estimated_efficiency = res.efficiency;
        }
    }

    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
        project: Option<&Project>,
    ) -> Option<ExportPackedAtlasAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.pack_atlas.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(340.0);
                ui.label(i18n.tr("modal.pack_atlas.prompt"));
                ui.add_space(8.0);

                let mut settings_changed = false;

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.pack_atlas.padding"));
                    if ui.add(egui::Slider::new(&mut self.settings.padding, 0..=8).suffix(" px")).changed() {
                        settings_changed = true;
                    }
                });

                if ui.checkbox(&mut self.settings.trim, i18n.tr("modal.pack_atlas.trim")).changed() {
                    settings_changed = true;
                }

                if ui.checkbox(&mut self.settings.power_of_two, i18n.tr("modal.pack_atlas.power_of_two")).changed() {
                    settings_changed = true;
                }

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.pack_atlas.scale"));
                    for &s in &[1, 2, 4] {
                        if ui.selectable_value(&mut self.settings.scale, s, format!("{}x", s)).changed() {
                            settings_changed = true;
                        }
                    }
                });

                if settings_changed {
                    if let Some(p) = project {
                        if let Ok(res) = pack_project_atlas(p, &self.settings) {
                            self.estimated_dims = (res.width, res.height);
                            self.estimated_efficiency = res.efficiency;
                        }
                    }
                }

                ui.add_space(6.0);
                egui::Frame::canvas(ui.style())
                    .fill(ui.visuals().extreme_bg_color)
                    .corner_radius(4.0)
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        ui.small(format!(
                            "{} {}×{} px  •  {} {:.1}%",
                            i18n.tr("modal.pack_atlas.sheet_size"),
                            self.estimated_dims.0,
                            self.estimated_dims.1,
                            i18n.tr("modal.pack_atlas.efficiency"),
                            self.estimated_efficiency
                        ));
                    });

                ui.add_space(10.0);
                let btn_w = (ui.available_width() - 8.0) * 0.5;
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_EXPORT} {}", i18n.tr("modal.pack_atlas.export_btn"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(ExportPackedAtlasAction {
                            settings: self.settings,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceColorAction {
    pub from_color: [u8; 4],
    pub to_color: [u8; 4],
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub tolerance: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplaceColorPickTarget {
    From,
    To,
}

pub struct ReplaceColorModalState {
    pub open: bool,
    pub from_color: [u8; 4],
    pub to_color: [u8; 4],
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub tolerance: u8,
    pub preview: bool,
    pub picking: Option<ReplaceColorPickTarget>,
}

impl Default for ReplaceColorModalState {
    fn default() -> Self {
        Self {
            open: false,
            from_color: [0, 0, 0, 255],
            to_color: [255, 255, 255, 255],
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            tolerance: 0,
            preview: true,
            picking: None,
        }
    }
}

impl ReplaceColorModalState {
    pub fn open_with(&mut self, primary: [u8; 4], secondary: [u8; 4], has_selection: bool) {
        self.open = true;
        self.from_color = primary;
        self.to_color = secondary;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.tolerance = 0;
        self.preview = true;
        self.picking = None;
    }

    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
        current_primary: [u8; 4],
        current_secondary: [u8; 4],
        mut settings: Option<&mut AppSettings>,
        palette: Option<&[[u8; 4]]>,
    ) -> Option<ReplaceColorAction> {
        if !self.open {
            return None;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.picking = None;
        }

        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.replace_color.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.replace_color.prompt"));
                ui.add_space(8.0);

                // Row: From color & quick buttons
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.replace_color.from"));
                    let s_opt = settings.as_deref_mut();
                    crate::ui::color_picker::modern_color_picker_button(
                        ui,
                        "replace_color_from",
                        &mut self.from_color,
                        i18n,
                        s_opt,
                    );
                    ui.monospace(crate::ui::color_picker::format_hex(self.from_color));

                    let is_picking_from = self.picking == Some(ReplaceColorPickTarget::From);
                    let eyedropper_btn = egui::Button::new(RichText::new(FA_EYEDROPPER).size(13.0))
                        .selected(is_picking_from);
                    if ui
                        .add(eyedropper_btn)
                        .on_hover_text(i18n.tr("modal.replace_color.pick_from_hint"))
                        .clicked()
                    {
                        if is_picking_from {
                            self.picking = None;
                        } else {
                            self.picking = Some(ReplaceColorPickTarget::From);
                        }
                    }

                    if ui.small_button(i18n.tr("modal.replace_color.use_fg")).clicked() {
                        self.from_color = current_primary;
                    }
                    if ui.small_button(i18n.tr("modal.replace_color.use_bg")).clicked() {
                        self.from_color = current_secondary;
                    }
                });

                // Row: Swap button
                ui.horizontal(|ui| {
                    ui.add_space(60.0);
                    if ui.small_button(format!("{FA_SWAP} {}", i18n.tr("modal.replace_color.swap"))).clicked() {
                        std::mem::swap(&mut self.from_color, &mut self.to_color);
                    }
                });

                // Row: To color & quick buttons
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.replace_color.to"));
                    crate::ui::color_picker::modern_color_picker_button(
                        ui,
                        "replace_color_to",
                        &mut self.to_color,
                        i18n,
                        settings,
                    );
                    ui.monospace(crate::ui::color_picker::format_hex(self.to_color));

                    let is_picking_to = self.picking == Some(ReplaceColorPickTarget::To);
                    let eyedropper_btn = egui::Button::new(RichText::new(FA_EYEDROPPER).size(13.0))
                        .selected(is_picking_to);
                    if ui
                        .add(eyedropper_btn)
                        .on_hover_text(i18n.tr("modal.replace_color.pick_to_hint"))
                        .clicked()
                    {
                        if is_picking_to {
                            self.picking = None;
                        } else {
                            self.picking = Some(ReplaceColorPickTarget::To);
                        }
                    }

                    if ui.small_button(i18n.tr("modal.replace_color.use_fg")).clicked() {
                        self.to_color = current_primary;
                    }
                    if ui.small_button(i18n.tr("modal.replace_color.use_bg")).clicked() {
                        self.to_color = current_secondary;
                    }
                });

                // Active pipette indicator banner
                if let Some(target) = self.picking {
                    ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
                    ui.add_space(4.0);
                    let target_label = match target {
                        ReplaceColorPickTarget::From => i18n.tr("modal.replace_color.picking_from_active"),
                        ReplaceColorPickTarget::To => i18n.tr("modal.replace_color.picking_to_active"),
                    };
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{FA_EYEDROPPER} {target_label}"))
                                .color(ui.visuals().warn_fg_color)
                                .small(),
                        );
                        if ui.small_button(i18n.tr("modal.replace_color.cancel_pick")).clicked() {
                            self.picking = None;
                        }
                    });
                }

                // Quick Palette Swatches strip
                if let Some(pal) = palette {
                    if !pal.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(i18n.tr("modal.replace_color.palette_quick_pick")).small().strong());
                        egui::ScrollArea::horizontal()
                            .id_salt("replace_color_pal_scroll")
                            .max_height(28.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 3.0;
                                    for col in pal {
                                        let [r, g, b, a] = *col;
                                        let c32 = Color32::from_rgba_unmultiplied(r, g, b, a);
                                        let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::click());
                                        ui.painter().rect_filled(rect, 2.0, c32);
                                        let is_from = *col == self.from_color;
                                        let is_to = *col == self.to_color;
                                        let border_color = if is_from || is_to {
                                            Color32::WHITE
                                        } else if resp.hovered() {
                                            ui.visuals().widgets.hovered.bg_stroke.color
                                        } else {
                                            Color32::from_black_alpha(100)
                                        };
                                        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, border_color), StrokeKind::Inside);

                                        let resp = resp.on_hover_text(format!(
                                            "#{:02X}{:02X}{:02X}\n{}",
                                            r, g, b, i18n.tr("modal.replace_color.palette_swatch_hint")
                                        ));
                                        if resp.clicked() {
                                            match self.picking {
                                                Some(ReplaceColorPickTarget::To) => {
                                                    self.to_color = *col;
                                                    self.picking = None;
                                                }
                                                _ => {
                                                    self.from_color = *col;
                                                    if self.picking == Some(ReplaceColorPickTarget::From) {
                                                        self.picking = None;
                                                    }
                                                }
                                            }
                                        } else if resp.secondary_clicked() {
                                            self.to_color = *col;
                                            if self.picking == Some(ReplaceColorPickTarget::To) {
                                                self.picking = None;
                                            }
                                        }
                                    }
                                });
                            });
                    }
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                ui.checkbox(&mut self.all_frames, i18n.tr("modal.replace_color.all_frames"));
                if self.has_selection {
                    ui.checkbox(&mut self.in_selection_only, i18n.tr("modal.replace_color.in_selection"));
                }
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.replace_color.tolerance"));
                    ui.add(egui::DragValue::new(&mut self.tolerance).range(0..=255));
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 166.0;

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_FLOOD_FILL} {}", i18n.tr("modal.replace_color.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(ReplaceColorAction {
                            from_color: self.from_color,
                            to_color: self.to_color,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                            tolerance: self.tolerance,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel")))
                                    .strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorAdjustAction {
    pub hue_shift: f32,
    pub sat_shift: f32,
    pub val_shift: f32,
    pub contrast: f32,
    pub all_frames: bool,
    pub in_selection_only: bool,
}

pub struct ColorAdjustModalState {
    pub open: bool,
    pub hue_shift: f32,
    pub sat_shift: f32,
    pub val_shift: f32,
    pub contrast: f32,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub preview: bool,
}

impl Default for ColorAdjustModalState {
    fn default() -> Self {
        Self {
            open: false,
            hue_shift: 0.0,
            sat_shift: 0.0,
            val_shift: 0.0,
            contrast: 0.0,
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            preview: true,
        }
    }
}

impl ColorAdjustModalState {
    pub fn open_with(&mut self, has_selection: bool) {
        self.open = true;
        self.hue_shift = 0.0;
        self.sat_shift = 0.0;
        self.val_shift = 0.0;
        self.contrast = 0.0;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.preview = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<ColorAdjustAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.color_adjust.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(360.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(360.0);
                ui.label(i18n.tr("modal.color_adjust.prompt"));
                ui.add_space(8.0);

                egui::Grid::new("color_adjust_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.color_adjust.hue"));
                        ui.add(egui::Slider::new(&mut self.hue_shift, -180.0..=180.0).suffix("°"));
                        ui.end_row();

                        ui.label(i18n.tr("modal.color_adjust.sat"));
                        ui.add(egui::Slider::new(&mut self.sat_shift, -100.0..=100.0).suffix("%"));
                        ui.end_row();

                        ui.label(i18n.tr("modal.color_adjust.val"));
                        ui.add(egui::Slider::new(&mut self.val_shift, -100.0..=100.0).suffix("%"));
                        ui.end_row();

                        ui.label(i18n.tr("modal.color_adjust.contrast"));
                        ui.add(egui::Slider::new(&mut self.contrast, -100.0..=100.0).suffix("%"));
                        ui.end_row();
                    });

                ui.add_space(6.0);
                if ui
                    .small_button(format!("{FA_RESET} {}", i18n.tr("modal.color_adjust.reset")))
                    .clicked()
                {
                    self.hue_shift = 0.0;
                    self.sat_shift = 0.0;
                    self.val_shift = 0.0;
                    self.contrast = 0.0;
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                ui.checkbox(&mut self.all_frames, i18n.tr("modal.replace_color.all_frames"));
                if self.has_selection {
                    ui.checkbox(&mut self.in_selection_only, i18n.tr("modal.replace_color.in_selection"));
                }
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 176.0;

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_SLIDERS} {}", i18n.tr("modal.color_adjust.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(ColorAdjustAction {
                            hue_shift: self.hue_shift,
                            sat_shift: self.sat_shift,
                            val_shift: self.val_shift,
                            contrast: self.contrast,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.new_project.cancel")))
                                    .strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModifySelectionMode {
    #[default]
    Expand,
    Contract,
    Border,
}

pub struct ModifySelectionModalState {
    pub open: bool,
    pub mode: ModifySelectionMode,
    pub shape: SelectionModifyShape,
    pub amount: i32,
    pub preview: bool,
}

impl Default for ModifySelectionModalState {
    fn default() -> Self {
        Self {
            open: false,
            mode: ModifySelectionMode::Expand,
            shape: SelectionModifyShape::Circle,
            amount: 1,
            preview: true,
        }
    }
}

impl ModifySelectionModalState {
    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
    ) -> Option<(ModifySelectionMode, SelectionModifyShape, i32)> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.modify_sel.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.modify_sel.prompt"));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.mode,
                        ModifySelectionMode::Expand,
                        i18n.tr("modal.modify_sel.expand"),
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        ModifySelectionMode::Contract,
                        i18n.tr("modal.modify_sel.contract"),
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        ModifySelectionMode::Border,
                        i18n.tr("modal.modify_sel.border"),
                    );
                });
                ui.add_space(6.0);

                egui::Grid::new("modify_sel_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.modify_sel.pixels"));
                        ui.add(egui::DragValue::new(&mut self.amount).range(1..=32).suffix("px"));
                        ui.end_row();

                        ui.label(i18n.tr("modal.modify_sel.shape"));
                        egui::ComboBox::from_id_salt("modify_sel_shape_combo")
                            .selected_text(match self.shape {
                                SelectionModifyShape::Circle => {
                                    i18n.tr("modal.modify_sel.shape_circle")
                                }
                                SelectionModifyShape::Square => {
                                    i18n.tr("modal.modify_sel.shape_square")
                                }
                                SelectionModifyShape::Diamond => {
                                    i18n.tr("modal.modify_sel.shape_diamond")
                                }
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.shape,
                                    SelectionModifyShape::Circle,
                                    i18n.tr("modal.modify_sel.shape_circle"),
                                );
                                ui.selectable_value(
                                    &mut self.shape,
                                    SelectionModifyShape::Square,
                                    i18n.tr("modal.modify_sel.shape_square"),
                                );
                                ui.selectable_value(
                                    &mut self.shape,
                                    SelectionModifyShape::Diamond,
                                    i18n.tr("modal.modify_sel.shape_diamond"),
                                );
                            });
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 166.0;

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CHECK} {}", i18n.tr("modal.modify_sel.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some((self.mode, self.shape, self.amount.clamp(1, 32)));
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineFxAction {
    pub color: [u8; 4],
    pub thickness: i32,
    pub corner_style: OutlineCornerStyle,
    pub placement: OutlinePlacement,
    pub all_frames: bool,
    pub in_selection_only: bool,
}

pub struct OutlineFxModalState {
    pub open: bool,
    pub color: [u8; 4],
    pub thickness: i32,
    pub corner_style: OutlineCornerStyle,
    pub placement: OutlinePlacement,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub preview: bool,
}

impl Default for OutlineFxModalState {
    fn default() -> Self {
        Self {
            open: false,
            color: [0, 0, 0, 255],
            thickness: 1,
            corner_style: OutlineCornerStyle::Circle,
            placement: OutlinePlacement::Outside,
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            preview: true,
        }
    }
}

impl OutlineFxModalState {
    pub fn open_with(&mut self, default_color: [u8; 4], has_selection: bool) {
        self.open = true;
        self.color = default_color;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.preview = true;
    }

    pub fn show(
        &mut self,
        ctx: &Context,
        i18n: &I18n,
        current_fg: [u8; 4],
        current_bg: [u8; 4],
        settings: Option<&mut AppSettings>,
    ) -> Option<OutlineFxAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.outline_fx.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.outline_fx.prompt"));
                ui.add_space(8.0);

                // Color swatch & picker
                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.outline_fx.color"));
                    crate::ui::color_picker::modern_color_picker_button(
                        ui,
                        "outline_fx_color",
                        &mut self.color,
                        i18n,
                        settings,
                    );
                    ui.monospace(crate::ui::color_picker::format_hex(self.color));
                    if ui.small_button(i18n.tr("modal.replace_color.use_fg")).clicked() {
                        self.color = current_fg;
                    }
                    if ui.small_button(i18n.tr("modal.replace_color.use_bg")).clicked() {
                        self.color = current_bg;
                    }
                });

                ui.add_space(6.0);

                egui::Grid::new("outline_fx_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.outline_fx.thickness"));
                        ui.add(egui::DragValue::new(&mut self.thickness).range(1..=8).suffix("px"));
                        ui.end_row();

                        ui.label(i18n.tr("modal.outline_fx.placement"));
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.placement,
                                OutlinePlacement::Outside,
                                i18n.tr("modal.outline_fx.outside"),
                            );
                            ui.selectable_value(
                                &mut self.placement,
                                OutlinePlacement::Inside,
                                i18n.tr("modal.outline_fx.inside"),
                            );
                        });
                        ui.end_row();

                        ui.label(i18n.tr("modal.outline_fx.corners"));
                        egui::ComboBox::from_id_salt("outline_fx_corners_combo")
                            .selected_text(match self.corner_style {
                                OutlineCornerStyle::Circle => i18n.tr("modal.modify_sel.shape_circle"),
                                OutlineCornerStyle::Square => i18n.tr("modal.modify_sel.shape_square"),
                                OutlineCornerStyle::Diamond => i18n.tr("modal.modify_sel.shape_diamond"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.corner_style,
                                    OutlineCornerStyle::Circle,
                                    i18n.tr("modal.modify_sel.shape_circle"),
                                );
                                ui.selectable_value(
                                    &mut self.corner_style,
                                    OutlineCornerStyle::Square,
                                    i18n.tr("modal.modify_sel.shape_square"),
                                );
                                ui.selectable_value(
                                    &mut self.corner_style,
                                    OutlineCornerStyle::Diamond,
                                    i18n.tr("modal.modify_sel.shape_diamond"),
                                );
                            });
                        ui.end_row();
                    });

                ui.add_space(6.0);
                ui.checkbox(&mut self.all_frames, i18n.tr("modal.replace_color.all_frames"));
                if self.has_selection {
                    ui.checkbox(&mut self.in_selection_only, i18n.tr("modal.replace_color.in_selection"));
                }
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 166.0;

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_OUTLINE} {}", i18n.tr("modal.outline_fx.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(OutlineFxAction {
                            color: self.color,
                            thickness: self.thickness.clamp(1, 8),
                            corner_style: self.corner_style,
                            placement: self.placement,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvolutionAction {
    pub kernel: [[f32; 3]; 3],
    pub divisor: f32,
    pub bias: f32,
    pub preserve_alpha: bool,
    pub all_frames: bool,
    pub in_selection_only: bool,
}

pub struct ConvolutionModalState {
    pub open: bool,
    pub preset: ConvolutionPreset,
    pub kernel: [[f32; 3]; 3],
    pub divisor: f32,
    pub bias: f32,
    pub preserve_alpha: bool,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub preview: bool,
}

impl Default for ConvolutionModalState {
    fn default() -> Self {
        let (k, div, bias) = ConvolutionPreset::Blur.kernel();
        Self {
            open: false,
            preset: ConvolutionPreset::Blur,
            kernel: k,
            divisor: div,
            bias,
            preserve_alpha: true,
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            preview: true,
        }
    }
}

impl ConvolutionModalState {
    pub fn open_with(&mut self, has_selection: bool) {
        self.open = true;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.preview = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<ConvolutionAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.convolution.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.convolution.prompt"));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.convolution.preset"));
                    let mut changed_preset = false;
                    egui::ComboBox::from_id_salt("convolution_preset_combo")
                        .selected_text(match self.preset {
                            ConvolutionPreset::Blur => i18n.tr("modal.convolution.blur"),
                            ConvolutionPreset::Sharpen => i18n.tr("modal.convolution.sharpen"),
                            ConvolutionPreset::EdgeDetect => i18n.tr("modal.convolution.edge_detect"),
                            ConvolutionPreset::Emboss => i18n.tr("modal.convolution.emboss"),
                            ConvolutionPreset::Custom => i18n.tr("modal.convolution.custom"),
                        })
                        .show_ui(ui, |ui| {
                            for p in ConvolutionPreset::ALL {
                                let label = match p {
                                    ConvolutionPreset::Blur => i18n.tr("modal.convolution.blur"),
                                    ConvolutionPreset::Sharpen => i18n.tr("modal.convolution.sharpen"),
                                    ConvolutionPreset::EdgeDetect => i18n.tr("modal.convolution.edge_detect"),
                                    ConvolutionPreset::Emboss => i18n.tr("modal.convolution.emboss"),
                                    ConvolutionPreset::Custom => i18n.tr("modal.convolution.custom"),
                                };
                                if ui.selectable_value(&mut self.preset, p, label).clicked() {
                                    changed_preset = true;
                                }
                            }
                        });
                    if changed_preset && self.preset != ConvolutionPreset::Custom {
                        let (k, div, bias) = self.preset.kernel();
                        self.kernel = k;
                        self.divisor = div;
                        self.bias = bias;
                    }
                });

                ui.add_space(8.0);

                // 3x3 Matrix grid
                ui.label(RichText::new(i18n.tr("modal.convolution.matrix")).strong());
                egui::Grid::new("convolution_3x3_grid")
                    .num_columns(3)
                    .spacing([8.0, 6.0])
                    .show(ui, |ui| {
                        for r in 0..3 {
                            for c in 0..3 {
                                if ui.add(egui::DragValue::new(&mut self.kernel[r][c]).speed(0.1)).changed() {
                                    self.preset = ConvolutionPreset::Custom;
                                }
                            }
                            ui.end_row();
                        }
                    });

                ui.add_space(6.0);
                egui::Grid::new("convolution_div_bias_grid")
                    .num_columns(2)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(i18n.tr("modal.convolution.divisor"));
                        if ui.add(egui::DragValue::new(&mut self.divisor).speed(0.1).range(0.01..=1000.0)).changed() {
                            self.preset = ConvolutionPreset::Custom;
                        }
                        ui.end_row();

                        ui.label(i18n.tr("modal.convolution.bias"));
                        if ui.add(egui::DragValue::new(&mut self.bias).speed(1.0).range(-255.0..=255.0)).changed() {
                            self.preset = ConvolutionPreset::Custom;
                        }
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.checkbox(&mut self.preserve_alpha, i18n.tr("modal.convolution.preserve_alpha"));
                ui.checkbox(&mut self.all_frames, i18n.tr("modal.replace_color.all_frames"));
                if self.has_selection {
                    ui.checkbox(&mut self.in_selection_only, i18n.tr("modal.replace_color.in_selection"));
                }
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 166.0;

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CHECK} {}", i18n.tr("modal.convolution.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(ConvolutionAction {
                            kernel: self.kernel,
                            divisor: self.divisor,
                            bias: self.bias,
                            preserve_alpha: self.preserve_alpha,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RemapPaletteAction {
    pub dither: PaletteDitherMode,
    pub dither_strength: f32,
    pub all_frames: bool,
    pub in_selection_only: bool,
}

pub struct RemapPaletteModalState {
    pub open: bool,
    pub dither: PaletteDitherMode,
    pub dither_strength: f32,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub palette_count: usize,
    pub preview: bool,
}

impl Default for RemapPaletteModalState {
    fn default() -> Self {
        Self {
            open: false,
            dither: PaletteDitherMode::None,
            dither_strength: 16.0,
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            palette_count: 0,
            preview: true,
        }
    }
}

impl RemapPaletteModalState {
    pub fn open_with(&mut self, has_selection: bool, palette_count: usize) {
        self.open = true;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.palette_count = palette_count;
        self.preview = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<RemapPaletteAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.remap_palette.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.remap_palette.prompt"));
                ui.add_space(4.0);

                ui.small(format!(
                    "{} {}",
                    i18n.tr("modal.remap_palette.palette_colors"),
                    self.palette_count
                ));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.remap_palette.dither"));
                    egui::ComboBox::from_id_salt("remap_palette_dither_combo")
                        .selected_text(match self.dither {
                            PaletteDitherMode::None => i18n.tr("context.gradient_dither_none"),
                            PaletteDitherMode::Bayer4x4 => i18n.tr("context.gradient_dither_bayer4"),
                            PaletteDitherMode::Bayer8x8 => i18n.tr("context.gradient_dither_bayer8"),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.dither,
                                PaletteDitherMode::None,
                                i18n.tr("context.gradient_dither_none"),
                            );
                            ui.selectable_value(
                                &mut self.dither,
                                PaletteDitherMode::Bayer4x4,
                                i18n.tr("context.gradient_dither_bayer4"),
                            );
                            ui.selectable_value(
                                &mut self.dither,
                                PaletteDitherMode::Bayer8x8,
                                i18n.tr("context.gradient_dither_bayer8"),
                            );
                        });
                });

                if self.dither != PaletteDitherMode::None {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(i18n.tr("modal.remap_palette.dither_strength"));
                        ui.add(egui::Slider::new(&mut self.dither_strength, 1.0..=64.0).suffix(" px"));
                    });
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Scope checkboxes
                ui.checkbox(&mut self.all_frames, i18n.tr("modal.remap_palette.all_frames"));

                if self.has_selection {
                    ui.checkbox(
                        &mut self.in_selection_only,
                        i18n.tr("modal.remap_palette.selection_only"),
                    );
                }
                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let btn_w = 166.0;
                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CHECK} {}", i18n.tr("modal.remap_palette.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(RemapPaletteAction {
                            dither: self.dither,
                            dither_strength: self.dither_strength,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotateAction {
    pub angle_deg: f32,
    pub algorithm: RotationAlgorithm,
    pub all_frames: bool,
    pub in_selection_only: bool,
}

pub struct RotateModalState {
    pub open: bool,
    pub angle_deg: f32,
    pub algorithm: RotationAlgorithm,
    pub all_frames: bool,
    pub in_selection_only: bool,
    pub has_selection: bool,
    pub preview: bool,
}

impl Default for RotateModalState {
    fn default() -> Self {
        Self {
            open: false,
            angle_deg: 45.0,
            algorithm: RotationAlgorithm::RotSprite,
            all_frames: false,
            in_selection_only: false,
            has_selection: false,
            preview: true,
        }
    }
}

impl RotateModalState {
    pub fn open_with(&mut self, has_selection: bool) {
        self.open = true;
        self.has_selection = has_selection;
        self.in_selection_only = has_selection;
        self.preview = true;
    }

    pub fn show(&mut self, ctx: &Context, i18n: &I18n) -> Option<RotateAction> {
        if !self.open {
            return None;
        }
        let mut triggered = None;
        let mut window_open = self.open;
        let mut close_requested = false;

        Window::new(i18n.tr("modal.rotate.title"))
            .open(&mut window_open)
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(340.0);
                ui.label(i18n.tr("modal.rotate.prompt"));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.rotate.angle"));
                    ui.add(egui::DragValue::new(&mut self.angle_deg).range(-180.0..=180.0).suffix("°"));
                    ui.add(egui::Slider::new(&mut self.angle_deg, -180.0..=180.0).suffix("°"));
                });

                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.small(i18n.tr("modal.rotate.quick"));
                    for &preset in &[-90.0, -45.0, -15.0, 15.0, 45.0, 90.0, 180.0] {
                        let text = format!("{:+0.0}°", preset);
                        if ui.small_button(text).clicked() {
                            self.angle_deg = preset;
                        }
                    }
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(i18n.tr("modal.rotate.algorithm"));
                    egui::ComboBox::from_id_salt("rotate_algorithm_combo")
                        .selected_text(match self.algorithm {
                            RotationAlgorithm::RotSprite => i18n.tr("modal.rotate.rotsprite"),
                            RotationAlgorithm::NearestNeighbor => i18n.tr("modal.rotate.nearest"),
                        })
                        .show_ui(ui, |ui| {
                            for alg in RotationAlgorithm::ALL {
                                let label = match alg {
                                    RotationAlgorithm::RotSprite => i18n.tr("modal.rotate.rotsprite"),
                                    RotationAlgorithm::NearestNeighbor => i18n.tr("modal.rotate.nearest"),
                                };
                                ui.selectable_value(&mut self.algorithm, alg, label);
                            }
                        });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Scope checkboxes
                ui.checkbox(&mut self.all_frames, i18n.tr("modal.rotate.all_frames"));

                if self.has_selection {
                    ui.checkbox(
                        &mut self.in_selection_only,
                        i18n.tr("modal.rotate.selection_only"),
                    );
                }

                ui.checkbox(&mut self.preview, i18n.tr("modal.preview"));

                ui.add_space(12.0);
                let btn_w = (ui.available_width() - 8.0) * 0.5;
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CHECK} {}", i18n.tr("modal.rotate.apply"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        triggered = Some(RotateAction {
                            angle_deg: self.angle_deg,
                            algorithm: self.algorithm,
                            all_frames: self.all_frames,
                            in_selection_only: self.in_selection_only && self.has_selection,
                        });
                        close_requested = true;
                    }

                    if ui
                        .add_sized(
                            [btn_w, 28.0],
                            egui::Button::new(
                                RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.modify_sel.cancel"))).strong(),
                            ),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });

        self.open = window_open && !close_requested;
        triggered
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    Restore,
    Discard,
}

pub fn show_recovery_modal(
    ctx: &Context,
    i18n: &I18n,
    has_candidate: bool,
) -> Option<RecoveryAction> {
    if !has_candidate {
        return None;
    }
    let mut action = None;
    Window::new(i18n.tr("modal.recovery.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(i18n.tr("modal.recovery.prompt"));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_RESET} {}", i18n.tr("modal.recovery.restore"))),
                    )
                    .clicked()
                {
                    action = Some(RecoveryAction::Restore);
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!("{FA_TRASH} {}", i18n.tr("modal.recovery.discard")))
                                .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    action = Some(RecoveryAction::Discard);
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                action = Some(RecoveryAction::Discard);
            }
        });
    action
}

pub fn show_confirm_delete_frame_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_frame: &mut Option<usize>,
) -> Option<usize> {
    let target_idx = (*pending_frame)?;
    let mut confirmed = None;
    let mut close = false;

    Window::new(i18n.tr("timeline.confirm_delete.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} (#{})",
                    i18n.tr("timeline.confirm_delete.prompt"),
                    target_idx + 1
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!("{FA_TRASH} {}", i18n.tr("timeline.confirm_delete.yes")))
                                .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(target_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_frame = None;
    }
    confirmed
}

pub fn show_confirm_delete_layer_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_layer: &mut Option<(usize, String)>,
) -> Option<usize> {
    let (target_idx, layer_name) = pending_layer.as_ref()?.clone();
    let mut confirmed = None;
    let mut close = false;

    Window::new(i18n.tr("timeline.confirm_delete_layer.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} «{}»?",
                    i18n.tr("timeline.confirm_delete_layer.prompt"),
                    layer_name
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("timeline.confirm_delete_layer.yes")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(target_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!(
                            "{FA_CLOSE} {}",
                            i18n.tr("timeline.confirm_delete_layer.no")
                        )),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_layer = None;
    }
    confirmed
}

pub fn show_confirm_delete_skin_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_skin: &mut Option<(usize, String)>,
) -> Option<usize> {
    let (target_idx, skin_name) = pending_skin.as_ref()?.clone();
    let mut confirmed = None;
    let mut close = false;

    Window::new(i18n.tr("panel.palette.confirm_del_skin.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} \"{}\"?",
                    i18n.tr("panel.palette.confirm_del_skin.prompt"),
                    skin_name
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("panel.palette.confirm_del_skin.yes")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(target_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_skin = None;
    }
    confirmed
}

pub fn show_confirm_delete_recent_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_path: &mut Option<PathBuf>,
) -> Option<PathBuf> {
    let path = pending_path.as_ref()?.clone();
    let mut confirmed = None;
    let mut close = false;

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");

    Window::new(i18n.tr("start.confirm_delete.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} «{}»?",
                    i18n.tr("start.confirm_delete.prompt"),
                    filename
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!("{FA_TRASH} {}", i18n.tr("start.confirm_delete.yes")))
                                .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(path.clone());
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_path = None;
    }
    confirmed
}

pub fn show_confirm_delete_palette_slot_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_slot: &mut Option<(usize, [u8; 4])>,
) -> Option<usize> {
    let (slot_idx, color) = (*pending_slot)?;
    let mut confirmed = None;
    let mut close = false;

    let hex = crate::ui::color_picker::format_hex(color);

    Window::new(i18n.tr("panel.palette.confirm_del_slot.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} #{} ({})?",
                    i18n.tr("panel.palette.confirm_del_slot.prompt"),
                    slot_idx + 1,
                    hex
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("panel.palette.confirm_del_slot.yes")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(slot_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_slot = None;
    }
    confirmed
}

pub fn show_confirm_delete_pivot_point_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_pt: &mut Option<(usize, String)>,
) -> Option<usize> {
    let (target_idx, pt_name) = pending_pt.as_ref()?.clone();
    let mut confirmed = None;
    let mut close = false;

    Window::new(i18n.tr("panel.pivot.confirm_del.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} «{}»?",
                    i18n.tr("panel.pivot.confirm_del.prompt"),
                    pt_name
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("panel.pivot.confirm_del.yes")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(target_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_pt = None;
    }
    confirmed
}

pub fn show_confirm_delete_tag_modal(
    ctx: &Context,
    i18n: &I18n,
    pending_tag: &mut Option<(usize, String)>,
) -> Option<usize> {
    let (target_idx, tag_name) = pending_tag.as_ref()?.clone();
    let mut confirmed = None;
    let mut close = false;

    Window::new(i18n.tr("timeline.confirm_del_tag.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(340.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} «{}»?",
                    i18n.tr("timeline.confirm_del_tag.prompt"),
                    tag_name
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(130.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("timeline.confirm_del_tag.yes")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = Some(target_idx);
                    close = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("timeline.confirm_delete.no"))),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });

    if close {
        *pending_tag = None;
    }
    confirmed
}

pub fn show_about_modal(ctx: &Context, i18n: &I18n, open: &mut bool) {
    if !*open {
        return;
    }

    let mut window_open = *open;
    let mut close_requested = false;
    Window::new(i18n.tr("modal.about.title"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(320.0);
            ui.label(i18n.tr("modal.about.description"));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(i18n.tr("modal.about.version"));
                ui.strong(env!("CARGO_PKG_VERSION"));
            });
            ui.horizontal(|ui| {
                ui.label(i18n.tr("modal.about.repo"));
                ui.hyperlink_to(GITHUB_REPO_URL, GITHUB_REPO_URL);
            });
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);
            ui.label(RichText::new(i18n.tr("modal.about.credits")).strong());
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n.tr("modal.about.credits_cat"));
                ui.hyperlink_to("xzany (itch.io)", "https://xzany.itch.io/cat-2d-pixel-art");
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("DB8, DB16, DB32 palettes: ");
                ui.hyperlink_to("DawnBringer / Richard Fhager", "https://github.com/geoffb/dawnbringer-palettes");
            });
            ui.add_space(8.0);
            ui.label(
                RichText::new(i18n.tr("modal.about.disclaimer"))
                    .size(11.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui.button(format!("{FA_CLOSE} {}", i18n.tr("modal.about.close"))).clicked() {
                    close_requested = true;
                }
            });
        });
    *open = window_open && !close_requested;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmCloseDocAction {
    SaveAndClose,
    DiscardAndClose,
    Cancel,
}

pub fn show_confirm_close_doc_modal(
    ctx: &Context,
    i18n: &I18n,
    doc_name: &str,
    pending_close: &mut Option<usize>,
) -> Option<ConfirmCloseDocAction> {
    let _ = (*pending_close)?;
    let mut chosen_action = None;
    let mut close_modal = false;

    Window::new(i18n.tr("modal.close_doc.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(360.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(format!(
                    "{} «{}»?",
                    i18n.tr("modal.close_doc.prompt"),
                    doc_name
                ));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(104.0, 26.0);
            let spacing = 8.0;
            let total_btn_width = btn_size.x * 3.0 + spacing * 2.0;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_SAVE} {}", i18n.tr("modal.close_doc.save"))),
                    )
                    .clicked()
                {
                    chosen_action = Some(ConfirmCloseDocAction::SaveAndClose);
                    close_modal = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!(
                                "{FA_TRASH} {}",
                                i18n.tr("modal.close_doc.discard")
                            ))
                            .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    chosen_action = Some(ConfirmCloseDocAction::DiscardAndClose);
                    close_modal = true;
                }
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("modal.close_doc.cancel"))),
                    )
                    .clicked()
                {
                    chosen_action = Some(ConfirmCloseDocAction::Cancel);
                    close_modal = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                chosen_action = Some(ConfirmCloseDocAction::Cancel);
                close_modal = true;
            }
        });

    if close_modal {
        *pending_close = None;
    }
    chosen_action
}

pub fn show_confirm_exit_app_modal(
    ctx: &Context,
    i18n: &I18n,
    open: &mut bool,
) -> bool {
    if !*open {
        return false;
    }
    let mut confirmed = false;
    let mut close_modal = false;

    Window::new(i18n.tr("modal.confirm_exit.title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(320.0);
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(i18n.tr("modal.confirm_exit.prompt"));
                ui.add_space(14.0);
            });

            let btn_size = egui::vec2(100.0, 26.0);
            let spacing = 12.0;
            let total_btn_width = btn_size.x * 2.0 + spacing;
            let offset = ((ui.available_width() - total_btn_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;
                ui.add_space(offset);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            egui::RichText::new(format!("{FA_CLOSE} {}", i18n.tr("modal.confirm_exit.yes")))
                                .color(Color32::from_rgb(240, 110, 110)),
                        ),
                    )
                    .clicked()
                {
                    confirmed = true;
                    close_modal = true;
                }
                if ui
                    .add_sized(btn_size, egui::Button::new(i18n.tr("modal.confirm_exit.no")))
                    .clicked()
                {
                    close_modal = true;
                }
            });
            ui.add_space(4.0);

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_modal = true;
            }
        });

    if close_modal {
        *open = false;
    }
    confirmed
}

fn render_keycap_pill(ui: &mut Ui, token: &str) {
    egui::Frame::NONE
        .fill(ui.visuals().widgets.inactive.bg_fill)
        .stroke(ui.visuals().widgets.inactive.bg_stroke)
        .corner_radius(4.0)
        .inner_margin(egui::Margin::symmetric(6, 2))
        .show(ui, |ui| {
            ui.label(
                RichText::new(token)
                    .size(11.0)
                    .strong()
                    .monospace()
                    .color(ui.visuals().text_color()),
            );
        });
}

fn render_key_combo(ui: &mut Ui, combo: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        let plus_parts: Vec<&str> = combo.split('+').collect();
        for (pi, ppart) in plus_parts.iter().enumerate() {
            let slash_parts: Vec<&str> = ppart.split('/').collect();
            for (si, spart) in slash_parts.iter().enumerate() {
                render_keycap_pill(ui, spart.trim());
                if si + 1 < slash_parts.len() {
                    ui.label(
                        RichText::new("/")
                            .size(10.5)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
            }
            if pi + 1 < plus_parts.len() {
                ui.label(
                    RichText::new("+")
                        .size(10.5)
                        .color(ui.visuals().weak_text_color()),
                );
            }
        }
    });
}

fn render_hotkey_section(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    accent_color: Color32,
    grid_id: &str,
    items: &[(&str, &str)],
) {
    egui::Frame::NONE
        .fill(ui.visuals().faint_bg_color)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .corner_radius(8.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.label(RichText::new(icon).size(13.5).color(accent_color));
                ui.label(
                    RichText::new(title)
                        .strong()
                        .size(12.5)
                        .color(ui.visuals().text_color()),
                );
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new(grid_id)
                .num_columns(2)
                .spacing([12.0, 5.0])
                .show(ui, |ui| {
                    for &(key_combo, label) in items {
                        render_key_combo(ui, key_combo);
                        ui.label(
                            RichText::new(label)
                                .size(12.0)
                                .color(ui.visuals().text_color()),
                        );
                        ui.end_row();
                    }
                });
        });
}

pub fn show_hotkeys_modal(ctx: &Context, open: &mut bool, i18n: &I18n) {
    if !*open {
        return;
    }
    let mut window_open = *open;
    let mut close_clicked = false;

    let tools_items = [
        ("B", i18n.tr("tools.pencil.title")),
        ("E", i18n.tr("tools.eraser.title")),
        ("I", i18n.tr("tools.eyedropper.title")),
        ("G", i18n.tr("modal.hotkeys.fill_or_gradient")),
        ("L", i18n.tr("tools.line.title")),
        ("C", i18n.tr("tools.curve.title")),
        ("U", i18n.tr("tools.rect.title")),
        ("O", i18n.tr("tools.ellipse.title")),
        ("D", i18n.tr("tools.dither.title")),
        ("S", i18n.tr("tools.shading.title")),
        ("M", i18n.tr("tools.select_rect.title")),
        ("Q", i18n.tr("tools.lasso.title")),
        ("W", i18n.tr("tools.magic_wand.title")),
        ("P", i18n.tr("tools.pivot.title")),
        ("LMB ×3 / Esc", i18n.tr("modal.hotkeys.curve_3click")),
        ("X", i18n.tr("modal.hotkeys.swap_colors")),
        ("[ / ]", i18n.tr("modal.hotkeys.brush_size")),
    ];

    let canvas_items = [
        ("Space + Drag", i18n.tr("modal.hotkeys.pan_canvas")),
        ("Wheel", i18n.tr("modal.hotkeys.zoom_canvas")),
        ("Ctrl/Cmd + +/-/0", i18n.tr("modal.hotkeys.zoom_keys")),
        ("F", i18n.tr("modal.hotkeys.fit_canvas")),
        ("1 / 2", i18n.tr("modal.hotkeys.switch_view_mode")),
        ("Esc / Ctrl/Cmd+D", i18n.tr("modal.hotkeys.deselect")),
        ("Ctrl/Cmd+A / Shift+I", i18n.tr("modal.hotkeys.select_all")),
        ("Ctrl/Cmd+C / X / V", i18n.tr("modal.hotkeys.copy_paste")),
        ("Delete", i18n.tr("modal.hotkeys.delete_selection")),
        ("Shift + R", i18n.tr("modal.hotkeys.replace_color")),
        ("Shift + O", i18n.tr("modal.hotkeys.outline_fx")),
        ("Ctrl/Cmd + Shift + R", i18n.tr("modal.hotkeys.rotate_arbitrary")),
        ("Ctrl/Cmd + U", i18n.tr("modal.hotkeys.color_adjust")),
        ("Ctrl/Cmd + B", i18n.tr("modal.hotkeys.custom_brush")),
    ];

    let timeline_items = [
        ("Enter / Ctrl/Cmd+Space", i18n.tr("modal.hotkeys.play_pause")),
        ("Ctrl/Cmd + Shift + D", i18n.tr("timeline.dup_frame")),
        ("Wheel / Shift+Wheel", i18n.tr("modal.hotkeys.timeline_scroll")),
    ];

    let file_items = [
        ("Ctrl/Cmd + Z", i18n.tr("menu.edit.undo")),
        ("Ctrl/Cmd + Y", i18n.tr("menu.edit.redo")),
        ("Ctrl/Cmd + S", i18n.tr("menu.file.save")),
        ("Ctrl/Cmd + N", i18n.tr("menu.file.new")),
        ("Ctrl/Cmd + W", i18n.tr("menu.file.close")),
    ];

    egui::Window::new(i18n.tr("modal.hotkeys.title"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(640.0);
            ui.set_max_width(680.0);

            ui.horizontal_top(|ui| {
                let card_w = (ui.available_width() - 12.0) / 2.0;

                // Left Column: Drawing Tools (13 items)
                ui.allocate_ui_with_layout(
                    egui::vec2(card_w, 0.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        render_hotkey_section(
                            ui,
                            FA_PENCIL,
                            i18n.tr("modal.hotkeys.sec_tools"),
                            Color32::from_rgb(240, 185, 75),
                            "hotkeys_tools_grid",
                            &tools_items,
                        );
                    },
                );

                ui.add_space(12.0);

                // Right Column: Canvas, Timeline, File (5 + 3 + 5 = 13 items)
                ui.allocate_ui_with_layout(
                    egui::vec2(card_w, 0.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        render_hotkey_section(
                            ui,
                            FA_VIEW_FRAME,
                            i18n.tr("modal.hotkeys.sec_canvas"),
                            Color32::from_rgb(105, 190, 245),
                            "hotkeys_canvas_grid",
                            &canvas_items,
                        );

                        ui.add_space(8.0);

                        render_hotkey_section(
                            ui,
                            FA_PLAY,
                            i18n.tr("modal.hotkeys.sec_timeline"),
                            Color32::from_rgb(130, 215, 125),
                            "hotkeys_timeline_grid",
                            &timeline_items,
                        );

                        ui.add_space(8.0);

                        render_hotkey_section(
                            ui,
                            FA_FOLDER,
                            i18n.tr("modal.hotkeys.sec_file"),
                            Color32::from_rgb(205, 165, 250),
                            "hotkeys_file_grid",
                            &file_items,
                        );
                    },
                );
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(6.0);
            ui.vertical_centered(|ui| {
                if ui
                    .add_sized(
                        [140.0, 30.0],
                        egui::Button::new(
                            RichText::new(format!("{FA_CHECK} {}", i18n.tr("modal.hotkeys.close")))
                                .strong(),
                        ),
                    )
                    .clicked()
                {
                    close_clicked = true;
                }
            });
        });

    if close_clicked {
        window_open = false;
    }
    *open = window_open;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_slicer_auto_detect_cat_walk() {
        let bytes = include_bytes!("../../assets/samples/cat_walk.png");
        let img = image::load_from_memory(bytes)
            .expect("load cat_walk.png")
            .to_rgba8();

        let mut slicer = SheetSlicerModalState::default();
        slicer.open_with_sheet("cat_walk.png".to_string(), img);

        assert_eq!(slicer.frame_w, 80);
        assert_eq!(slicer.frame_h, 64);
        assert_eq!(slicer.margin_x, 0);
        assert_eq!(slicer.margin_y, 0);
        assert_eq!(slicer.spacing_x, 0);
        assert_eq!(slicer.spacing_y, 0);
        assert_eq!(slicer.target_columns, 12);

        let (cols, rows) = slicer.count_detected_frames();
        assert_eq!(cols, 12);
        assert_eq!(rows, 1);
        assert_eq!(slicer.selected_frames.len(), 12);
        assert!(slicer.selected_frames.iter().all(|&selected| selected));
    }

    #[test]
    fn test_hotkeys_modal_render() {
        let ctx = egui::Context::default();
        for lang in [crate::i18n::Language::Russian, crate::i18n::Language::English] {
            let i18n = I18n::new(lang);
            let mut open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                show_hotkeys_modal(ctx, &mut open, &i18n);
            });
            assert!(open);
        }
    }

    #[test]
    fn test_confirm_exit_modal_render() {
        let ctx = egui::Context::default();
        for lang in [crate::i18n::Language::Russian, crate::i18n::Language::English] {
            let i18n = I18n::new(lang);
            let mut open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let confirmed = show_confirm_exit_app_modal(ctx, &i18n, &mut open);
                assert!(!confirmed);
            });
            assert!(open);
        }
    }

    #[test]
    fn test_all_confirmation_modals_render() {
        let ctx = egui::Context::default();
        for lang in [crate::i18n::Language::Russian, crate::i18n::Language::English] {
            let i18n = I18n::new(lang);

            // 1. Delete skin modal
            let mut pending_skin = Some((1, "Skin 2".to_string()));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_skin_modal(ctx, &i18n, &mut pending_skin);
                assert_eq!(res, None);
            });
            assert!(pending_skin.is_some());

            // 2. Delete frame modal
            let mut pending_frame = Some(2);
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_frame_modal(ctx, &i18n, &mut pending_frame);
                assert_eq!(res, None);
            });
            assert!(pending_frame.is_some());

            // 2b. Delete layer modal
            let mut pending_layer = Some((1, "Layer 2".to_string()));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_layer_modal(ctx, &i18n, &mut pending_layer);
                assert_eq!(res, None);
            });
            assert!(pending_layer.is_some());

            // 3. Close doc modal
            let mut pending_close = Some(0);
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_close_doc_modal(ctx, &i18n, "cat_walk", &mut pending_close);
                assert_eq!(res, None);
            });
            assert!(pending_close.is_some());

            // 4. Recovery modal
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_recovery_modal(ctx, &i18n, true);
                assert_eq!(res, None);
            });

            // 5. Unsaved modal
            let mut unsaved = UnsavedChangesModalState {
                open: true,
                ..Default::default()
            };
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let action = unsaved.show(ctx, &i18n);
                assert_eq!(action, UnsavedAction::None);
            });
            assert!(unsaved.open);

            // 6. Delete recent project modal
            let mut pending_recent = Some(PathBuf::from("hero.png"));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_recent_modal(ctx, &i18n, &mut pending_recent);
                assert_eq!(res, None);
            });
            assert!(pending_recent.is_some());

            // 7. Delete palette slot modal
            let mut pending_slot = Some((0, [255, 0, 0, 255]));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_palette_slot_modal(ctx, &i18n, &mut pending_slot);
                assert_eq!(res, None);
            });
            assert!(pending_slot.is_some());

            // 8. Delete pivot point modal
            let mut pending_pt = Some((0, "Head".to_string()));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_pivot_point_modal(ctx, &i18n, &mut pending_pt);
                assert_eq!(res, None);
            });
            assert!(pending_pt.is_some());

            // 9. Delete tag modal
            let mut pending_tag = Some((0, "Run".to_string()));
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = show_confirm_delete_tag_modal(ctx, &i18n, &mut pending_tag);
                assert_eq!(res, None);
            });
            assert!(pending_tag.is_some());

            // 10. Replace color modal
            let mut replace_color = ReplaceColorModalState::default();
            replace_color.open_with([255, 0, 0, 255], [0, 255, 0, 255], true);
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = replace_color.show(
                    ctx,
                    &i18n,
                    [255, 0, 0, 255],
                    [0, 255, 0, 255],
                    None,
                    None,
                );
                assert_eq!(res, None);
            });
            assert!(replace_color.open);

            // 11. Modify selection modal
            let mut modify_sel = ModifySelectionModalState::default();
            modify_sel.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = modify_sel.show(ctx, &i18n);
                assert_eq!(res, None);
            });
            assert!(modify_sel.open);

            // 12. Outline FX modal
            let mut outline_fx = OutlineFxModalState::default();
            outline_fx.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = outline_fx.show(
                    ctx,
                    &i18n,
                    [255, 255, 255, 255],
                    [0, 0, 0, 255],
                    None,
                );
                assert_eq!(res, None);
            });
            assert!(outline_fx.open);

            // 13. Convolution matrix modal
            let mut convolution = ConvolutionModalState::default();
            convolution.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = convolution.show(ctx, &i18n);
                assert_eq!(res, None);
            });
            assert!(convolution.open);

            // 14. Remap palette modal
            let mut remap_palette = RemapPaletteModalState::default();
            remap_palette.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = remap_palette.show(ctx, &i18n);
                assert_eq!(res, None);
            });
            assert!(remap_palette.open);

            // 15. Free rotate modal
            let mut rotate_modal = RotateModalState::default();
            rotate_modal.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = rotate_modal.show(ctx, &i18n);
                assert_eq!(res, None);
            });
            assert!(rotate_modal.open);

            // 16. Export packed atlas modal
            let mut pack_modal = ExportPackedAtlasModalState::default();
            pack_modal.open = true;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let res = pack_modal.show(ctx, &i18n, None);
                assert_eq!(res, None);
            });
            assert!(pack_modal.open);
        }
    }
}

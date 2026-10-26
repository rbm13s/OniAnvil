use crate::canvas::viewport::{BackgroundMode, CanvasViewMode, ViewportState};
use crate::history::command::HistoryStack;
use crate::i18n::{I18n, Language};
use crate::io::updater::UpdateInfo;
use crate::model::palette::PaletteSortCriterion;
use crate::model::project::Project;
use crate::model::sketch::QuickPeekMode;
use crate::theme::*;
use crate::tools::{DitherPattern, GradientDither, GradientKind, SymmetryMode, ToolKind, ToolSettings};
use egui::{Color32, Context, TopBottomPanel, Ui};

#[derive(Default)]
pub struct TopBarActions {
    pub open_new_project_wizard: bool,
    pub trigger_open_project: bool,
    pub trigger_close_project: bool,
    pub trigger_open_slicer: bool,
    pub trigger_import_frame_sequence: bool,
    pub trigger_save_project: bool,
    pub trigger_save_project_as: bool,
    pub open_export_png_modal: bool,
    pub trigger_export_frame_sequence: bool,
    pub open_export_gif_modal: bool,
    pub trigger_export_sprite_json: bool,
    pub open_export_packed_atlas_modal: bool,
    pub trigger_undo: bool,
    pub trigger_redo: bool,
    pub trigger_copy: bool,
    pub trigger_cut: bool,
    pub trigger_paste: bool,
    pub trigger_select_all: bool,
    pub trigger_invert_selection: bool,
    pub trigger_deselect: bool,
    pub trigger_expand_selection: bool,
    pub trigger_contract_selection: bool,
    pub trigger_border_selection: bool,
    pub open_modify_selection_modal: bool,
    pub trigger_flip_h: bool,
    pub trigger_flip_v: bool,
    pub trigger_rotate_90: bool,
    pub open_rotate_modal: bool,
    pub trigger_auto_outline: bool,
    pub open_outline_fx_modal: bool,
    pub trigger_replace_color: bool,
    pub trigger_color_adjust: bool,
    pub open_convolution_modal: bool,
    pub open_remap_palette_modal: bool,
    pub trigger_sort_palette: Option<PaletteSortCriterion>,
    pub trigger_custom_brush: bool,
    pub trigger_auto_crop: bool,
    pub open_regrid_modal: bool,
    pub open_resize9_modal: bool,
    pub open_about_modal: bool,
    pub open_hotkeys_modal: bool,
    pub open_demo_project: bool,
    pub language_changed: Option<Language>,
    pub theme_changed: Option<AppTheme>,
}

#[allow(clippy::too_many_arguments)]
pub fn show_top_bars(
    ctx: &Context,
    i18n: &mut I18n,
    mut active_doc: Option<(&mut Project, &mut ViewportState, &HistoryStack)>,
    tools: &mut ToolSettings,
    show_right_dock: &mut bool,
    show_bottom_dock: &mut bool,
    update_info: Option<&UpdateInfo>,
    current_theme: &mut AppTheme,
) -> TopBarActions {
    let mut actions = TopBarActions::default();
    let has_doc = active_doc.is_some();

    TopBottomPanel::top("onianvil_main_menu_bar").show(ctx, |ui: &mut Ui| {
        egui::menu::bar(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            // File Menu
            ui.menu_button(i18n.tr("menu.file"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if ui.button(format!("{FA_PLUS} {}", i18n.tr("menu.file.new"))).clicked() {
                    actions.open_new_project_wizard = true;
                    ui.close_menu();
                }
                if ui.button(format!("{FA_FOLDER} {}", i18n.tr("menu.file.open"))).clicked() {
                    actions.trigger_open_project = true;
                    ui.close_menu();
                }
                if ui.button(format!("{FA_PLAY} {}", i18n.tr("menu.file.open_demo"))).clicked() {
                    actions.open_demo_project = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_CLOSE} {}", i18n.tr("menu.file.close"))),
                    )
                    .clicked()
                {
                    actions.trigger_close_project = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_SAVE} {}", i18n.tr("menu.file.save"))),
                    )
                    .clicked()
                {
                    actions.trigger_save_project = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_SAVE} {}", i18n.tr("menu.file.save_as"))),
                    )
                    .clicked()
                {
                    actions.trigger_save_project_as = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button(format!("{FA_VIEW_ATLAS} {}", i18n.tr("menu.file.slicer"))).clicked() {
                    actions.trigger_open_slicer = true;
                    ui.close_menu();
                }
                if ui.button(format!("{FA_IMPORT} {}", i18n.tr("menu.file.import_seq"))).clicked() {
                    actions.trigger_import_frame_sequence = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_EXPORT} {}", i18n.tr("menu.file.export_scaled"))),
                    )
                    .clicked()
                {
                    actions.open_export_png_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_EXPORT} {}", i18n.tr("menu.file.export_seq"))),
                    )
                    .clicked()
                {
                    actions.trigger_export_frame_sequence = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_PLAY} {}", i18n.tr("menu.file.export_gif"))),
                    )
                    .clicked()
                {
                    actions.open_export_gif_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_EXPORT} {}", i18n.tr("menu.file.export_sprite_json"))),
                    )
                    .clicked()
                {
                    actions.trigger_export_sprite_json = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_VIEW_ATLAS} {}", i18n.tr("menu.file.export_packed_atlas"))),
                    )
                    .clicked()
                {
                    actions.open_export_packed_atlas_modal = true;
                    ui.close_menu();
                }
            });

            // Edit Menu
            ui.menu_button(i18n.tr("menu.edit"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                let can_undo = active_doc.as_ref().map(|(_, _, h)| h.can_undo()).unwrap_or(false);
                let can_redo = active_doc.as_ref().map(|(_, _, h)| h.can_redo()).unwrap_or(false);
                if ui
                    .add_enabled(
                        can_undo,
                        egui::Button::new(format!("{FA_UNDO} {}", i18n.tr("menu.edit.undo"))),
                    )
                    .clicked()
                {
                    actions.trigger_undo = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        can_redo,
                        egui::Button::new(format!("{FA_REDO} {}", i18n.tr("menu.edit.redo"))),
                    )
                    .clicked()
                {
                    actions.trigger_redo = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_COPY} {}", i18n.tr("menu.edit.copy"))),
                    )
                    .clicked()
                {
                    actions.trigger_copy = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_SCISSORS} {}", i18n.tr("menu.edit.cut"))),
                    )
                    .clicked()
                {
                    actions.trigger_cut = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_CLIPBOARD} {}", i18n.tr("menu.edit.paste"))),
                    )
                    .clicked()
                {
                    actions.trigger_paste = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(i18n.tr("menu.edit.select_all")),
                    )
                    .clicked()
                {
                    actions.trigger_select_all = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(i18n.tr("menu.edit.invert_selection")),
                    )
                    .clicked()
                {
                    actions.trigger_invert_selection = true;
                    ui.close_menu();
                }
                ui.separator();
                ui.checkbox(&mut tools.batch_mode, i18n.tr("menu.edit.batch_mode"));
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_FLIP_H} {}", i18n.tr("menu.edit.flip_h"))),
                    )
                    .clicked()
                {
                    actions.trigger_flip_h = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_FLIP_V} {}", i18n.tr("menu.edit.flip_v"))),
                    )
                    .clicked()
                {
                    actions.trigger_flip_v = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_ROTATE_90} {}", i18n.tr("menu.edit.rotate_90"))),
                    )
                    .clicked()
                {
                    actions.trigger_rotate_90 = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_ROTATE_90} {}", i18n.tr("menu.edit.rotate_arbitrary"))),
                    )
                    .clicked()
                {
                    actions.open_rotate_modal = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_OUTLINE} {}", i18n.tr("menu.edit.outline_fx"))),
                    )
                    .clicked()
                {
                    actions.open_outline_fx_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(i18n.tr("menu.edit.auto_outline")),
                    )
                    .clicked()
                {
                    actions.trigger_auto_outline = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_FLOOD_FILL} {}", i18n.tr("menu.edit.replace_color"))),
                    )
                    .clicked()
                {
                    actions.trigger_replace_color = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_SLIDERS} {}", i18n.tr("menu.edit.color_adjust"))),
                    )
                    .clicked()
                {
                    actions.trigger_color_adjust = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_GRID_PIXEL} {}", i18n.tr("menu.edit.convolution"))),
                    )
                    .clicked()
                {
                    actions.open_convolution_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_PALETTE} {}", i18n.tr("menu.edit.remap_palette"))),
                    )
                    .clicked()
                {
                    actions.open_remap_palette_modal = true;
                    ui.close_menu();
                }
                ui.menu_button(format!("{FA_SORT} {}", i18n.tr("menu.edit.sort_palette")), |ui| {
                    if ui.add_enabled(has_doc, egui::Button::new(i18n.tr("panel.palette.sort_hue"))).clicked() {
                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Hue);
                        ui.close_menu();
                    }
                    if ui.add_enabled(has_doc, egui::Button::new(i18n.tr("panel.palette.sort_luminance"))).clicked() {
                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Luminance);
                        ui.close_menu();
                    }
                    if ui.add_enabled(has_doc, egui::Button::new(i18n.tr("panel.palette.sort_saturation"))).clicked() {
                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Saturation);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.add_enabled(has_doc, egui::Button::new(i18n.tr("panel.palette.sort_reverse"))).clicked() {
                        actions.trigger_sort_palette = Some(PaletteSortCriterion::Reverse);
                        ui.close_menu();
                    }
                });
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_PENCIL} {}", i18n.tr("menu.edit.custom_brush"))),
                    )
                    .clicked()
                {
                    actions.trigger_custom_brush = true;
                    ui.close_menu();
                }
            });

            // Select Menu
            ui.menu_button(i18n.tr("menu.select"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.edit.select_all")))
                    .clicked()
                {
                    actions.trigger_select_all = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.edit.invert_selection")))
                    .clicked()
                {
                    actions.trigger_invert_selection = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.select.deselect")))
                    .clicked()
                {
                    actions.trigger_deselect = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_RECT_SELECT} {}", i18n.tr("menu.select.modify"))),
                    )
                    .clicked()
                {
                    actions.open_modify_selection_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.select.expand_1px")))
                    .clicked()
                {
                    actions.trigger_expand_selection = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.select.contract_1px")))
                    .clicked()
                {
                    actions.trigger_contract_selection = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_doc, egui::Button::new(i18n.tr("menu.select.border_1px")))
                    .clicked()
                {
                    actions.trigger_border_selection = true;
                    ui.close_menu();
                }
            });

            // View Menu
            ui.menu_button(i18n.tr("menu.view"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if let Some((_, viewport, _)) = active_doc.as_mut() {
                    ui.radio_value(
                        &mut viewport.view_mode,
                        CanvasViewMode::Frame,
                        format!("{FA_VIEW_FRAME} {}", i18n.tr("menu.view.frame_mode")),
                    );
                    ui.radio_value(
                        &mut viewport.view_mode,
                        CanvasViewMode::Atlas,
                        format!("{FA_VIEW_ATLAS} {}", i18n.tr("menu.view.atlas_mode")),
                    );
                    ui.separator();
                    ui.checkbox(&mut viewport.show_pixel_grid, format!("{FA_GRID_PIXEL} {}", i18n.tr("menu.view.pixel_grid")));
                    ui.checkbox(&mut viewport.show_frame_grid, i18n.tr("menu.view.frame_grid"));
                    ui.checkbox(
                        &mut viewport.tiled_preview_3x3,
                        format!("{FA_GRID_TILED} {}", i18n.tr("menu.view.tiled_preview")),
                    );
                    ui.separator();
                    ui.checkbox(show_right_dock, i18n.tr("menu.view.right_dock"));
                    ui.checkbox(show_bottom_dock, i18n.tr("menu.view.bottom_dock"));
                    ui.separator();
                    ui.radio_value(
                        &mut viewport.background_mode,
                        BackgroundMode::DarkCheckerboard,
                        i18n.tr("menu.view.bg_dark"),
                    );
                    ui.radio_value(
                        &mut viewport.background_mode,
                        BackgroundMode::LightCheckerboard,
                        i18n.tr("menu.view.bg_light"),
                    );
                    ui.radio_value(
                        &mut viewport.background_mode,
                        BackgroundMode::SolidColor,
                        i18n.tr("menu.view.bg_solid"),
                    );
                    ui.separator();
                    if ui.button(format!("{FA_ZOOM_IN} {}", i18n.tr("menu.view.zoom_in"))).clicked() {
                        viewport.zoom_in();
                        ui.close_menu();
                    }
                    if ui.button(format!("{FA_ZOOM_OUT} {}", i18n.tr("menu.view.zoom_out"))).clicked() {
                        viewport.zoom_out();
                        ui.close_menu();
                    }
                    if ui.button(i18n.tr("menu.view.zoom_100")).clicked() {
                        viewport.set_zoom(1.0);
                        ui.close_menu();
                    }
                    if ui.button(format!("{FA_FIT} {}", i18n.tr("menu.view.zoom_fit"))).clicked() {
                        viewport.needs_fit_to_view = true;
                        ui.close_menu();
                    }
                } else {
                    ui.checkbox(show_right_dock, i18n.tr("menu.view.right_dock"));
                    ui.checkbox(show_bottom_dock, i18n.tr("menu.view.bottom_dock"));
                }
            });

            // Project Menu
            ui.menu_button(i18n.tr("menu.project"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_VIEW_ATLAS} {}", i18n.tr("menu.project.regrid"))),
                    )
                    .clicked()
                {
                    actions.open_regrid_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_FIT} {}", i18n.tr("menu.project.resize_9grid"))),
                    )
                    .clicked()
                {
                    actions.open_resize9_modal = true;
                    ui.close_menu();
                }
                if ui
                    .add_enabled(
                        has_doc,
                        egui::Button::new(format!("{FA_OUTLINE} {}", i18n.tr("menu.project.auto_crop"))),
                    )
                    .clicked()
                {
                    actions.trigger_auto_crop = true;
                    ui.close_menu();
                }
            });

            // Language Menu (2 columns, 37 languages)
            let mut new_lang = None;
            ui.menu_button(i18n.tr("menu.language"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                ui.set_min_width(370.0);
                ui.set_min_height(460.0);
                ui.spacing_mut().item_spacing = egui::vec2(16.0, 2.0);
                ui.spacing_mut().button_padding = egui::vec2(6.0, 2.0);
                egui::Grid::new("language_grid_selector")
                    .num_columns(2)
                    .spacing([16.0, 2.0])
                    .show(ui, |ui| {
                        let all = Language::ALL;
                        let total = all.len();
                        let rows = total.div_ceil(2);
                        for r in 0..rows {
                            for c in 0..2 {
                                let idx = r + c * rows;
                                if idx < total {
                                    let candidate = all[idx];
                                    let is_selected = i18n.language() == candidate;
                                    if ui.selectable_label(is_selected, candidate.label()).clicked() {
                                        new_lang = Some(candidate);
                                        ui.close_menu();
                                    }
                                } else {
                                    ui.label("");
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
            if let Some(lang) = new_lang {
                i18n.set_language(lang);
                actions.language_changed = Some(lang);
            }

            // Help Menu
            ui.menu_button(i18n.tr("menu.help"), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if ui.button(format!("{FA_QUESTION} {}", i18n.tr("menu.help.hotkeys"))).clicked() {
                    actions.open_hotkeys_modal = true;
                    ui.close_menu();
                }
                if ui.button(format!("{FA_PLAY} {}", i18n.tr("menu.file.open_demo"))).clicked() {
                    actions.open_demo_project = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button(format!("{FA_QUESTION} {}", i18n.tr("menu.help.about"))).clicked() {
                    actions.open_about_modal = true;
                    ui.close_menu();
                }
            });

            // Right-side update notification badge & theme selector
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Theme selector menu button with 35 retro, console, classic computer, modern and studio themes
                ui.menu_button(format!("{FA_PALETTE} {}", i18n.tr("menu.theme")), |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    let mut prev_category = 0;
                    for th in AppTheme::ALL {
                        let category = match th {
                            AppTheme::StudioDark | AppTheme::StudioLight | AppTheme::System => 1,
                            AppTheme::CrtAmber | AppTheme::CrtGreen | AppTheme::CrtCyan => 2,
                            AppTheme::Famicom
                            | AppTheme::MegaDrive
                            | AppTheme::Snes
                            | AppTheme::GameBoy
                            | AppTheme::GameBoyLight
                            | AppTheme::SuperGameBoy
                            | AppTheme::Ps1
                            | AppTheme::Dreamcast
                            | AppTheme::Ps2 => 3,
                            AppTheme::ZxSpectrum
                            | AppTheme::Commodore64
                            | AppTheme::AmigaWorkbench
                            | AppTheme::AtariSt
                            | AppTheme::Msx
                            | AppTheme::Apple2
                            | AppTheme::DosCga
                            | AppTheme::MacClassic
                            | AppTheme::Windows98
                            | AppTheme::Pico8 => 4,
                            _ => 5,
                        };
                        if prev_category != 0 && category != prev_category {
                            ui.separator();
                        }
                        prev_category = category;

                        let selected = *current_theme == th;
                        if ui.selectable_label(selected, i18n.tr(th.i18n_key())).clicked() {
                            *current_theme = th;
                            th.apply(ui.ctx());
                            actions.theme_changed = Some(th);
                            ui.close_menu();
                        }
                    }
                });

                ui.separator();

                if let Some(info) = update_info {
                    let badge_label = format!("{FA_ARROW_UP} {} ({})", i18n.tr("app.update.available"), info.tag_name);
                    let resp = ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(badge_label)
                                    .color(Color32::from_rgb(230, 200, 110))
                                    .small(),
                            )
                            .fill(Color32::from_rgb(48, 44, 32)),
                        )
                        .on_hover_text(i18n.tr("app.update.tooltip"));
                    if resp.clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(&info.release_url));
                    }
                    ui.separator();
                }
            });
        });
    });

    // Row 2: Contextual Tool Options Bar immediately below the main menu bar (when a document is open)
    if let Some((project, viewport, history)) = active_doc {
        TopBottomPanel::top("onianvil_contextual_options_bar").show(ctx, |ui: &mut Ui| {
        ui.horizontal_centered(|ui| {
            if ui
                .add_enabled(history.can_undo(), egui::Button::new(FA_UNDO))
                .on_hover_text(i18n.tr("tooltip.undo"))
                .clicked()
            {
                actions.trigger_undo = true;
            }
            if ui
                .add_enabled(history.can_redo(), egui::Button::new(FA_REDO))
                .on_hover_text(i18n.tr("tooltip.redo"))
                .clicked()
            {
                actions.trigger_redo = true;
            }
            ui.separator();

            ui.label(
                egui::RichText::new(i18n.tr("context.active_tool"))
                    .color(Color32::from_rgb(160, 160, 160))
                    .small(),
            );
            ui.strong(format!(
                "{} {}",
                tools.active_tool.icon(),
                i18n.tr(tools.active_tool.i18n_title_key())
            ))
            .on_hover_text(i18n.tr(tools.active_tool.i18n_name_key()));
            ui.separator();

            match tools.active_tool {
                ToolKind::Pencil | ToolKind::Eraser | ToolKind::DitherBrush | ToolKind::Shading => {
                    if tools.active_tool == ToolKind::Pencil && tools.custom_brush.is_some() {
                        let (w, h) = if let Some(cb) = &tools.custom_brush {
                            (cb.width, cb.height)
                        } else {
                            (0, 0)
                        };
                        ui.label(
                            egui::RichText::new(format!(
                                "{}: {}×{}",
                                i18n.tr("context.custom_brush"),
                                w,
                                h
                            ))
                            .strong()
                            .color(ui.visuals().selection.bg_fill),
                        );
                        if ui
                            .small_button(crate::theme::FA_CLOSE)
                            .on_hover_text(i18n.tr("tooltip.reset_custom_brush"))
                            .clicked()
                        {
                            tools.custom_brush = None;
                        }
                        ui.separator();
                    } else {
                        ui.label(i18n.tr("context.brush_size"));
                        ui.add(egui::DragValue::new(&mut tools.brush_size).range(1..=32).suffix("px"));
                        ui.separator();
                    }

                    if matches!(tools.active_tool, ToolKind::Pencil | ToolKind::Eraser) {
                        ui.checkbox(&mut tools.pixel_perfect, i18n.tr("context.pixel_perfect"))
                            .on_hover_text(i18n.tr("tooltip.pixel_perfect"));
                    }

                    if tools.active_tool == ToolKind::DitherBrush {
                        ui.label(i18n.tr("context.dither"));
                        egui::ComboBox::from_id_salt("context_dither_combo")
                            .selected_text(match tools.dither_pattern {
                                DitherPattern::Checker50 => i18n.tr("context.dither_50"),
                                DitherPattern::Sparse25 => i18n.tr("context.dither_25"),
                                DitherPattern::HorizontalLines => i18n.tr("context.dither_h"),
                                DitherPattern::VerticalLines => i18n.tr("context.dither_v"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut tools.dither_pattern,
                                    DitherPattern::Checker50,
                                    i18n.tr("context.dither_50"),
                                );
                                ui.selectable_value(
                                    &mut tools.dither_pattern,
                                    DitherPattern::Sparse25,
                                    i18n.tr("context.dither_25"),
                                );
                                ui.selectable_value(
                                    &mut tools.dither_pattern,
                                    DitherPattern::HorizontalLines,
                                    i18n.tr("context.dither_h"),
                                );
                                ui.selectable_value(
                                    &mut tools.dither_pattern,
                                    DitherPattern::VerticalLines,
                                    i18n.tr("context.dither_v"),
                                );
                            });
                    }

                    ui.checkbox(&mut tools.lock_alpha, i18n.tr("context.lock_alpha"))
                        .on_hover_text(i18n.tr("tooltip.lock_alpha"));
                }
                ToolKind::Line | ToolKind::Curve | ToolKind::Rectangle | ToolKind::Ellipse => {
                    ui.label(i18n.tr("context.brush_size"));
                    ui.add(egui::DragValue::new(&mut tools.brush_size).range(1..=16).suffix("px"));
                    if tools.active_tool == ToolKind::Curve {
                        ui.checkbox(&mut tools.pixel_perfect, i18n.tr("context.pixel_perfect"))
                            .on_hover_text(i18n.tr("tooltip.pixel_perfect"));
                    }
                    if matches!(tools.active_tool, ToolKind::Rectangle | ToolKind::Ellipse) {
                        ui.checkbox(&mut tools.filled_shape, i18n.tr("context.filled_shape"))
                            .on_hover_text(i18n.tr("tooltip.filled_shape"));
                    }
                    ui.checkbox(&mut tools.lock_alpha, i18n.tr("context.lock_alpha"))
                        .on_hover_text(i18n.tr("tooltip.lock_alpha"));
                }
                ToolKind::FloodFill => {
                    ui.label(i18n.tr("context.tolerance"));
                    ui.add(egui::DragValue::new(&mut tools.tolerance).range(0..=255));
                    ui.checkbox(&mut tools.lock_alpha, i18n.tr("context.lock_alpha"))
                        .on_hover_text(i18n.tr("tooltip.lock_alpha"));
                }
                ToolKind::Gradient => {
                    ui.label(i18n.tr("context.gradient_kind"));
                    egui::ComboBox::from_id_salt("context_gradient_kind_combo")
                        .selected_text(match tools.gradient_kind {
                            GradientKind::Linear => i18n.tr("context.gradient_linear"),
                            GradientKind::Radial => i18n.tr("context.gradient_radial"),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut tools.gradient_kind,
                                GradientKind::Linear,
                                i18n.tr("context.gradient_linear"),
                            );
                            ui.selectable_value(
                                &mut tools.gradient_kind,
                                GradientKind::Radial,
                                i18n.tr("context.gradient_radial"),
                            );
                        });

                    ui.separator();

                    ui.label(i18n.tr("context.gradient_dither"));
                    egui::ComboBox::from_id_salt("context_gradient_dither_combo")
                        .selected_text(match tools.gradient_dither {
                            GradientDither::None => i18n.tr("context.gradient_dither_none"),
                            GradientDither::Bayer4x4 => i18n.tr("context.gradient_dither_bayer4"),
                            GradientDither::Bayer8x8 => i18n.tr("context.gradient_dither_bayer8"),
                            GradientDither::Dither50 => i18n.tr("context.gradient_dither_50"),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut tools.gradient_dither,
                                GradientDither::None,
                                i18n.tr("context.gradient_dither_none"),
                            );
                            ui.selectable_value(
                                &mut tools.gradient_dither,
                                GradientDither::Bayer4x4,
                                i18n.tr("context.gradient_dither_bayer4"),
                            );
                            ui.selectable_value(
                                &mut tools.gradient_dither,
                                GradientDither::Bayer8x8,
                                i18n.tr("context.gradient_dither_bayer8"),
                            );
                            ui.selectable_value(
                                &mut tools.gradient_dither,
                                GradientDither::Dither50,
                                i18n.tr("context.gradient_dither_50"),
                            );
                        });

                    ui.checkbox(&mut tools.lock_alpha, i18n.tr("context.lock_alpha"))
                        .on_hover_text(i18n.tr("tooltip.lock_alpha"));
                }
                ToolKind::MagicWand => {
                    ui.label(i18n.tr("context.tolerance"));
                    ui.add(egui::DragValue::new(&mut tools.tolerance).range(0..=255));
                    ui.checkbox(&mut tools.wand_global, i18n.tr("context.wand_global"))
                        .on_hover_text(i18n.tr("tooltip.wand_global"));
                }
                ToolKind::RectSelect | ToolKind::Lasso | ToolKind::Eyedropper | ToolKind::PivotPoint => {
                    ui.checkbox(&mut tools.wrap_around, i18n.tr("context.wrap_around"))
                        .on_hover_text(i18n.tr("tooltip.wrap_around"));
                }
            }

            ui.separator();
            ui.label(i18n.tr("context.symmetry"))
                .on_hover_text(i18n.tr("tooltip.symmetry"));
            ui.selectable_value(
                &mut tools.symmetry,
                SymmetryMode::None,
                i18n.tr("context.sym_none"),
            )
            .on_hover_text(i18n.tr("tooltip.symmetry"));
            ui.selectable_value(
                &mut tools.symmetry,
                SymmetryMode::Horizontal,
                i18n.tr("context.sym_h"),
            )
            .on_hover_text(i18n.tr("tooltip.symmetry"));
            ui.selectable_value(
                &mut tools.symmetry,
                SymmetryMode::Vertical,
                i18n.tr("context.sym_v"),
            )
            .on_hover_text(i18n.tr("tooltip.symmetry"));
            ui.selectable_value(
                &mut tools.symmetry,
                SymmetryMode::Quad,
                i18n.tr("context.sym_quad"),
            )
            .on_hover_text(i18n.tr("tooltip.symmetry"));
            if tools.symmetry != SymmetryMode::None
                && ((viewport.symmetry_norm_x - 0.5).abs() > 0.005
                    || (viewport.symmetry_norm_y - 0.5).abs() > 0.005)
                && ui
                    .small_button(format!("{FA_RESET} {}", i18n.tr("context.sym_reset")))
                    .on_hover_text(i18n.tr("tooltip.sym_reset"))
                    .clicked()
                {
                    viewport.symmetry_norm_x = 0.5;
                    viewport.symmetry_norm_y = 0.5;
                }

            ui.separator();
            if ui
                .small_button(FA_FLIP_H)
                .on_hover_text(i18n.tr("tooltip.flip_h"))
                .clicked()
            {
                actions.trigger_flip_h = true;
            }
            if ui
                .small_button(FA_FLIP_V)
                .on_hover_text(i18n.tr("tooltip.flip_v"))
                .clicked()
            {
                actions.trigger_flip_v = true;
            }
            if ui
                .small_button(FA_ROTATE_90)
                .on_hover_text(i18n.tr("tooltip.rotate_90"))
                .clicked()
            {
                actions.trigger_rotate_90 = true;
            }
            if ui
                .small_button(FA_OUTLINE)
                .on_hover_text(i18n.tr("tooltip.auto_outline"))
                .clicked()
            {
                actions.trigger_auto_outline = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::ComboBox::from_id_salt("context_quick_peek_combo")
                    .selected_text(match project.metadata.onion_skin.quick_peek_mode {
                        QuickPeekMode::PreviousFrame => i18n.tr("context.peek_prev_frame"),
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
                ui.label(i18n.tr("context.quick_peek"))
                    .on_hover_text(i18n.tr("tooltip.quick_peek"));
                ui.separator();
                ui.checkbox(&mut tools.batch_mode, i18n.tr("context.batch_mode"))
                    .on_hover_text(i18n.tr("tooltip.batch_mode"));
            });
        });
        });
    }

    actions
}

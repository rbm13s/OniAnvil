use crate::canvas::renderer::CanvasRenderer;
use crate::canvas::viewport::{BackgroundMode, CanvasViewMode, ViewportState};
use crate::history::command::HistoryStack;
use crate::i18n::I18n;
use crate::model::project::Project;
use crate::theme::*;
use crate::tools::ToolSettings;
use crate::ui::color_picker::modern_color_picker_button;
use egui::{CentralPanel, Context, Ui};

pub fn show_center_workspace(
    ctx: &Context,
    i18n: &I18n,
    project: &mut Project,
    viewport: &mut ViewportState,
    tools: &mut ToolSettings,
    history: &mut HistoryStack,
    renderer: &mut CanvasRenderer,
) {
    CentralPanel::default()
        .frame(egui::Frame::NONE.fill(ctx.style().visuals.extreme_bg_color))
        .show(ctx, |ui: &mut Ui| {
            // Subtle top viewport control strip
            egui::Frame::NONE
                .fill(ui.visuals().panel_fill)
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Frame vs Atlas mode switch
                        ui.selectable_value(
                            &mut viewport.view_mode,
                            CanvasViewMode::Frame,
                            format!("{FA_VIEW_FRAME} {}", i18n.tr("viewport.mode.frame")),
                        )
                        .on_hover_text(i18n.tr("tooltip.view_frame"));
                        ui.selectable_value(
                            &mut viewport.view_mode,
                            CanvasViewMode::Atlas,
                            format!("{FA_VIEW_ATLAS} {}", i18n.tr("viewport.mode.atlas")),
                        )
                        .on_hover_text(i18n.tr("tooltip.view_atlas"));

                        ui.separator();

                        if ui.button(FA_ZOOM_OUT).on_hover_text(i18n.tr("tooltip.zoom_out")).clicked() {
                            viewport.zoom_out();
                        }

                        let zoom_pct = (viewport.zoom * 100.0).round() as u32;
                        egui::ComboBox::from_id_salt("viewport_zoom_combo")
                            .width(68.0)
                            .selected_text(format!("{}%", zoom_pct))
                            .show_ui(ui, |ui| {
                                for preset in ViewportState::ZOOM_PRESETS {
                                    let pct = (preset * 100.0).round() as u32;
                                    if ui
                                        .selectable_label(
                                            (viewport.zoom - preset).abs() < 0.01,
                                            format!("{}%", pct),
                                        )
                                        .clicked()
                                    {
                                        viewport.set_zoom(preset);
                                    }
                                }
                            });

                        if ui.button(FA_ZOOM_IN).on_hover_text(i18n.tr("tooltip.zoom_in")).clicked() {
                            viewport.zoom_in();
                        }

                        if ui
                            .button(i18n.tr("viewport.zoom.100"))
                            .on_hover_text(i18n.tr("tooltip.zoom_100"))
                            .clicked()
                        {
                            viewport.set_zoom(1.0);
                        }
                        if ui
                            .button(format!("{FA_FIT} {}", i18n.tr("viewport.zoom.fit")))
                            .on_hover_text(i18n.tr("tooltip.zoom_fit"))
                            .clicked()
                        {
                            viewport.needs_fit_to_view = true;
                        }

                        ui.separator();

                        ui.toggle_value(
                            &mut viewport.show_pixel_grid,
                            format!("{FA_GRID_PIXEL} {}", i18n.tr("viewport.grid.pixel")),
                        )
                        .on_hover_text(i18n.tr("tooltip.pixel_grid"));
                        ui.toggle_value(
                            &mut viewport.tiled_preview_3x3,
                            format!("{FA_GRID_TILED} {}", i18n.tr("viewport.grid.tiled")),
                        )
                        .on_hover_text(i18n.tr("tooltip.tiled_3x3"));

                        ui.separator();
                        ui.small(i18n.tr("viewport.bg.label"));
                        egui::ComboBox::from_id_salt("viewport_bg_mode_combo")
                            .selected_text(match viewport.background_mode {
                                BackgroundMode::DarkCheckerboard => i18n.tr("menu.view.bg_dark"),
                                BackgroundMode::LightCheckerboard => i18n.tr("menu.view.bg_light"),
                                BackgroundMode::SolidColor => i18n.tr("menu.view.bg_solid"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut viewport.background_mode,
                                    BackgroundMode::DarkCheckerboard,
                                    i18n.tr("menu.view.bg_dark"),
                                );
                                ui.selectable_value(
                                    &mut viewport.background_mode,
                                    BackgroundMode::LightCheckerboard,
                                    i18n.tr("menu.view.bg_light"),
                                );
                                ui.selectable_value(
                                    &mut viewport.background_mode,
                                    BackgroundMode::SolidColor,
                                    i18n.tr("menu.view.bg_solid"),
                                );
                            })
                            .response
                            .on_hover_text(i18n.tr("tooltip.bg_mode"));

                        if viewport.background_mode == BackgroundMode::SolidColor {
                            modern_color_picker_button(
                                ui,
                                "viewport_custom_bg_col",
                                &mut viewport.custom_bg_color,
                                i18n,
                                None,
                            );
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (aw, ah) = project.metadata.atlas_dimensions();
                            let xy_text = match viewport.hover_pixel {
                                Some((x, y)) => format!("{}, {}", x, y),
                                None => String::from("—"),
                            };
                            ui.small(format!(
                                "{} {}  |  {} {}×{}px  |  {} {}×{}px ({}/{})",
                                i18n.tr("viewport.info.cursor"),
                                xy_text,
                                i18n.tr("viewport.info.atlas"),
                                aw,
                                ah,
                                i18n.tr("viewport.info.frame"),
                                project.metadata.frame_width,
                                project.metadata.frame_height,
                                project.active_frame + 1,
                                project.metadata.frame_count
                            ));
                        });
                    });
                });

            renderer.show(ui, project, viewport, tools, history);
        });
}

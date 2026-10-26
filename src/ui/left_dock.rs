use crate::i18n::I18n;
use crate::io::settings::AppSettings;
use crate::theme::FA_SWAP;
use crate::tools::{ToolKind, ToolSettings};
use crate::ui::color_picker::modern_color_picker_button;
use egui::{Context, FontId, SidePanel, Ui, Vec2};

pub fn show_left_dock(
    ctx: &Context,
    i18n: &I18n,
    tools: &mut ToolSettings,
    mut settings: Option<&mut AppSettings>,
) {
    let panel_w = if tools.two_columns { 106.0 } else { 58.0 };

    SidePanel::left("onianvil_left_tool_dock")
        .resizable(false)
        .exact_width(panel_w)
        .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(6, 4)))
        .show(ctx, |ui: &mut Ui| {
            ui.vertical_centered(|ui| {
                // Top control strip with « / » column collapse/expand toggle
                ui.horizontal(|ui| {
                    let arrow = if tools.two_columns { "«" } else { "»" };
                    if ui
                        .small_button(arrow)
                        .on_hover_text(i18n.tr("tooltip.tools_columns"))
                        .clicked()
                    {
                        tools.two_columns = !tools.two_columns;
                    }
                    if tools.two_columns {
                        ui.small(i18n.tr("tools.header"));
                    }
                });

                ui.separator();

                let cols = if tools.two_columns { 2 } else { 1 };
                let btn_size = Vec2::new(44.0, 40.0);

                egui::Grid::new("left_dock_tool_grid")
                    .num_columns(cols)
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for (idx, tool) in ToolKind::ALL.iter().copied().enumerate() {
                            let is_active = tools.active_tool == tool;

                            let (rect, resp) =
                                ui.allocate_exact_size(btn_size, egui::Sense::click());
                            let visuals = ui.style().interact_selectable(&resp, is_active);

                            ui.painter().rect(
                                rect,
                                visuals.corner_radius,
                                visuals.bg_fill,
                                visuals.bg_stroke,
                                egui::StrokeKind::Inside,
                            );

                            let icon_color = if is_active || resp.hovered() {
                                visuals.fg_stroke.color
                            } else {
                                ui.visuals().text_color()
                            };

                            let label_color = if is_active || resp.hovered() {
                                visuals.fg_stroke.color
                            } else {
                                ui.visuals().weak_text_color()
                            };

                            let icon_pos = egui::pos2(rect.center().x, rect.center().y - 6.5);
                            ui.painter().text(
                                icon_pos,
                                egui::Align2::CENTER_CENTER,
                                tool.icon(),
                                FontId::proportional(14.0),
                                icon_color,
                            );

                            let label_pos = egui::pos2(rect.center().x, rect.center().y + 8.5);
                            ui.painter().text(
                                label_pos,
                                egui::Align2::CENTER_CENTER,
                                i18n.tr(tool.i18n_short_key()),
                                FontId::proportional(10.0),
                                label_color,
                            );

                            if resp.clicked() {
                                tools.active_tool = tool;
                            }
                            resp.on_hover_text(i18n.tr(tool.i18n_name_key()));

                            if (idx + 1) % cols == 0 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(6.0);
                ui.separator();
                if tools.two_columns {
                    ui.small(i18n.tr("tools.colors"));
                    ui.add_space(3.0);

                    let row_w = 64.0;
                    let pad = ((ui.available_width() - row_w) * 0.5).max(0.0);

                    ui.horizontal(|ui| {
                        ui.add_space(pad);
                        ui.allocate_ui_with_layout(
                            egui::vec2(row_w, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                ui.allocate_ui_with_layout(
                                    egui::vec2(28.0, 24.0),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.small(i18n.tr("tools.fg"));
                                    },
                                );
                                modern_color_picker_button(
                                    ui,
                                    "tools_fg_color",
                                    &mut tools.fg_color,
                                    i18n,
                                    settings.as_deref_mut(),
                                );
                            },
                        );
                    });

                    ui.add_space(3.0);

                    ui.horizontal(|ui| {
                        ui.add_space(pad);
                        ui.allocate_ui_with_layout(
                            egui::vec2(row_w, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                ui.allocate_ui_with_layout(
                                    egui::vec2(28.0, 24.0),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.small(i18n.tr("tools.bg"));
                                    },
                                );
                                modern_color_picker_button(
                                    ui,
                                    "tools_bg_color",
                                    &mut tools.bg_color,
                                    i18n,
                                    settings,
                                );
                            },
                        );
                    });
                } else {
                    modern_color_picker_button(
                        ui,
                        "tools_fg_color",
                        &mut tools.fg_color,
                        i18n,
                        settings.as_deref_mut(),
                    );
                    ui.add_space(4.0);
                    modern_color_picker_button(
                        ui,
                        "tools_bg_color",
                        &mut tools.bg_color,
                        i18n,
                        settings,
                    );
                }

                ui.add_space(4.0);
                let swap_label = egui::RichText::new(format!("{} X", FA_SWAP)).size(12.0);
                if ui
                    .button(swap_label)
                    .on_hover_text(i18n.tr("tools.swap_colors"))
                    .clicked()
                {
                    tools.swap_fg_bg();
                }
            });
        });
}

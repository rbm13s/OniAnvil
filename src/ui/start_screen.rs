use crate::i18n::I18n;
use crate::theme::{FA_CLOSE, FA_FOLDER, FA_IMPORT, FA_PLAY, FA_PLUS, FA_VIEW_ATLAS};
use egui::{Color32, FontId, RichText, Ui};
use std::path::PathBuf;

pub enum StartScreenAction {
    None,
    NewProject,
    OpenProject,
    OpenSlicer,
    OpenDemo,
    OpenRecent(PathBuf),
    RemoveRecent(PathBuf),
}

pub fn show_start_screen(
    ui: &mut Ui,
    i18n: &I18n,
    recent_projects: &[PathBuf],
    _settings_path: &std::path::Path,
    logo_texture: Option<&egui::TextureHandle>,
) -> StartScreenAction {
    let mut action = StartScreenAction::None;

    egui::ScrollArea::vertical()
        .id_salt("onianvil_start_screen_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                let avail_h = ui.available_height();
                let top_pad = if recent_projects.is_empty() {
                    (avail_h * 0.12).max(20.0)
                } else {
                    (avail_h * 0.05).max(16.0)
                };
                ui.add_space(top_pad);

                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::symmetric(36, 24))
                    .corner_radius(8)
                    .show(ui, |ui| {
                        ui.set_max_width(560.0);
                        ui.vertical_centered(|ui| {
                            // Logo / Title
                            if let Some(tex) = logo_texture {
                                ui.add(
                                    egui::Image::new((tex.id(), egui::vec2(72.0, 72.0)))
                                        .corner_radius(12),
                                );
                                ui.add_space(4.0);
                            } else {
                                ui.label(
                                    RichText::new(FA_VIEW_ATLAS)
                                        .size(36.0)
                                        .color(Color32::from_rgb(235, 175, 60)),
                                );
                                ui.add_space(4.0);
                            }
                            ui.label(
                                RichText::new(format!("OniAnvil v{}", env!("CARGO_PKG_VERSION")))
                                    .strong()
                                    .size(26.0)
                                    .color(Color32::from_rgb(240, 240, 240)),
                            );
                            ui.add_space(2.0);
                            ui.label(
                                RichText::new(i18n.tr("start.subtitle"))
                                    .size(13.5)
                                    .color(Color32::from_rgb(160, 160, 160)),
                            );

                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(14.0);

                            // Primary Action Buttons
                            let btn_w = 460.0;
                            if ui
                                .add_sized(
                                    [btn_w, 42.0],
                                    egui::Button::new(
                                        RichText::new(format!(
                                            "{FA_PLUS} {}",
                                            i18n.tr("start.new_project")
                                        ))
                                        .strong()
                                        .size(15.0),
                                    ),
                                )
                                .clicked()
                            {
                                action = StartScreenAction::NewProject;
                            }

                            ui.add_space(6.0);

                            if ui
                                .add_sized(
                                    [btn_w, 36.0],
                                    egui::Button::new(
                                        RichText::new(format!(
                                            "{FA_FOLDER} {}",
                                            i18n.tr("start.open_project")
                                        ))
                                        .size(13.5),
                                    ),
                                )
                                .clicked()
                            {
                                action = StartScreenAction::OpenProject;
                            }

                            ui.add_space(6.0);

                            if ui
                                .add_sized(
                                    [btn_w, 34.0],
                                    egui::Button::new(
                                        RichText::new(format!(
                                            "{FA_IMPORT} {}",
                                            i18n.tr("start.slicer")
                                        ))
                                        .size(13.0),
                                    ),
                                )
                                .clicked()
                            {
                                action = StartScreenAction::OpenSlicer;
                            }

                            ui.add_space(6.0);

                            let demo_btn = ui
                                .add_sized(
                                    [btn_w, 34.0],
                                    egui::Button::new(
                                        RichText::new(format!(
                                            "{FA_PLAY} {}",
                                            i18n.tr("start.open_demo")
                                        ))
                                        .size(13.0)
                                        .color(Color32::from_rgb(240, 185, 75)),
                                    ),
                                )
                                .on_hover_text(i18n.tr("start.demo_cat_tooltip"));

                            if demo_btn.clicked() {
                                action = StartScreenAction::OpenDemo;
                            }

                            ui.add_space(2.0);

                            let font_id = egui::FontId::proportional(11.0);
                            let credit_prefix = i18n.tr("start.demo_cat_credit");
                            let author_text = "xzany (itch.io)";
                            let w1 = ui.fonts(|f| {
                                f.layout_no_wrap(
                                    credit_prefix.to_string(),
                                    font_id.clone(),
                                    Color32::WHITE,
                                )
                                .size()
                                .x
                            });
                            let w2 = ui.fonts(|f| {
                                f.layout_no_wrap(
                                    author_text.to_string(),
                                    font_id,
                                    Color32::WHITE,
                                )
                                .size()
                                .x
                            });
                            let spacing = 4.0;
                            let total_w = w1 + spacing + w2;

                            ui.allocate_ui_with_layout(
                                egui::vec2(total_w, 18.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = spacing;
                                    ui.label(
                                        RichText::new(credit_prefix)
                                            .size(11.0)
                                            .color(Color32::from_rgb(140, 140, 140)),
                                    );
                                    ui.hyperlink_to(
                                        RichText::new(author_text)
                                            .size(11.0)
                                            .color(Color32::from_rgb(110, 175, 235)),
                                        "https://xzany.itch.io/cat-2d-pixel-art",
                                    );
                                },
                            );

                            ui.add_space(18.0);
                            ui.separator();
                            ui.add_space(10.0);

                            // Recent Projects Section
                            let count = recent_projects.len().min(10);
                            ui.label(
                                RichText::new(format!(
                                    "{} ({}/10):",
                                    i18n.tr("start.recent_title"),
                                    count
                                ))
                                .strong()
                                .size(14.0),
                            );
                            ui.add_space(6.0);

                            if recent_projects.is_empty() {
                                ui.label(
                                    RichText::new(i18n.tr("start.recent_empty"))
                                        .size(12.5)
                                        .color(Color32::from_rgb(140, 140, 140)),
                                );
                            } else {
                                for (idx, rpath) in recent_projects.iter().take(10).enumerate() {
                                    let exists = rpath.exists();
                                    let file_title = rpath
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("project");

                                    let icon = if exists { FA_FOLDER } else { "⚠" };
                                    let label = if exists {
                                        format!("{}. {} {} — {}", idx + 1, icon, file_title, rpath.display())
                                    } else {
                                        format!(
                                            "{}. {} {} ({}) — {}",
                                            idx + 1,
                                            icon,
                                            file_title,
                                            i18n.tr("start.not_found"),
                                            rpath.display()
                                        )
                                    };

                                    ui.allocate_ui_with_layout(
                                        egui::vec2(btn_w, 28.0),
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| {
                                            ui.spacing_mut().item_spacing.x = 6.0;
                                            let del_btn_w = 26.0;
                                            let file_btn_w = btn_w - del_btn_w - ui.spacing().item_spacing.x;

                                            let btn = egui::Button::new(
                                                RichText::new(label)
                                                    .font(FontId::proportional(12.0))
                                                    .color(if exists {
                                                        Color32::from_rgb(220, 220, 220)
                                                    } else {
                                                        Color32::from_rgb(220, 160, 100)
                                                    }),
                                            )
                                            .truncate();

                                            let resp = ui.add_sized([file_btn_w, 28.0], btn);
                                            if resp
                                                .on_hover_text(rpath.display().to_string())
                                                .clicked()
                                            {
                                                action = StartScreenAction::OpenRecent(rpath.clone());
                                            }

                                            if ui
                                                .add_sized(
                                                    [del_btn_w, 28.0],
                                                    egui::Button::new(RichText::new(FA_CLOSE).size(10.0)),
                                                )
                                                .on_hover_text(i18n.tr("start.remove_recent"))
                                                .clicked()
                                            {
                                                action = StartScreenAction::RemoveRecent(rpath.clone());
                                            }
                                        },
                                    );
                                }
                            }

                            ui.add_space(14.0);
                            ui.separator();
                            ui.add_space(8.0);

                            #[cfg(not(target_arch = "wasm32"))]
                            let settings_msg = format!(
                                "{}: {}",
                                i18n.tr("start.settings_saved_in"),
                                _settings_path.display()
                            );
                            #[cfg(target_arch = "wasm32")]
                            let settings_msg = format!(
                                "{}: {}",
                                i18n.tr("start.settings_saved_in"),
                                "Web Storage (LocalStorage & Cookie)"
                            );

                            ui.label(
                                RichText::new(settings_msg)
                                    .size(11.5)
                                    .color(Color32::from_rgb(140, 140, 140)),
                            );
                        });
                    });

                ui.add_space(20.0);
            });
        });

    action
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    #[test]
    fn test_start_screen_render() {
        let ctx = egui::Context::default();
        let i18n = I18n::new(Language::Russian);
        let recents = vec![
            PathBuf::from("E:/Projects/game_hero.oniproj"),
            PathBuf::from("C:/Oni/atlas_sprites.oniproj"),
        ];
        let p = PathBuf::from("C:/test/settings.json");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let action = show_start_screen(ui, &i18n, &recents, &p, None);
                assert!(matches!(action, StartScreenAction::None));
            });
        });
    }

    #[test]
    fn test_start_screen_empty_recents() {
        let ctx = egui::Context::default();
        let i18n = I18n::new(Language::English);
        let p = PathBuf::from("C:/test/settings.json");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let action = show_start_screen(ui, &i18n, &[], &p, None);
                assert!(matches!(action, StartScreenAction::None));
            });
        });
    }
}

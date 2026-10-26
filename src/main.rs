#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod canvas;
mod history;
mod i18n;
mod io;
mod model;
mod theme;
mod tools;
mod ui;
pub mod web;

use app::OniAnvilApp;

#[cfg(not(target_arch = "wasm32"))]
fn load_app_icon() -> Option<std::sync::Arc<egui::IconData>> {
    let bytes = include_bytes!("../assets/icon.ico");
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(std::sync::Arc::new(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    }))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let app_title = format!("OniAnvil v{}", env!("CARGO_PKG_VERSION"));
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1360.0, 820.0])
        .with_min_inner_size([900.0, 560.0])
        .with_title(&app_title);

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        &app_title,
        native_options,
        Box::new(|cc| Ok(Box::new(OniAnvilApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    crate::web::preload_web_fonts();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("onianvil_canvas")
            .expect("Failed to find onianvil_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("onianvil_canvas was not an HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(OniAnvilApp::new(cc)))),
            )
            .await;

        if let Err(e) = start_result {
            log::error!("Failed to start OniAnvil WebRunner: {:?}", e);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_rasterization() {
        let ctx = egui::Context::default();
        theme::setup_custom_fonts(&ctx);

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            let font_id = egui::FontId::proportional(14.0);
            for sample in [
                "#1",
                "#1 (2x)",
                "Влево",
                "Вправо",
                "Left",
                "Right",
                "ქართული",
                "Հայերեն",
                "Azərbaycan",
                "Oʻzbekcha",
                "\u{f5ee}",
                "\u{f043}",
                "\u{f5c3}",
                "\u{f5aa}",
            ] {
                let galley = ctx.fonts(|f| f.layout_no_wrap(sample.to_string(), font_id.clone(), egui::Color32::WHITE));
                assert!(galley.size().x > 0.0);
            }
        });
    }
}

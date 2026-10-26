use crate::model::metadata::ProjectMetadata;
use crate::model::project::Project;
use base64::Engine as _;
use image::{ImageFormat, RgbaImage};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub enum IoEvent {
    ProjectLoaded(Project),
    ProjectSaved(Option<PathBuf>),
    SingleFrameLoaded {
        frame_index: usize,
        rgba: RgbaImage,
    },
    SingleFrameExported,
    FrameSequenceLoaded(Project),
    FrameSequenceExported,
    SketchLoaded {
        rgba: RgbaImage,
        path_str: Option<String>,
        base64_png: String,
    },
    SlicerSheetLoaded {
        name: String,
        rgba: RgbaImage,
    },
    PaletteLoaded(Vec<[u8; 4]>),
    PaletteSaved,
    ExportedPngSaved,
    ExportedGifSaved,
    Error(String),
}

fn cmp_natural(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ia = a.chars().peekable();
    let mut ib = b.chars().peekable();

    loop {
        match (ia.peek(), ib.peek()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(&ca), Some(&cb)) => {
                if ca.is_ascii_digit() && cb.is_ascii_digit() {
                    let mut sa = String::new();
                    while let Some(&d) = ia.peek() {
                        if d.is_ascii_digit() {
                            sa.push(d);
                            ia.next();
                        } else {
                            break;
                        }
                    }
                    let mut sb = String::new();
                    while let Some(&d) = ib.peek() {
                        if d.is_ascii_digit() {
                            sb.push(d);
                            ib.next();
                        } else {
                            break;
                        }
                    }
                    let na = sa.parse::<u64>().unwrap_or(0);
                    let nb = sb.parse::<u64>().unwrap_or(0);
                    match na.cmp(&nb) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                } else {
                    let la = ca.to_ascii_lowercase();
                    let lb = cb.to_ascii_lowercase();
                    ia.next();
                    ib.next();
                    match la.cmp(&lb) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
            }
        }
    }
}

fn derive_sequence_base_name(first_stem: &str) -> String {
    let trimmed = first_stem
        .trim_end_matches(|c: char| c.is_ascii_digit())
        .trim_end_matches(['_', '-', ' ']);
    if trimmed.is_empty() {
        first_stem.to_string()
    } else {
        trimmed.to_string()
    }
}

#[derive(Clone, Default)]
pub struct IoManager {
    events: Arc<Mutex<Vec<IoEvent>>>,
}

impl IoManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_event(&self, event: IoEvent) {
        if let Ok(mut guard) = self.events.lock() {
            guard.push(event);
        }
    }

    pub fn drain_events(&self) -> Vec<IoEvent> {
        if let Ok(mut guard) = self.events.lock() {
            std::mem::take(&mut *guard)
        } else {
            Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn companion_json_path(png_path: &Path) -> PathBuf {
        let stem = png_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("sprite");
        let clean_stem = stem.strip_suffix(".onianvil").unwrap_or(stem);
        let file_name = format!("{}.onianvil.json", clean_stem);
        match png_path.parent() {
            Some(parent) => parent.join(file_name),
            None => PathBuf::from(file_name),
        }
    }

    #[allow(dead_code)]
    pub fn companion_png_path(json_path: &Path) -> PathBuf {
        let name = json_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("sprite.onianvil.json");
        let stem = name
            .strip_suffix(".onianvil.json")
            .or_else(|| name.strip_suffix(".json"))
            .unwrap_or("sprite");
        let file_name = format!("{}.png", stem);
        match json_path.parent() {
            Some(parent) => parent.join(file_name),
            None => PathBuf::from(file_name),
        }
    }

    pub fn encode_png_bytes(img: &RgbaImage) -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        img.write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| e.to_string())?;
        Ok(buf)
    }

    pub fn decode_image_bytes(bytes: &[u8]) -> Result<RgbaImage, String> {
        let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
        Ok(img.to_rgba8())
    }

    pub fn decode_png_bytes(bytes: &[u8]) -> Result<RgbaImage, String> {
        let img = image::load_from_memory_with_format(bytes, ImageFormat::Png)
            .or_else(|_| image::load_from_memory(bytes))
            .map_err(|e| e.to_string())?;
        Ok(img.to_rgba8())
    }

    pub fn build_project_from_bytes(
        png_bytes: &[u8],
        json_bytes: Option<&[u8]>,
        file_path: Option<PathBuf>,
    ) -> Result<Project, String> {
        let rgba = Self::decode_png_bytes(png_bytes)?;
        let width = rgba.width().max(1);
        let height = rgba.height().max(1);

        let default_name = file_path
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .map(|s| s.strip_suffix(".onianvil").unwrap_or(s).to_string())
            .unwrap_or_else(|| "sprite".to_string());

        let mut metadata = if let Some(raw_json) = json_bytes {
            serde_json::from_slice::<ProjectMetadata>(raw_json).unwrap_or_else(|_| {
                ProjectMetadata::new_config(default_name.clone(), width, height, 1, 1, None)
            })
        } else {
            ProjectMetadata::new_config(default_name, width, height, 1, 1, None)
        };

        let mut active_rgba = rgba;
        if metadata.layers.len() > 1 {
            for layer in &mut metadata.layers {
                if let Some(b64) = &layer.embedded_base64 {
                    if let Ok(decoded_bytes) =
                        base64::engine::general_purpose::STANDARD.decode(b64.trim())
                    {
                        if let Ok(layer_img) = Self::decode_png_bytes(&decoded_bytes) {
                            layer.layer_rgba = Some(layer_img);
                        }
                    }
                }
            }
            let active_idx = metadata
                .active_layer
                .min(metadata.layers.len().saturating_sub(1));
            metadata.active_layer = active_idx;
            if let Some(img) = metadata.layers[active_idx].layer_rgba.clone() {
                active_rgba = img;
            }
        }

        let mut project = Project::from_loaded(metadata, active_rgba, file_path.clone());
        if json_bytes.is_none() {
            let extracted = project.extract_unique_colors(64);
            if !extracted.is_empty() {
                project.metadata.palette.colors = extracted;
            }
        }

        // Restore embedded or relative-path sketch reference if present
        if let Some(b64) = &project.metadata.sketch.embedded_base64 {
            if let Ok(decoded_bytes) = base64::engine::general_purpose::STANDARD.decode(b64.trim())
            {
                if let Ok(sketch_img) = Self::decode_image_bytes(&decoded_bytes) {
                    project.sketch_rgba = Some(sketch_img);
                    project.sketch_dirty = true;
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if project.sketch_rgba.is_none() {
            if let Some(rel) = &project.metadata.sketch.relative_path {
                let candidate = PathBuf::from(rel);
                let resolved = if candidate.is_absolute() || candidate.exists() {
                    Some(candidate)
                } else {
                    file_path
                        .as_ref()
                        .and_then(|p| p.parent())
                        .map(|parent| parent.join(rel))
                };
                if let Some(sketch_path) = resolved {
                    if let Ok(bytes) = std::fs::read(&sketch_path) {
                        if let Ok(sketch_img) = Self::decode_image_bytes(&bytes) {
                            project.sketch_rgba = Some(sketch_img);
                            project.sketch_dirty = true;
                        }
                    }
                }
            }
        }

        Ok(project)
    }

    pub fn trigger_load_sketch_dialog(&self, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Reference Images", &["png", "jpg", "jpeg", "webp", "gif"])
                .pick_file()
            {
                match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|b| {
                    let rgba = Self::decode_image_bytes(&b)?;
                    let png_bytes = Self::encode_png_bytes(&rgba)?;
                    let b64 = base64::engine::general_purpose::STANDARD.encode(png_bytes);
                    Ok((rgba, b64))
                }) {
                    Ok((rgba, base64_png)) => {
                        self.push_event(IoEvent::SketchLoaded {
                            rgba,
                            path_str: Some(path.to_string_lossy().to_string()),
                            base64_png,
                        });
                    }
                    Err(err) => self.push_event(IoEvent::Error(err)),
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("Reference Images", &["png", "jpg", "jpeg", "webp", "gif"])
                    .pick_file()
                    .await
                {
                    let name = file.file_name();
                    let data = file.read().await;
                    match Self::decode_image_bytes(&data).and_then(|rgba| {
                        let png_bytes = Self::encode_png_bytes(&rgba)?;
                        let b64 = base64::engine::general_purpose::STANDARD.encode(png_bytes);
                        Ok((rgba, b64))
                    }) {
                        Ok((rgba, base64_png)) => {
                            io_clone.push_event(IoEvent::SketchLoaded {
                                rgba,
                                path_str: Some(name),
                                base64_png,
                            });
                        }
                        Err(err) => io_clone.push_event(IoEvent::Error(err)),
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_open_dialog(&self, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("OniAnvil Sprite / Metadata", &["png", "json"])
                .pick_file()
            {
                match Self::load_from_native_path(&path) {
                    Ok(project) => self.push_event(IoEvent::ProjectLoaded(project)),
                    Err(err) => self.push_event(IoEvent::Error(err)),
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(files) = rfd::AsyncFileDialog::new()
                    .add_filter("OniAnvil Sprite / Metadata", &["png", "json"])
                    .pick_files()
                    .await
                {
                    let mut png_bytes: Option<(String, Vec<u8>)> = None;
                    let mut json_bytes: Option<Vec<u8>> = None;

                    for f in files {
                        let name = f.file_name();
                        let data = f.read().await;
                        if name.to_lowercase().ends_with(".png") {
                            png_bytes = Some((name, data));
                        } else if name.to_lowercase().ends_with(".json") {
                            json_bytes = Some(data);
                        }
                    }

                    if let Some((name, p_bytes)) = png_bytes {
                        match Self::build_project_from_bytes(
                            &p_bytes,
                            json_bytes.as_deref(),
                            Some(PathBuf::from(name)),
                        ) {
                            Ok(project) => io_clone.push_event(IoEvent::ProjectLoaded(project)),
                            Err(err) => io_clone.push_event(IoEvent::Error(err)),
                        }
                        ctx.request_repaint();
                    }
                }
            });
        }
    }

    pub fn trigger_save(&self, project: &Project, force_dialog: bool, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let target_path = if !force_dialog {
                project.file_path.clone()
            } else {
                None
            };

            let resolved_path = target_path.or_else(|| {
                let default_file = format!("{}.png", project.metadata.project_name);
                rfd::FileDialog::new()
                    .set_file_name(&default_file)
                    .add_filter("PNG Sprite Sheet", &["png"])
                    .save_file()
            });

            if let Some(png_path) = resolved_path {
                match Self::save_to_native_path(project, &png_path) {
                    Ok(()) => self.push_event(IoEvent::ProjectSaved(Some(png_path))),
                    Err(err) => self.push_event(IoEvent::Error(err)),
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let _ = force_dialog;
            let io_clone = self.clone();
            let composited = project.composited_atlas_rgba();
            let png_res = Self::encode_png_bytes(&composited);
            let mut meta_clone = project.metadata.clone();
            if meta_clone.layers.len() > 1 {
                let active_idx = project
                    .metadata
                    .active_layer
                    .min(meta_clone.layers.len().saturating_sub(1));
                for (idx, layer) in meta_clone.layers.iter_mut().enumerate() {
                    let src_opt = if idx == active_idx {
                        Some(&project.atlas_rgba)
                    } else {
                        layer.layer_rgba.as_ref()
                    };
                    if let Some(img) = src_opt {
                        if let Ok(l_png) = Self::encode_png_bytes(img) {
                            layer.embedded_base64 =
                                Some(base64::engine::general_purpose::STANDARD.encode(l_png));
                        }
                    }
                }
            } else {
                for layer in &mut meta_clone.layers {
                    layer.embedded_base64 = None;
                }
            }
            if let Some(sketch_img) = &project.sketch_rgba {
                if let Ok(s_png) = Self::encode_png_bytes(sketch_img) {
                    meta_clone.sketch.embedded_base64 =
                        Some(base64::engine::general_purpose::STANDARD.encode(s_png));
                }
            }
            let json_res = serde_json::to_vec_pretty(&meta_clone).map_err(|e| e.to_string());
            let base_name = project.metadata.project_name.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match (png_res, json_res) {
                    (Ok(png_bytes), Ok(json_bytes)) => {
                        if let Some(handle) = rfd::AsyncFileDialog::new()
                            .set_file_name(format!("{}.png", base_name))
                            .save_file()
                            .await
                        {
                            let _ = handle.write(&png_bytes).await;
                        }
                        if let Some(handle) = rfd::AsyncFileDialog::new()
                            .set_file_name(format!("{}.onianvil.json", base_name))
                            .save_file()
                            .await
                        {
                            let _ = handle.write(&json_bytes).await;
                        }
                        io_clone.push_event(IoEvent::ProjectSaved(None));
                        ctx.request_repaint();
                    }
                    (Err(e), _) | (_, Err(e)) => {
                        io_clone.push_event(IoEvent::Error(e));
                        ctx.request_repaint();
                    }
                }
            });
        }
    }

    pub fn handle_dropped_files(&self, dropped: &[egui::DroppedFile]) {
        if dropped.is_empty() {
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut png_paths: Vec<PathBuf> = dropped
                .iter()
                .filter_map(|f| f.path.clone())
                .filter(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e.eq_ignore_ascii_case("png"))
                        .unwrap_or(false)
                })
                .collect();

            if png_paths.len() > 1 {
                png_paths.sort_by(|a, b| {
                    let na = a.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    let nb = b.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    cmp_natural(na, nb)
                });
                let first_stem = png_paths[0]
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("anim");
                let base_name = derive_sequence_base_name(first_stem);
                let mut frames = Vec::new();
                for p in png_paths {
                    if let Ok(bytes) = std::fs::read(&p) {
                        if let Ok(rgba) = Self::decode_png_bytes(&bytes) {
                            frames.push(rgba);
                        }
                    }
                }
                if let Some(proj) = Project::from_frame_sequence(base_name, &frames) {
                    self.push_event(IoEvent::FrameSequenceLoaded(proj));
                    return;
                }
            }

            for file in dropped {
                if let Some(path) = &file.path {
                    if let Ok(project) = Self::load_from_native_path(path) {
                        self.push_event(IoEvent::ProjectLoaded(project));
                        return;
                    }
                }
            }
        }

        let mut png_bytes: Option<(String, Vec<u8>)> = None;
        let mut json_bytes: Option<Vec<u8>> = None;

        for file in dropped {
            if let Some(bytes) = &file.bytes {
                let name = file.name.clone();
                if name.to_lowercase().ends_with(".png") {
                    png_bytes = Some((name, bytes.to_vec()));
                } else if name.to_lowercase().ends_with(".json") {
                    json_bytes = Some(bytes.to_vec());
                }
            }
        }

        if let Some((name, p_bytes)) = png_bytes {
            if let Ok(project) = Self::build_project_from_bytes(
                &p_bytes,
                json_bytes.as_deref(),
                Some(PathBuf::from(name)),
            ) {
                self.push_event(IoEvent::ProjectLoaded(project));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_native_path(path: &Path) -> Result<Project, String> {
        let is_json = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("json"))
            .unwrap_or(false);

        let (png_path, json_path) = if is_json {
            (Self::companion_png_path(path), path.to_path_buf())
        } else {
            (path.to_path_buf(), Self::companion_json_path(path))
        };

        let png_bytes = std::fs::read(&png_path).map_err(|e| e.to_string())?;
        let json_bytes = std::fs::read(&json_path).ok();

        Self::build_project_from_bytes(&png_bytes, json_bytes.as_deref(), Some(png_path))
    }

    /// Loads the embedded demo project (Walking Cat, 12 frames, 80x64).
    pub fn load_demo_cat_project() -> Result<Project, String> {
        let png_bytes = include_bytes!("../../assets/samples/cat_walk.png");
        let json_bytes = include_bytes!("../../assets/samples/cat_walk.onianvil.json");
        Self::build_project_from_bytes(png_bytes, Some(json_bytes), None)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_native_path(project: &Project, png_path: &Path) -> Result<(), String> {
        let json_path = Self::companion_json_path(png_path);
        let composited = project.composited_atlas_rgba();
        let png_bytes = Self::encode_png_bytes(&composited)?;

        let mut meta_to_save = project.metadata.clone();
        if meta_to_save.layers.len() > 1 {
            let active_idx = project
                .metadata
                .active_layer
                .min(meta_to_save.layers.len().saturating_sub(1));
            for (idx, layer) in meta_to_save.layers.iter_mut().enumerate() {
                let src_opt = if idx == active_idx {
                    Some(&project.atlas_rgba)
                } else {
                    layer.layer_rgba.as_ref()
                };
                if let Some(img) = src_opt {
                    if let Ok(l_png) = Self::encode_png_bytes(img) {
                        layer.embedded_base64 =
                            Some(base64::engine::general_purpose::STANDARD.encode(l_png));
                    }
                }
            }
        } else {
            for layer in &mut meta_to_save.layers {
                layer.embedded_base64 = None;
            }
        }
        if meta_to_save.sketch.embed_in_json {
            if meta_to_save.sketch.embedded_base64.is_none() {
                if let Some(sketch_img) = &project.sketch_rgba {
                    if let Ok(s_png) = Self::encode_png_bytes(sketch_img) {
                        meta_to_save.sketch.embedded_base64 =
                            Some(base64::engine::general_purpose::STANDARD.encode(s_png));
                    }
                }
            }
        } else {
            meta_to_save.sketch.embedded_base64 = None;
        }

        // Convert absolute sketch path to relative path if in the same folder
        if let (Some(parent), Some(rel_str)) = (
            png_path.parent(),
            meta_to_save.sketch.relative_path.clone(),
        ) {
            let sp = Path::new(&rel_str);
            if let Ok(stripped) = sp.strip_prefix(parent) {
                meta_to_save.sketch.relative_path =
                    Some(stripped.to_string_lossy().to_string());
            }
        }

        let json_string =
            serde_json::to_string_pretty(&meta_to_save).map_err(|e| e.to_string())?;

        let tmp_png = png_path.with_extension("png.tmp");
        let tmp_json = json_path.with_extension("json.tmp");

        if let Err(e) = std::fs::write(&tmp_png, png_bytes) {
            let _ = std::fs::remove_file(&tmp_png);
            return Err(format!("Failed to write temporary PNG: {}", e));
        }
        if let Err(e) = std::fs::write(&tmp_json, json_string) {
            let _ = std::fs::remove_file(&tmp_png);
            let _ = std::fs::remove_file(&tmp_json);
            return Err(format!("Failed to write temporary metadata JSON: {}", e));
        }

        if let Err(_e) = std::fs::rename(&tmp_png, png_path) {
            let _ = std::fs::remove_file(png_path);
            if let Err(e2) = std::fs::rename(&tmp_png, png_path) {
                let _ = std::fs::remove_file(&tmp_png);
                let _ = std::fs::remove_file(&tmp_json);
                return Err(format!("Failed to finalize PNG save: {}", e2));
            }
        }
        if let Err(_e) = std::fs::rename(&tmp_json, &json_path) {
            let _ = std::fs::remove_file(&json_path);
            if let Err(e2) = std::fs::rename(&tmp_json, &json_path) {
                let _ = std::fs::remove_file(&tmp_json);
                return Err(format!("Failed to finalize JSON save: {}", e2));
            }
        }
        Ok(())
    }

    pub fn trigger_load_slicer_sheet_dialog(&self, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Sprite Sheet PNG", &["png"])
                .pick_file()
            {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("sliced_sprite")
                    .to_string();
                match std::fs::read(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|b| Self::decode_png_bytes(&b))
                {
                    Ok(rgba) => self.push_event(IoEvent::SlicerSheetLoaded { name, rgba }),
                    Err(err) => self.push_event(IoEvent::Error(err)),
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("Sprite Sheet PNG", &["png"])
                    .pick_file()
                    .await
                {
                    let raw_name = file.file_name();
                    let name = raw_name
                        .strip_suffix(".png")
                        .unwrap_or(&raw_name)
                        .to_string();
                    let data = file.read().await;
                    match Self::decode_png_bytes(&data) {
                        Ok(rgba) => io_clone.push_event(IoEvent::SlicerSheetLoaded { name, rgba }),
                        Err(err) => io_clone.push_event(IoEvent::Error(err)),
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_import_single_frame_dialog(&self, frame_index: usize, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Frame PNG", &["png"])
                .pick_file()
            {
                match std::fs::read(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|b| Self::decode_png_bytes(&b))
                {
                    Ok(rgba) => {
                        self.push_event(IoEvent::SingleFrameLoaded { frame_index, rgba })
                    }
                    Err(err) => self.push_event(IoEvent::Error(err)),
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("Frame PNG", &["png"])
                    .pick_file()
                    .await
                {
                    let data = file.read().await;
                    match Self::decode_png_bytes(&data) {
                        Ok(rgba) => io_clone
                            .push_event(IoEvent::SingleFrameLoaded { frame_index, rgba }),
                        Err(err) => io_clone.push_event(IoEvent::Error(err)),
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_single_frame_dialog(
        &self,
        default_name: &str,
        frame_rgba: RgbaImage,
        ctx: egui::Context,
    ) {
        let png_res = Self::encode_png_bytes(&frame_rgba);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(png_bytes) = png_res {
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name(default_name)
                    .add_filter("Frame PNG", &["png"])
                    .save_file()
                {
                    if std::fs::write(path, png_bytes).is_ok() {
                        self.push_event(IoEvent::SingleFrameExported);
                    }
                    ctx.request_repaint();
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            let file_name = default_name.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(png_bytes) = png_res {
                    if let Some(handle) = rfd::AsyncFileDialog::new()
                        .set_file_name(&file_name)
                        .save_file()
                        .await
                    {
                        if handle.write(&png_bytes).await.is_ok() {
                            io_clone.push_event(IoEvent::SingleFrameExported);
                        }
                        ctx.request_repaint();
                    }
                }
            });
        }
    }

    pub fn trigger_import_frame_sequence_dialog(&self, ctx: egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(mut paths) = rfd::FileDialog::new()
                .add_filter("Frame Sequence PNGs", &["png"])
                .pick_files()
            {
                if paths.is_empty() {
                    return;
                }
                paths.sort_by(|a, b| {
                    let na = a.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    let nb = b.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    cmp_natural(na, nb)
                });
                let first_stem = paths[0]
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("anim");
                let base_name = derive_sequence_base_name(first_stem);

                let mut frames = Vec::with_capacity(paths.len());
                for p in paths {
                    if let Ok(bytes) = std::fs::read(&p) {
                        if let Ok(rgba) = Self::decode_png_bytes(&bytes) {
                            frames.push(rgba);
                        }
                    }
                }

                if let Some(proj) = Project::from_frame_sequence(base_name, &frames) {
                    self.push_event(IoEvent::FrameSequenceLoaded(proj));
                } else {
                    self.push_event(IoEvent::Error("No valid PNG frames selected".to_string()));
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(files) = rfd::AsyncFileDialog::new()
                    .add_filter("Frame Sequence PNGs", &["png"])
                    .pick_files()
                    .await
                {
                    if files.is_empty() {
                        return;
                    }
                    let mut named_data = Vec::with_capacity(files.len());
                    for f in files {
                        named_data.push((f.file_name(), f.read().await));
                    }
                    named_data.sort_by(|a, b| cmp_natural(&a.0, &b.0));

                    let first_stem = named_data[0]
                        .0
                        .strip_suffix(".png")
                        .or_else(|| named_data[0].0.strip_suffix(".PNG"))
                        .unwrap_or(&named_data[0].0);
                    let base_name = derive_sequence_base_name(first_stem);

                    let mut frames = Vec::with_capacity(named_data.len());
                    for (_, bytes) in named_data {
                        if let Ok(rgba) = Self::decode_png_bytes(&bytes) {
                            frames.push(rgba);
                        }
                    }

                    if let Some(proj) = Project::from_frame_sequence(base_name, &frames) {
                        io_clone.push_event(IoEvent::FrameSequenceLoaded(proj));
                    } else {
                        io_clone.push_event(IoEvent::Error(
                            "No valid PNG frames selected".to_string(),
                        ));
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_frame_sequence_dialog(&self, project: &Project, ctx: egui::Context) {
        let frames = project.extract_all_frames();
        if frames.is_empty() {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let default_file = format!("{}.png", project.metadata.project_name);
            if let Some(chosen_path) = rfd::FileDialog::new()
                .set_file_name(&default_file)
                .add_filter("Frame Sequence Base PNG (e.g. walk.png)", &["png"])
                .save_file()
            {
                let parent = chosen_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."));
                let stem = chosen_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&project.metadata.project_name);

                let mut all_ok = true;
                for (i, frame_img) in frames.iter().enumerate() {
                    let file_name = format!("{}{}.png", stem, i + 1);
                    let target = parent.join(file_name);
                    match Self::encode_png_bytes(frame_img)
                        .and_then(|bytes| std::fs::write(&target, bytes).map_err(|e| e.to_string()))
                    {
                        Ok(()) => {}
                        Err(e) => {
                            self.push_event(IoEvent::Error(e));
                            all_ok = false;
                            break;
                        }
                    }
                }
                if all_ok {
                    self.push_event(IoEvent::FrameSequenceExported);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            let base_stem = project.metadata.project_name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                for (i, frame_img) in frames.iter().enumerate() {
                    if let Ok(png_bytes) = Self::encode_png_bytes(frame_img) {
                        let fname = format!("{}{}.png", base_stem, i + 1);
                        if let Some(handle) = rfd::AsyncFileDialog::new()
                            .set_file_name(&fname)
                            .save_file()
                            .await
                        {
                            let _ = handle.write(&png_bytes).await;
                        }
                    }
                }
                io_clone.push_event(IoEvent::FrameSequenceExported);
                ctx.request_repaint();
            });
        }
    }

    pub fn trigger_import_palette_dialog(&self, ctx: egui::Context) {
        use crate::model::palette::{parse_palette_png, parse_palette_text};

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Palette (.png, .hex, .pal)", &["png", "hex", "pal", "txt"])
                .pick_file()
            {
                let is_png_ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("png"))
                    .unwrap_or(false);

                if let Ok(bytes) = std::fs::read(&path) {
                    let cols = if is_png_ext || bytes.starts_with(b"\x89PNG") {
                        parse_palette_png(&bytes).unwrap_or_default()
                    } else if let Ok(text) = String::from_utf8(bytes) {
                        parse_palette_text(&text)
                    } else {
                        Vec::new()
                    };
                    if !cols.is_empty() {
                        self.push_event(IoEvent::PaletteLoaded(cols));
                    }
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("Palette (.png, .hex, .pal)", &["png", "hex", "pal", "txt"])
                    .pick_file()
                    .await
                {
                    let name = file.file_name().to_lowercase();
                    let bytes = file.read().await;
                    let cols = if name.ends_with(".png") || bytes.starts_with(b"\x89PNG") {
                        parse_palette_png(&bytes).unwrap_or_default()
                    } else if let Ok(text) = String::from_utf8(bytes) {
                        parse_palette_text(&text)
                    } else {
                        Vec::new()
                    };
                    if !cols.is_empty() {
                        io_clone.push_event(IoEvent::PaletteLoaded(cols));
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_palette_dialog(&self, colors: &[[u8; 4]], ctx: egui::Context) {
        use crate::model::palette::{encode_palette_png, format_palette_hex};
        let hex_text = format_palette_hex(colors);
        let png_bytes_opt = encode_palette_png(colors).ok();

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("palette.png")
                .add_filter("PNG Palette (.png)", &["png"])
                .add_filter("Hex Palette (.hex)", &["hex"])
                .save_file()
            {
                let is_png = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("png"))
                    .unwrap_or(false);

                let res = if is_png {
                    if let Some(png_bytes) = png_bytes_opt {
                        std::fs::write(path, png_bytes)
                    } else {
                        std::fs::write(path, hex_text)
                    }
                } else {
                    std::fs::write(path, hex_text)
                };

                if res.is_ok() {
                    self.push_event(IoEvent::PaletteSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name("palette.png")
                    .save_file()
                    .await
                {
                    let is_png = handle.file_name().to_lowercase().ends_with(".png");
                    let res = if is_png {
                        if let Some(png_bytes) = png_bytes_opt {
                            handle.write(&png_bytes).await
                        } else {
                            handle.write(hex_text.as_bytes()).await
                        }
                    } else {
                        handle.write(hex_text.as_bytes()).await
                    };
                    if res.is_ok() {
                        io_clone.push_event(IoEvent::PaletteSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_palette_hex_dialog(&self, colors: &[[u8; 4]], ctx: egui::Context) {
        use crate::model::palette::format_palette_hex;
        let hex_text = format_palette_hex(colors);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("palette.hex")
                .add_filter("Hex Palette (.hex)", &["hex"])
                .save_file()
            {
                if std::fs::write(path, hex_text).is_ok() {
                    self.push_event(IoEvent::PaletteSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name("palette.hex")
                    .save_file()
                    .await
                {
                    if handle.write(hex_text.as_bytes()).await.is_ok() {
                        io_clone.push_event(IoEvent::PaletteSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_palette_gpl_dialog(&self, name: &str, colors: &[[u8; 4]], ctx: egui::Context) {
        use crate::model::palette::format_palette_gpl;
        let gpl_text = format_palette_gpl(name, colors);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(format!("{}.gpl", name))
                .add_filter("GIMP Palette (.gpl)", &["gpl"])
                .save_file()
            {
                if std::fs::write(path, gpl_text).is_ok() {
                    self.push_event(IoEvent::PaletteSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            let default_filename = format!("{}.gpl", name);
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name(&default_filename)
                    .save_file()
                    .await
                {
                    if handle.write(gpl_text.as_bytes()).await.is_ok() {
                        io_clone.push_event(IoEvent::PaletteSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_sprite_sheet_json_dialog(&self, project: &Project, ctx: egui::Context) {
        use crate::io::export::encode_sprite_sheet_json;
        let image_filename = format!("{}.png", project.metadata.project_name);
        let json_text = encode_sprite_sheet_json(project, &image_filename);
        let default_name = format!("{}_spritesheet.json", project.metadata.project_name);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&default_name)
                .add_filter("Sprite Sheet JSON (.json)", &["json"])
                .save_file()
            {
                if std::fs::write(path, json_text).is_ok() {
                    self.push_event(IoEvent::ProjectSaved(None));
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name(&default_name)
                    .save_file()
                    .await
                {
                    if handle.write(json_text.as_bytes()).await.is_ok() {
                        io_clone.push_event(IoEvent::ProjectSaved(None));
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_export_packed_atlas_dialog(
        &self,
        default_name: &str,
        png_bytes: Vec<u8>,
        json_text: String,
        ctx: egui::Context,
    ) {
        #[cfg(target_arch = "wasm32")]
        let _ = json_text;
        let file_name = format!("{}_packed.png", default_name);
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&file_name)
                .add_filter("PNG Image (.png)", &["png"])
                .save_file()
            {
                if std::fs::write(&path, png_bytes).is_ok() {
                    let json_path = path.with_extension("json");
                    let _ = std::fs::write(json_path, json_text);
                    self.push_event(IoEvent::ExportedPngSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name(&file_name)
                    .save_file()
                    .await
                {
                    if handle.write(&png_bytes).await.is_ok() {
                        io_clone.push_event(IoEvent::ExportedPngSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_save_exported_png(
        &self,
        default_name: &str,
        png_bytes: Vec<u8>,
        ctx: egui::Context,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(default_name)
                .add_filter("PNG Image", &["png"])
                .save_file()
            {
                if std::fs::write(path, png_bytes).is_ok() {
                    self.push_event(IoEvent::ExportedPngSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            let file_name = default_name.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name(&file_name)
                    .save_file()
                    .await
                {
                    if handle.write(&png_bytes).await.is_ok() {
                        io_clone.push_event(IoEvent::ExportedPngSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }

    pub fn trigger_save_exported_gif(
        &self,
        default_name: &str,
        gif_bytes: Vec<u8>,
        ctx: egui::Context,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(default_name)
                .add_filter("Animated GIF", &["gif"])
                .save_file()
            {
                if std::fs::write(path, gif_bytes).is_ok() {
                    self.push_event(IoEvent::ExportedGifSaved);
                }
                ctx.request_repaint();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let io_clone = self.clone();
            let file_name = default_name.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(handle) = rfd::AsyncFileDialog::new()
                    .set_file_name(&file_name)
                    .save_file()
                    .await
                {
                    if handle.write(&gif_bytes).await.is_ok() {
                        io_clone.push_event(IoEvent::ExportedGifSaved);
                    }
                    ctx.request_repaint();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_companion_paths() {
        let p = Path::new("characters/knight.png");
        let j = IoManager::companion_json_path(p);
        assert_eq!(j, PathBuf::from("characters/knight.onianvil.json"));

        let p2 = IoManager::companion_png_path(&j);
        assert_eq!(p2, PathBuf::from("characters/knight.png"));
    }

    #[test]
    fn test_png_and_metadata_roundtrip() {
        let meta = ProjectMetadata::new_config("hero".to_string(), 16, 16, 4, 2, Some(16));
        let proj = Project::new(meta.clone());
        let png_bytes = IoManager::encode_png_bytes(&proj.atlas_rgba).expect("encode png");
        let json_bytes = serde_json::to_vec_pretty(&proj.metadata).expect("encode json");

        let loaded = IoManager::build_project_from_bytes(&png_bytes, Some(&json_bytes), None)
            .expect("load project from bytes");

        assert_eq!(loaded.metadata, meta);
        assert_eq!(loaded.atlas_rgba.dimensions(), (32, 32));
    }

    #[test]
    fn test_atomic_save_to_native_path() {
        let temp_dir = std::env::temp_dir().join(format!(
            "onianvil_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let png_path = temp_dir.join("test_save.png");
        let meta = ProjectMetadata::new_config("hero".to_string(), 16, 16, 2, 2, Some(16));
        let proj = Project::new(meta);
        let res = IoManager::save_to_native_path(&proj, &png_path);
        assert!(res.is_ok());
        assert!(png_path.exists());
        assert!(IoManager::companion_json_path(&png_path).exists());
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

use crate::io::storage::IoManager;
use crate::model::project::Project;
#[cfg(target_arch = "wasm32")]
use base64::Engine as _;

pub const AUTOSAVE_INTERVAL_SEC: f32 = 45.0;

#[cfg(target_arch = "wasm32")]
const WEB_KEY_PNG_B64: &str = "onianvil_autosave_png_b64";
#[cfg(target_arch = "wasm32")]
const WEB_KEY_JSON: &str = "onianvil_autosave_json";

pub struct AutosaveService {
    elapsed_sec: f32,
    pub recovery_candidate: Option<Project>,
}

impl Default for AutosaveService {
    fn default() -> Self {
        Self {
            elapsed_sec: 0.0,
            recovery_candidate: None,
        }
    }
}

impl AutosaveService {
    pub fn new(storage: Option<&(dyn eframe::Storage + '_)>) -> Self {
        let candidate = Self::load_recovery(storage);
        Self {
            elapsed_sec: 0.0,
            recovery_candidate: candidate,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn native_recovery_paths() -> Option<(std::path::PathBuf, std::path::PathBuf)> {
        let proj_dirs = directories::ProjectDirs::from("", "", "OniAnvil")?;
        let dir = proj_dirs.data_local_dir();
        let _ = std::fs::create_dir_all(dir);
        Some((
            dir.join("autosave_recovery.png"),
            dir.join("autosave_recovery.onianvil.json"),
        ))
    }

    pub fn load_recovery(storage: Option<&(dyn eframe::Storage + '_)>) -> Option<Project> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = storage;
            let (png_path, json_path) = Self::native_recovery_paths()?;
            if !png_path.exists() || !json_path.exists() {
                return None;
            }
            let png_bytes = std::fs::read(&png_path).ok()?;
            let json_bytes = std::fs::read(&json_path).ok()?;
            let proj =
                IoManager::build_project_from_bytes(&png_bytes, Some(&json_bytes), None).ok()?;
            if proj.atlas_rgba.as_raw().chunks_exact(4).any(|p| p[3] > 0) {
                return Some(proj);
            }
            None
        }

        #[cfg(target_arch = "wasm32")]
        {
            let store = storage?;
            let b64 = store.get_string(WEB_KEY_PNG_B64)?;
            let json_str = store.get_string(WEB_KEY_JSON)?;
            let png_bytes = base64::engine::general_purpose::STANDARD
                .decode(b64.trim())
                .ok()?;
            let proj = IoManager::build_project_from_bytes(
                &png_bytes,
                Some(json_str.as_bytes()),
                None,
            )
            .ok()?;
            if proj.atlas_rgba.as_raw().chunks_exact(4).any(|p| p[3] > 0) {
                return Some(proj);
            }
            None
        }
    }

    pub fn save_recovery_snapshot(
        project: &Project,
        storage: Option<&mut (dyn eframe::Storage + '_)>,
    ) -> bool {
        // Skip snapshot if project is already saved to disk and has no unsaved modifications
        if !project.has_unsaved_changes && project.file_path.is_some() {
            return false;
        }

        // Fast chunk-based alpha check without per-pixel point allocations
        if !project.atlas_rgba.as_raw().chunks_exact(4).any(|p| p[3] > 0) {
            return false;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = storage;
            if let Some((png_path, _)) = Self::native_recovery_paths() {
                return IoManager::save_to_native_path(project, &png_path).is_ok();
            }
            false
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(store) = storage {
                if let (Ok(png_bytes), Ok(json_str)) = (
                    IoManager::encode_png_bytes(&project.atlas_rgba),
                    serde_json::to_string(&project.metadata),
                ) {
                    let b64 = base64::engine::general_purpose::STANDARD.encode(png_bytes);
                    store.set_string(WEB_KEY_PNG_B64, b64);
                    store.set_string(WEB_KEY_JSON, json_str);
                    return true;
                }
            }
            false
        }
    }

    pub fn clear_recovery(storage: Option<&mut (dyn eframe::Storage + '_)>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = storage;
            if let Some((png_path, json_path)) = Self::native_recovery_paths() {
                let _ = std::fs::remove_file(png_path);
                let _ = std::fs::remove_file(json_path);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(store) = storage {
                store.set_string(WEB_KEY_PNG_B64, String::new());
                store.set_string(WEB_KEY_JSON, String::new());
            }
        }
    }

    pub fn tick(
        &mut self,
        dt_sec: f32,
        project: &Project,
        storage: Option<&mut (dyn eframe::Storage + '_)>,
    ) -> bool {
        self.elapsed_sec += dt_sec.clamp(0.0, 0.5);
        if self.elapsed_sec >= AUTOSAVE_INTERVAL_SEC {
            self.elapsed_sec = 0.0;
            return Self::save_recovery_snapshot(project, storage);
        }
        false
    }
}

use crate::i18n::Language;
use crate::theme::AppTheme;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn default_tools_two_columns() -> bool {
    true
}

pub fn default_recent_colors() -> Vec<[u8; 4]> {
    vec![
        [0, 0, 0, 255],       // Black
        [255, 255, 255, 255], // White
        [128, 128, 128, 255], // Gray
        [230, 41, 55, 255],   // Red
        [255, 161, 0, 255],   // Orange
        [253, 249, 0, 255],   // Yellow
        [0, 228, 48, 255],    // Green
        [0, 180, 240, 255],   // Cyan
        [0, 121, 241, 255],   // Blue
        [153, 51, 238, 255],  // Purple
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub theme: AppTheme,
    #[serde(default = "default_tools_two_columns")]
    pub tools_two_columns: bool,
    #[serde(default)]
    pub recent_projects: Vec<PathBuf>,
    #[serde(default = "default_recent_colors")]
    pub recent_colors: Vec<[u8; 4]>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: Language::detect_system_or_en(),
            theme: AppTheme::StudioDark,
            tools_two_columns: true,
            recent_projects: Vec::new(),
            recent_colors: default_recent_colors(),
        }
    }
}

impl AppSettings {
    pub fn settings_file_path() -> PathBuf {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(appdata) = std::env::var("APPDATA") {
                return PathBuf::from(appdata)
                    .join("OniAnvil")
                    .join("settings.json");
            }
            if let Ok(userprofile) = std::env::var("USERPROFILE") {
                return PathBuf::from(userprofile)
                    .join("AppData")
                    .join("Roaming")
                    .join("OniAnvil")
                    .join("settings.json");
            }
            #[cfg(target_os = "macos")]
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("OniAnvil")
                    .join("settings.json");
            }
            if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
                if !xdg_config.trim().is_empty() {
                    return PathBuf::from(xdg_config)
                        .join("OniAnvil")
                        .join("settings.json");
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home)
                    .join(".config")
                    .join("OniAnvil")
                    .join("settings.json");
            }
        }
        PathBuf::from("onianvil_settings.json")
    }

    #[allow(dead_code)]
    pub fn file_path() -> PathBuf {
        Self::settings_file_path()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_web_cookie(json: &str) {
        use base64::Engine as _;
        use wasm_bindgen::JsCast as _;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Ok(html_doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
                    let encoded = base64::engine::general_purpose::STANDARD.encode(json);
                    let cookie = format!(
                        "onianvil_settings={}; max-age=31536000; path=/; SameSite=Lax",
                        encoded
                    );
                    let _ = html_doc.set_cookie(&cookie);
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_web_cookie() -> Option<String> {
        use base64::Engine as _;
        use wasm_bindgen::JsCast as _;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Ok(html_doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
                    if let Ok(cookies) = html_doc.cookie() {
                        for item in cookies.split(';') {
                            let mut parts = item.trim().splitn(2, '=');
                            if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                                if k == "onianvil_settings" {
                                    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(v) {
                                        if let Ok(s) = String::from_utf8(bytes) {
                                            return Some(s);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn load(storage: Option<&dyn eframe::Storage>) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = storage;
            let path = Self::settings_file_path();
            if let Ok(txt) = std::fs::read_to_string(&path) {
                if let Ok(mut cfg) = serde_json::from_str::<Self>(&txt) {
                    cfg.recent_projects.truncate(10);
                    if cfg.recent_colors.is_empty() {
                        cfg.recent_colors = default_recent_colors();
                    }
                    cfg.recent_colors.truncate(10);
                    return cfg;
                }
            }
            let def = Self::default();
            let _ = def.save();
            def
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = storage {
                if let Some(json) = storage.get_string("onianvil_settings") {
                    if let Ok(mut cfg) = serde_json::from_str::<Self>(&json) {
                        cfg.recent_projects.truncate(10);
                        if cfg.recent_colors.is_empty() {
                            cfg.recent_colors = default_recent_colors();
                        }
                        cfg.recent_colors.truncate(10);
                        return cfg;
                    }
                }
            }
            if let Some(json) = Self::load_web_cookie() {
                if let Ok(mut cfg) = serde_json::from_str::<Self>(&json) {
                    cfg.recent_projects.truncate(10);
                    if cfg.recent_colors.is_empty() {
                        cfg.recent_colors = default_recent_colors();
                    }
                    cfg.recent_colors.truncate(10);
                    return cfg;
                }
            }
            Self::default()
        }
    }

    pub fn save(&self) -> Result<PathBuf, String> {
        let path = Self::settings_file_path();
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let json = serde_json::to_string_pretty(self)
                .map_err(|e| format!("Failed to serialize settings: {}", e))?;
            std::fs::write(&path, json)
                .map_err(|e| format!("Failed to write settings to {}: {}", path.display(), e))?;
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(json) = serde_json::to_string(self) {
                Self::save_web_cookie(&json);
            }
        }

        Ok(path)
    }

    pub fn add_recent(&mut self, path: PathBuf) {
        self.recent_projects.retain(|p| p != &path);
        self.recent_projects.insert(0, path);
        self.recent_projects.truncate(10);
        let _ = self.save();
    }

    pub fn remove_recent(&mut self, path: &Path) {
        self.recent_projects.retain(|p| p != path);
        let _ = self.save();
    }

    pub fn add_recent_color(&mut self, color: [u8; 4]) {
        self.recent_colors.retain(|c| c != &color);
        self.recent_colors.insert(0, color);
        self.recent_colors.truncate(10);
        let _ = self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_recent_projects_cap_and_order() {
        let mut settings = AppSettings::default();
        for i in 1..=15 {
            let p = PathBuf::from(format!("project_{}.onianvil.json", i));
            settings.recent_projects.retain(|item| item != &p);
            settings.recent_projects.insert(0, p);
            settings.recent_projects.truncate(10);
        }
        assert_eq!(settings.recent_projects.len(), 10);
        assert_eq!(
            settings.recent_projects[0],
            PathBuf::from("project_15.onianvil.json")
        );
        assert_eq!(
            settings.recent_projects[9],
            PathBuf::from("project_6.onianvil.json")
        );
    }

    #[test]
    fn test_app_settings_recent_colors_cap_and_order() {
        let mut settings = AppSettings::default();
        for i in 0..15 {
            settings.add_recent_color([i as u8, 10, 20, 255]);
        }
        assert_eq!(settings.recent_colors.len(), 10);
        assert_eq!(settings.recent_colors[0], [14, 10, 20, 255]);
        assert_eq!(settings.recent_colors[9], [5, 10, 20, 255]);

        // Re-adding existing color bumps it to front
        settings.add_recent_color([5, 10, 20, 255]);
        assert_eq!(settings.recent_colors[0], [5, 10, 20, 255]);
        assert_eq!(settings.recent_colors.len(), 10);
    }

    #[test]
    fn test_app_settings_json_roundtrip() {
        let original = AppSettings {
            language: Language::Russian,
            theme: AppTheme::CyberpunkNeon,
            tools_two_columns: false,
            recent_projects: vec![PathBuf::from("hero.png")],
            recent_colors: vec![[10, 20, 30, 255]],
        };
        let json = serde_json::to_string(&original).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.language, Language::Russian);
        assert_eq!(loaded.theme, AppTheme::CyberpunkNeon);
        assert!(!loaded.tools_two_columns);
        assert_eq!(loaded.recent_projects.len(), 1);
        assert_eq!(loaded.recent_colors, vec![[10, 20, 30, 255]]);
    }
}

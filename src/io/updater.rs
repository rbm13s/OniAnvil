use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub const GITHUB_REPO: &str = "rbm13s/OniAnvil";
pub const GITHUB_REPO_URL: &str = "https://github.com/rbm13s/OniAnvil";

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub tag_name: String,
    pub release_url: String,
}

#[derive(Clone, Default)]
pub struct UpdateChecker {
    result: Arc<Mutex<Option<UpdateInfo>>>,
}

impl UpdateChecker {
    pub fn start(ctx: egui::Context) -> Self {
        let checker = Self {
            result: Arc::new(Mutex::new(None)),
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let result_ref = Arc::clone(&checker.result);
            let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
            let mut request = ehttp::Request::get(url);
            request
                .headers
                .insert("User-Agent".to_string(), "OniAnvil-UpdateChecker".to_string());
            request
                .headers
                .insert("Accept".to_string(), "application/vnd.github+json".to_string());

            ehttp::fetch(request, move |response| {
                if let Ok(resp) = response {
                    if resp.status == 200 {
                        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&resp.bytes) {
                            let tag = json
                                .get("tag_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default();
                            let html_url = json
                                .get("html_url")
                                .and_then(|v| v.as_str())
                                .unwrap_or(GITHUB_REPO_URL);

                            if is_newer_version(env!("CARGO_PKG_VERSION"), tag) {
                                if let Ok(mut guard) = result_ref.lock() {
                                    *guard = Some(UpdateInfo {
                                        tag_name: tag.to_string(),
                                        release_url: html_url.to_string(),
                                    });
                                }
                                ctx.request_repaint();
                            }
                        }
                    }
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
        }

        checker
    }

    pub fn available_update(&self) -> Option<UpdateInfo> {
        self.result.lock().ok().and_then(|g| g.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_semver(version: &str) -> (u32, u32, u32) {
    let trimmed = version.trim().trim_start_matches(['v', 'V']);
    let mut parts = trimmed.split(['.', '-']);
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_newer_version(current: &str, candidate: &str) -> bool {
    if candidate.is_empty() {
        return false;
    }
    parse_semver(candidate) > parse_semver(current)
}

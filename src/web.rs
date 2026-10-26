use crate::i18n::Language;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{LazyLock, Mutex};

type PendingFontList = Vec<(String, Vec<u8>)>;
type InstalledFontMap = std::collections::BTreeMap<String, Vec<u8>>;

pub static PENDING_FONT_LOADS: LazyLock<Mutex<PendingFontList>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static INSTALLED_FONTS: LazyLock<Mutex<InstalledFontMap>> =
    LazyLock::new(|| Mutex::new(std::collections::BTreeMap::new()));

// Fetch states: 0 = not started, 1 = in progress, 2 = done
static CJK_FETCH_STATE: AtomicU8 = AtomicU8::new(0);
static GEORGIAN_FETCH_STATE: AtomicU8 = AtomicU8::new(0);
static ARMENIAN_FETCH_STATE: AtomicU8 = AtomicU8::new(0);
static EXTENDED_FETCH_STATE: AtomicU8 = AtomicU8::new(0);

pub fn is_cjk_font_fetching_web() -> bool {
    CJK_FETCH_STATE.load(Ordering::SeqCst) == 1
}

pub fn is_cjk_font_loaded_web() -> bool {
    CJK_FETCH_STATE.load(Ordering::SeqCst) == 2
}

pub fn is_georgian_font_fetching_web() -> bool {
    GEORGIAN_FETCH_STATE.load(Ordering::SeqCst) == 1
}

pub fn is_georgian_font_loaded_web() -> bool {
    GEORGIAN_FETCH_STATE.load(Ordering::SeqCst) == 2
}

pub fn is_armenian_font_fetching_web() -> bool {
    ARMENIAN_FETCH_STATE.load(Ordering::SeqCst) == 1
}

pub fn is_armenian_font_loaded_web() -> bool {
    ARMENIAN_FETCH_STATE.load(Ordering::SeqCst) == 2
}

pub fn is_extended_font_fetching_web() -> bool {
    EXTENDED_FETCH_STATE.load(Ordering::SeqCst) == 1
}

pub fn is_extended_font_loaded_web() -> bool {
    EXTENDED_FETCH_STATE.load(Ordering::SeqCst) == 2
}

pub fn queue_pending_font(name: String, bytes: Vec<u8>) {
    if let Ok(mut lock) = PENDING_FONT_LOADS.lock() {
        lock.push((name, bytes));
    }
}

pub fn take_pending_fonts() -> Vec<(String, Vec<u8>)> {
    if let Ok(mut guard) = PENDING_FONT_LOADS.lock() {
        guard.drain(..).collect()
    } else {
        Vec::new()
    }
}

pub fn set_installed_font(name: String, bytes: Vec<u8>) {
    if let Ok(mut guard) = INSTALLED_FONTS.lock() {
        guard.insert(name, bytes);
    }
}

pub fn remove_installed_font(name: &str) {
    if let Ok(mut guard) = INSTALLED_FONTS.lock() {
        guard.remove(name);
    }
}

pub fn get_all_installed_fonts() -> Vec<(String, Vec<u8>)> {
    INSTALLED_FONTS
        .lock()
        .ok()
        .map(|guard| guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}

fn fetch_font_from_urls(
    font_name: &'static str,
    state: &'static AtomicU8,
    urls: &'static [&'static str],
) {
    if state
        .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    try_fetch_next_url(font_name, state, urls, 0);
}

fn try_fetch_next_url(
    font_name: &'static str,
    state: &'static AtomicU8,
    urls: &'static [&'static str],
    idx: usize,
) {
    if idx >= urls.len() {
        state.store(0, Ordering::SeqCst);
        return;
    }
    let url = urls[idx];
    let req = ehttp::Request::get(url);
    ehttp::fetch(req, move |result| match result {
        Ok(resp) if resp.ok && !resp.bytes.is_empty() => {
            state.store(2, Ordering::SeqCst);
            queue_pending_font(font_name.to_string(), resp.bytes);
        }
        _ => {
            try_fetch_next_url(font_name, state, urls, idx + 1);
        }
    });
}

/// Asynchronously fetches Noto Sans CJK font bytes from CDN when CJK language is selected.
pub fn fetch_cjk_font_web() {
    static URLS: &[&str] = &[
        "https://cdn.jsdelivr.net/gh/googlefonts/noto-cjk@main/Sans/OTF/SimplifiedChinese/NotoSansCJKsc-Regular.otf",
        "https://raw.githubusercontent.com/googlefonts/noto-cjk/main/Sans/OTF/SimplifiedChinese/NotoSansCJKsc-Regular.otf",
    ];
    fetch_font_from_urls("cjk_noto_web", &CJK_FETCH_STATE, URLS);
}

/// Asynchronously fetches Noto Sans Georgian font bytes when Georgian (ka) language is selected.
pub fn fetch_georgian_font_web() {
    static URLS: &[&str] = &[
        "fonts/NotoSansGeorgian-Regular.ttf",
        "https://raw.githubusercontent.com/googlefonts/noto-fonts/main/hinted/ttf/NotoSansGeorgian/NotoSansGeorgian-Regular.ttf",
        "https://cdn.jsdelivr.net/gh/googlefonts/noto-fonts@main/hinted/ttf/NotoSansGeorgian/NotoSansGeorgian-Regular.ttf",
    ];
    fetch_font_from_urls("noto_sans_georgian", &GEORGIAN_FETCH_STATE, URLS);
}

/// Asynchronously fetches Noto Sans Armenian font bytes when Armenian (hy) language is selected.
pub fn fetch_armenian_font_web() {
    static URLS: &[&str] = &[
        "fonts/NotoSansArmenian-Regular.ttf",
        "https://raw.githubusercontent.com/googlefonts/noto-fonts/main/hinted/ttf/NotoSansArmenian/NotoSansArmenian-Regular.ttf",
    ];
    fetch_font_from_urls("noto_sans_armenian", &ARMENIAN_FETCH_STATE, URLS);
}

/// Asynchronously fetches Noto Sans font bytes when Azerbaijani (az) or Uzbek (uz) language is selected.
pub fn fetch_extended_font_web() {
    static URLS: &[&str] = &[
        "fonts/NotoSans-Regular.ttf",
        "https://raw.githubusercontent.com/googlefonts/noto-fonts/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf",
        "https://cdn.jsdelivr.net/gh/googlefonts/noto-fonts@main/hinted/ttf/NotoSans/NotoSans-Regular.ttf",
    ];
    fetch_font_from_urls("noto_sans_ext", &EXTENDED_FETCH_STATE, URLS);
}

/// Preloads all web fonts (Asian/CJK, Georgian, Armenian, and Azerbaijani/Uzbek) for WebAssembly.
pub fn preload_web_fonts() {
    fetch_cjk_font_web();
    fetch_georgian_font_web();
    fetch_armenian_font_web();
    fetch_extended_font_web();
}

/// Checks if the current language requires a dynamic font on WebAssembly,
/// and starts fetching it if not yet loaded.
/// Returns true if any font is currently being fetched (useful for requesting repaint).
pub fn check_and_fetch_web_fonts(lang: Language) -> bool {
    let mut fetching = false;
    if matches!(lang, Language::Ja | Language::ZhCn | Language::ZhTw | Language::Ko) {
        if is_cjk_font_fetching_web() {
            fetching = true;
        } else if !is_cjk_font_loaded_web() {
            fetch_cjk_font_web();
            fetching = true;
        }
    } else if lang == Language::Ka {
        if is_georgian_font_fetching_web() {
            fetching = true;
        } else if !is_georgian_font_loaded_web() {
            fetch_georgian_font_web();
            fetching = true;
        }
    } else if lang == Language::Hy {
        if is_armenian_font_fetching_web() {
            fetching = true;
        } else if !is_armenian_font_loaded_web() {
            fetch_armenian_font_web();
            fetching = true;
        }
    } else if matches!(lang, Language::Az | Language::Uz) {
        if is_extended_font_fetching_web() {
            fetching = true;
        } else if !is_extended_font_loaded_web() {
            fetch_extended_font_web();
            fetching = true;
        }
    }
    fetching
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_fonts_files_exist_and_non_empty() {
        let ka_bytes = std::fs::read("web/fonts/NotoSansGeorgian-Regular.ttf")
            .expect("web/fonts/NotoSansGeorgian-Regular.ttf must exist");
        assert!(ka_bytes.len() > 10_000);

        let hy_bytes = std::fs::read("web/fonts/NotoSansArmenian-Regular.ttf")
            .expect("web/fonts/NotoSansArmenian-Regular.ttf must exist");
        assert!(hy_bytes.len() > 10_000);

        let ext_bytes = std::fs::read("web/fonts/NotoSans-Regular.ttf")
            .expect("web/fonts/NotoSans-Regular.ttf must exist");
        assert!(ext_bytes.len() > 10_000);
    }

    #[test]
    fn test_font_queue_operations() {
        let dummy = vec![1, 2, 3, 4];
        queue_pending_font("test_font".to_string(), dummy.clone());
        let pending = take_pending_fonts();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, "test_font");
        assert_eq!(pending[0].1, dummy);

        set_installed_font("test_font".to_string(), dummy.clone());
        let installed = get_all_installed_fonts();
        assert!(installed.iter().any(|(k, v)| k == "test_font" && v == &dummy));
        remove_installed_font("test_font");
        assert!(!get_all_installed_fonts().iter().any(|(k, _)| k == "test_font"));
    }

    #[test]
    fn test_check_and_fetch_web_fonts_languages() {
        // Calling check_and_fetch_web_fonts for different languages returns bool
        let _ = check_and_fetch_web_fonts(Language::Ka);
        let _ = check_and_fetch_web_fonts(Language::Hy);
        let _ = check_and_fetch_web_fonts(Language::Az);
        let _ = check_and_fetch_web_fonts(Language::Ja);
        let _ = check_and_fetch_web_fonts(Language::En);
    }
}

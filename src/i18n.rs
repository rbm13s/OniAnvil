use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en", alias = "English")]
    En,
    #[serde(rename = "ru", alias = "Russian")]
    Ru,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "zh-cn", alias = "zh")]
    ZhCn,
    #[serde(rename = "zh-tw")]
    ZhTw,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
}

#[allow(non_upper_case_globals)]
impl Language {
    pub const English: Self = Self::En;
    pub const Russian: Self = Self::Ru;

    pub const ALL: [Language; 37] = [
        Language::En,
        Language::Ru,
        Language::Uk,
        Language::Be,
        Language::Kk,
        Language::Ka,
        Language::Hy,
        Language::Az,
        Language::Uz,
        Language::De,
        Language::Fr,
        Language::Es,
        Language::It,
        Language::Pt,
        Language::Pl,
        Language::Cs,
        Language::Sk,
        Language::Sl,
        Language::Bg,
        Language::Sr,
        Language::Hr,
        Language::Nl,
        Language::Da,
        Language::No,
        Language::Sv,
        Language::Fi,
        Language::Et,
        Language::Lv,
        Language::Lt,
        Language::Hu,
        Language::Ro,
        Language::Tr,
        Language::El,
        Language::ZhCn,
        Language::ZhTw,
        Language::Ja,
        Language::Ko,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::Ru => "RU",
            Self::Uk => "UK",
            Self::Be => "BE",
            Self::Kk => "KK",
            Self::Ka => "KA",
            Self::Hy => "HY",
            Self::Az => "AZ",
            Self::Uz => "UZ",
            Self::De => "DE",
            Self::Fr => "FR",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Pt => "PT",
            Self::Pl => "PL",
            Self::Cs => "CS",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Bg => "BG",
            Self::Sr => "SR",
            Self::Hr => "HR",
            Self::Nl => "NL",
            Self::Da => "DA",
            Self::No => "NO",
            Self::Sv => "SV",
            Self::Fi => "FI",
            Self::Et => "ET",
            Self::Lv => "LV",
            Self::Lt => "LT",
            Self::Hu => "HU",
            Self::Ro => "RO",
            Self::Tr => "TR",
            Self::El => "EL",
            Self::ZhCn => "ZH",
            Self::ZhTw => "ZH-TW",
            Self::Ja => "JA",
            Self::Ko => "KO",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ru => "Русский (Russian)",
            Self::Uk => "Українська (Ukrainian)",
            Self::Be => "Беларуская (Belarusian)",
            Self::Kk => "Қазақша (Kazakh)",
            Self::Ka => "ქართული (Georgian)",
            Self::Hy => "Հայերեն (Armenian)",
            Self::Az => "Azərbaycan (Azerbaijani)",
            Self::Uz => "Oʻzbekcha (Uzbek)",
            Self::De => "Deutsch (German)",
            Self::Fr => "Français (French)",
            Self::Es => "Español (Spanish)",
            Self::It => "Italiano (Italian)",
            Self::Pt => "Português (Portuguese)",
            Self::Pl => "Polski (Polish)",
            Self::Cs => "Čeština (Czech)",
            Self::Sk => "Slovenčina (Slovak)",
            Self::Sl => "Slovenščina (Slovenian)",
            Self::Bg => "Български (Bulgarian)",
            Self::Sr => "Српски (Serbian)",
            Self::Hr => "Hrvatski (Croatian)",
            Self::Nl => "Nederlands (Dutch)",
            Self::Da => "Dansk (Danish)",
            Self::No => "Norsk (Bokmål)",
            Self::Sv => "Svenska (Swedish)",
            Self::Fi => "Suomi (Finnish)",
            Self::Et => "Eesti (Estonian)",
            Self::Lv => "Latviešu (Latvian)",
            Self::Lt => "Lietuvių (Lithuanian)",
            Self::Hu => "Magyar (Hungarian)",
            Self::Ro => "Română (Romanian)",
            Self::Tr => "Türkçe (Turkish)",
            Self::El => "Ελληνικά (Greek)",
            Self::ZhCn => "简体中文 (Chinese Simpl.)",
            Self::ZhTw => "繁體中文 (Chinese Trad.)",
            Self::Ja => "日本語 (Japanese)",
            Self::Ko => "한국어 (Korean)",
        }
    }

    pub fn from_locale_tag(tag: &str) -> Option<Self> {
        let clean = tag.trim().to_ascii_lowercase();
        if clean.is_empty() || clean == "c" || clean == "posix" {
            return None;
        }
        if clean == "zh-tw"
            || clean == "zh_tw"
            || clean == "zh-hk"
            || clean == "zh_hk"
            || clean.contains("hant")
        {
            return Some(Self::ZhTw);
        }
        let primary = clean.split(['-', '_', '.', '@']).next().unwrap_or("");
        match primary {
            "en" => Some(Self::En),
            "ru" => Some(Self::Ru),
            "uk" => Some(Self::Uk),
            "be" => Some(Self::Be),
            "kk" => Some(Self::Kk),
            "ka" | "geo" => Some(Self::Ka),
            "hy" | "arm" => Some(Self::Hy),
            "az" => Some(Self::Az),
            "uz" => Some(Self::Uz),
            "de" => Some(Self::De),
            "fr" => Some(Self::Fr),
            "es" => Some(Self::Es),
            "it" => Some(Self::It),
            "pt" => Some(Self::Pt),
            "pl" => Some(Self::Pl),
            "cs" | "cz" => Some(Self::Cs),
            "sk" => Some(Self::Sk),
            "sl" => Some(Self::Sl),
            "bg" => Some(Self::Bg),
            "sr" => Some(Self::Sr),
            "hr" => Some(Self::Hr),
            "nl" => Some(Self::Nl),
            "da" => Some(Self::Da),
            "no" | "nb" | "nn" => Some(Self::No),
            "sv" | "se" => Some(Self::Sv),
            "fi" => Some(Self::Fi),
            "et" | "ee" => Some(Self::Et),
            "lv" => Some(Self::Lv),
            "lt" => Some(Self::Lt),
            "hu" => Some(Self::Hu),
            "ro" => Some(Self::Ro),
            "tr" => Some(Self::Tr),
            "el" | "gr" => Some(Self::El),
            "zh" => Some(Self::ZhCn),
            "ja" | "jp" => Some(Self::Ja),
            "ko" | "kr" => Some(Self::Ko),
            _ => None,
        }
    }

    pub fn detect_system_or_en() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(search) = window.location().search() {
                    let search = search.trim_start_matches('?');
                    for pair in search.split('&') {
                        let mut parts = pair.split('=');
                        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                            if k.eq_ignore_ascii_case("lang") {
                                if let Some(lang) = Self::from_locale_tag(v) {
                                    return lang;
                                }
                            }
                        }
                    }
                }
                let nav = window.navigator();
                if let Some(lang_tag) = nav.language() {
                    if let Some(lang) = Self::from_locale_tag(&lang_tag) {
                        return lang;
                    }
                }
            }
            Self::En
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            for var in ["ONIANVIL_LANG", "LC_ALL", "LC_MESSAGES", "LANG"] {
                if let Ok(val) = std::env::var(var) {
                    if let Some(lang) = Self::from_locale_tag(&val) {
                        return lang;
                    }
                }
            }

            #[cfg(target_os = "windows")]
            {
                #[link(name = "kernel32")]
                extern "system" {
                    fn GetUserPreferredUILanguages(
                        dw_flags: u32,
                        pul_num_languages: *mut u32,
                        pwsz_languages_buffer: *mut u16,
                        pcch_languages_buffer: *mut u32,
                    ) -> i32;
                    fn GetUserDefaultLocaleName(lp_locale_name: *mut u16, cch_locale_name: i32) -> i32;
                }

                let mut num_langs = 0u32;
                let mut buf = [0u16; 256];
                let mut buf_len = buf.len() as u32;
                let ok = unsafe {
                    GetUserPreferredUILanguages(0x8, &mut num_langs, buf.as_mut_ptr(), &mut buf_len)
                };
                if ok != 0 && buf_len > 1 {
                    let slice = &buf[..(buf_len as usize)];
                    for part in slice.split(|&ch| ch == 0) {
                        if !part.is_empty() {
                            let tag = String::from_utf16_lossy(part);
                            if let Some(lang) = Self::from_locale_tag(&tag) {
                                return lang;
                            }
                        }
                    }
                }

                let mut loc_buf = [0u16; 85];
                let len = unsafe {
                    GetUserDefaultLocaleName(loc_buf.as_mut_ptr(), loc_buf.len() as i32)
                };
                if len > 1 {
                    let tag = String::from_utf16_lossy(&loc_buf[..(len as usize - 1)]);
                    if let Some(lang) = Self::from_locale_tag(&tag) {
                        return lang;
                    }
                }
            }

            Self::En
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::detect_system_or_en()
    }
}

pub fn load_locale_json(lang: Language) -> &'static str {
    match lang {
        Language::En => include_str!("../assets/locales/en.json"),
        Language::Ru => include_str!("../assets/locales/ru.json"),
        Language::Uk => include_str!("../assets/locales/uk.json"),
        Language::Be => include_str!("../assets/locales/be.json"),
        Language::Kk => include_str!("../assets/locales/kk.json"),
        Language::Ka => include_str!("../assets/locales/ka.json"),
        Language::Hy => include_str!("../assets/locales/hy.json"),
        Language::Az => include_str!("../assets/locales/az.json"),
        Language::Uz => include_str!("../assets/locales/uz.json"),
        Language::De => include_str!("../assets/locales/de.json"),
        Language::Fr => include_str!("../assets/locales/fr.json"),
        Language::Es => include_str!("../assets/locales/es.json"),
        Language::It => include_str!("../assets/locales/it.json"),
        Language::Pt => include_str!("../assets/locales/pt.json"),
        Language::Pl => include_str!("../assets/locales/pl.json"),
        Language::Cs => include_str!("../assets/locales/cs.json"),
        Language::Sk => include_str!("../assets/locales/sk.json"),
        Language::Sl => include_str!("../assets/locales/sl.json"),
        Language::Bg => include_str!("../assets/locales/bg.json"),
        Language::Sr => include_str!("../assets/locales/sr.json"),
        Language::Hr => include_str!("../assets/locales/hr.json"),
        Language::Nl => include_str!("../assets/locales/nl.json"),
        Language::Da => include_str!("../assets/locales/da.json"),
        Language::No => include_str!("../assets/locales/no.json"),
        Language::Sv => include_str!("../assets/locales/sv.json"),
        Language::Fi => include_str!("../assets/locales/fi.json"),
        Language::Et => include_str!("../assets/locales/et.json"),
        Language::Lv => include_str!("../assets/locales/lv.json"),
        Language::Lt => include_str!("../assets/locales/lt.json"),
        Language::Hu => include_str!("../assets/locales/hu.json"),
        Language::Ro => include_str!("../assets/locales/ro.json"),
        Language::Tr => include_str!("../assets/locales/tr.json"),
        Language::El => include_str!("../assets/locales/el.json"),
        Language::ZhCn => include_str!("../assets/locales/zh-cn.json"),
        Language::ZhTw => include_str!("../assets/locales/zh-tw.json"),
        Language::Ja => include_str!("../assets/locales/ja.json"),
        Language::Ko => include_str!("../assets/locales/ko.json"),
    }
}

pub struct I18n {
    current: Language,
    en_map: HashMap<String, String>,
    active_map: HashMap<String, String>,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new(Language::En)
    }
}

impl I18n {
    pub fn new(initial: Language) -> Self {
        let en_map: HashMap<String, String> =
            serde_json::from_str(include_str!("../assets/locales/en.json"))
                .expect("Failed to parse assets/locales/en.json");

        let active_map = if initial == Language::En {
            en_map.clone()
        } else {
            serde_json::from_str(load_locale_json(initial)).unwrap_or_else(|_| en_map.clone())
        };

        Self {
            current: initial,
            en_map,
            active_map,
        }
    }

    pub fn language(&self) -> Language {
        self.current
    }

    pub fn set_language(&mut self, language: Language) {
        if self.current != language {
            self.current = language;
            if language == Language::En {
                self.active_map = self.en_map.clone();
            } else {
                self.active_map = serde_json::from_str(load_locale_json(language))
                    .unwrap_or_else(|_| self.en_map.clone());
            }
        }
    }

    pub fn tr<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(val) = self.active_map.get(key) {
            val.as_str()
        } else if let Some(fallback) = self.en_map.get(key) {
            fallback.as_str()
        } else {
            key
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_dictionaries_key_parity() {
        let i18n = I18n::new(Language::En);
        assert!(!i18n.en_map.is_empty());

        for lang in Language::ALL {
            let json_str = load_locale_json(lang);
            let map: HashMap<String, String> = serde_json::from_str(json_str)
                .unwrap_or_else(|e| panic!("Failed to parse locale json for {:?}: {:?}", lang, e));
            assert_eq!(
                i18n.en_map.len(),
                map.len(),
                "Locale {:?} must have the exact same number of keys as en.json",
                lang
            );
            for key in i18n.en_map.keys() {
                assert!(
                    map.contains_key(key),
                    "Missing key in {:?}: {}",
                    lang,
                    key
                );
            }
        }
    }
}

use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use toml::Value;
use tracing::{debug, error};

#[derive(RustEmbed)]
#[folder = "assets/i18n/"]
struct Asset;

// Global storage for translations: "key" -> "translation"
// We flatten nested TOML: "section.key"
static TRANSLATIONS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

static CURRENT_LANG: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("en".to_string())
});

pub fn normalize_language_code(lang: &str) -> String {
    let lower = lang.to_lowercase();
    let lower = lower.replace("_", "-");
    
    // Exact matches or starts with for common variations
    if lower == "en" || lower.starts_with("en-") {
        return "en".to_string();
    }
    if lower == "zh-tw" || lower == "zh-hk" || lower == "zh-mo" || lower.starts_with("zh-hant") {
        return "zh-TW".to_string();
    }
    if lower == "zh" || lower == "zh-cn" || lower == "zh-sg" || lower.starts_with("zh-hans") {
        return "zh-CN".to_string();
    }
    if lower == "fr" || lower.starts_with("fr-") {
        return "fr".to_string();
    }
    if lower == "es" || lower.starts_with("es-") {
        return "es".to_string();
    }
    if lower == "ru" || lower.starts_with("ru-") {
        return "ru".to_string();
    }
    if lower == "pt" || lower.starts_with("pt-") {
        return "pt".to_string();
    }
    if lower == "de" || lower.starts_with("de-") {
        return "de".to_string();
    }
    if lower == "ja" || lower.starts_with("ja-") {
        return "ja".to_string();
    }
    if lower == "hi" || lower.starts_with("hi-") {
        return "hi".to_string();
    }
    if lower == "bn" || lower.starts_with("bn-") {
        return "bn".to_string();
    }
    if lower == "id" || lower.starts_with("id-") {
        return "id".to_string();
    }
    if lower == "it" || lower.starts_with("it-") {
        return "it".to_string();
    }
    if lower == "tr" || lower.starts_with("tr-") {
        return "tr".to_string();
    }
    
    // Default fallback
    "en".to_string()
}

#[allow(dead_code)]
pub fn current_lang() -> String {
    CURRENT_LANG.lock().unwrap().clone()
}

pub fn load_resources(lang_code: &str) {
    let normalized = normalize_language_code(lang_code);
    let mut map = TRANSLATIONS.lock().unwrap();
    map.clear();
    
    // 1. Load English (Base)
    load_file_to_map("en", &mut map);
    
    // 2. Load Target (if not en)
    if normalized != "en" {
        load_file_to_map(&normalized, &mut map);
    }
    
    println!("i18n: Map populated with {} keys", map.len());
    *CURRENT_LANG.lock().unwrap() = normalized;
}

fn load_file_to_map(lang: &str, map: &mut HashMap<String, String>) {
    let filename = format!("{}.toml", lang);

    let content = if cfg!(debug_assertions) {
        let path = std::path::Path::new("assets/i18n").join(&filename);
        if let Ok(c) = std::fs::read_to_string(&path) {
            debug!("i18n: loaded {} from filesystem", path.display());
            Some(c)
        } else {
            debug!("i18n: failed to load {} from filesystem, falling back to embedded", path.display());
            Asset::get(&filename).and_then(|f| std::str::from_utf8(f.data.as_ref()).ok().map(|s| s.to_string()))
        }
    } else {
        Asset::get(&filename).and_then(|f| std::str::from_utf8(f.data.as_ref()).ok().map(|s| s.to_string()))
    };

    if let Some(mut content) = content {
        // Remove BOM if present
        if content.starts_with('\u{feff}') {
            content.remove(0);
        }

        match toml::from_str::<toml::Table>(&content) {
            Ok(table) => {
                flatten_toml("", &Value::Table(table), map);
            }
            Err(e) => {
                error!("i18n: failed to parse TOML for {}: {}", lang, e);
                // Try parsing as generic Value if Table fail (though shouldn't happen for TOML)
                if let Ok(v) = content.parse::<Value>() {
                     flatten_toml("", &v, map);
                }
            }
        }
    } else {
        error!("i18n: content not found for {}", lang);
    }
}


fn flatten_toml(prefix: &str, value: &Value, map: &mut HashMap<String, String>) {
    match value {
        Value::Table(table) => {
            for (k, v) in table {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml(&new_key, v, map);
            }
        }
        Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        _ => {} // Ignore other types for now
    }
}

pub fn t(key: &str) -> String {
    let map = TRANSLATIONS.lock().unwrap();
    map.get(key).cloned().unwrap_or_else(|| {
        key.to_string()
    })
}

pub fn tr(key: &str, args: &[String]) -> String {
    let mut text = t(key);
    for (i, arg) in args.iter().enumerate() {
        text = text.replace(&format!("{{{}}}", i), arg);
    }
    text
}

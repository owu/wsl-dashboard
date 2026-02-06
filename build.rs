#[path = "src/app/constants.rs"]
mod constants;

use std::fs;
use std::path::Path;
use toml::Value;
use std::collections::HashSet;

fn main() {
    slint_build::compile("src/ui/app.slint").expect("Slint compilation failed");

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=assets/i18n");
    println!("cargo:rerun-if-changed=src/ui/app.slint");

    // Read Cargo.toml to get custom 'expire' field
    let cargo_toml_path = Path::new("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    let cargo_toml: Value = toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    let mut expire_time = cargo_toml
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("expire"))
        .and_then(|e| e.as_integer())
        .unwrap_or(0);

    // If expire is 0, set it to 1 year from now
    if expire_time == 0 && cargo_toml_content.contains("expire = 0") {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;
            
        // 365 days later
        expire_time = now + (365 * 24 * 60 * 60 * 1000);
    }

    println!("cargo:rustc-env=APP_EXPIRE_TIME={}", expire_time);


    #[cfg(windows)]
    {
        use image::ImageReader;

        let png_path = Path::new("assets/logo/logo.png");
        let ico_path = Path::new("assets/logo/logo.ico");

        if png_path.exists() {
            let img = ImageReader::open(png_path)
                .expect("Failed to open PNG file")
                .decode()
                .expect("Failed to decode PNG");

            use image::imageops::FilterType;
            let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
            let sizes = [16, 32, 48, 64, 128, 256];
            
            for &size in &sizes {
                let resized = img.resize_exact(size, size, FilterType::Lanczos3);
                let rgba = resized.to_rgba8();
                let icon_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
                let entry = ico::IconDirEntry::encode(&icon_image).expect("Failed to encode icon image");
                icon_dir.add_entry(entry);
            }

            let file = std::fs::File::create(ico_path).expect("Failed to create ICO file");
            icon_dir.write(file).expect("Failed to write ICO file");

            let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.1".to_string());
            let mut version_parts = version.split('.').map(|p| p.parse::<u16>().unwrap_or(0)).collect::<Vec<_>>();
            while version_parts.len() < 3 {
                version_parts.push(0);
            }
            let (major, minor, patch) = (version_parts[0], version_parts[1], version_parts[2]);

            let icon_rc_path = Path::new("assets/logo/icon.rc");
            let file_description = format!("{} - Management Tool for WSL", constants::APP_NAME);
            let original_filename = format!("{}.exe", constants::APP_ID);

            std::fs::write(
                icon_rc_path,
                format!(r#"#include <windows.h>

1 ICON "logo.ico"

VS_VERSION_INFO VERSIONINFO
 FILEVERSION {major},{minor},{patch},0
 PRODUCTVERSION {major},{minor},{patch},0
 FILEFLAGSMASK 0x3fL
#ifdef _DEBUG
 FILEFLAGS 0x1L
#else
 FILEFLAGS 0x0L
#endif
 FILEOS 0x40004L
 FILETYPE 0x1L
 FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName", "{company_name}"
            VALUE "FileDescription", "{file_description}"
            VALUE "FileVersion", "{major}.{minor}.{patch}.0"
            VALUE "InternalName", "{app_id}"
            VALUE "LegalCopyright", "{copyright}"
            VALUE "LegalTrademarks", "{github_url}"
            VALUE "OriginalFilename", "{original_filename}"
            VALUE "ProductName", "{app_name}"
            VALUE "ProductVersion", "{major}.{minor}.{patch}.0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1200
    END
END
"#, 
    company_name = constants::COMPANY_NAME,
    file_description = file_description,
    app_id = constants::APP_ID,
    copyright = constants::LEGAL_COPYRIGHT,
    github_url = constants::GITHUB_URL,
    original_filename = original_filename,
    app_name = constants::APP_NAME,
    major = major,
    minor = minor,
    patch = patch
)
            ).expect("Failed to write icon.rc");

            embed_resource::compile(icon_rc_path, std::iter::empty::<&std::ffi::OsStr>());
        }
    }

    // i18n Integrity Check
    verify_translations();
}

fn verify_translations() {
    let i18n_dir = Path::new("assets/i18n");
    if !i18n_dir.exists() { return; }

    let en_path = i18n_dir.join("en.toml");
    if !en_path.exists() { return; }

    let en_content = fs::read_to_string(&en_path).unwrap_or_default();
    let en_toml: Value = toml::from_str(&en_content).unwrap_or(Value::Table(Default::default()));
    
    let mut en_keys = HashSet::new();
    flatten_keys("", &en_toml, &mut en_keys);

    println!("cargo:warning=--- i18n Integrity Check (Base: en.toml) ---");
    
    if let Ok(entries) = fs::read_dir(i18n_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if filename == "en.toml" { continue; }

                let content = fs::read_to_string(&path).unwrap_or_default();
                let toml_val: Value = toml::from_str(&content).unwrap_or(Value::Table(Default::default()));
                
                let mut lang_keys = HashSet::new();
                flatten_keys("", &toml_val, &mut lang_keys);

                let mut missing = Vec::new();
                for key in &en_keys {
                    if !lang_keys.contains(key) {
                        missing.push(key);
                    }
                }

                if !missing.is_empty() {
                    println!("cargo:warning=[!] Language '{}' is missing {} keys:", filename, missing.len());
                    for key in missing {
                        println!("cargo:warning=    - {}", key);
                    }
                }
            }
        }
    }
    println!("cargo:warning=------------------------------------------");
}

fn flatten_keys(prefix: &str, value: &Value, keys: &mut HashSet<String>) {
    match value {
        Value::Table(table) => {
            for (k, v) in table {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_keys(&new_key, v, keys);
            }
        }
        Value::String(_) | Value::Integer(_) | Value::Float(_) | Value::Boolean(_) | Value::Datetime(_) => {
            if !prefix.is_empty() {
                keys.insert(prefix.to_string());
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_key = format!("{}[{}]", prefix, i);
                flatten_keys(&new_key, v, keys);
            }
        }
    }
}

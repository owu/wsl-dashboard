use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegEnumKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, HKEY,
    KEY_READ, REG_SZ, REG_DWORD, REG_VALUE_TYPE, REG_OPEN_CREATE_OPTIONS
};
use windows::core::{PCWSTR, PWSTR};

pub struct WslRegInfo {
    pub name: String,
    pub base_path: String,
    pub version: u32,
    pub package_family_name: String,
}

pub fn get_wsl_distros_from_reg() -> Vec<WslRegInfo> {
    let mut distros = Vec::new();
    let subkey = encode_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Lxss");
    
    let mut hkey = HKEY::default();
    unsafe {
        if RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut hkey).is_err() {
            return distros;
        }
        
        let mut index = 0;
        let mut name_buf = [0u16; 256];
        loop {
            let mut name_len = name_buf.len() as u32;
            if RegEnumKeyExW(hkey, index, PWSTR(name_buf.as_mut_ptr()), &mut name_len, None, PWSTR::null(), None, None).is_err() {
                break;
            }
            
            let guid = String::from_utf16_lossy(&name_buf[..name_len as usize]);
            if let Some(info) = get_distro_details_by_guid(hkey, &guid) {
                distros.push(info);
            }
            
            index += 1;
        }
    }
    distros
}

fn get_distro_details_by_guid(parent_hkey: HKEY, guid: &str) -> Option<WslRegInfo> {
    let mut sub_hkey = HKEY::default();
    let guid_wide = encode_wide(guid);
    
    unsafe {
        if RegOpenKeyExW(parent_hkey, PCWSTR(guid_wide.as_ptr()), 0, KEY_READ, &mut sub_hkey).is_err() {
            return None;
        }
        
        let name = read_reg_string(sub_hkey, "DistributionName").unwrap_or_default();
        let base_path = read_reg_string(sub_hkey, "BasePath").unwrap_or_default();
        let pfn = read_reg_string(sub_hkey, "PackageFamilyName").unwrap_or_default();
        let version = read_reg_dword(sub_hkey, "Version").unwrap_or(1);
        
        if name.is_empty() {
            return None;
        }
        
        Some(WslRegInfo {
            name,
            base_path,
            version,
            package_family_name: pfn,
        })
    }
}

pub fn read_reg_string(hkey: HKEY, value_name: &str) -> Option<String> {
    let value_name_wide = encode_wide(value_name);
    let mut buf = [0u8; 1024];
    let mut buf_len = buf.len() as u32;
    let mut value_type = REG_VALUE_TYPE::default();
    
    unsafe {
        if RegQueryValueExW(hkey, PCWSTR(value_name_wide.as_ptr()), None, Some(&mut value_type), Some(buf.as_mut_ptr()), Some(&mut buf_len)).is_err() {
            return None;
        }
        
        if value_type == REG_SZ {
            let u16_data = std::slice::from_raw_parts(buf.as_ptr() as *const u16, (buf_len / 2) as usize);
            // Remove trailing null
            let end = u16_data.iter().position(|&c| c == 0).unwrap_or(u16_data.len());
            Some(String::from_utf16_lossy(&u16_data[..end]))
        } else {
            None
        }
    }
}

pub fn read_reg_dword(hkey: HKEY, value_name: &str) -> Option<u32> {
    let value_name_wide = encode_wide(value_name);
    let mut data = 0u32;
    let mut data_len = std::mem::size_of::<u32>() as u32;
    let mut value_type = REG_VALUE_TYPE::default();
    
    unsafe {
        if RegQueryValueExW(hkey, PCWSTR(value_name_wide.as_ptr()), None, Some(&mut value_type), Some(&mut data as *mut u32 as *mut u8), Some(&mut data_len)).is_err() {
            return None;
        }
        
        if value_type == REG_DWORD {
            Some(data)
        } else {
            None
        }
    }
}

pub fn get_system_locale() -> String {
    let subkey = "Control Panel\\International";
    let mut hkey = HKEY::default();
    unsafe {
        if RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(encode_wide(subkey).as_ptr()), 0, KEY_READ, &mut hkey).is_ok() {
            if let Some(locale) = read_reg_string(hkey, "LocaleName") {
                return locale;
            }
        }
    }
    "en-US".to_string()
}

pub fn get_system_timezone() -> String {
    use chrono::{Local, Offset};
    let now = Local::now();
    let offset = now.offset().fix();
    let total_secs = offset.local_minus_utc();
    let sign = if total_secs >= 0 { "+" } else { "-" };
    let abs_secs = total_secs.abs();
    let hours = abs_secs / 3600;
    let mins = (abs_secs % 3600) / 60;
    format!("UTC{}{:02}:{:02}", sign, hours, mins)
}

pub fn write_reg_string(root: HKEY, subkey: &str, value_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subkey_wide = encode_wide(subkey);
    let value_name_wide = encode_wide(value_name);
    let value_wide = encode_wide(value);
    
    let mut hkey = HKEY::default();
    unsafe {
        use windows::Win32::System::Registry::{RegCreateKeyExW, RegSetValueExW, REG_SZ, KEY_WRITE};
        RegCreateKeyExW(root, PCWSTR(subkey_wide.as_ptr()), 0, None, REG_OPEN_CREATE_OPTIONS(0), KEY_WRITE, None, &mut hkey, None)
            .ok()
            .map_err(|e: windows::core::Error| e.to_string())?;
            
        let data = std::slice::from_raw_parts(value_wide.as_ptr() as *const u8, value_wide.len() * 2);
        RegSetValueExW(hkey, PCWSTR(value_name_wide.as_ptr()), 0, REG_SZ, Some(data))
            .ok()
            .map_err(|e: windows::core::Error| e.to_string())?;
    }
    Ok(())
}

pub fn delete_reg_value(root: HKEY, subkey: &str, value_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subkey_wide = encode_wide(subkey);
    let value_name_wide = encode_wide(value_name);
    
    let mut hkey = HKEY::default();
    unsafe {
        use windows::Win32::System::Registry::{RegOpenKeyExW, RegDeleteValueW, KEY_SET_VALUE};
        RegOpenKeyExW(root, PCWSTR(subkey_wide.as_ptr()), 0, KEY_SET_VALUE, &mut hkey)
            .ok()
            .map_err(|e: windows::core::Error| e.to_string())?;
            
        RegDeleteValueW(hkey, PCWSTR(value_name_wide.as_ptr()))
            .ok()
            .map_err(|e: windows::core::Error| e.to_string())?;
    }
    Ok(())
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

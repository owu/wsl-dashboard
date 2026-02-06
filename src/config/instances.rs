use std::path::Path;
use crate::config::models::{InstancesContainer, INSTANCES_VERSION};
use crate::config::migration;
use std::fs;

pub fn load_instances(path: &Path) -> InstancesContainer {
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(mut container) = toml::from_str::<InstancesContainer>(&content) {
                let old_version = container.common.setting_version;
                migration::migrate_instances_config(&mut container);
                
                // If version was upgraded, save it back immediately to complete fields
                if old_version < INSTANCES_VERSION {
                    let _ = save_instances_to_disk(path, &container);
                }
                return container;
            }
        }
    }
    InstancesContainer::new()
}

pub fn save_instances_to_disk(path: &Path, container: &InstancesContainer) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut toml_string = toml::to_string_pretty(container)?;
    // Ensure UNIX line endings 
    toml_string = toml_string.replace("\r\n", "\n");
    
    fs::write(path, toml_string)?;
    Ok(())
}

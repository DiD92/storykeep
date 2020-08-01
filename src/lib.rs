use std::env::current_dir;
use std::fs::write as write_to_file;
use std::path::Path;
use toml::to_string as toml_to_string;

mod config;
mod constants;
mod fs;

pub use config::{get_app_config, get_app_config_or_create, get_keep_config};

/// Tries to create a `.keep` folder in the current working directory.
///
/// The method will first check if there already exists a `.keep` folder
/// in the current path down to the root, if not it will try to create
/// the folder.
pub fn initialize_keep() -> Result<String, String> {
    match current_dir() {
        Ok(working_dir) => initialize_keep_at(working_dir.as_path()),
        Err(reason) => Err(reason.to_string()),
    }
}

pub fn initialize_keep_at(path: &Path) -> Result<String, String> {
    match fs::create_keep_folder_at(path) {
        Ok(keep_path) => {
            let keep_config = keep_path.to_path_buf().join(constants::KEEP_CONFIG_FILE);

            if let Some(app_config) = get_app_config() {
                match toml_to_string(&app_config.global_keep_config) {
                    Ok(content) => match write_to_file(keep_config.into_boxed_path(), &content) {
                        Ok(_) => Ok(format!("Keep created at {}", path.to_str().unwrap())),
                        Err(err) => Err(format!("{}", err)),
                    },
                    Err(_) => Err("Error initializing keep configuration!".into()),
                }
            } else {
                Err("Error initializing keep configuration!".into())
            }
        }
        Err(message) => Err(message),
    }
}

/// Checks if a keep can be created at the current working directory.
pub fn can_initialize_keep() -> bool {
    match fs::get_current_dir() {
        Some(path) => can_initialize_keep_at(path.as_ref()),
        None => false,
    }
}

pub fn can_initialize_keep_at(path: &Path) -> bool {
    !fs::contains_keep(path)
}

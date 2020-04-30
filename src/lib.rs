use std::env::current_dir as get_wd;
use std::fs::create_dir;
use std::path::Path;

mod constants;

pub fn initialize_keep() -> Result<String, String> {
    match create_keep_at_wd() {
        // TODO: Properly handle non UTF-8 paths
        Ok(path) => Ok(format!("Keep created at {}", path.to_str().unwrap())),
        Err(message) => Err(message),
    }
}

fn can_initialize_keep_at_wd() -> bool {
    if let Ok(current_path) = get_wd() {
        can_initialize_keep_at(current_path.as_path())
    } else {
        false
    }
}

fn can_initialize_keep_at(path: &Path) -> bool {
    let mut path = path.to_path_buf();
    if path.is_dir() {
        path.push(constants::KEEP_FOLDER);
        !path.is_dir()
    } else {
        false
    }
}

fn create_keep_at_wd() -> Result<Box<Path>, String> {
    match get_wd() {
        Ok(working_dir) => create_keep_at(working_dir.as_path()),
        Err(reason) => Err(reason.to_string()),
    }
}

fn create_keep_at(path: &Path) -> Result<Box<Path>, String> {
    if can_initialize_keep_at(path) {
        let mut keep_path = path.to_path_buf();
        keep_path.push(constants::KEEP_FOLDER);

        match create_dir(keep_path.as_path()) {
            Ok(_) => {
                keep_path.pop();
                Ok(keep_path.into_boxed_path())
            }
            Err(reason) => Err(reason.to_string()),
        }
    } else {
        Err(String::from(
            "Could not initialize keep! Check your current directory",
        ))
    }
}

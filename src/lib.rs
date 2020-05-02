use std::env::current_dir as get_wd;
use std::fs::create_dir;
use std::path::Path;

mod constants;

/// Tries to create a `.keep` folder in the current working directory.
///
/// The method will first check if there already exists a `.keep` folder
/// in the current path down to the root, if not it will try to create
/// the folder.
pub fn initialize_keep() -> Result<String, String> {
    match create_keep_at_wd() {
        // TODO: Properly handle non UTF-8 paths
        Ok(path) => Ok(format!("Keep created at {}", path.to_str().unwrap())),
        Err(message) => Err(message),
    }
}

/// Checks if a keep can be created at the current working directory.
pub fn can_initialize_keep_at_wd() -> bool {
    if let Ok(current_path) = get_wd() {
        can_initialize_keep_at(current_path.as_path())
    } else {
        false
    }
}

fn can_initialize_keep_at(path: &Path) -> bool {
    let mut path = path.to_path_buf();
    if path.is_dir() {
        loop {
            path.push(constants::KEEP_FOLDER);

            if path.is_dir() {
                return false;
            }

            path.pop();

            if path.pop() {
                // We've reached the root path with no keep found.
                return true;
            }
        }
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

use crate::constants::KEEP_FOLDER;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::Path;

pub fn get_current_dir<'a>() -> Option<Box<Path>> {
    if let Ok(current_path) = current_dir() {
        Some(current_path.into_boxed_path())
    } else {
        None
    }
}

pub fn contains_keep(path: &Path) -> bool {
    let mut path = path.to_path_buf();
    if path.is_dir() {
        loop {
            path.push(KEEP_FOLDER);

            if path.is_dir() {
                return true;
            }

            path.pop();

            if path.pop() {
                // We've reached the root path with no .keep directoty found.
                return false;
            }
        }
    }

    false
}

pub fn create_keep_folder_at(path: &Path) -> Result<Box<Path>, String> {
    if !contains_keep(path) {
        let mut keep_path = path.to_path_buf();
        keep_path.push(KEEP_FOLDER);

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

pub fn get_keep_base_dir() -> Option<Box<Path>> {
    if let Some(path) = get_current_dir() {
        let mut path = path.to_path_buf();

        loop {
            path.push(KEEP_FOLDER);

            if path.is_dir() {
                return Some(path.into_boxed_path());
            }

            path.pop();

            if path.pop() {
                return None;
            }
        }
    } else {
        None
    }
}

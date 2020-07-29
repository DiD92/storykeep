use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use toml::{de::Error, from_str as toml_from_str};
use crate::{fs::get_keep_base_dir, constants::KEEP_CONFIG_FILE};

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub author: Author,
    pub formatting: Formatting,
}

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    #[serde(rename = "pen-name")]
    pub pen_name: String,
}

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Formatting {
    #[serde(rename = "paragraph-separation-length")]
    pub paragraph_separation_length: u8,
    #[serde(rename = "chapter-indicator-character")]
    pub chapter_indicator_character: String,
}

pub fn extract_config_from_file(config_file_path: &Path) -> Option<Config> {
    match read_to_string(config_file_path) {
        Ok(file_content) => extract_config_from_str(file_content),
        Err(error) => None,
    }
}

pub fn extract_config_from_str<T: AsRef<str>>(config_str: T) -> Option<Config> {
    let config: Result<Config, Error> = toml_from_str(config_str.as_ref());

    match config {
        Ok(config) => Some(config),
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

pub fn get_keep_config() -> Option<Config> {
    if let Some(path) = get_keep_base_dir() {
        let mut config_file_path = path.into_path_buf();

        config_file_path.push(KEEP_CONFIG_FILE);

        if config_file_path.is_file() {
            return extract_config_from_file(config_file_path.as_ref());
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_config_load() {
        let sample_config = r##"
        [author]
        name = "John"
        email = "anemail@emailer.com"
        pen-name = "Winston"

        [formatting]
        paragraph-separation-length = 2
        chapter-indicator-character = "#"
        "##;

        let target_config = Config {
            author: Author {
                name: "John".to_string(),
                email: "anemail@emailer.com".to_string(),
                pen_name: "Winston".to_string(),
            },
            formatting: Formatting {
                paragraph_separation_length: 2,
                chapter_indicator_character: "#".to_string(),
            },
        };

        let config = extract_config_from_str(sample_config);

        assert!(config.is_some());

        assert_eq!(config.unwrap(), target_config);
    }
}

use crate::{
    constants::{APP_CONFIG_FILE, KEEP_CONFIG_FILE},
    fs::{get_app_base_dir, get_keep_base_dir},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use toml::from_str as toml_from_str;

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub global_keep_config: KeepConfig,
}

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct KeepConfig {
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

trait ConfigKind {}

impl ConfigKind for AppConfig {}

impl ConfigKind for KeepConfig {}

impl Author {
    pub fn default() -> Self {
        Author {
            name: "".into(),
            email: "".into(),
            pen_name: "".into(),
        }
    }
}

impl Formatting {
    pub fn default() -> Self {
        Formatting {
            paragraph_separation_length: 2,
            chapter_indicator_character: "#".into(),
        }
    }
}

impl KeepConfig {
    pub fn default() -> Self {
        KeepConfig {
            author: Author::default(),
            formatting: Formatting::default(),
        }
    }
}

impl AppConfig {
    pub fn default() -> Self {
        AppConfig {
            global_keep_config: KeepConfig::default(),
        }
    }
}

fn extract_config_from_file<C>(config_file_path: &Path) -> Option<C>
where
    C: ConfigKind + DeserializeOwned,
{
    match read_to_string(config_file_path) {
        Ok(file_content) => extract_config_from_str(&file_content),
        Err(error) => None,
    }
}

fn extract_config_from_str<'de, C>(config_str: &'de str) -> Option<C>
where
    C: ConfigKind + Deserialize<'de>,
{
    match toml_from_str(config_str) {
        Ok(config) => Some(config),
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

pub fn get_keep_config() -> Option<KeepConfig> {
    if let Some(path) = get_keep_base_dir() {
        let mut config_file_path = path.into_path_buf();

        config_file_path.push(KEEP_CONFIG_FILE);

        if config_file_path.is_file() {
            return extract_config_from_file(config_file_path.as_ref());
        }
    }

    None
}

pub fn get_app_config() -> Option<AppConfig> {
    if let Some(path) = get_app_base_dir() {
        let mut config_file_path = path.into_path_buf();

        config_file_path.push(APP_CONFIG_FILE);

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

        let target_config = KeepConfig {
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

        let config: Option<KeepConfig> = extract_config_from_str(sample_config);

        assert!(config.is_some());

        assert_eq!(config.unwrap(), target_config);
    }
}

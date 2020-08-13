use crate::{
    constants::{APP_CONFIG_FILE, KEEP_CONFIG_FILE},
    fs::{get_app_config_dir, get_keep_base_dir, initialize_app_config_dir},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::{read_to_string, write as write_to_file};
use std::{
    fmt::{Display, Formatter, Result},
    path::Path,
};
use toml::{from_str as toml_from_str, to_string as toml_to_string};

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "global-keep-config")]
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
    pub name: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "pen-name")]
    pub pen_name: Option<String>,
}

#[cfg_attr(debug_assertions, derive(Debug, std::cmp::PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Formatting {
    #[serde(rename = "paragraph-separation-length")]
    pub paragraph_separation_length: u8,
    #[serde(rename = "chapter-indicator-character")]
    pub chapter_indicator_character: String,
}

pub trait ConfigKind {
    fn get_value(&self, key: &str) -> Option<&dyn Display>;

    fn set_value(&mut self, key: &str, value: &str) -> Option<&dyn Display>;
}

impl ConfigKind for AppConfig {
    fn get_value(&self, key: &str) -> Option<&dyn Display> {
        let separator: &str = ".";

        let key_components = key.split(separator).collect::<Vec<&str>>();

        if key_components.len() != 3 {
            return None;
        }

        match key_components[0] {
            "global-keep-config" => self
                .global_keep_config
                .get_value(&key_components[1..=2].join(separator)),
            _ => None,
        }
    }

    fn set_value(&mut self, key: &str, value: &str) -> Option<&dyn Display> {
        let separator: &str = ".";

        let key_components = key.split(separator).collect::<Vec<&str>>();

        if key_components.len() != 3 {
            return None;
        }

        match key_components[0] {
            "global-keep-config" => self
                .global_keep_config
                .set_value(&key_components[1..=2].join(separator), value),
            _ => None,
        }
    }
}

fn to_displayable<'a>(enter: Option<&'a String>) -> Option<&'a dyn Display> {
    // TODO: There has to be a better way to do this
    match enter {
        Some(item) => Some(item),
        None => None,
    }
}

impl ConfigKind for KeepConfig {
    fn get_value(&self, key: &str) -> Option<&dyn Display> {
        let key_components = key.split('.').collect::<Vec<&str>>();

        if key_components.len() != 2 {
            return None;
        }

        match key_components[0] {
            "author" => match key_components[1] {
                "name" => to_displayable(self.author.name.as_ref()),
                "email" => to_displayable(self.author.email.as_ref()),
                "pen-name" => to_displayable(self.author.pen_name.as_ref()),
                _ => None,
            },
            "formatting" => match key_components[1] {
                "paragraph-separation-length" => Some(&self.formatting.paragraph_separation_length),
                "chapter-indicator-character" => Some(&self.formatting.chapter_indicator_character),
                _ => None,
            },
            _ => None,
        }
    }

    fn set_value(&mut self, key: &str, value: &str) -> Option<&dyn Display> {
        let key_components = key.split('.').collect::<Vec<&str>>();

        if key_components.len() != 2 {
            return None;
        }

        match key_components[0] {
            "author" => match key_components[1] {
                "name" => {
                    self.author.name = Some(value.to_string());
                    to_displayable(self.author.name.as_ref())
                }
                "email" => {
                    self.author.email = Some(value.to_string());
                    to_displayable(self.author.email.as_ref())
                }
                "pen-name" => {
                    self.author.pen_name = Some(value.to_string());
                    to_displayable(self.author.pen_name.as_ref())
                }
                _ => None,
            },
            "formatting" => match key_components[1] {
                "paragraph-separation-length" => match value.parse::<u8>() {
                    Ok(parsed) => {
                        self.formatting.paragraph_separation_length = parsed;
                        Some(&self.formatting.paragraph_separation_length)
                    }
                    Err(_) => None,
                },
                "chapter-indicator-character" => {
                    if value.len() == 1 {
                        self.formatting.chapter_indicator_character = value.to_string();
                        return Some(&self.formatting.chapter_indicator_character);
                    }

                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl Author {
    pub fn default() -> Self {
        Author {
            name: None,
            email: None,
            pen_name: None,
        }
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(name) = &self.name {
            writeln!(f, "author.name = \"{}\"", name.as_str())?;
        }

        if let Some(email) = &self.email {
            writeln!(f, "author.email = \"{}\"", email.as_str())?;
        }

        if let Some(pen_name) = &self.pen_name {
            writeln!(f, "author.pen-name = \"{}\"", pen_name.as_str())?;
        }

        write!(f, "")
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

impl Display for Formatting {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "formatting.paragraph-separation-length = {}",
            &self.paragraph_separation_length
        )?;

        writeln!(
            f,
            "formatting.chapter-indicator-character = \"{}\"",
            &self.chapter_indicator_character
        )?;

        write!(f, "")
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

impl Display for KeepConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Keep config:")?;
        write!(f, "{}", self.author)?;
        writeln!(f)?;
        write!(f, "{}", self.formatting)?;
        write!(f, "")
    }
}

impl AppConfig {
    pub fn default() -> Self {
        AppConfig {
            global_keep_config: KeepConfig::default(),
        }
    }
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "App config:")?;
        write!(f, "{}", self.global_keep_config.author)?;
        writeln!(f)?;
        write!(f, "{}", self.global_keep_config.formatting)?;
        write!(f, "")
    }
}

fn extract_config_from_file<C>(config_file_path: &Path) -> Option<C>
where
    C: ConfigKind + DeserializeOwned,
{
    match read_to_string(config_file_path) {
        Ok(file_content) => extract_config_from_str(&file_content),
        Err(_error) => None,
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
        let config_file_path = path.into_path_buf().join(KEEP_CONFIG_FILE);

        if config_file_path.is_file() {
            return extract_config_from_file(config_file_path.as_ref());
        }
    }

    None
}

pub fn get_app_config() -> Option<AppConfig> {
    if let Some(path) = get_app_config_dir() {
        let config_file_path = path.into_path_buf().join(APP_CONFIG_FILE);

        if config_file_path.is_file() {
            return extract_config_from_file(config_file_path.as_ref());
        }
    }

    None
}

pub fn get_app_config_or_create() -> Option<AppConfig> {
    if let Some(config) = get_app_config() {
        return Some(config);
    }

    let default_config = AppConfig::default();

    if let Some(path) = initialize_app_config_dir() {
        let config_file_path = path.into_path_buf().join(APP_CONFIG_FILE);

        if !config_file_path.exists() {
            if let Ok(result) = toml_to_string(&default_config) {
                if let Err(err) = write_to_file(config_file_path, result) {
                    println!("{}", err);
                }
            }
        }
    }

    Some(default_config)
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
                name: Some("John".into()),
                email: Some("anemail@emailer.com".into()),
                pen_name: Some("Winston".into()),
            },
            formatting: Formatting {
                paragraph_separation_length: 2,
                chapter_indicator_character: "#".into(),
            },
        };

        let config: Option<KeepConfig> = extract_config_from_str(sample_config);

        assert!(config.is_some());

        assert_eq!(config.unwrap(), target_config);
    }
}

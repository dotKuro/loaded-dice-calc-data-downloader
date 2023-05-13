use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Champion {
    pub name: String,
    pub cost: i8,
    pub traits: Vec<String>,
}

#[derive(Error, Debug)]
pub enum SaveToDiskError {
    #[error("serialization error: {source:?}")]
    Serialization { source: serde_json::Error },
    #[error("Cannot pen file: {source:?}")]
    OpenPath { source: std::io::Error },
    #[error("Cannot write to file: {source:?}")]
    Write { source: std::io::Error },
}

pub trait ChampionCollection {
    fn save_to_disk(self, path: String) -> Result<(), SaveToDiskError>;
}

impl ChampionCollection for Vec<Champion> {
    fn save_to_disk(self, path: String) -> Result<(), SaveToDiskError> {
        let serialized_champions = serde_json::to_string(&self)
            .map_err(|err| SaveToDiskError::Serialization { source: err })?;

        let mut file =
            File::create(path).map_err(|err| SaveToDiskError::OpenPath { source: err })?;
        file.write_all(serialized_champions.as_bytes())
            .map_err(|err| SaveToDiskError::Write { source: err })
    }
}

#[cfg(test)]
mod tests {
    mod save_to_disk {
        use crate::champion::{Champion, ChampionCollection};
        use spectral::prelude::*;
        use std::fs;
        use uuid::Uuid;

        #[test]
        fn writes_empty_array_in_correct_file() {
            let champions = Vec::<Champion>::new();
            let path = format!(
                "/tmp/loaded-dice-calc-data-downloader-test-{}",
                Uuid::new_v4()
            );

            let result = champions.save_to_disk(path.clone());
            let contents = fs::read_to_string(&path);

            assert_that(&result).is_ok();
            assert_that!(&contents).is_ok_containing("[]".to_string());
        }

        #[test]
        fn writes_array_with_content_in_correct_file() {
            let mut champions = Vec::<Champion>::new();
            let path = format!(
                "/tmp/loaded-dice-calc-data-downloader-test-{}",
                Uuid::new_v4()
            );
            champions.push(Champion {
                name: "Olaf".to_string(),
                cost: 5,
                traits: vec!["Axtwerfer".to_string()],
            });

            let result = champions.save_to_disk(path.clone());
            let contents = fs::read_to_string(&path);

            assert_that(&result).is_ok();
            let expected_content = r#"[{"name":"Olaf","cost":5,"traits":["Axtwerfer"]}]"#;
            assert_that!(&contents).is_ok_containing(expected_content.to_string());
        }

        #[test]
        fn returns_error_for_invalid_path() {
            let champions = Vec::<Champion>::new();
            let path = "";

            let result = champions.save_to_disk(path.to_string());

            assert_that!(result).is_err();
        }
    }
}

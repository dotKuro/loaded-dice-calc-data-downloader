use crate::champion::Champion;
use crate::set::Set;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct DataFile {
    sets: HashMap<String, Set>,
}

#[derive(Error, Debug)]
pub enum GetChampionsError {
    #[error("Set with id {set_id:?} was not found.")]
    SetNotFound { set_id: String },
}

impl DataFile {
    pub fn get_champions(self, set_id: String) -> Result<Vec<Champion>, GetChampionsError> {
        match self.sets.get(&set_id) {
            None => Err(GetChampionsError::SetNotFound { set_id }),
            Some(set) => Ok(set.get_champions()),
        }
    }
}

#[derive(Error, Debug)]
pub enum DownloadDataFileError {
    #[error("request error: {source:?}")]
    RequestError { source: reqwest::Error },
    #[error("deserialization error: {source:?}")]
    DeserializationError { source: reqwest::Error },
}

pub async fn download_data_file(
    data_file_url_template: String,
    patch: String,
) -> Result<DataFile, DownloadDataFileError> {
    let data_file_url = data_file_url_template.replace("{patch}", &patch);
    let data_file = reqwest::get(data_file_url)
        .await
        .map_err(|err| DownloadDataFileError::RequestError { source: err })?
        .json::<DataFile>()
        .await
        .map_err(|err| DownloadDataFileError::DeserializationError { source: err })?;

    Ok(data_file)
}

#[cfg(test)]
mod tests {
    mod get_champions {
        use crate::champion::Champion;
        use crate::data_file::DataFile;
        use crate::set::Set;
        use spectral::prelude::*;
        use std::collections::HashMap;

        #[test]
        fn gets_champions_from_the_correct_set() {
            let mut sets = HashMap::new();
            sets.insert(
                "1".to_string(),
                Set {
                    champions: vec![Champion {
                        name: "Olaf".to_string(),
                        cost: 1,
                        traits: vec!["Axtwerfer".to_string()],
                    }],
                },
            );
            sets.insert(
                "2".to_string(),
                Set {
                    champions: vec![Champion {
                        name: "Maokai".to_string(),
                        cost: 1,
                        traits: vec!["Baum".to_string()],
                    }],
                },
            );
            let data_file = DataFile { sets };

            let result = data_file.get_champions("2".to_string());

            assert_that!(&result).is_ok();

            let champions = result.unwrap();
            assert_that!(&champions).has_length(1);
            assert_that!(&champions.get(0).unwrap().name).is_equal_to("Maokai".to_string());
        }

        #[test]
        fn ignores_champions_without_traits() {
            let mut sets = HashMap::new();
            sets.insert(
                "1".to_string(),
                Set {
                    champions: vec![Champion {
                        name: "Olaf".to_string(),
                        cost: 1,
                        traits: vec![],
                    }],
                },
            );
            let data_file = DataFile { sets };

            let result = data_file.get_champions("1".to_string());

            assert_that!(&result).is_ok();

            let champions = result.unwrap();
            assert_that!(&champions).has_length(0);
        }

        #[test]
        fn returns_an_error_if_a_non_existing_set_is_specified() {
            let data_file = DataFile {
                sets: HashMap::new(),
            };

            let result = data_file.get_champions("1".to_string());

            assert_that!(&result).is_err();
        }
    }
}

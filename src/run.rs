use crate::champion::{ChampionCollection, SaveToDiskError};
use crate::data_file::{download_data_file, DownloadDataFileError, GetChampionsError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("Failed to download data file: {source:?}")]
    DownloadError { source: DownloadDataFileError },
    #[error("Failed to retrieve champions from data file: {source:?}")]
    RetrieveChampionsError { source: GetChampionsError },
    #[error("Failed to save champion data to disk: {source:?}")]
    SaveToDiskError { source: SaveToDiskError },
}

pub async fn run(
    data_file_url_template: String,
    patch: String,
    set_id: String,
    output_path: String,
) -> Result<(), RunError> {
    let data_file = download_data_file(data_file_url_template, patch)
        .await
        .map_err(|err| RunError::DownloadError { source: err })?;

    let champions = data_file
        .get_champions(set_id)
        .map_err(|err| RunError::RetrieveChampionsError { source: err })?;

    champions
        .save_to_disk(output_path)
        .map_err(|err| RunError::SaveToDiskError { source: err })
}

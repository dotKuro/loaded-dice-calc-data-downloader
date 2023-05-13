use loaded_dice_calc_data_downloader::run;
use loaded_dice_calc_data_downloader::util::assert_env_variable;

#[tokio::main]
async fn main() {
    let data_file_url_template = assert_env_variable("DATA_FILE_URL_TEMPLATE");
    let patch = assert_env_variable("PATCH");
    let set_id = assert_env_variable("SET_ID");
    let output_path = assert_env_variable("OUTPUT_PATH");

    run(data_file_url_template, patch, set_id, output_path)
        .await
        .unwrap()
}

use crate::path_helper::get_current_statics_path;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_json_data(path: &str) -> Result<String> {
    let path = get_current_statics_path(path)?;
    let mut file = File::open(path).await?;
    let mut data_str: String = "".to_string();
    file.read_to_string(&mut data_str).await?;
    Ok(data_str.clone())
}

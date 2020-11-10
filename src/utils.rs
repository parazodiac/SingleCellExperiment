use std::path::PathBuf;
use std::error::Error;

pub fn read_features(file_path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let features: Vec<String> = file_content.split("\n")
        .map(|x| x.to_owned())
        .collect();

    Ok(features)
}
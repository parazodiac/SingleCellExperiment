use std::error::Error;
use std::path::PathBuf;

pub fn read_features(file_path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let features: Vec<String> = file_content.split("\n").map(|x| x.to_owned()).collect();

    println!("Found {} lines", features.len());
    Ok(features)
}

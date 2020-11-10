use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use flate2::read::GzDecoder;

pub fn read_features(file_path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let mut features: Vec<String> = file_content.split("\n").map(|x| x.to_owned()).collect();
    features.pop();

    Ok(features)
}

pub fn read_compressed_features(file_path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut gz = GzDecoder::new(File::open(file_path)?);
    let mut file_content = String::new();
    gz.read_to_string(&mut file_content)?;

    let mut features: Vec<String> = file_content.split("\n").map(|x| x.to_owned()).collect();
    features.pop();

    Ok(features)
}

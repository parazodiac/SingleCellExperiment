use std::error::Error;
use std::path::PathBuf;

pub fn alevin_file_names(mut path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    path.push("alevin");

    let mut mat_file = path.clone();
    mat_file.push("quants_mat.gz");

    let mut column_file = path.clone();
    column_file.push("quants_mat_cols.txt");

    let mut row_file = path.clone();
    row_file.push("quants_mat_rows.txt");

    let file_paths = vec![mat_file, row_file, column_file];
    Ok(file_paths)
}

pub fn tenx_v2_file_names(path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut mat_file = path.clone();
    mat_file.push("matrix.mtx");

    let mut column_file = path.clone();
    column_file.push("genes.tsv");

    let mut row_file = path.clone();
    row_file.push("barcodes.tsv");

    let file_paths = vec![mat_file, row_file, column_file];
    Ok(file_paths)
}

pub fn tenx_v3_file_names(path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut mat_file = path.clone();
    mat_file.push("matrix.mtx.gz");

    let mut column_file = path.clone();
    column_file.push("features.tsv.gz");

    let mut row_file = path.clone();
    row_file.push("barcodes.tsv.gz");

    let file_paths = vec![mat_file, row_file, column_file];
    Ok(file_paths)
}

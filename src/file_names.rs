use std::error::Error;
use std::path::PathBuf;

pub struct MatFileNames {
    matrix_file: PathBuf,
    column_file: PathBuf,
    row_file: PathBuf,
}

impl MatFileNames {
    pub fn matrix_file(&self) -> PathBuf {
        self.matrix_file.clone()
    }

    pub fn column_file(&self) -> PathBuf {
        self.column_file.clone()
    }

    pub fn row_file(&self) -> PathBuf {
        self.row_file.clone()
    }
}

pub fn alevin_file_names(mut path: PathBuf) -> Result<MatFileNames, Box<dyn Error>> {
    path.push("alevin");

    let mut matrix_file = path.clone();
    matrix_file.push("quants_mat.gz");

    let mut column_file = path.clone();
    column_file.push("quants_mat_cols.txt");

    let mut row_file = path.clone();
    row_file.push("quants_mat_rows.txt");

    Ok(MatFileNames {
        matrix_file,
        column_file,
        row_file,
    })
}

pub fn tenx_v2_file_names(path: PathBuf) -> Result<MatFileNames, Box<dyn Error>> {
    let mut matrix_file = path.clone();
    matrix_file.push("matrix.mtx");

    let mut column_file = path.clone();
    column_file.push("genes.tsv");

    let mut row_file = path.clone();
    row_file.push("barcodes.tsv");

    Ok(MatFileNames {
        matrix_file,
        column_file,
        row_file,
    })
}

pub fn tenx_v3_file_names(path: PathBuf) -> Result<MatFileNames, Box<dyn Error>> {
    let mut matrix_file = path.clone();
    matrix_file.push("matrix.mtx.gz");

    let mut column_file = path.clone();
    column_file.push("features.tsv.gz");

    let mut row_file = path.clone();
    row_file.push("barcodes.tsv.gz");

    Ok(MatFileNames {
        matrix_file,
        column_file,
        row_file,
    })
}

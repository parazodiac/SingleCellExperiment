use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};

use flate2::write::GzEncoder;
use flate2::Compression;
use sprs::CsMat;

// reads the CSV format single cell matrix from the given path
pub fn reader<MatValT, ReaderT>(
    buffered: BufReader<ReaderT>,
    num_rows: usize,
    num_cols: usize,
) -> Result<CsMat<MatValT>, Box<dyn Error>>
where
    MatValT: std::str::FromStr + Copy,
    ReaderT: std::io::Read,
{
    let ballpark: usize = ((num_rows * num_cols) as f64 / 8.0) as usize;
    let mut ind_ptr = vec![0; num_rows + 1];
    let mut data = Vec::with_capacity(ballpark);
    let mut indices = Vec::with_capacity(ballpark);

    let mut total_entries = 0;
    for (row_id, line) in buffered.lines().enumerate() {
        let record = line?;
        let values: Vec<MatValT> = record.split(",").flat_map(str::parse::<MatValT>).collect();

        total_entries += values.len();
        ind_ptr[row_id + 1] = total_entries;

        assert_eq!(values.len(), num_cols);
        for (column_id, val) in values.into_iter().enumerate() {
            indices.push(column_id);
            data.push(val as MatValT);
        }
    }

    Ok(CsMat::new_csc((num_rows, num_cols), ind_ptr, indices, data))
}

// writes the CSV format single cell matrix into the given path
pub fn writer<MatValT>(path_str: &str, matrix: &CsMat<MatValT>) -> Result<(), Box<dyn Error>>
where
    MatValT: std::fmt::Display + num::traits::Zero + Copy,
{
    let quants_file_handle = File::create(path_str)?;

    // writing matrix
    let buffered = BufWriter::new(quants_file_handle);
    let mut file = GzEncoder::new(buffered, Compression::default());

    let num_columns = matrix.cols();
    let zero: MatValT = MatValT::zero();
    for row_vec in matrix.outer_iterator() {
        let mut it = row_vec.iter();
        let mut column_id_validator = 0;

        match it.next() {
            Some((0, &val)) => {
                column_id_validator += 1;
                write!(&mut file, "{}", val)?;
            }
            Some((idx, &val)) => {
                for _ in 0..idx {
                    write!(&mut file, "{},", zero)?;
                }

                column_id_validator += idx + 1;
                write!(&mut file, "{}", val)?;
            }
            None => {
                for _ in 1..num_columns {
                    write!(&mut file, "{},", zero)?;
                }

                write!(&mut file, "{}", zero)?;
                continue;
            }
        };

        while let Some((column_ind, &val)) = it.next() {
            while column_id_validator < column_ind {
                write!(&mut file, ",{}", zero)?;
                column_id_validator += 1;
            }
            write!(&mut file, ",{}", val)?;
            column_id_validator += 1;
        }

        while column_id_validator < num_columns {
            write!(&mut file, ",{}", zero)?;
            column_id_validator += 1;
        }
    } // end row iterator

    Ok(())
}

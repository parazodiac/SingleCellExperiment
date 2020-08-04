use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use sprs::{CsMat, TriMat};

// reads the CSV format single cell matrix from the given path
pub fn reader<MatValT, ReaderT>(
    buffered: BufReader<ReaderT>,
    num_rows: usize,
    num_cols: usize,
) -> Result<CsMat<MatValT>, Box<dyn Error>>
where
    MatValT: std::str::FromStr + num::Num + Clone,
    ReaderT: std::io::Read,
{
    let mut tri_matrix = TriMat::new((num_rows, num_cols));
    let mut rdr = ext_csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buffered);

    for (row_id, line) in rdr.records().enumerate() {
        let record = line?;
        let values: Vec<MatValT> = record.into_iter().flat_map(str::parse::<MatValT>).collect();
        assert_eq!(values.len(), num_cols);

        for (column_id, val) in values.into_iter().enumerate() {
            if val == MatValT::zero() {
                continue;
            }
            tri_matrix.add_triplet(row_id, column_id, val);
        }
    }

    Ok(tri_matrix.to_csr())
}

// writes the CSV format single cell matrix into the given path
pub fn writer<MatValT>(path_str: &str, matrix: &CsMat<MatValT>) -> Result<(), Box<dyn Error>>
where
    MatValT: Copy + num::traits::Zero + std::fmt::Display,
{
    // writing matrix
    let file = File::create(path_str)?;
    let mut wtr = ext_csv::WriterBuilder::new().from_writer(file);

    let num_columns = matrix.cols();
    let zero: MatValT = MatValT::zero();
    let mut columns: Vec<MatValT> = vec![zero; num_columns];

    for row_vec in matrix.outer_iterator() {
        columns.iter_mut().for_each(|x| *x = zero);

        let mut it = row_vec.iter();
        loop {
            match it.next() {
                Some((col_idx, &val)) => {
                    columns[col_idx] = val;
                }
                None => break,
            }
        }

        let record: Vec<String> = columns.iter().map(|x| x.to_string()).collect();
        wtr.write_record(&record)?;
    } // end row iterator

    wtr.flush()?;
    Ok(())
}

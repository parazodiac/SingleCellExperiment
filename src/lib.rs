extern crate byteorder;
extern crate flate2;
extern crate math;
extern crate sprs;

mod eds;

use sprs::CsMat;
use std::error::Error;

#[derive(Debug)]
pub struct SingleCellExperiment<T> {
    counts: CsMat<T>,
    rows: Vec<String>,
    cols: Vec<String>,
}

impl<T> SingleCellExperiment<T> {
    pub fn cols(&self) -> usize {
        return self.counts.cols();
    }

    pub fn rows(&self) -> usize {
        return self.counts.rows();
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.counts.rows(), self.counts.cols());
    }

    pub fn nnz(&self) -> usize {
        return self.counts.nnz();
    }

    pub fn row_names(&self) -> &Vec<String> {
        return &self.rows;
    }

    pub fn col_names(&self) -> &Vec<String> {
        return &self.cols;
    }

    pub fn counts(&self) -> &CsMat<T> {
        return &self.counts;
    }

    pub fn from_csr(
        counts: CsMat<T>,
        rows: Vec<String>,
        cols: Vec<String>,
    ) -> Result<SingleCellExperiment<T>, String> {
        if rows.len() != counts.rows() {
            return Err("Number of rows in the matrix doesn't match the row names".to_owned());
        }

        if cols.len() != counts.cols() {
            return Err(
                "Number of columns in the matrix doesn't match the column names".to_owned(),
            );
        }

        Ok(SingleCellExperiment {
            counts: counts,
            rows: rows,
            cols: cols,
        })
    }

    //pub fn from_csv(file_path: &str) -> Result<SingleCellExperiment<T>, Box<dyn Error>> {
    //    let sce: SingleCellExperiment<T> = csv::reader(file_path)?;
    //    return sce
    //}

    //pub fn from_mtx(
    //    file_path: &str,
    //    rows: Vec<String>,
    //    cols: Vec<String>,
    //) -> Result<SingleCellExperiment<T>, Box<dyn Error>> {
    //    let counts_matrix: CsMat<T> = mtx::reader(file_path)?;

    //    Ok(SingleCellExperiment{
    //        counts: counts_matrix,
    //        rows: rows,
    //        cols: cols,
    //    })
    //}
}

impl SingleCellExperiment<f32> {
    pub fn from_eds(
        file_path: &str,
        rows: Vec<String>,
        cols: Vec<String>,
    ) -> Result<SingleCellExperiment<f32>, Box<dyn Error>> {
        let counts_matrix: CsMat<f32> = eds::reader(file_path, rows.len(), cols.len())?;

        Ok(SingleCellExperiment {
            counts: counts_matrix,
            rows: rows,
            cols: cols,
        })
    }

    pub fn to_eds(file_path: &str, sce: &SingleCellExperiment<f32>) -> Result<(), Box<dyn Error>> {
        eds::writer(file_path, sce.counts())
    }
}

#[cfg(test)]
mod tests {
    use super::SingleCellExperiment;
    use sprs::CsMat;

    fn get_test_matrix() -> CsMat<f32> {
        let a = CsMat::new_csc(
            (3, 3),
            vec![0, 2, 4, 5],
            vec![0, 1, 0, 2, 2],
            vec![1., 2., 3., 4., 5.],
        );
        return a;
    }

    #[test]
    fn test_single_cell_experiment() {
        let a = get_test_matrix();
        let b: Vec<String> = vec!["1", "2", "3"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let c: Vec<String> = vec!["4", "5", "6"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let sce = match SingleCellExperiment::from_csr(a, b.clone(), c.clone()) {
            Ok(x) => x,
            Err(_) => unreachable!(),
        };

        assert_eq!(sce.cols(), 3);
        assert_eq!(sce.rows(), 3);

        assert_eq!(sce.row_names().to_owned(), b);
        assert_eq!(sce.col_names().to_owned(), c);
        assert_eq!(sce.dimensions(), (3, 3));
        assert_eq!(sce.nnz(), 5);
        assert_eq!(sce.counts().to_owned(), get_test_matrix());
    }
}

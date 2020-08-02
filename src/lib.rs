extern crate sprs;

use sprs::CsMat;

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

    pub fn row_names(&self) -> Vec<String> {
        return self.rows.clone();
    }

    pub fn col_names(&self) -> Vec<String> {
        return self.cols.clone();
    }

    pub fn from_csr(
        counts: CsMat<T>,
        rows: Vec<String>,
        cols: Vec<String>,
    ) -> Result<SingleCellExperiment<T>, &'static str> {
        if rows.len() != counts.rows() {
            return Err("Number of rows in the matrix doesn't match the row names");
        }

        if cols.len() != counts.cols() {
            return Err("Number of columns in the matrix doesn't match the column names");
        }

        Ok(SingleCellExperiment {
            counts: counts,
            rows: rows,
            cols: cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SingleCellExperiment;
    use sprs::CsMat;

    #[test]
    fn test_single_cell_experiment() {
        let a = CsMat::new_csc(
            (3, 3),
            vec![0, 2, 4, 5],
            vec![0, 1, 0, 2, 2],
            vec![1., 2., 3., 4., 5.],
        );
        let b: Vec<String> = vec!["1", "2", "3"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let c: Vec<String> = vec!["4", "5", "6"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let sce = SingleCellExperiment::from_csr(a, b, c).unwrap();
        assert_eq!(sce.cols(), 3);
        assert_eq!(sce.rows(), 3);
    }
}

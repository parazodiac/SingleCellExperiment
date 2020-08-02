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
        return self.counts.cols()
    }

    pub fn rows(&self) -> usize {
        return self.counts.rows()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.counts.rows(), self.counts.cols())
    }

    pub fn nnz(&self) -> usize {
        return self.counts.nnz()
    }

    pub fn row_names(&self) -> Vec<String> {
        return self.rows.clone()
    }

    pub fn col_names(&self) -> Vec<String> {
        return self.cols.clone()
    }

    pub fn from_csr(
        counts: CsMat<T>, 
        rows: Vec<String>, 
        cols: Vec<String>
    ) -> Result<SingleCellExperiment<T>, &'static str> {
        if rows.len() != counts.rows() {
            return Err("Number of rows in the matrix doesn't match the row names");
        }

        if cols.len() != counts.cols() {
            return Err("Number of columns in the matrix doesn't match the column names");
        }

        Ok(SingleCellExperiment{
            counts: counts,
            rows: rows,
            cols: cols,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

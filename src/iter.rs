use crate::SingleCellExperiment;

impl<'a, T> IntoIterator for &'a SingleCellExperiment<T> {
    type Item = SingleCellExperimentRow<'a, T>;
    type IntoIter = SingleCellExperimentIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SingleCellExperimentIntoIterator {
            sce: self,
            row_id: 0,
            row_it: self.counts.outer_iterator(),
        }
    }
}

pub struct SingleCellExperimentRow<'a, T> {
    row_id: usize,
    row_name: &'a String,
    row_counts: sprs::CsVecBase<&'a [usize], &'a [T]>,
}

impl<'a, T> SingleCellExperimentRow<'a, T> {
    pub fn name(&'a self) -> &'a String {
        self.row_name
    }

    pub fn id(&'a self) -> usize {
        self.row_id
    }
}

impl<'a, T> IntoIterator for &'a SingleCellExperimentRow<'a, T> {
    type Item = SingleCellExperimentEntry<'a, T>;
    type IntoIter = SingleCellExperimentIntoRow<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SingleCellExperimentIntoRow {
            row_it: self.row_counts.iter(),
        }
    }
}

pub struct SingleCellExperimentEntry<'a, T> {
    count: &'a T,
    col_id: usize,
    // col_name: &'a String,
}

impl<'a, T> SingleCellExperimentEntry<'a, T> {
    pub fn get_col_idx(&self) -> usize {
        self.col_id
    }

    pub fn get_count(&self) -> &T {
        self.count
    }
}

pub struct SingleCellExperimentIntoRow<'a, T> {
    row_it: sprs::vec::VectorIterator<'a, T, usize>,
}

impl<'a, T> Iterator for SingleCellExperimentIntoRow<'a, T> {
    type Item = SingleCellExperimentEntry<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let nval = self.row_it.next();
        match nval {
            Some(x) => Some(SingleCellExperimentEntry {
                count: x.1,
                col_id: x.0,
            }),
            None => None,
        }
    }
}

pub struct SingleCellExperimentIntoIterator<'a, T> {
    sce: &'a SingleCellExperiment<T>,
    row_id: usize,
    row_it: sprs::OuterIterator<'a, T, usize>,
}

impl<'a, T> Iterator for SingleCellExperimentIntoIterator<'a, T> {
    type Item = SingleCellExperimentRow<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_id >= self.sce.rows() {
            return None;
        }

        let row_id = self.row_id;
        let row_name = self.sce.get_row_name(row_id).expect("can't get rowname");
        let row_counts = self.row_it.next().expect("can't get rowdata");

        self.row_id += 1;
        Some(SingleCellExperimentRow {
            row_id,
            row_name,
            row_counts,
        })
    }
}

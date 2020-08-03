use sprs::CsMat;
use std::error::Error;
use std::io::BufReader;

// reads the MTX format single cell matrix from the given path
pub fn reader<MatValT, ReaderT>(
    mut buffered: BufReader<ReaderT>,
) -> Result<CsMat<MatValT>, Box<dyn Error>>
where
    MatValT: Clone + num_traits::Num + num_traits::NumCast,
    ReaderT: std::io::Read,
{
    let matrix = sprs::io::read_matrix_market_from_bufread(&mut buffered)?;
    Ok(matrix.to_csr())
}

// reads the MTX format single cell matrix from the given path
pub fn writer<MatValT>(_path_str: &str, _matrix: &CsMat<MatValT>) -> Result<(), Box<dyn Error>>
where
    MatValT: std::fmt::Display + Copy,
{
    //sprs::io::write_matrix_market(_path_str, &_matrix)?;
    Ok(())
}

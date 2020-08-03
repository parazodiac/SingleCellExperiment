use std::io::BufReader;
use std::error::Error;
use sprs::CsMat;

// reads the MTX format single cell matrix from the given path
pub fn reader<MatValT, ReaderT>(
    mut buffered: BufReader<ReaderT>,
) -> Result<CsMat<MatValT>, Box<dyn Error>> 
 where MatValT: Clone + num_traits::Num + num_traits::NumCast, ReaderT: std::io::Read {
    let matrix = sprs::io::read_matrix_market_from_bufread(&mut buffered)?;
    Ok(matrix.to_csr())
}

// reads the MTX format single cell matrix from the given path
pub fn writer<MatValT>(
    _buffered: &str,
    _cmatrix: &CsMat<MatValT>,
) -> Result<(), Box<dyn Error>>
where MatValT: std::fmt::Display + Copy +  {
    //sprs::io::write_matrix_market(buffered, &matrix)?;
    Ok(())
}
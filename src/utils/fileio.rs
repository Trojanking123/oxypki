use std::path::Path;

use crate::error::PkiResult;

pub fn read_file_to_der<P: AsRef<Path>>(path: P) -> PkiResult<Vec<u8>> {
    unimplemented!()
}

use std::path::Path;

use x509_parser::prelude::*;

use crate::error::{PkiResult, PkiError};
use crate::utils::fileio::read_file_to_der;
use crate::utils::FileFormat;

pub fn parser_cert<P: AsRef<Path>>(path: P, tp: FileFormat) -> PkiResult<()> {
    let der_buf = read_file_to_der(path, tp)?;

    let res = X509Certificate::from_der(&der_buf);
    match res {
        Ok((rem, cert)) => {
            assert!(rem.is_empty());
            //
            assert_eq!(cert.version(), X509Version::V3);
            Ok(())
        }
        _ => Err( PkiError::InvalidFormat )
    }
}

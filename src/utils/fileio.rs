use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use rustls_pemfile::{read_one, Item};

use crate::error::{PkiError, PkiResult};

use super::FormatType;

pub fn read_file_to_der<P: AsRef<Path>>(path: P, tp: FormatType) -> PkiResult<Vec<u8>> {
    let path = path.as_ref();
    match tp {
        FormatType::Pem => {
            let fd = match File::open(path) {
                Ok(fd) => fd,
                _ => return Err(PkiError::FileNotExsit(path.to_owned())),
            };
            let mut reader = BufReader::new(fd);
            let item = match read_one(&mut reader) {
                Ok(it) => it,
                _ => return Err(PkiError::InvalidFormat),
            };

            let res = match item {
                Some(Item::X509Certificate(cert)) => cert,
                _ => return Err(PkiError::InvalidFormat),
            };
            Ok(res)
        },
        FormatType::Der => {
            let mut fd = match File::open(path) {
                Ok(fd) => fd,
                _ => return Err(PkiError::FileNotExsit(path.to_owned())),
            };
            let file_size = fd.metadata()?.len();
            let mut res = vec![0; file_size as usize];
            fd.read_to_end(&mut res)?;
            Ok(res)
        },

        _ => Err(PkiError::InvalidFormat),
    }
}

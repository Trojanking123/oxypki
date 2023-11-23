use std::path::Path;

use x509_parser::prelude::*;

use crate::error::{PkiError, PkiResult};
use crate::utils::fileio::read_file_to_der;
use crate::utils::format::{Formatter, PutChar};
use crate::utils::FormatType;

impl<'a> Formatter for X509Certificate<'a> {
    type Target = Vec<u8>;
    fn format(&self, tp: FormatType) -> PkiResult<Self::Target> {
        match tp {
            FormatType::Text => {
                let mut buf = String::new();
                buf.put_str("Certificate:\n");
                buf.put_tab(1);
                buf.put_str("Data:\n");
                buf.put_tab(2);
                let ver = format!("Version: {}\n", self.version());
                buf.put_str(ver.as_str());
                Ok(buf.into_bytes())
            },
            FormatType::Json => {
                unimplemented!()
            },
            _ => Err(PkiError::InvalidFormat),
        }
    }
}

pub fn parser_cert<P: AsRef<Path>>(path: P, tp: FormatType) -> PkiResult<()> {
    let der_buf = read_file_to_der(path, tp)?;

    let res = X509Certificate::from_der(&der_buf);
    match res {
        Ok((rem, cert)) => {
            assert!(rem.is_empty());

            let buf = cert.format(FormatType::Text)?;
            let s = String::from_utf8(buf).unwrap();
            println!("{s}");
            Ok(())
        },
        _ => Err(PkiError::InvalidFormat),
    }
}

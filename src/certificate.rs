use std::path::Path;

use bytes::BytesMut;
use x509_parser::nom::AsBytes;
use x509_parser::prelude::*;

use crate::error::{PkiError, PkiResult};
use crate::utils::fileio::read_file_to_der;
use crate::utils::format::{Formatter, PutChar};
use crate::utils::FormatType;

impl<'a> Formatter for X509Certificate<'a> {
    fn format(&self, tp: FormatType) -> PkiResult<BytesMut> {
        let mut buf = BytesMut::with_capacity(1024);
        match tp {
            FormatType::Text => {
                buf.put_str("Certificate:");
                buf.put_tab(1);
                buf.put_str("Data:\n");
                buf.put_tab(2);
                buf.put_str("Version:");
                buf.put_str( self.version().to_string().as_str() );
                Ok(buf)
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
            let buf = buf.as_bytes();
            Ok(())
        },
        _ => Err(PkiError::InvalidFormat),
    }
}

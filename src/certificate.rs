use std::path::Path;

use oid_registry::OidRegistry;

use crate::error::{PkiError, PkiResult};
use crate::utils::fileio::read_file_to_der;
use crate::utils::format::{Formatter, PutChar};
use crate::utils::FormatType;
use asn1_rs::Oid;
use oid_registry::asn1_rs;
use x509_parser::prelude::*;

fn add_line<T: AsRef<str>>(s: &mut String, n_tab: usize, content: T, n_line: usize) {
    s.put_tab(n_tab);
    s.put_str(content.as_ref());
    s.put_newline(n_line)
}

fn get_oid_sn(oid: &Oid) -> String {
    let reg = OidRegistry::default().with_all_crypto();
    let e = reg.get(oid).unwrap();
    e.sn().to_owned()
}

impl<'a> Formatter for X509Certificate<'a> {
    type Target = Vec<u8>;
    fn format(&self, tp: FormatType) -> PkiResult<Self::Target> {
        match tp {
            FormatType::Text => {
                let mut buf = String::new();
                add_line(&mut buf, 0, "Certificate:", 1);
                add_line(&mut buf, 1, "Data:", 1);
                add_line(&mut buf, 2, format!("Version: {}", self.version()), 1);
                add_line(
                    &mut buf,
                    2,
                    format!("Serial Number: {}", self.raw_serial_as_string()),
                    1,
                );

                let sn = get_oid_sn(self.signature.oid());
                add_line(&mut buf, 2, format!("Signature Algorithm: {}", sn), 1);
                add_line(&mut buf, 2, format!("Issuer: {}", self.issuer()), 1);
                add_line(&mut buf, 2, "Validity: ", 1);
                add_line(&mut buf, 3, format!("Not Before: {}", self.validity().not_before), 1);
                add_line(&mut buf, 3, format!("Not Before: {}", self.validity().not_after), 1);
                add_line(&mut buf, 2, format!("Subject: {}", self.subject()), 1);
                add_line(&mut buf, 2, "Subject Public Key Info: ", 1);
                let a = self.public_key();


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

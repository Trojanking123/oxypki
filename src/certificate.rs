use std::path::Path;
use std::sync::OnceLock;

use oid_registry::{OidRegistry, OidEntry};
use x509_parser::public_key::PublicKey;

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

fn add_bits(s: &mut String, n_tab: usize, content: &[u8], n_line: usize) {
    let every_line_bit_num = 16;
    let line_num = content.len() / every_line_bit_num;
    for i in 0..line_num {
        let tmp = &content[i * every_line_bit_num..((i + 1) * every_line_bit_num)];
        let formatted_string = tmp
            .iter()
            .map(|byte| format!("{:02x}:", byte))
            .collect::<Vec<String>>()
            .join("");
        add_line(s, n_tab, formatted_string, n_line)
    }
}

fn get_oid_sn(oid: &Oid) -> String {
    println!("{}", oid);
    static REG: OnceLock<OidRegistry> = OnceLock::new();
    REG.get_or_init(|| OidRegistry::default().with_all_crypto());
    let b = OidEntry::new(oid.to_string(), "");
    let e = REG.get().unwrap().get(oid).unwrap_or(&b);
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
                add_line(
                    &mut buf,
                    3,
                    format!("Not Before: {}", self.validity().not_before),
                    1,
                );
                add_line(
                    &mut buf,
                    3,
                    format!("Not Before: {}", self.validity().not_after),
                    1,
                );
                add_line(&mut buf, 2, format!("Subject: {}", self.subject()), 1);
                add_line(&mut buf, 2, "Subject Public Key Info: ", 1);
                let pk_ref = self.public_key();
                add_line(
                    &mut buf,
                    3,
                    format!(
                        "Public Key Algorithm: {}",
                        get_oid_sn(&pk_ref.algorithm.algorithm)
                    ),
                    1,
                );
                let pk = pk_ref.parsed().unwrap();
                add_line(
                    &mut buf,
                    4,
                    format!("Public-Key: ({} bit)", pk.key_size()),
                    1,
                );
                match pk {
                    PublicKey::RSA(rsa_pk) => {
                        add_line(&mut buf, 4, "Modulus: ", 1);

                        let module = if rsa_pk.modulus[0] & 0x80 == 0 {
                            &rsa_pk.modulus[1..]
                        } else {
                            rsa_pk.modulus
                        };
                        add_bits(&mut buf, 5, module, 1);
                        let exponent = rsa_pk.try_exponent().unwrap();
                        add_line(
                            &mut buf,
                            4,
                            format!("Exponent: {} (0x{:x})", exponent, exponent),
                            1,
                        );
                    },
                    PublicKey::EC(_ecp) => {},
                    _ => {},
                }

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

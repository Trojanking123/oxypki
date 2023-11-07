
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use x509_parser::nom::AsBytes;
use x509_parser::prelude::*;

pub fn parser_cert<P: AsRef<Path>>( path: P ){
    let mut fd = File::open(path).unwrap();
    let mut buf = Vec::new();
    let a = fd.read_to_end(&mut buf).unwrap();
    let der = buf.as_bytes();

    let res = X509Certificate::from_der(der);
    match res {
        Ok((rem, cert)) => {
            assert!(rem.is_empty());
            //
            assert_eq!(cert.version(), X509Version::V3);
        },
        _ => panic!("x509 parsing failed: {:?}", res),
    }

}


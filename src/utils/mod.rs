use std::{fmt, str::FromStr};

pub mod fileio;

#[derive(Debug, PartialEq, Clone)]
pub enum FileFormat {
    PEM,
    DER,
}

impl FromStr for FileFormat {
    type Err = ();

    fn from_str(input: &str) -> Result<FileFormat, Self::Err> {
        match input {
            "pem" => Ok(FileFormat::PEM),
            "der" => Ok(FileFormat::DER),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FileFormat::PEM => write!(f, "pem"),
            FileFormat::DER => write!(f, "der"),
        }
    }
}

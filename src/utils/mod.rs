use std::{fmt, str::FromStr};

pub mod fileio;
pub mod format;

#[derive(Debug, PartialEq, Clone)]
pub enum FormatType {
    PEM,
    DER,
    JSON,
    TEXT,
}

impl FromStr for FormatType {
    type Err = ();

    fn from_str(input: &str) -> Result<FormatType, Self::Err> {
        match input {
            "pem" => Ok(FormatType::PEM),
            "der" => Ok(FormatType::DER),
            "json" => Ok(FormatType::DER),
            "text" => Ok(FormatType::DER),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FormatType::PEM => write!(f, "pem"),
            FormatType::DER => write!(f, "der"),
            FormatType::JSON => write!(f, "json"),
            FormatType::TEXT => write!(f, "text"),
        }
    }
}

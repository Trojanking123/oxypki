use std::{fmt, str::FromStr};

pub mod fileio;
pub mod format;

#[derive(Debug, PartialEq, Clone)]
pub enum FormatType {
    Pem,
    Der,
    Json,
    Text,
}

impl FromStr for FormatType {
    type Err = ();

    fn from_str(input: &str) -> Result<FormatType, Self::Err> {
        match input {
            "pem" => Ok(FormatType::Pem),
            "der" => Ok(FormatType::Der),
            "json" => Ok(FormatType::Json),
            "text" => Ok(FormatType::Text),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FormatType::Pem => write!(f, "pem"),
            FormatType::Der => write!(f, "der"),
            FormatType::Json => write!(f, "json"),
            FormatType::Text => write!(f, "text"),
        }
    }
}

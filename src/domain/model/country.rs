use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Country {
    FR,
}

impl Country {
    pub fn long_value(&self) -> String {
        match self {
            Country::FR => String::from("France"),
        }
    }

    pub fn short_value(&self) -> String {
        match self {
            Country::FR => String::from("FR"),
        }
    }

    pub fn from(country: &str) -> Result<Country, Error> {
        match country {
            "France" | "FR" => Ok(Country::FR),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid country")),
        }
    }
}

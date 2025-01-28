use crate::domain::model::french_address::FrenchAddress;
use crate::domain::model::structured_address::StructuredAddress;
use std::io::Error;

pub type ConvertFrenchToStructured<'a> =
    Box<dyn Fn(FrenchAddress) -> Result<StructuredAddress, Error> + 'a>;

pub type ConvertStructuredToFrench<'a> =
    Box<dyn Fn(StructuredAddress) -> Result<FrenchAddress, Error> + 'a>;

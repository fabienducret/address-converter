use crate::domain::error::AddressError;
use crate::domain::model::french_address::FrenchAddress;
use crate::domain::model::structured_address::StructuredAddress;

pub type ConvertFrenchToStructured<'a> =
    Box<dyn Fn(FrenchAddress) -> Result<StructuredAddress, AddressError> + 'a>;

pub type ConvertStructuredToFrench<'a> =
    Box<dyn Fn(StructuredAddress) -> Result<FrenchAddress, AddressError> + 'a>;

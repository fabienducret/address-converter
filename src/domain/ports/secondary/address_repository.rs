use crate::domain::error::AddressError;
use crate::domain::model::address::Address;

pub trait AddressRepository {
    fn save(&self, address: Address) -> Result<(), AddressError>;
}

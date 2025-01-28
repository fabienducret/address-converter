use crate::domain::model::address::Address;
use std::io::Error;

pub trait AddressRepository {
    fn save(&self, address: Address) -> Result<(), Error>;
}

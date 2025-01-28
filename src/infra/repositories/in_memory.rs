use crate::domain::model::address::Address;
use crate::domain::ports::secondary::address_repository::AddressRepository;
use std::cell::RefCell;

pub struct InMemoryAddressRepository {
    pub addresses: RefCell<Vec<Address>>,
}

impl InMemoryAddressRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        InMemoryAddressRepository {
            addresses: RefCell::new(Vec::new()),
        }
    }
}

impl AddressRepository for InMemoryAddressRepository {
    fn save(&self, address: Address) -> std::io::Result<()> {
        self.addresses.borrow_mut().push(address);

        Ok(())
    }
}

use crate::domain::model::structured_address::StructuredAddress;
use crate::domain::ports::primary::usecases::ConvertFrenchToStructured;
use crate::domain::ports::secondary::address_repository::AddressRepository;
use uuid::Uuid;

pub fn init_convert_french_to_structured_address<'a>(
    repository: &'a impl AddressRepository,
    generate_uuid: impl Fn() -> Uuid + 'a,
) -> ConvertFrenchToStructured<'a> {
    Box::new(move |french_address| {
        let address_id = generate_uuid().to_string();

        let address = french_address.to_address(address_id)?;
        let structured_address = StructuredAddress::from(address.clone())?;

        repository.save(address)?;

        Ok(structured_address)
    })
}

use crate::domain::model::french_address::FrenchAddress;
use crate::domain::ports::primary::usecases::ConvertStructuredToFrench;
use crate::domain::ports::secondary::address_repository::AddressRepository;
use uuid::Uuid;

pub fn init_convert_structured_to_french_address<'a>(
    repository: &'a impl AddressRepository,
    generate_uuid: impl Fn() -> Uuid + 'a,
) -> ConvertStructuredToFrench<'a> {
    Box::new(move |structured_address| {
        let address_id = generate_uuid().to_string();

        let address = structured_address.to_address(address_id)?;
        let french_address = FrenchAddress::from(address.clone())?;

        repository.save(address)?;

        Ok(french_address)
    })
}

#[cfg(test)]
mod tests {
    use crate::domain::model::address::Address;
    use crate::domain::model::country::Country;
    use crate::domain::model::french_address::FrenchAddress;
    use crate::domain::model::structured_address::StructuredAddress;
    use crate::domain::usecases::convert_structured_to_french::init_convert_structured_to_french_address;
    use crate::infra::repositories::in_memory::InMemoryAddressRepository;
    use uuid::{uuid, Uuid};

    fn generate_uuid() -> Uuid {
        uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")
    }

    #[test]
    fn convert_structured_to_french_and_save_to_repository() {
        let repository = InMemoryAddressRepository::new();
        let convert_french_to_structured =
            init_convert_structured_to_french_address(&repository, generate_uuid);

        let result = convert_french_to_structured(StructuredAddress {
            department: String::from(""),
            sub_department: String::from(""),
            building_name: String::from(""),
            floor: String::from("2ème étage"),
            room: String::from(""),
            street_name: String::from("Rue Rabelais"),
            building_number: String::from("4"),
            post_box: String::from("BP 90432"),
            town_location_name: String::from("MONTFERRIER SUR LEZ"),
            post_code: String::from("42300"),
            town_name: String::from("Roanne"),
            country: String::from("FR"),
            district_name: String::from(""),
            country_sub_division: String::from(""),
        });

        assert_eq!(
            result.unwrap(),
            FrenchAddress {
                line_1: "".to_string(),
                line_2: String::from(""),
                line_3: "2ème étage".to_string(),
                line_4: "4 Rue Rabelais".to_string(),
                line_5: String::from("BP 90432 MONTFERRIER SUR LEZ"),
                line_6: "42300 Roanne".to_string(),
                line_7: "France".to_string(),
            }
        );
        assert_eq!(
            repository.addresses.borrow_mut()[0],
            Address {
                id: String::from("67e55044-10b1-426f-9247-bb680e5fe0c8"),
                name: String::from(""),
                department: String::from(""),
                building_name: String::from(""),
                floor: String::from("2ème étage"),
                room: String::from(""),
                street_name: String::from("Rue Rabelais"),
                street_number: String::from("4"),
                post_box: String::from("BP 90432"),
                city_location_name: String::from("MONTFERRIER SUR LEZ"),
                postal_code: String::from("42300"),
                city: String::from("Roanne"),
                country: Country::FR,
            }
        );
    }

    #[test]
    fn dont_save_address_in_case_of_error() {
        let repository = InMemoryAddressRepository::new();
        let convert_french_to_structured =
            init_convert_structured_to_french_address(&repository, generate_uuid);

        let result = convert_french_to_structured(StructuredAddress {
            department: String::from(""),
            sub_department: String::from(""),
            building_name: String::from(""),
            floor: String::from("2ème étage"),
            room: String::from(""),
            street_name: String::from("Rue Rabelais"),
            building_number: String::from("4"),
            post_box: String::from("BP 90432"),
            town_location_name: String::from("MONTFERRIER SUR LEZ verrrryyyyyy loooong"), // too long field
            post_code: String::from("42300"),
            town_name: String::from("Roanne"),
            country: String::from("FR"),
            district_name: String::from(""),
            country_sub_division: String::from(""),
        });

        assert!(result.is_err());
        assert!(repository.addresses.borrow_mut().is_empty());
    }
}

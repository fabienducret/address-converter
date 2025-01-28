#[cfg(test)]
mod tests {
    use crate::domain::model::address::Address;
    use crate::domain::model::country::Country;
    use crate::domain::model::structured_address::StructuredAddress;

    #[test]
    fn create_structured_individual_address_from_address() {
        let address = Address {
            id: String::from("id"),
            name: String::from("M Fabien Ducret"),
            department: String::from(""),
            building_name: String::from(""),
            floor: String::from("2ème étage"),
            room: String::from(""),
            street_name: String::from("Rue Rabelais"),
            street_number: String::from("4"),
            post_box: String::from(""),
            city_location_name: String::from(""),
            postal_code: String::from("42300"),
            city: String::from("Roanne"),
            country: Country::FR,
        };

        let structured_address = StructuredAddress::from(address).unwrap();

        assert_eq!(
            structured_address,
            StructuredAddress {
                department: String::from(""),
                sub_department: String::from(""),
                building_name: String::from(""),
                floor: String::from("2ème étage"),
                room: String::from(""),
                street_name: String::from("Rue Rabelais"),
                building_number: String::from("4"),
                post_box: String::from(""),
                town_location_name: String::from(""),
                post_code: String::from("42300"),
                town_name: String::from("Roanne"),
                country: String::from("FR"),
                district_name: String::from(""),
                country_sub_division: String::from(""),
            }
        );
    }

    #[test]
    fn create_structured_enterprise_address_from_address() {
        let address = Address {
            id: String::from("id"),
            name: String::from(""),
            department: String::from("Mademoiselle Lucie MARTIN"),
            building_name: String::from(""),
            floor: String::from("Résidence des Capucins Bâtiment Quater"),
            room: String::from(""),
            street_name: String::from("56 RUE EMILE ZOLA"),
            street_number: String::from(""),
            post_box: String::from("BP 90432"),
            city_location_name: String::from("MONTFERRIER SUR LEZ"),
            postal_code: String::from("34092"),
            city: String::from("MONTPELLIER CEDEX 5"),
            country: Country::FR,
        };

        let structured_address = StructuredAddress::from(address).unwrap();

        assert_eq!(
            structured_address,
            StructuredAddress {
                department: String::from("Mademoiselle Lucie MARTIN"),
                sub_department: String::from(""),
                building_name: String::from(""),
                floor: String::from("Résidence des Capucins Bâtiment Quater"),
                room: String::from(""),
                street_name: String::from("56 RUE EMILE ZOLA"),
                building_number: String::from(""),
                post_box: String::from("BP 90432"),
                town_location_name: String::from("MONTFERRIER SUR LEZ"),
                post_code: String::from("34092"),
                town_name: String::from("MONTPELLIER CEDEX 5"),
                country: String::from("FR"),
                district_name: String::from(""),
                country_sub_division: String::from(""),
            }
        );
    }

    #[test]
    fn building_name_is_too_long() {
        let address = Address {
            id: String::from("id"),
            name: String::from(""),
            department: String::from(""),
            building_name: String::from("a with a veryyyyy long building name"),
            floor: String::from(""),
            room: String::from(""),
            street_name: String::from(""),
            street_number: String::from(""),
            post_box: String::from(""),
            city_location_name: String::from(""),
            postal_code: String::from(""),
            city: String::from(""),
            country: Country::FR,
        };

        assert!(StructuredAddress::from(address).is_err());
    }

    #[test]
    fn create_address_from_structured_enterprise_address() {
        let id = String::from("id");
        let structured_address = StructuredAddress {
            department: String::from("Mademoiselle Lucie MARTIN"),
            sub_department: String::from(""),
            building_name: String::from(""),
            floor: String::from("Résidence des Capucins Bâtiment Quater"),
            room: String::from(""),
            street_name: String::from("RUE EMILE ZOLA"),
            building_number: String::from("56"),
            post_box: String::from("BP 90432"),
            town_location_name: String::from("MONTFERRIER SUR LEZ"),
            post_code: String::from("34092"),
            town_name: String::from("MONTPELLIER CEDEX 5"),
            country: String::from("FR"),
            district_name: String::from(""),
            country_sub_division: String::from(""),
        };

        let address = structured_address.to_address(id.clone()).unwrap();

        assert_eq!(
            address,
            Address {
                id,
                name: String::from(""),
                department: String::from("Mademoiselle Lucie MARTIN"),
                building_name: String::from(""),
                floor: String::from("Résidence des Capucins Bâtiment Quater"),
                room: String::from(""),
                street_number: String::from("56"),
                street_name: String::from("RUE EMILE ZOLA"),
                post_box: String::from("BP 90432"),
                city_location_name: String::from("MONTFERRIER SUR LEZ"),
                postal_code: String::from("34092"),
                city: String::from("MONTPELLIER CEDEX 5"),
                country: Country::FR,
            }
        );
    }
}

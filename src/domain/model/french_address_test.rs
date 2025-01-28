#[cfg(test)]
mod tests {
    use crate::domain::model::address::Address;
    use crate::domain::model::country::Country;
    use crate::domain::model::french_address::FrenchAddress;

    #[test]
    fn create_french_individual_address_from_address() {
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

        let french_address = FrenchAddress::from(address).unwrap();

        assert_eq!(
            french_address,
            FrenchAddress {
                line_1: String::from("M Fabien Ducret"),
                line_2: String::from(""),
                line_3: String::from("2ème étage"),
                line_4: String::from("4 Rue Rabelais"),
                line_5: String::from(""),
                line_6: String::from("42300 Roanne"),
                line_7: String::from("France"),
            }
        );
    }

    #[test]
    fn create_french_enterprise_address_from_address() {
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

        let french_address = FrenchAddress::from(address).unwrap();

        assert_eq!(
            french_address,
            FrenchAddress {
                line_1: String::from(""),
                line_2: String::from("Mademoiselle Lucie MARTIN"),
                line_3: String::from("Résidence des Capucins Bâtiment Quater"),
                line_4: String::from("56 RUE EMILE ZOLA"),
                line_5: String::from("BP 90432 MONTFERRIER SUR LEZ"),
                line_6: String::from("34092 MONTPELLIER CEDEX 5"),
                line_7: String::from("France"),
            }
        );
    }

    #[test]
    fn line_1_is_too_long() {
        let address = Address {
            id: String::from("id"),
            name: String::from("M Fabien Ducret, a very toooo long name"),
            department: String::from(""),
            building_name: String::from(""),
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

        assert!(FrenchAddress::from(address).is_err());
    }

    #[test]
    fn create_address_from_french_individual_address_with_invalid_street_number() {
        let id = String::from("id");
        let french_address = FrenchAddress {
            line_1: String::from(""),
            line_2: String::from(""),
            line_3: String::from(""),
            line_4: String::from("25D RUE DES FLEURS"), // invalid street number
            line_5: String::from(""),
            line_6: String::from("33500 LIBOURNE"),
            line_7: String::from("France"),
        };

        let address = french_address.to_address(id.clone()).unwrap();

        assert_eq!(
            address,
            Address {
                id,
                name: String::from(""),
                department: String::from(""),
                building_name: String::from(""),
                floor: String::from(""),
                room: String::from(""),
                street_number: String::from(""),
                street_name: String::from("25D RUE DES FLEURS"),
                post_box: String::from(""),
                city_location_name: String::from(""),
                postal_code: String::from("33500"),
                city: String::from("LIBOURNE"),
                country: Country::FR,
            }
        );
    }

    #[test]
    fn create_address_from_french_individual_address_with_valid_street_number() {
        let id = String::from("id");
        let french_address = FrenchAddress {
            line_1: String::from(""),
            line_2: String::from(""),
            line_3: String::from(""),
            line_4: String::from("25 RUE DES FLEURS"), // valid street number
            line_5: String::from(""),
            line_6: String::from("33500 LIBOURNE"),
            line_7: String::from("France"),
        };

        let address = french_address.to_address(id.clone()).unwrap();

        assert_eq!(
            address,
            Address {
                id,
                name: String::from(""),
                department: String::from(""),
                building_name: String::from(""),
                floor: String::from(""),
                room: String::from(""),
                street_number: String::from("25"),
                street_name: String::from("RUE DES FLEURS"),
                post_box: String::from(""),
                city_location_name: String::from(""),
                postal_code: String::from("33500"),
                city: String::from("LIBOURNE"),
                country: Country::FR,
            }
        );
    }

    #[test]
    fn create_address_from_french_enterprise_address() {
        let id = String::from("id");
        let french_address = FrenchAddress {
            line_1: String::from("Service des achats"),
            line_2: String::from("Mademoiselle Lucie MARTIN"),
            line_3: String::from("Résidence des Capucins Bâtiment Quater"),
            line_4: String::from("56 RUE EMILE ZOLA"),
            line_5: String::from("BP 90432 MONTFERRIER SUR LEZ"),
            line_6: String::from("34092 MONTPELLIER CEDEX 5"),
            line_7: String::from("France"),
        };

        let address = french_address.to_address(id.clone()).unwrap();

        assert_eq!(
            address,
            Address {
                id,
                name: String::from("Service des achats"),
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

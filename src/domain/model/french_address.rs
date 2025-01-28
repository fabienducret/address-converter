use crate::domain::model::address::Address;
use crate::domain::model::country::Country;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct FrenchAddress {
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub line_4: String,
    pub line_5: String,
    pub line_6: String,
    pub line_7: String,
}

impl FrenchAddress {
    pub fn from(address: Address) -> Result<FrenchAddress, Error> {
        let french_address = FrenchAddress {
            line_1: address.name,
            line_2: address.department,
            line_3: address.floor,
            line_4: format!("{} {}", address.street_number, address.street_name)
                .trim()
                .to_string(),
            line_5: format!("{} {}", address.post_box, address.city_location_name)
                .trim()
                .to_string(),
            line_6: format!("{} {}", address.postal_code, address.city)
                .trim()
                .to_string(),
            line_7: address.country.long_value(),
        };

        if !french_address.is_valid() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid address"));
        }

        Ok(french_address)
    }

    pub fn to_address(&self, id: String) -> Result<Address, Error> {
        let (street_number, street_name) = street_number_and_street_name(&self.line_4);
        let (post_box, city_location_name) = post_box_and_city_location_name(&self.line_5);
        let (postal_code, city) = postal_code_and_city(&self.line_6);
        let country = Country::from(&self.line_7)?;

        Ok(Address {
            id,
            name: self.line_1.clone(),
            department: self.line_2.clone(),
            floor: self.line_3.clone(),
            street_number: street_number.clone(),
            street_name: street_name.clone(),
            post_box: post_box.clone(),
            city_location_name: city_location_name.clone(),
            postal_code: postal_code.clone(),
            city: city.clone(),
            building_name: String::from(""),
            room: String::from(""),
            country,
        })
    }

    fn is_valid(&self) -> bool {
        let max_field_length = 38;

        if self.line_1.chars().count() > max_field_length
            || self.line_2.chars().count() > max_field_length
            || self.line_3.chars().count() > max_field_length
            || self.line_4.chars().count() > max_field_length
            || self.line_5.chars().count() > max_field_length
            || self.line_6.chars().count() > max_field_length
            || self.line_7.chars().count() > max_field_length
        {
            return false;
        }

        true
    }
}

// line can contain both street number and street
// if we can't get a valid street number, we return the value as it is
// example of line: "56 RUE EMILE ZOLA"
fn street_number_and_street_name(line: &str) -> (String, String) {
    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.is_empty() {
        return ("".to_string(), "".to_string());
    }

    let has_building_number = parts[0].parse::<u32>().is_ok();

    if !has_building_number {
        return ("".to_string(), parts[0..].join(" "));
    }

    (parts[0].to_string(), parts[1..].join(" "))
}

// line can contain both post box and city location name
// a valid post box must be something like "BP 90432": a string followed by a number
// example of line: "BP 90432 MONTFERRIER SUR LEZ"
fn post_box_and_city_location_name(line: &str) -> (String, String) {
    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.len() <= 2 {
        return ("".to_string(), parts[0..].join(" "));
    }

    let has_bp = parts[0].parse::<String>().is_ok() && parts[1].parse::<u32>().is_ok();

    if !has_bp {
        return ("".to_string(), parts[0..].join(" "));
    }

    (parts[0..2].join(" "), parts[2..].join(" "))
}

// we always consider the first part as the postal code and the rest as the city
// example of line: "34092 MONTPELLIER CEDEX 5"
fn postal_code_and_city(line: &str) -> (String, String) {
    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.is_empty() {
        return ("".to_string(), "".to_string());
    }

    (parts[0].to_string(), parts[1..].join(" "))
}

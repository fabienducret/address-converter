use crate::domain::model::address::Address;
use crate::domain::model::country::Country;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct StructuredAddress {
    pub department: String,
    pub sub_department: String,
    pub building_name: String,
    pub floor: String,
    pub room: String,
    pub street_name: String,
    pub building_number: String,
    pub post_box: String,
    pub town_location_name: String,
    pub post_code: String,
    pub town_name: String,
    pub country: String,
    pub district_name: String,
    pub country_sub_division: String,
}

impl StructuredAddress {
    pub fn from(address: Address) -> Result<StructuredAddress, Error> {
        let structured_address = StructuredAddress {
            department: address.department,
            sub_department: String::from(""),
            building_name: address.building_name,
            floor: address.floor,
            room: address.room,
            street_name: address.street_name,
            building_number: address.street_number,
            post_box: address.post_box,
            town_location_name: address.city_location_name,
            post_code: address.postal_code,
            town_name: address.city,
            country: address.country.short_value(),
            district_name: String::from(""),
            country_sub_division: String::from(""),
        };

        if !structured_address.is_valid() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid address"));
        }

        Ok(structured_address)
    }

    pub fn to_address(&self, id: String) -> Result<Address, Error> {
        let country = Country::from(&self.country)?;

        Ok(Address {
            id,
            name: String::from(""),
            department: self.department.clone(),
            building_name: self.building_name.clone(),
            floor: self.floor.clone(),
            room: self.room.clone(),
            street_name: self.street_name.clone(),
            street_number: self.building_number.clone(),
            post_box: self.post_box.clone(),
            city_location_name: self.town_location_name.clone(),
            postal_code: self.post_code.clone(),
            city: self.town_name.clone(),
            country,
        })
    }

    fn is_valid(&self) -> bool {
        let max_long_field_length = 70;
        let max_middle_field_length = 35;
        let max_short_field_length = 16;
        let max_country_field_length = 2;

        if self.department.chars().count() > max_long_field_length
            || self.sub_department.chars().count() > max_long_field_length
            || self.building_name.chars().count() > max_middle_field_length
            || self.floor.chars().count() > max_long_field_length
            || self.room.chars().count() > max_long_field_length
            || self.street_name.chars().count() > max_long_field_length
            || self.building_number.chars().count() > max_short_field_length
            || self.post_box.chars().count() > max_short_field_length
            || self.town_location_name.chars().count() > max_middle_field_length
            || self.post_code.chars().count() > max_short_field_length
            || self.town_name.chars().count() > max_middle_field_length
            || self.country.chars().count() > max_country_field_length
        {
            return false;
        }

        true
    }
}

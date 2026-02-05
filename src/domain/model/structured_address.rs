use crate::domain::error::AddressError;
use crate::domain::model::address::Address;
use crate::domain::model::country::Country;

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
    pub fn from(address: Address) -> Result<StructuredAddress, AddressError> {
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

        structured_address.validate()?;

        Ok(structured_address)
    }

    pub fn to_address(&self, id: String) -> Result<Address, AddressError> {
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

    fn validate(&self) -> Result<(), AddressError> {
        let max_long_field_length = 70;
        let max_middle_field_length = 35;
        let max_short_field_length = 16;
        let max_country_field_length = 2;

        let fields: [(&str, &str, usize); 12] = [
            ("department", &self.department, max_long_field_length),
            ("sub_department", &self.sub_department, max_long_field_length),
            ("building_name", &self.building_name, max_middle_field_length),
            ("floor", &self.floor, max_long_field_length),
            ("room", &self.room, max_long_field_length),
            ("street_name", &self.street_name, max_long_field_length),
            ("building_number", &self.building_number, max_short_field_length),
            ("post_box", &self.post_box, max_short_field_length),
            ("town_location_name", &self.town_location_name, max_middle_field_length),
            ("post_code", &self.post_code, max_short_field_length),
            ("town_name", &self.town_name, max_middle_field_length),
            ("country", &self.country, max_country_field_length),
        ];

        for (name, value, max) in fields {
            let len = value.chars().count();
            if len > max {
                return Err(AddressError::ValidationError {
                    field: name.to_string(),
                    max,
                    actual: len,
                });
            }
        }

        Ok(())
    }
}

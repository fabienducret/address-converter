use crate::domain::model::country::Country;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: String,
    pub name: String,
    pub department: String,
    pub building_name: String,
    pub floor: String,
    pub room: String,
    pub street_name: String,
    pub street_number: String,
    pub post_box: String,
    pub city_location_name: String,
    pub postal_code: String,
    pub city: String,
    pub country: Country,
}

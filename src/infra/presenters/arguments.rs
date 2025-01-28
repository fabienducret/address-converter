use crate::domain::model::french_address::FrenchAddress;
use crate::domain::model::structured_address::StructuredAddress;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum AddressKind {
    French,
    Structured,
}

#[derive(Parser)]
pub struct Arguments {
    #[clap(long, value_enum)]
    pub address_kind: AddressKind,
    #[clap(long, default_value_t)]
    pub line1: String,
    #[clap(long, default_value_t)]
    pub line2: String,
    #[clap(long, default_value_t)]
    pub line3: String,
    #[clap(long, default_value_t)]
    pub line4: String,
    #[clap(long, default_value_t)]
    pub line5: String,
    #[clap(long, default_value_t)]
    pub line6: String,
    #[clap(long, default_value_t)]
    pub line7: String,
    #[clap(long, default_value_t)]
    pub department: String,
    #[clap(long, default_value_t)]
    pub sub_department: String,
    #[clap(long, default_value_t)]
    pub building_name: String,
    #[clap(long, default_value_t)]
    pub floor: String,
    #[clap(long, default_value_t)]
    pub room: String,
    #[clap(long, default_value_t)]
    pub street_name: String,
    #[clap(long, default_value_t)]
    pub building_number: String,
    #[clap(long, default_value_t)]
    pub post_box: String,
    #[clap(long, default_value_t)]
    pub town_location_name: String,
    #[clap(long, default_value_t)]
    pub post_code: String,
    #[clap(long, default_value_t)]
    pub town_name: String,
    #[clap(long, default_value_t)]
    pub country: String,
    #[clap(long, default_value_t)]
    pub district_name: String,
    #[clap(long, default_value_t)]
    pub country_sub_division: String,
}

pub fn args_to_french_address(args: Arguments) -> FrenchAddress {
    FrenchAddress {
        line_1: args.line1,
        line_2: args.line2,
        line_3: args.line3,
        line_4: args.line4,
        line_5: args.line5,
        line_6: args.line6,
        line_7: args.line7,
    }
}

pub fn args_to_structured_address(args: Arguments) -> StructuredAddress {
    StructuredAddress {
        department: args.department,
        sub_department: args.sub_department,
        building_name: args.building_name,
        floor: args.floor,
        room: args.room,
        street_name: args.street_name,
        building_number: args.building_number,
        post_box: args.post_box,
        town_location_name: args.town_location_name,
        post_code: args.post_code,
        town_name: args.town_name,
        country: args.country,
        district_name: args.district_name,
        country_sub_division: args.country_sub_division,
    }
}

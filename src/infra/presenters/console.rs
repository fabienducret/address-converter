use crate::domain::ports::primary::usecases::{
    ConvertFrenchToStructured, ConvertStructuredToFrench,
};
use crate::infra::presenters::arguments::{
    args_to_french_address, args_to_structured_address, Arguments,
};
use std::io::Error;

pub struct ConsolePresenter<'a> {
    convert_french_to_structured: ConvertFrenchToStructured<'a>,
    convert_structured_to_french: ConvertStructuredToFrench<'a>,
}

impl<'a> ConsolePresenter<'a> {
    pub fn new(
        convert_french_to_structured: ConvertFrenchToStructured<'a>,
        convert_structured_to_french: ConvertStructuredToFrench<'a>,
    ) -> Self {
        ConsolePresenter {
            convert_french_to_structured,
            convert_structured_to_french,
        }
    }

    pub fn french_to_structured(&self, args: Arguments) {
        let french_address = args_to_french_address(args);
        let result = (self.convert_french_to_structured)(french_address);

        result.map_or_else(display_error, |structured_address| {
            println!("{:?}", structured_address)
        });
    }

    pub fn structured_to_french(&self, args: Arguments) {
        let structured_address = args_to_structured_address(args);
        let result = (self.convert_structured_to_french)(structured_address);

        result.map_or_else(display_error, |french_address| {
            println!("{:?}", french_address)
        });
    }
}

fn display_error(error: Error) {
    eprintln!("{}", error)
}

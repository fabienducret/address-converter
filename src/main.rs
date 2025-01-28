use crate::domain::usecases::convert_french_to_structured::init_convert_french_to_structured_address;
use crate::domain::usecases::convert_structured_to_french::init_convert_structured_to_french_address;
use crate::infra::config;
use crate::infra::presenters::arguments::{AddressKind, Arguments};
use crate::infra::presenters::console::ConsolePresenter;
use crate::infra::repositories::file::FileAddressRepository;
use clap::Parser;
use uuid::Uuid;

mod domain;
mod infra;

fn main() {
    let config = config::init();

    // we can change the address repository here (a real database for example)
    let address_repository = FileAddressRepository::new(config.db_file_path);
    let convert_french_to_structured =
        init_convert_french_to_structured_address(&address_repository, Uuid::new_v4);
    let convert_structured_to_french =
        init_convert_structured_to_french_address(&address_repository, Uuid::new_v4);
    // we can change the presenter here (an API for example)
    let console_presenter =
        ConsolePresenter::new(convert_french_to_structured, convert_structured_to_french);

    let arguments = Arguments::parse();

    match arguments.address_kind {
        AddressKind::French => console_presenter.french_to_structured(arguments),
        AddressKind::Structured => console_presenter.structured_to_french(arguments),
    }
}

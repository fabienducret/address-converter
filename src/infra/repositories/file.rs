use crate::domain::model::address::Address;
use crate::domain::ports::secondary::address_repository::AddressRepository;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct FileAddressRepository {
    file_path: String,
}

impl FileAddressRepository {
    pub fn new(file_path: String) -> Self {
        FileAddressRepository { file_path }
    }

    fn get_all(&self) -> Result<Vec<Address>, std::io::Error> {
        let data = fs::read_to_string(self.file_path.clone())?;

        if data.is_empty() {
            return Ok(vec![]);
        }

        let addresses: Vec<Address> = serde_json::from_str(&data)?;

        Ok(addresses)
    }
}

impl AddressRepository for FileAddressRepository {
    fn save(&self, address: Address) -> std::io::Result<()> {
        if file_does_not_exist(&self.file_path) {
            get_or_create_file(&self.file_path)?;
        }

        let mut addresses: Vec<Address> = self.get_all()?;
        addresses.push(address);
        write_to_file(&self.file_path, addresses)?;

        Ok(())
    }
}

fn file_does_not_exist(file_path: &str) -> bool {
    !Path::new(file_path).exists()
}

fn get_or_create_file(file_path: &str) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
}

fn write_to_file(file_path: &str, addresses: Vec<Address>) -> std::io::Result<()> {
    let file = get_or_create_file(file_path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &addresses)?;
    writer.flush()?;

    Ok(())
}

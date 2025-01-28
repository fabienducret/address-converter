use std::fs;
use std::fs::OpenOptions;
use std::process::Command;

const DB_PATH: &str = "./tests/db-test.json";

#[test]
fn test_cli_execution_of_two_commands() {
    clear_file();

    execute_command_french_to_structured();
    execute_command_structured_to_french();

    let file_content = read_file_content();
    assert!(file_content.contains("\"name\":\"M Fabien Ducret\",\"department\":\"\",\"building_name\":\"\",\"floor\":\"2ème étage\",\"room\":\"\",\"street_name\":\"Rue Rabelais\",\"street_number\":\"4\",\"post_box\":\"\",\"city_location_name\":\"\",\"postal_code\":\"42300\",\"city\":\"Roanne\",\"country\":\"FR\"}"));
    assert!(file_content.contains("\"name\":\"\",\"department\":\"Service achat\",\"building_name\":\"\",\"floor\":\"Zone industrielle de la Ballastrierre\",\"room\":\"\",\"street_name\":\"22BIS RUE DES FLEURS\",\"street_number\":\"\",\"post_box\":\"BP 40122\",\"city_location_name\":\"\",\"postal_code\":\"33506\",\"city\":\"LIBOURNE CEDEX\",\"country\":\"FR\"}"));
}

fn execute_command_french_to_structured() {
    Command::new("cargo")
        .env("DB_FILE", DB_PATH)
        .arg("run")
        .arg("--")
        .arg("--address-kind")
        .arg("french")
        .arg("--line1")
        .arg("M Fabien Ducret")
        .arg("--line3")
        .arg("2ème étage")
        .arg("--line4")
        .arg("4 Rue Rabelais")
        .arg("--line6")
        .arg("42300 Roanne")
        .arg("--line7")
        .arg("France")
        .output()
        .expect("Failed to execute command");
}

fn execute_command_structured_to_french() {
    Command::new("cargo")
        .env("DB_FILE", DB_PATH)
        .arg("run")
        .arg("--")
        .arg("--address-kind")
        .arg("structured")
        .arg("--department")
        .arg("Service achat")
        .arg("--street-name")
        .arg("22BIS RUE DES FLEURS")
        .arg("--floor")
        .arg("Zone industrielle de la Ballastrierre")
        .arg("--post-box")
        .arg("BP 40122")
        .arg("--post-code")
        .arg("33506")
        .arg("--town-name")
        .arg("LIBOURNE CEDEX")
        .arg("--country")
        .arg("FR")
        .output()
        .expect("Failed to execute command");
}

fn clear_file() {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DB_PATH)
        .expect("Failed to open file");
}

fn read_file_content() -> String {
    fs::read_to_string(DB_PATH).expect("Failed to read file")
}

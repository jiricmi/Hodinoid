use super::files::{read_file, read_input, save_file};
use serde::{Deserialize, Serialize};
use std::panic::panic_any;

pub const CONFIG_NAME: &str = "hodinoid_config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub person: Person,
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    ic: String,
    dic: String,
    email: String,
    phone: String,
    address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    city: String,
    street: String,
    building_number: String,
    post_code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyConfig {
    pub company_info: CompanyInfo,
    pub company_address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInfo {
    pub name: String,
}

pub fn config_cli_first_setup() -> Config {
    let person = get_input_person();
    let location = get_input_location();

    let config = Config { person, location };
    match toml::to_string(&config) {
        Ok(config_str) => save_file(CONFIG_NAME, config_str),
        Err(e) => {
            println!("Error while parsing");
            panic_any(e);
        }
    }

    return config;
}

fn get_input_person() -> Person {
    let first_name = read_input("Enter first name: ");
    let last_name = read_input("Enter last name: ");
    let ic = read_input("Enter IC: ");
    let dic = read_input("Enter DIC: ");
    let email = read_input("Enter email: ");
    let phone = read_input("Enter phone: ");

    let city = read_input("Enter city: ");
    let street = read_input("Enter street: ");
    let building_number = read_input("Enter building number: ");
    let post_code = read_input("Enter post code: ");
    let state = read_input("Enter state: ");

    return Person {
        first_name,
        last_name,
        ic,
        dic,
        email,
        phone,
        address: Address {
            city,
            street,
            building_number,
            post_code,
            state,
        },
    };
}

fn get_input_location() -> Location {
    let root = read_input("Enter path to location: ");
    return Location { root };
}

pub fn load_company_config(path: &str) -> Result<CompanyConfig, Box<dyn std::error::Error>> {
    let config_string = read_file(path)?;
    let config: CompanyConfig = toml::from_str(&config_string).expect("pes");
    Ok(config)
}

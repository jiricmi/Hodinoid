use super::files::{read_file, read_input, save_file};
use crate::utils::files::{get_config_dir, APP_DIR_NAME};
use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::panic::panic_any;
use std::path::PathBuf;

const CONFIG_NAME: &str = "config.toml";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInfo {
    pub name: String,
    pub hour_pay: u32,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractReportTime {
    pub description: String,
    pub date: String,
    pub from: String,
    pub to: String,
    pub location: String,
    note: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContractReportNonTime {
    description: String,
    date: String,
    value: i32,
    note: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractFile {
    pub info: ContractInfo,
    pub report_time: Vec<ContractReportTime>, // Dynamické pole
    pub report_non_time: Vec<ContractReportNonTime>, // Dynamické pole
}

fn get_app_config_dir() -> PathBuf {
    get_config_dir().join(APP_DIR_NAME)
}

pub fn get_config_path() -> PathBuf {
    get_app_config_dir().join(CONFIG_NAME)
}

pub fn config_cli_first_setup() -> Config {
    let config_dir = get_app_config_dir();

    if !config_dir.exists() {
        create_dir_all(config_dir).expect("Cannot be created dir!");
    }

    let person = get_input_person();
    let location = get_input_location();

    let config = Config { person, location };
    match toml::to_string(&config) {
        Ok(config_str) => save_file(get_config_path(), config_str),
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
    let address = get_input_address();
    return Person {
        first_name,
        last_name,
        ic,
        dic,
        email,
        phone,
        address,
    };
}

pub fn get_input_company_info() -> CompanyInfo {
    let name = read_input("Enter company name: ");

    return CompanyInfo { name };
}

pub fn get_input_address() -> Address {
    let city = read_input("Enter city: ");
    let street = read_input("Enter street: ");
    let building_number = read_input("Enter building number: ");
    let post_code = read_input("Enter post code: ");
    let state = read_input("Enter state: ");

    return Address {
        city,
        street,
        building_number,
        post_code,
        state,
    };
}

fn get_input_location() -> Location {
    let config_dir = get_app_config_dir();
    let data_dir = config_dir.join("data");

    if !data_dir.exists() {
        create_dir_all(&data_dir).expect("Cannot be created dir");
    }

    let root = data_dir.to_string_lossy().to_string();
    return Location { root };
}

pub fn load_company_config(path: &str) -> Result<CompanyConfig, Box<dyn std::error::Error>> {
    let config_string = read_file(path)?;
    let config: CompanyConfig = toml::from_str(&config_string).expect("pes");
    Ok(config)
}

pub fn create_contract_config() -> ContractFile {
    let name = read_input("Enter contract name: ");
    let hour_pay: u32 = read_input("Enter hour pay number: ")
        .parse()
        .expect("Cannot parse pay to int");
    let note = read_input("Enter note: ");

    let info = ContractInfo {
        name,
        hour_pay,
        note,
    };

    let report_time: Vec<ContractReportTime> = vec![];
    let report_non_time: Vec<ContractReportNonTime> = vec![];

    return ContractFile {
        info,
        report_time,
        report_non_time,
    };
}

fn validate_date(date_str: &str) -> bool {
    let re = Regex::new(r"^\d{2}\.\d{2}\.\d{4}$").unwrap();
    re.is_match(date_str)
}

fn validate_time(time_str: &str) -> bool {
    let re = Regex::new(r"^\d{2}:\d{2}$").unwrap();
    re.is_match(time_str)
}

fn validate_location(location: &str) -> bool {
    location == "HO" || location == "Office" || location.is_empty()
}

pub fn get_time_contract_record() -> ContractReportTime {
    let mut date = read_input("Enter date dd.mm.yyyy (leave blank for today): ");
    let description = read_input("Enter description of the job: ");
    let from = read_input("From hh:mm format: ");
    let to = read_input("To hh:mm format: ");
    let location = read_input("HO/Office (leave blank for Office)");
    let note = read_input("Add note: ");

    if date.is_empty() {
        date = Local::now().format("%d.%m.%Y").to_string();
    }

    if !date.is_empty() && !validate_date(&date) {
        println!("Invalid date format. Please use dd.mm.yyyy.");
    }

    if !validate_time(&from) {
        println!("Invalid 'From' time format. Please use hh:mm.");
    }

    if !validate_time(&to) {
        println!("Invalid 'To' time format. Please use hh:mm.");
    }

    if !validate_location(&location) {
        println!("Invalid location. Please enter 'HO' or 'Office'.");
    }

    ContractReportTime {
        description,
        date,
        from,
        to,
        location,
        note,
    }
}

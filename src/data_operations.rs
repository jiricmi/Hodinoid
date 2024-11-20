use crate::utils::config::{
    create_contract_config, get_input_address, get_input_company_info, get_time_contract_record,
    load_company_config, CompanyConfig, ContractFile,
};
use crate::utils::files::{ensure_path, read_file, save_file};
use serde_json;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::io::Result;

const COMPANY_CONFIG_NAME: &str = "config.toml";

pub fn load_companies(path: &str) -> Result<Vec<(i32, String)>> {
    let mut companies = Vec::new();

    let entries = read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();

        if entry_path.is_dir() {
            if let Some(file_name) = entry_path.file_name() {
                let company_id_str = file_name.to_str().unwrap();
                let company_id: i32 = company_id_str.parse().expect("Cannot parse id");
                let company_name = get_company_name(path, company_id);
                companies.push((company_id, company_name));
            }
        }
    }
    Ok(companies)
}

fn get_company_name(company_path: &str, company_id: i32) -> String {
    let id_str = company_id.to_string();
    let path = format!("{}/{}/{}", company_path, id_str, COMPANY_CONFIG_NAME);

    match load_company_config(&path) {
        Ok(company_config) => {
            return company_config.company_info.name;
        }
        Err(_) => {
            println!("Cannot find company config");
            return "".to_string();
        }
    }
}

pub fn create_company(root_path: &str) {
    let companies = load_companies(&root_path).expect("Cannot load companies");
    let mut id = 0;
    loop {
        let mut flag = false;
        for (company_id, _) in &companies {
            if *company_id == id {
                flag = true;
                break;
            }
        }
        if !flag {
            break;
        }
        id += 1;
    }

    let company_info = get_input_company_info();
    let company_address = get_input_address();
    let company_config = CompanyConfig {
        company_info,
        company_address,
    };

    let company_path = format!("{}/{}", root_path, id.to_string());
    ensure_path(&company_path).expect("Should ensure");

    let config_file = format!("{}/{}", company_path, COMPANY_CONFIG_NAME);
    match toml::to_string(&company_config) {
        Ok(company_string) => save_file(&config_file, company_string),
        Err(_) => println!("Cannot create company!"),
    }
}

pub fn load_contracts(path: &str) -> Result<Vec<(i32, String)>> {
    let mut contracts = Vec::new();

    let entries = read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();

        if !entry_path.is_dir() && entry_path.extension().map_or(false, |ext| ext == "json") {
            if let Some(file_name) = entry_path.file_stem() {
                let content = read_to_string(&entry_path)?;

                let contract_data: ContractFile = serde_json::from_str(&content)?;
                let contract_id_str = file_name.to_str().unwrap();
                let contract_id: i32 = contract_id_str.parse().expect("cannto parse id");
                let contract_name = contract_data.info.name;
                contracts.push((contract_id, contract_name));
            }
        }
    }

    Ok(contracts)
}

pub fn create_contract(company_path: &str) {
    let contracts = load_contracts(&company_path).expect("Cannot load contracts");
    let mut id = 0;
    loop {
        let mut flag = false;
        for (contract_id, _) in &contracts {
            if *contract_id == id {
                flag = true;
                break;
            }
        }
        if !flag {
            break;
        }
        id += 1;
    }

    let contract_config = create_contract_config();

    let contract_path = format!("{}/{}.json", company_path, id.to_string());

    match serde_json::to_string(&contract_config) {
        Ok(contract_string) => save_file(&contract_path, contract_string),
        Err(_) => println!("Cannot create contract!"),
    }
}

pub fn load_contract_content(company_path: &str, contract_id: i32) -> ContractFile {
    let contract_path = format!("{}/{}.json", company_path, contract_id.to_string());
    let content = read_file(&contract_path).expect("Cannot read file");

    let contract_file: ContractFile = serde_json::from_str(&content).expect("Cannot parse json");
    return contract_file;
}

pub fn create_time_record(contract_file: &mut ContractFile, contract_path: &str) {
    let time_record = get_time_contract_record();
    contract_file.report_time.push(time_record);

    match serde_json::to_string(&contract_file) {
        Ok(contract_string) => save_file(&contract_path, contract_string),
        Err(_) => println!("Cannot rewrite contract"),
    }
}

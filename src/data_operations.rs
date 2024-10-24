use crate::utils::config::{
    get_input_address, get_input_company_info, load_company_config, CompanyConfig,
};
use crate::utils::files::{ensure_path, save_file};
use std::fs::read_dir;
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

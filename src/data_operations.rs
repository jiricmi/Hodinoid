use super::utils::config::load_company_config;
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

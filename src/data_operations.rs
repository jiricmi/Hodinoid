use std::fs::read_dir;
use std::io::Result;
use super::utils::config::load_company_config;

const COMPANY_CONFIG_NAME: &str = "config.toml";

pub fn load_companies(path: &str) -> Result<Vec<(i32, String)>> {
    let mut companies = Vec::new();
    
    let entries = read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();
        
        if entry_path.is_dir() {
            let company_id_str = entry_path.to_str().expect("Entry is not valid!");
            let company_id: i32 = company_id_str.parse().expect("Cannot parse id");
            let company_name = get_company_name(path, company_id);
            companies.push((company_id, company_name));
        }
    }
    Ok(companies)
}

fn get_company_name(company_path: &str, company_id: i32) -> String {
    let id_str = company_id.to_string();
    let path = format!("{}/{}", company_path, id_str);
    
    match load_company_config(&path) {
        Ok(company_config) => {
            return company_config.company_info.name;
        },
        Err(_) => {
            println!("Cannot find company config");
            return "".to_string();
        }
    }
}

use chrono::NaiveTime;

use crate::data_operations::load_contract_content;

use super::data_operations::{
    create_company, create_contract, create_time_record, load_companies, load_contracts,
};
use super::utils::config::Config;
use std::io::{self, Write};

fn get_input_int_wrapper() -> i32 {
    let mut input = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(choice) => return choice,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            return 0;
        }
    }
}

fn separator() {
    println!("#############################");
}

pub fn main_menu_cli(config: &Config) {
    loop {
        separator();
        println!("1. Select company");
        println!("0. Exit");

        match get_input_int_wrapper() {
            1 => {
                println!("You selected Option 1");
                company_cli(&config);
            }
            0 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn company_cli(config: &Config) {
    loop {
        separator();
        println!("1. Add company");

        let companies = load_companies(&config.location.root).expect("Failed");

        for (index, value) in companies.iter().enumerate() {
            let number_company = index + 2;
            let (_, name) = value;
            println!("{}. {}", number_company, name)
        }

        println!("0. Exit company list");

        let selected_num = get_input_int_wrapper();

        if selected_num == 1 {
            create_company(&config.location.root);
        } else if selected_num == 0 {
            break;
        } else if selected_num as usize <= companies.len() + 1 {
            println!("You selected {}", companies[(selected_num - 2) as usize].1);
            contracts_cli(&config, companies[(selected_num - 2) as usize].clone())
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

fn contracts_cli(config: &Config, company_tuple: (i32, String)) {
    loop {
        separator();
        println!("1. Add contract");
        let company_path = format!("{}/{}", &config.location.root, company_tuple.0);

        let contracts = load_contracts(&company_path).expect("Failed");

        for (index, value) in contracts.iter().enumerate() {
            let number_contract = index + 2;
            let (_, name) = value;
            println!("{}. {}", number_contract, name)
        }

        println!("0. Exit contracts");

        let selected_num = get_input_int_wrapper();

        if selected_num == 1 {
            create_contract(&company_path);
        } else if selected_num == 0 {
            break;
        } else if selected_num as usize <= contracts.len() + 1 {
            println!("You selected {}", contracts[(selected_num - 2) as usize].1);
            records_cli(&company_path, contracts[(selected_num - 2) as usize].0);
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

fn records_cli(company_path: &str, contract_id: i32) {
    loop {
        separator();
        println!("---RECORDS---");
        println!("1. add time record");
        println!("2. add non time record");
        println!("0. Exit records");

        println!("-------------------------------");

        let mut contract_file = load_contract_content(&company_path, contract_id);

        println!(
            "name: {} \nhour pay: {} Kč \nnotes: {}",
            contract_file.info.name,
            contract_file.info.hour_pay.to_string(),
            contract_file.info.note
        );

        println!("-------------------------------");
        println!("      Time records");
        println!("-------------------------------");

        let mut time_money_sum = 0.0;

        for entry in &contract_file.report_time {
            let from = NaiveTime::parse_from_str(&entry.from, "%H:%M").expect("Cannot parse time");
            let to = NaiveTime::parse_from_str(&entry.to, "%H:%M").expect("Cannot parse time");

            let duration = to.signed_duration_since(from);

            let hours_worked = duration.num_seconds() as f64 / 3600.0;
            let hours_worked_rounded = (hours_worked * 100.0).round() / 100.0;
            time_money_sum =
                time_money_sum + (hours_worked_rounded * (contract_file.info.hour_pay as f64));

            println!(
                "{} | {} | from: {} to: {} = {} hours",
                entry.date, entry.description, entry.from, entry.to, hours_worked_rounded
            );
        }

        let mut non_time_money_sum = 0.0;
        println!("========================");
        println!(" Time based: {} Kč", time_money_sum);
        println!(" Non Time based: {} Kč", non_time_money_sum);
        println!(" TOTAL: {} Kč", time_money_sum + non_time_money_sum);
        println!("========================");

        let selected_num = get_input_int_wrapper();

        if selected_num == 1 {
            let contract_path = format!("{}/{}.json", company_path, contract_id);
            create_time_record(&mut contract_file, &contract_path);
        } else if selected_num == 0 {
            break;
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

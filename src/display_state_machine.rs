use super::data_operations::{create_company, create_contract, load_companies, load_contracts};
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
            contract_cli(&config, companies[(selected_num - 2) as usize].clone())
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

fn contract_cli(config: &Config, company_tuple: (i32, String)) {
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
            contract_cli(&config, contracts[(selected_num - 2) as usize].clone())
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

use super::data_operations::load_companies;
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

        println!("0. Exit company");

        match get_input_int_wrapper() {
            1 => {
                println!("You selected Option 1");
            }
            2 => {
                println!("You selected {}", companies[0].1);
            }
            0 => {
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

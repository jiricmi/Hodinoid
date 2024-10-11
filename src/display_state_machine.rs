use super::utils::config::Config;
use std::io::{self, Write};

pub fn main_menu_cli(config: &Config) {
    loop {
        println!("1. Select company");
        println!("0. Exit");

        let mut input = String::new();

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        println!("You selected Option 1");
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
            Err(_) => {
                println!("Invalid input, please enter a number.");
            }
        }
    }
}

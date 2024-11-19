mod data_operations;
mod display_state_machine;
mod utils;
use utils::config::{get_config_path, Config};

fn setup_config() -> Config {
    match utils::files::read_file(get_config_path()) {
        Ok(config_string) => match toml::from_str(&config_string) {
            Ok(config) => return config,
            Err(e) => {
                println!("Error: Config not exists {}", e);
                return utils::config::config_cli_first_setup();
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            return utils::config::config_cli_first_setup();
        }
    }
}

fn main() {
    let config = setup_config();
    println!(
        "Welcome back, {} {}!",
        config.person.first_name, config.person.last_name
    );
    display_state_machine::main_menu_cli(&config);
}

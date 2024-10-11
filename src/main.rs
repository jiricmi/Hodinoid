mod utils;
mod data_operations;
mod display_state_machine;
use utils::config::{Config, CONFIG_NAME};
use utils::files::ensure_path;

fn setup_config(config_name: &str) -> Config {
    match utils::files::read_file(config_name) {
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
    let config = setup_config(CONFIG_NAME);
    ensure_path(&config.location.root).expect("Cannot create path to the files!");
    println!(
        "Welcome back, {} {}!",
        config.person.first_name, config.person.last_name
    );
    display_state_machine::main_menu_cli(&config);
}

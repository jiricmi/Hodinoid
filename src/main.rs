mod utils;

fn setup(config_name: &str) -> utils::config::Config {
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
    let config = setup(utils::config::CONFIG_NAME);
    println!(
        "Welcome back, {} {}!",
        config.person.first_name, config.person.last_name
    )
}

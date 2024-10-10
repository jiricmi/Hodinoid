use std::fs::write;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn save_file(filepath: &str, content: String) {
    match write(filepath, content) {
        Ok(_) => println!("Successfully written"),
        Err(e) => println!("Error while save file {}", e),
    }
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

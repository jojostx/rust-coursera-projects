use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

fn main() {
    let filename: Vec<_> = env::args().collect();
    let filename = filename.get(1);

    // check if the filename is available, else prompt the user to supply it.
    let filename = match filename {
        Some(filename_) => filename_.to_string(),
        None => loop {
            let mut file_name = String::new();

            print!("Enter filename: ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut file_name)
                .expect("Provide the name of your file");

            let file_name = file_name.trim().to_string(); // Trim and convert to `String`

            if !file_name.is_empty() {
                break file_name; // Break loop and assign the value
            }
        },
    };

    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

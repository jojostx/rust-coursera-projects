use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    let mut words_freq: HashMap<String, i32> = HashMap::new();
    let mut file_name = String::new();

    println!("input the path to a file that contains words seperated by whitespace: ");
    let read_count = io::stdin().read_line(&mut file_name).unwrap();
    file_name = file_name.trim().to_string();

    if read_count != 0 {
        // read from file
        let content = fs::read_to_string(file_name).unwrap();

        for word in content.split_whitespace() {
            words_freq
                .entry(word.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    println!("{:?}", words_freq);
}

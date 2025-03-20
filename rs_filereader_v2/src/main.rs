use std::env;

#[derive(Debug)]
struct Sizes { 
    bytes: String, 
    kilobytes: String, 
    megabytes: String, 
    gigabytes: String
}

fn main() {
    match env::args().nth(1) {
        Some(first_arg) => {
            let size = make_size(first_arg).unwrap();
            print!("{:?}\n", size);
        },
        None => println!("No arguments passed."),
    }
}

fn make_size(input: String) -> Result<Sizes, ()> {
    match input.to_lowercase().split_once(' ') {
        Some((size, unit_)) => {
            match unit_ {
                "b" => {
                    match size.parse::<i32>() {
                        Ok(num) => Ok(Sizes { 
                            bytes: format!("{} bytes", num),
                            kilobytes: format!("{} bytes", num),
                            megabytes: format!("{} bytes", num),
                            gigabytes: format!("{} bytes", num),
                        }),
                        Err(_) => Err(()),
                    }
                },
                "kb" => {
                    match size.parse::<i32>() {
                        Ok(num) => Ok(Sizes { 
                            bytes: format!("{} bytes", num * 1000),
                            kilobytes: format!("{} kilobytes", num),
                            megabytes: format!("{} megabytes", num/1024),
                            gigabytes: format!("{} gigabytes", num/1_048_576),
                        }),
                        Err(_) => Err(()),
                    }
                },
                "mb" => {
                    match size.parse::<i32>() {
                        Ok(num) => Ok(Sizes { 
                            bytes: format!("{} bytes", num * 1024 * 1024),
                            kilobytes: format!("{} kilobytes", num * 1024),
                            megabytes: format!("{} megabytes", num),
                            gigabytes: format!("{} gigabytes", num/1024),
                        }),
                        Err(_) => Err(()),
                    }
                },
                "gb" => {
                    match size.parse::<i32>() {
                        Ok(num) => Ok(Sizes { 
                            bytes: format!("{} bytes", num * 1024 * 1024 * 1024),
                            kilobytes: format!("{} kilobytes", num * 1024 * 1024),
                            megabytes: format!("{} megabytes", num * 1024),
                            gigabytes: format!("{} gigabytes", num),
                        }),
                        Err(_) => Err(()),
                    }
                },
                _ => Err(())
            }
        },
        None => Err(())
    }
}

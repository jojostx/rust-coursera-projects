use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use std::io::{Read, stdin};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[clap(short = 'd', long = "dead")]
    dead: bool,

    #[clap(short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,

    #[clap(short = 's', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    let mut message = options.message;

    let eye = if options.dead { "x" } else { "o" };

    if options.stdin {
        message.clear();

        stdin()
            .read_to_string(&mut message)
            .with_context(|| "Could not read input")?;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
        message = "Meow".to_string();
    }

    let catfile = options.catfile;

    match &catfile {
        Some(path) => {
            // read from file
            let cat_picture = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;

            let eye = format!("{}", eye.red().bold());

            let cat_picture = cat_picture.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("   \\");
            println!("    /\\___/\\");
            println!("   ( {}   {} )", eye.red().bold(), eye.red().bold());
            println!("   =(  I  )=");
        }
    }

    Ok(())
}

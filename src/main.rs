use std::{
    fs::File,
    io::Read,
    io::{BufReader, Write},
};

use anyhow::{Context, Result};
use hex::encode;
use sha2::{Digest, Sha512};

#[allow(clippy::main_recursion)]
fn main() -> Result<()> {
    let mut input: String;
    let mut hasher = Sha512::new();
    let mut buffer = [0; 1024];

    let filepath = user_input("What file do you want to hash? ");
    let filename =
        File::open(&filepath).with_context(|| format!("Could not open file {}", filepath))?;
    let mut reader = BufReader::new(filename);

    loop {
        let counter = reader
            .read(&mut buffer)
            .with_context(|| format!("Cannot read file to hash: {}", filepath))?;
        if counter == 0 {
            break;
        }
        hasher.update(&buffer[..counter]);
    }

    let hash = hasher.finalize();
    let hex_hash = encode(hash);
    println!("Your hash: {}", hex_hash);

    input = user_input("Want to write the hash to a file? ");

    match input.to_lowercase().as_str() {
        "yes" => {
            println!("Done...");

            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("hashes.txt")
                .expect("Opening hashes file.");
            writeln!(&mut f, "Filename: {} | Hash: {}", filepath, hex_hash)
                .expect("Writing to the hashes file");
        }
        "no" => println!("Not writing to the file..."),
        _ => print!("Input needs to be either yes or no."),
    }

    input = user_input("Want to hash another file? ");

    match input.to_lowercase().as_str() {
        "yes" => {
            print!("\x1B[2J\x1B[1;1H");
            let _ = main();
            Ok(())
        }
        "no" => {
            println!("See you next time...");
            Ok(())
        }
        _ => {
            println!("Input needs to be either yes or no.");
            Ok(())
        }
    }
}

fn user_input(message: &str) -> String {
    let mut input = String::new();

    print!("{}", message);
    std::io::stdout()
        .flush()
        .expect("Error printing to the screen");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    return input.trim().to_owned();
}

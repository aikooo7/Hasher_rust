use std::{
    fs::File,
    io::Read,
    io::{BufReader, Write},
};

use hex::encode;
use sha2::{Digest, Sha512};

fn main() {
    let mut input: String;
    let mut hasher = Sha512::new();
    let mut buffer = [0; 1024];

    let filepath = user_input("What file you want to hash? ");
    let filename = File::open(&filepath).expect("Unable to open file.");
    let mut reader = BufReader::new(filename);
    loop {
        let counter = reader
            .read(&mut buffer)
            .expect("Unable to read specific file part.");
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
            let mut f = File::options()
                .append(true)
                .create(true)
                .open("hashes.txt")
                .expect("Opening hashes file.");
            writeln!(&mut f, "Filename: {} | Hash: {}", filepath, hex_hash)
                .expect("Writing to the hashes file");
        }
        "no" => println!("Not writting to the file..."),
        _ => print!("Input needs to be either yes or no."),
    }

    input = user_input("Want to hash other file? ");

    match input.to_lowercase().as_str() {
        "yes" => {
            print!("\x1B[2J\x1B[1;1H");
            main();
        }
        "no" => println!("See you next time..."),
        _ => println!("Input needs to be either yes or no."),
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

use std::{
    fs::File,
    io::Read,
    io::{BufReader, Write},
};

use hex::encode;
use sha2::{Digest, Sha512};

fn main() {
    // Initialize variables
    let mut input: String;
    let mut hasher = Sha512::new();
    let mut buffer = [0; 1024];

    // Get the file path from user input
    let filepath = user_input("What file do you want to hash? ");
    let filename = File::open(&filepath).expect("Unable to open file.");
    let mut reader = BufReader::new(filename);

    // Read and hash the file in chunks
    loop {
        let counter = reader
            .read(&mut buffer)
            .expect("Unable to read specific file part.");
        if counter == 0 {
            break;
        }
        hasher.update(&buffer[..counter]);
    }

    // Finalize the hash and display it in hexadecimal
    let hash = hasher.finalize();
    let hex_hash = encode(hash);
    println!("Your hash: {}", hex_hash);

    // Prompt user to write the hash to a file
    input = user_input("Want to write the hash to a file? ");

    match input.to_lowercase().as_str() {
        "yes" => {
            println!("Done...");

            // Open or create the "hashes.txt" file and append the hash information
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

    // Prompt user to hash another file
    input = user_input("Want to hash another file? ");

    match input.to_lowercase().as_str() {
        "yes" => {
            // Clear the terminal and call the main function again for another run
            print!("\x1B[2J\x1B[1;1H");
            main();
        }
        "no" => println!("See you next time..."),
        _ => println!("Input needs to be either yes or no."),
    }
}

// Function to get user input
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

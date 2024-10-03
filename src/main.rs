mod cli;
mod encryption;
mod file_ops;

use clap::Parser;
use cli::Cli;

fn main() {
    println!("Welcome to File Crypt \n\n");

    let cli = Cli::parse();

    let pw = cli.pw;

    if pw.is_empty() {
        eprintln!("Error, password is empty");
        std::process::exit(1);
    }

    let key = pw.as_bytes();

    match cli.operation.to_lowercase().as_str() {
        "enc" => {
            println!("Encrypting: {}", cli.file);

            match file_ops::read_file(&cli.file) {
                Ok(data) => {
                    let encrypted = encryption::encrypt(&data, key);
                    let output_file = format!("{}.encrypted", cli.file);
                    match file_ops::write_file(&output_file, &encrypted) {
                        Ok(_) => println!("Encrypted file written to {}", output_file),
                        Err(e) => eprintln!("Error writing encrypted file: {}", e),
                    }
                }
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        "dec" => {
            println!("Decrypting {}", cli.file);

            match file_ops::read_file(&cli.file) {
                Ok(data) => {
                    let decrypted = encryption::decrypt(&data, key);
                    let output_file = cli.file.replace(".encrypted", ".decrypted");
                    match file_ops::write_file(&output_file, &decrypted) {
                        Ok(_) => println!("Decrypted file written to {}", output_file),
                        Err(e) => eprintln!("Error reading file {}", e),
                    }
                }

                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        _ => {
            eprintln!("Invalid input. Use 'enc' to Encrypt and 'dec' to Decrypt");
            std::process::exit(1);
        }
    }
}

// find the file in question and encrypt it

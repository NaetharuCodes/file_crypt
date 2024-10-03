// COMMAND LINE INTERFACE

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // Choose the file we want to operate on
    #[arg(short, long)]
    pub file: String,

    // Choose if we want to encrypt or decrypt this file
    #[arg(short, long)]
    pub operation: String,

    // Choose if we want to encrypt or decrypt this file
    #[arg(short, long)]
    pub pw: String,
}

use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { key } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            process::exit(1)
        }
    }
}

use clap::{Parser, Subcommand};
use lenshift::{decode, encode};

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Decode { input } => decode(input),
        Commands::Encode { input } => encode(input),
    };
    println!("{}", result);
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Decode { input: String },
    Encode { input: String },
}

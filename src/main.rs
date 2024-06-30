use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: String,

    #[command(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}

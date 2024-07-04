use clap::Parser;
use std::io;

mod calculator;
mod grep;
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Grep(grep::GrepArgs),
    Calculate(calculator::CalculateArgs),
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Grep(grep_args) => {
            grep::process_file(&grep_args)?;
        }
        Commands::Calculate(calc_args) => {
            let result = calculator::calculate(&calc_args);
            println!("Result: {:?}", result);
        }
    }

    Ok(())
}

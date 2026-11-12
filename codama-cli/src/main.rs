use clap::{Parser, Subcommand};
use codama::Codama;
use codama_errors::CodamaResult;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate IDL from a Rust project
    #[command(name = "generate-idl")]
    GenerateIdl {
        /// Path to the Rust project (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Prettyprint the JSON output
        #[arg(short, long)]
        pretty: bool,
    },
}

/// Prints the message of any error, such as the list of validation items, and exits with code 1.
fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> CodamaResult<()> {
    match cli.command {
        Commands::GenerateIdl {
            path,
            output,
            pretty,
        } => {
            let codama = Codama::load(&path)?;
            let idl = codama.get_idl()?;

            let json = if pretty {
                serde_json::to_string_pretty(&idl)?
            } else {
                serde_json::to_string(&idl)?
            };

            match output {
                Some(output_path) => {
                    std::fs::write(&output_path, &json)?;
                    eprintln!("✓ IDL written to: {}", output_path.display());
                }
                None => {
                    println!("{}", json);
                }
            }
        }
    }

    Ok(())
}

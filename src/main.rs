use clap::{Parser, Subcommand};
use privacy_sexy::{self, collection::Recommend, OS};

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Recommend strict
    #[arg(short = 't', long)]
    strict: bool,
    /// Recommend standard
    #[arg(short = 'd', long)]
    standard: bool,
    /// Name of script(s) required
    #[arg(short, long)]
    name: Vec<String>,
    /// Revert script(s)
    #[arg(short, long)]
    revert: bool,
}

/// Commands
#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate & print the script
    Echo,
    /// Generate & run the script
    Run,
}

fn main() {
    let cli = Cli::parse();
    let cd = privacy_sexy::get_collection(OS::get_system_os()).unwrap();

    let script = cd
        .parse(
            if cli.name.is_empty() { None } else { Some(&cli.name) },
            cli.revert,
            if cli.strict {
                Some(Recommend::Strict)
            } else if cli.standard {
                Some(Recommend::Standard)
            } else {
                None
            },
        )
        .unwrap();

    match cli.command {
        Commands::Echo => println!("{script}"),
        Commands::Run => {
            privacy_sexy::run_script(&script, cd.scripting.file_extension).unwrap();
        }
    }
}

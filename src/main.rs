use clap::{Parser, Subcommand};
use privacy_sexy::{get_collection, run_script, Recommend, OS};

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Recommend Strict
    #[arg(short = 't', long)]
    strict: bool,
    /// Recommend Standard
    #[arg(short = 'd', long)]
    standard: bool,
    /// Revert
    #[arg(short, long)]
    revert: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate & Print the script
    Echo,
    /// Generate & Run the script
    Run,
}

fn main() {
    let os = match std::env::consts::OS {
        "macos" => OS::MacOs,
        "linux" => OS::Linux,
        "windows" => OS::Windows,
        _ => panic!("Unsupported OS!"),
    };

    let cli = Cli::parse();
    let cd = get_collection(&os).unwrap();

    let script = cd
        .parse(
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
            run_script(&script, cd.scripting.file_extension, &os).unwrap();
        }
    }
}

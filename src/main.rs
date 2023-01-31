extern crate clap;
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Stats {
        #[clap(long)]
        player: String,
        #[clap(long, default_value = None)]
        year: u16,
    },
    H2H {
        #[clap(long)]
        player: String,
        #[clap(long)]
        opponent: String,
    },
    Predict {
        #[clap(long)]
        player: String,
        #[clap(long)]
        opponent: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let df = cli_proj::read_csv("final_df.csv");

    match cli.command {
        Some(Commands::Stats { player, year }) => {
            println!("Test {0}, {1}", player, year);
        }
        Some(Commands::H2H { player, opponent }) => {
            println!("{}", opponent);
            println!("{}", player);
        }
        Some(Commands::Predict { player, opponent }) => {
            println!("{}", opponent);
            println!("{}", player);
        }

        None => {
            println!("No command given");
        }
    }
}

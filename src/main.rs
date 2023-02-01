extern crate clap;
use clap::Parser;
extern crate polars;
// use polars::prelude::*;

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
    // let train_df = cli_proj::read_csv("final_df.csv").expect("Could not read csv");
    let h2h_df = cli_proj::read_csv("h2h.csv").expect("Could not read csv");

    match cli.command {
        Some(Commands::Stats { player, year }) => {
            println!("Test {0}, {1}", player, year);
        }
        Some(Commands::H2H { player, opponent }) => {
            let first = if player < opponent {
                player.clone()
            } else {
                opponent.clone()
            };

            let second = if player > opponent { player } else { opponent };

            // let h2h_df = h2h_df.filter(&col("player1") == lit(first) & (&col("player2") == lit(second))).unwrap();

            println!("First player is {}", first);
            println!("Second player is {}", second);

            println!("{}", h2h_df.head(Some(1)));
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

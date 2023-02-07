extern crate clap;
use clap::Parser;
extern crate polars;
use polars::lazy::dsl::*;
use polars::prelude::*;

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
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Stats { player, year }) => {
            // read csv
            let df = cli_proj::read_csv("final_df.csv").expect("Could not read csv");
            // filter by inputted player and year
            let player_df = df
                .lazy()
                .filter(col("player").eq(lit(player.clone())))
                .filter(col("year").eq(lit(year as u32)))
                .collect()
                .unwrap();

            
            // find some average stats

            // print accordingly
            println!("Test {0}, {1}", player, year);
            println!("{}", player_df.head(Some(1)));
        }
        Some(Commands::H2H { player, opponent }) => {
            // read csv
            let df = cli_proj::read_csv("h2h.csv").expect("Could not read csv");
            // determine order of players
            let first = if player < opponent {
                player.clone()
            } else {
                opponent.clone()
            };

            let second = if player > opponent { player } else { opponent };

            // filter by player1 and player2 
            let h2h_df = df
                .lazy()
                .filter(col("player1").eq(lit(first.clone())))
                .filter(col("player2").eq(lit(second.clone())))
                .collect()
                .unwrap();

            // find aggregate stats

            // print accordingly
            println!("First player is {}", first);
            println!("Second player is {}", second);
            println!();
            println!("{}", h2h_df.head(Some(1)));
        }
        None => {
            println!("No command given");
        }
    }
}

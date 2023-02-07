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

            // check if h2h_df is empty
            if h2h_df.height() == 0 {
                println!("No head-to-head record found for {} and {}", first, second);
                return;
            }

            // find aggregate stats
            let agg_rec = h2h_df
                .clone()
                .lazy()
                .groupby([col("player1")])
                .agg(vec![col("h2h_win").sum(), col("h2h_lose").sum()])
                .collect()
                .unwrap();

            let year_agg_rec = h2h_df.clone()
                .lazy()
                .groupby([col("year")])
                .agg(vec![col("h2h_win").sum(), col("h2h_lose").sum()])
                .collect()
                .unwrap();
            //fix column names

            // rename columns in year_agg_rec from h2h_win to win and h2h_lose to lose
            let year_agg_rec_fin = year_agg_rec.clone()
                .lazy()
                .select(&[
                    col("year").alias("Year"),
                    col("h2h_win").alias(&first),
                    col("h2h_lose").alias(&second),
                ])
                .collect()
                .unwrap();


            // get value from agg_rec as a scalar value
            let first_win = agg_rec["h2h_win"].i64().unwrap().get(0).unwrap();
            let second_win = agg_rec["h2h_lose"].i64().unwrap().get(0).unwrap();
            // find min and max year from year agg rec
            let duration = year_agg_rec["year"].i64().unwrap().max().unwrap()
                - year_agg_rec["year"].i64().unwrap().min().unwrap();
            let total = first_win + second_win;

            // print accordingly
            println!(
                "{} and {} have played {} times over the course of {} years",
                first,
                second,
                total,
                duration
            );
            println!(
                "{} {} ({}%) vs {} ({}%) {}",
                first,
                first_win,
                ((first_win as f64 / total as f64) * 100.0).round(),
                second_win,
                ((second_win as f64 / total as f64) * 100.0).round(),
                second
            );

            println!();
            println!("Stats by Year:");
            println!("{:?}", year_agg_rec_fin);
        }
        None => {
            println!("No command given");
        }
    }
}

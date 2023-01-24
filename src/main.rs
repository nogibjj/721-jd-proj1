extern crate clap;
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(version = "1.0", author = "Jackie Du", about = "Finds duplicate files")]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

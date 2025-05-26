use clap::Parser;
use kalendarium::Kalendarium;

#[derive(Parser)]
struct Cli {
    year: String,
    month: String,
    day: String,
}

fn main() {
    let args = Cli::parse();

    let kal: Kalendarium = Kalendarium::new(&args.year, &args.month, &args.day).unwrap();
    println!("{}", kal);
}

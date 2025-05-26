use clap::Parser;
use kalendarium::RomanNumeral;

#[derive(Parser)]
struct Cli {
    number: u32,
}

fn main() {
    let args = Cli::parse();

    let num: RomanNumeral = RomanNumeral::new(args.number).unwrap();
    println!("{}", num);
}

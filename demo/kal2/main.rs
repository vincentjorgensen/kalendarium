use chrono::{DateTime, Datelike, Local};
use clap::Parser;
use kalendarium::Kalendarium;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(default_value_t = -43)]
    year: isize,
    #[arg(default_value_t = 3)]
    month: isize,
    #[arg(default_value_t = 15)]
    day: isize,

    #[arg(short, long)]
    now: bool,
}

fn main() {
    let args = Cli::parse();

    let kal: Kalendarium;

    if args.now {
        let date_time: DateTime<Local> = chrono::offset::Local::now();
        kal = Kalendarium::new(
            &date_time.year().to_string(),
            &date_time.month().to_string(),
            &date_time.day().to_string(),
        )
        .unwrap();
    } else {
        kal = Kalendarium::new(
            &args.year.to_string(),
            &args.month.to_string(),
            &args.day.to_string(),
        )
        .unwrap();
    }
    println!("{}", kal);
}

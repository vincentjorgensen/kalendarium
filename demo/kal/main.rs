use chrono::{DateTime, Datelike, Local};
use kalendarium::Kalendarium;

fn main() {
    //   let args = Cli::parse();

    let date_time: DateTime<Local> = chrono::offset::Local::now();
    let kal: Kalendarium = Kalendarium::new(
        &date_time.year().to_string(),
        &date_time.month().to_string(),
        &date_time.day().to_string(),
    )
    .unwrap();
    println!("{}", kal);
}

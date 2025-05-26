//! # Kalendarium
//!
//! This rust library contains modules for converting Arabic numerals into Roman numerals, and for
//! converting dates into the Kalendarium Romanum.
//!
//! The largest valid numeral is 9,999,999. Though the Romans did not have an explicit symbol for 0
//! (ZERO), this library employs nulla (N) as zero, always uppercase.
//!
//! Numbers 4000 and larger are represented using the vinculum system, wereby a bar over symbols,
//! which multiples everything underneath by 1000. For example: I̅V̅ for 4,000 (using unicode long
//! macron as the bar symbol).

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod date;
mod error;
mod numeral;

pub use date::Kalendarium;
pub use error::OutOfRangeError;
pub use numeral::RomanNumeral;

/// The value of the smallest Roman numeral
pub const MIN: u32 = 0;
/// The value of the largest Roman numeral
pub const MAX: u32 = 4_999_999;

/// [`Result`](std::result::Result) with error defaulted to [`xvii::Error`](Error)
pub type Result<T, E = OutOfRangeError> = core::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;
    //    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    #[test]
    fn test_roman_numeral_new() {
        assert_eq!(RomanNumeral::new(0), Ok(RomanNumeral(0_u32)));
        assert_eq!(RomanNumeral::new(1), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::new(1_u8.into()), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::new(1_u32), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::new(42), Ok(RomanNumeral(42_u32)));
        assert_eq!(RomanNumeral::new(616), Ok(RomanNumeral(616_u32)));
        assert_eq!(RomanNumeral::new(49_999), Ok(RomanNumeral(49_999_u32)));
        assert_eq!(RomanNumeral::new(99_999), Ok(RomanNumeral(99_999_u32)));
        assert_eq!(RomanNumeral::new(999_999), Ok(RomanNumeral(999_999_u32)));
        assert_eq!(RomanNumeral::new(MAX), Ok(RomanNumeral(4_999_999_u32)));
        assert!(matches!(RomanNumeral::new(5_000_000), Err(OutOfRangeError)));
        assert!(matches!(RomanNumeral::new(u32::MAX), Err(OutOfRangeError)));
    }

    #[test]
    fn test_try_from_one() {
        assert_eq!(RomanNumeral::try_from(1_u8), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_u16), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_u32), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_u64), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_u128), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_usize), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_i8), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_i16), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_i32), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_i64), Ok(RomanNumeral(1_u32)));
        assert_eq!(RomanNumeral::try_from(1_i128), Ok(RomanNumeral(1_u32)));
    }

    #[test]
    fn test_roman_numeral_to_string() {
        assert_eq!(RomanNumeral::new(0).unwrap().to_string(), "N");
        assert_eq!(RomanNumeral::new(1).unwrap().to_string(), "I");
        assert_eq!(RomanNumeral::new(616).unwrap().to_string(), "DCXVI");
        assert_eq!(RomanNumeral::new(1984).unwrap().to_string(), "MCMLXXXIV");
    }

    #[test]
    fn test_kalendarium_dates() {
        // The day of "Wish World" S2:E7
        let kal: Kalendarium = Kalendarium::new("2025", "5", "24").unwrap();
        //        dbg!("DEBUG: {}", kal.debug());
        assert_eq!(kal.roman_year(), "MMDCCLXXVIII a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Sāturnī");
        assert_eq!(kal.roman_day(), "ante diem IX Kal. Iun.");
        assert_eq!(
            kal.to_str(),
            "ante diem IX Kal. Iun. MMDCCLXXVIII a.u.c. diēs Sāturnī"
        );

        // Star Wars premiere day
        let kal: Kalendarium = Kalendarium::new("1977", "5", "25").unwrap();
        //        dbg!("DEBUG: {}", kal.debug());
        assert_eq!(kal.roman_year(), "MMDCCXXX a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Mercuriī");
        assert_eq!(kal.roman_day(), "ante diem VIII Kal. Iun.");
        assert_eq!(
            kal.to_str(),
            "ante diem VIII Kal. Iun. MMDCCXXX a.u.c. diēs Mercuriī"
        );

        // Declaration of Independence signed
        let kal: Kalendarium = Kalendarium::new("1776", "7", "4").unwrap();
        assert_eq!(kal.roman_year(), "MMDXXIX a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Iovis");
        assert_eq!(kal.roman_day(), "ante diem IV Nōn. Iul.");
        assert_eq!(
            kal.to_str(),
            "ante diem IV Nōn. Iul. MMDXXIX a.u.c. diēs Iovis"
        );

        // The fall of Constantinople to the Seljuk Turks
        let kal: Kalendarium = Kalendarium::new("1453", "5", "29").unwrap();
        assert_eq!(kal.roman_year(), "MMCCVI a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Mārtis");
        assert_eq!(kal.roman_day(), "ante diem IV Kal. Iun.");
        assert_eq!(
            kal.to_str(),
            "ante diem IV Kal. Iun. MMCCVI a.u.c. diēs Mārtis"
        );

        // The battle of Hastings. Oct 14, 1066 9am Saturday
        let kal: Kalendarium = Kalendarium::new("1066", "10", "14").unwrap();
        assert_eq!(kal.roman_year(), "MDCCCXIX a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Sāturnī");
        assert_eq!(kal.roman_day(), "prīdiē Īdūs Octobrās");
        assert_eq!(
            kal.to_str(),
            "prīdiē Īdūs Octobrās MDCCCXIX a.u.c. diēs Sāturnī"
        );

        // Leap day in 1000 CE (Julian)
        let kal: Kalendarium = Kalendarium::new("1000", "2", "25").unwrap();
        assert_eq!(kal.roman_year(), "MDCCLIII a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Sōlis");
        assert_eq!(kal.roman_day(), "ante diem bis VI Kal. Mart.");
        assert_eq!(
            kal.to_str(),
            "ante diem bis VI Kal. Mart. MDCCLIII a.u.c. diēs Sōlis"
        );

        // Charlemagne coronated on xmas day 800 CE
        let kal: Kalendarium = Kalendarium::new("800", "12", "25").unwrap();
        assert_eq!(kal.roman_year(), "MDLIII a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Veneris");
        assert_eq!(kal.roman_day(), "ante diem bis VI Kal. Mart.");
        assert_eq!(kal.roman_festival_day(), "Dīēs Nātālis Sōlis Invictī ");
        assert_eq!(
            kal.to_str(),
            "ante diem bis VI Kal. Mart. MDLIII a.u.c. Dīēs Nātālis Sōlis Invictī diēs Veneris"
        );

        // New Years day of 1 BC
        let kal: Kalendarium = Kalendarium::new("1", "1", "1").unwrap();
        assert_eq!(kal.roman_year(), "DCCLIV a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Sāturnī");
        assert_eq!(kal.roman_day(), "Kalendae Iānuāriae");
        assert_eq!(
            kal.to_str(),
            "Kalendae Iānuāriae DCCLIV a.u.c. diēs Sāturnī"
        );

        // Last day of -1 BCE
        let kal: Kalendarium = Kalendarium::new("-1", "12", "31").unwrap();
        assert_eq!(kal.roman_year(), "DCCLIII a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Veneris");
        assert_eq!(kal.roman_day(), "prīdiē Kalendae Iānuāriās");
        assert_eq!(
            kal.to_str(),
            "prīdiē Kalendae Iānuāriās DCCLIII a.u.c. diēs Veneris"
        );

        //The day Julius Caesar was assassinated The Ides of March 44 BCE
        let kal: Kalendarium = Kalendarium::new("-43", "03", "15").unwrap();
        assert_eq!(kal.roman_year(), "DCCXI a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Iovis");
        assert_eq!(kal.roman_day(), "Īdūs Mārtiae");
        assert_eq!(kal.to_str(), "Īdūs Mārtiae DCCXI a.u.c. diēs Iovis");

        // Traditional founding day of Rome
        let kal: Kalendarium = Kalendarium::new("-753", "4", "21").unwrap();
        assert_eq!(kal.roman_year(), "I a.u.c.");
        assert_eq!(kal.roman_day_of_week(), "diēs Lūnae");
        assert_eq!(kal.roman_day(), "ante diem XI Kal. Māi.");
        assert_eq!(kal.roman_festival_day(), "Parilia ");
        assert_eq!(
            kal.to_str(),
            "ante diem XI Kal. Māi. I a.u.c. Parilia diēs Lūnae"
        );

        // The year before Rome was founded
        assert!(matches!(
            Kalendarium::new("-755", "1", "3"),
            Err(OutOfRangeError)
        ));
    }
}

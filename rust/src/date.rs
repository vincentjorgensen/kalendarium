//! # kalendarium Roman date implementation
//!
//! A library for converting a date into the Kalendarium Romanum
//!
//! While testing dates in late antiquity and classical times, the day of the week kept coming
//! up as wrong, which led me down a rabbit hole, whereby I discovered that the Gregorian calendar
//! dropped 12 days in Sept 1752 when it switched from the Julian calendar. The "cal" function
//! built into the macbook does not implement proleptic Gregorian (whereby dates are extraporated
//! backward even though people who lived in the time periods would not have used the calendar), so
//! "cal" 'correctly' reported the Battle of Hastings took place on a Saturday. But rust chrono
//! implement this hybid calendar, instead using proleptic Gregorian, so it reports that October
//! 14, 1066 is a Monday. On the proleptic Calendar, the Battle took place on October 2nd (a
//! Monday).
//!
//! After coming up with a quick solution for just the day-of-the-week calculations, I then
//! learned that there is a leapyear in 1000 CE on the Julian Calendar (there isn't one in
//! proleptic Gregorian), which broke weekdays before 1000, too, as well as eliminating February
//! 29, 1000 from a valid date.  Fortunately, there is a rust crate Julian, that implements a
//! REFORMED1582 calendar, which is exactly what I wanted! I switched over my crate to use that
//! library, and everything is working!
//!
//! ## License
//!
//! GNU GPL 3

use crate::{OutOfRangeError, Result, RomanNumeral};
use core::fmt;
use julian::{Calendar, Date};

/// A Kalendarium Romanum object
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Kalendarium(Date);

impl Kalendarium {
    /// Creates a ``Kalendarium`` for any date. Year, Month, and day are required.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let kal: Kalendarium = Kalendarium::new("2025, "5", "23").unwrap();
    ///    assert_eq!("ante diem X Kal. Iun. MMDCCLXXVIII a.u.c. diēs Veneris", answer.to_str());
    pub fn new(year: &str, month: &str, day: &str) -> Result<Self, OutOfRangeError> {
        if year.parse::<i32>().unwrap() > -755_i32 {
            let mut date_str = String::new();

            if year.parse::<i32>().unwrap() < 0 {
                // Year 0 in proleptic julian doesn't exist on historical calendars.
                // year -1 (i.e. 1 BCE) is following by year 1 CE
                date_str.push_str(&(year.parse::<i32>().unwrap() + 1_i32).to_string());
            } else {
                date_str.push_str(year);
            }
            date_str.push_str("-");
            date_str.push_str(month);
            date_str.push_str("-");
            date_str.push_str(day);
            let cal = Calendar::REFORM1582;
            let date = cal.parse_date(&date_str).unwrap();
            Ok(Kalendarium(date))
        } else {
            Err(OutOfRangeError)
        }
    }

    /// Displays a ``Kalendarium`` as a ancient Roman date string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let kal: Kalendarium = Kalendarium::new("800", "12", "25").unwrap();
    ///    assert_eq!( "ante diem bis VI Kal. Mart. MDLIII a.u.c. Dīēs Nātālis Sōlis Invictī diēs Veneris", kal.to_str());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_str(&self) -> String {
        let mut out = String::new();

        out.push_str(&Self::roman_day(self));
        out.push_str(" ");
        out.push_str(&Self::roman_year(self));
        out.push_str(" ");
        out.push_str(&Self::roman_festival_day(self));
        out.push_str(&Self::roman_day_of_week(self));
        out
    }

    /// helper function to determine whether the date's year is a leap year
    fn is_leap_year(&self) -> bool {
        let year: i16 = self.0.year() as i16;
        if year == 1000 {
            // Even though it "shouldn't" be, 1000 was a leap year on the Julian Calendar
            true
        } else if year % 4 == 0 {
            if (year % 100) == 0 {
                if year % 400 == 0 {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Display the Gregorian calendar year as a Roman year, i.e. years since found of Rome
    pub fn roman_year(&self) -> String {
        let mut out = String::new();
        let year: i16 = self.0.year() as i16;
        // No longer have to offset for year zero because I do that in the constructor now
        out.push_str(
            &RomanNumeral::new((753_i16 + year) as u32)
                .unwrap()
                .to_uppercase(),
        );
        out.push_str(" a.u.c.");
        out
    }

    /// helper function that returns a festival if there is on on the data
    pub fn roman_festival_day(&self) -> String {
        let month: u8 = self.0.month() as u8;
        let day: u8 = self.0.day() as u8;
        let mut out = String::new();
        for (_index, festival_day) in FERIAE.iter().enumerate() {
            if festival_day.month == month && festival_day.day == day {
                out.push_str(festival_day.festival);
                out.push_str(" ");
                break;
            }
        }
        out
    }

    /// The day of the week in Latin
    pub fn roman_day_of_week(&self) -> String {
        let mut out = String::new();
        let day_of_week = self.0.weekday() as usize;
        out.push_str(DIES[day_of_week].day_of_week);
        out
    }

    /// The day of the month on the Kalendarium Romanum
    pub fn roman_day(&self) -> String {
        let mut out = String::new();

        let day: usize = self.0.day() as usize;
        let month: usize = self.0.month() as usize;

        let leap_day: usize = if Self::is_leap_year(self) && day > 25 && month == 2 {
            1
        } else {
            0
        };

        if day == 1 {
            // The First of the Month, the Kalends
            out.push_str("Kalendae ");
            out.push_str(MENSES[month - 1].nominis);
        } else if day < MENSES[month - 1].nones {
            // Days leading up to the Nones
            if MENSES[month - 1].nones - day < 2 {
                out.push_str("prīdiē Nōnās ");
                out.push_str(MENSES[month - 1].nomines);
            } else {
                out.push_str("ante diem ");
                out.push_str(
                    &RomanNumeral::new((1 + MENSES[month - 1].nones - day) as u32)
                        .unwrap()
                        .to_uppercase(),
                );
                out.push_str(" Nōn. ");
                out.push_str(MENSES[month - 1].nomunculus);
            }
        } else if day == MENSES[month - 1].nones {
            // The Nones herself
            out.push_str("Nōnae ");
            out.push_str(MENSES[month - 1].nominis);
        } else if day < MENSES[month - 1].ides {
            // Days leading up to the Ides
            if MENSES[month - 1].ides - day < 2 {
                out.push_str("prīdiē Īdūs ");
                out.push_str(MENSES[month - 1].nomines);
            } else {
                out.push_str("ante diem ");
                out.push_str(
                    &RomanNumeral::new((1 + MENSES[month - 1].ides - day) as u32)
                        .unwrap()
                        .to_uppercase(),
                );
                out.push_str(" Īd. ");
                out.push_str(MENSES[month - 1].nomunculus);
            }
        } else if day == MENSES[month - 1].ides {
            // The Ides herself
            out.push_str("Īdūs ");
            out.push_str(MENSES[month - 1].nominis);
        } else {
            if Self::is_leap_year(self) && day == 25 {
                out.push_str("ante diem bis VI Kal. Mart.");
            } else if (MENSES[month - 1].finis + leap_day) - day < 2 {
                out.push_str("prīdiē Kalendae ");
                out.push_str(MENSES[month].nomines);
            } else {
                out.push_str("ante diem ");
                out.push_str(
                    &RomanNumeral::new((2 + (MENSES[month - 1].finis + leap_day) - day) as u32)
                        .unwrap()
                        .to_uppercase(),
                );
                out.push_str(" Kal. ");
                out.push_str(MENSES[month].nomunculus);
            }
        }
        out
    }

    /// debugging function
    pub fn debug(&self) -> String {
        let mut out = String::new();
        out.push_str("day: ");
        out.push_str(&self.0.day().to_string());
        out.push_str(", month: ");
        out.push_str(&self.0.month().to_string());
        out.push_str(", year: ");
        out.push_str(&self.0.year().to_string());
        out.push_str(", weekday: ");
        out.push_str(&self.0.weekday().to_string());
        out
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Kalendarium {
    /// Displays a Date into the Kalendarium Romanum
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let kal: Kalendarium = Kalendarium::new("-43", "03", "15").unwrap();
    ///    assert_eq!("prīdiē Kalendae Iānuāriās DCCLIII a.u.c. diēs Veneris", kal.to_string());
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

/// Struct for defining the days of week in Latin
struct DiesStruct<'m> {
    day_of_week: &'m str,
}

static DIES: [DiesStruct; 8] = [
    DiesStruct {
        day_of_week: "diēs Sōlis",
    },
    DiesStruct {
        day_of_week: "diēs Lūnae",
    },
    DiesStruct {
        day_of_week: "diēs Mārtis",
    },
    DiesStruct {
        day_of_week: "diēs Mercuriī",
    },
    DiesStruct {
        day_of_week: "diēs Iovis",
    },
    DiesStruct {
        day_of_week: "diēs Veneris",
    },
    DiesStruct {
        day_of_week: "diēs Sāturnī",
    },
    DiesStruct {
        day_of_week: "diēs Sōlis",
    },
];

/// Struct for storing festival days
struct FeriaStruct<'m> {
    month: u8,
    day: u8,
    festival: &'m str,
}

static FERIAE: [FeriaStruct; 14] = [
    FeriaStruct {
        month: 12,
        day: 17,
        festival: "prīmus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 18,
        festival: "secundus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 19,
        festival: "tertius diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 20,
        festival: "quārtus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 21,
        festival: "quīntus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 22,
        festival: "sextus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 23,
        festival: "septimus diēs Saturnālium",
    },
    FeriaStruct {
        month: 12,
        day: 25,
        festival: "Dīēs Nātālis Sōlis Invictī",
    },
    FeriaStruct {
        month: 2,
        day: 15,
        festival: "Lupercālia",
    },
    FeriaStruct {
        month: 3,
        day: 17,
        festival: "Līberālia",
    },
    FeriaStruct {
        month: 4,
        day: 1,
        festival: "Venerālia",
    },
    FeriaStruct {
        month: 4,
        day: 27,
        festival: "Flōrālia",
    },
    FeriaStruct {
        month: 6,
        day: 24,
        festival: "Fors Fortūna",
    },
    FeriaStruct {
        month: 4,
        day: 21,
        festival: "Parilia",
    },
];

/// struct for defining what a Roman month looks like
struct MensStruct<'m> {
    nominis: &'m str,
    nomines: &'m str,
    nomunculus: &'m str,
    nones: usize,
    ides: usize,
    finis: usize,
}

static MENSES: [MensStruct; 13] = [
    MensStruct {
        nominis: "Iānuāriae",
        nomines: "Iānuāriās",
        nomunculus: "Iān.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Februāriae",
        nomines: "Februāriās",
        nomunculus: "Feb.",
        nones: 5,
        ides: 13,
        finis: 28,
    },
    MensStruct {
        nominis: "Mārtiae",
        nomines: "Mārtiās",
        nomunculus: "Mārt.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Aprīlis",
        nomines: "Aprīlēs",
        nomunculus: "Apr.",
        nones: 5,
        ides: 13,
        finis: 30,
    },
    MensStruct {
        nominis: "Māiae",
        nomines: "Māiās",
        nomunculus: "Māi.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Iūniae",
        nomines: "Iūniās",
        nomunculus: "Iun.",
        nones: 5,
        ides: 13,
        finis: 30,
    },
    MensStruct {
        nominis: "Iūliae",
        nomines: "Iūliās",
        nomunculus: "Iul.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Augustae",
        nomines: "Augustās",
        nomunculus: "Aug.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Septembrae",
        nomines: "Septembrās",
        nomunculus: "Sept.",
        nones: 5,
        ides: 13,
        finis: 30,
    },
    MensStruct {
        nominis: "Octobrae",
        nomines: "Octobrās",
        nomunculus: "Oct.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Novembrae",
        nomines: "Novembrās",
        nomunculus: "Nov.",
        nones: 5,
        ides: 13,
        finis: 30,
    },
    MensStruct {
        nominis: "Decembrae",
        nomines: "Decembrās",
        nomunculus: "Dec.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
    MensStruct {
        nominis: "Iānuāriae",
        nomines: "Iānuāriās",
        nomunculus: "Iān.",
        nones: 7,
        ides: 15,
        finis: 31,
    },
];

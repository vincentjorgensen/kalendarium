//use chrono::NaiveDateTime;
//use chrono::TimeZone;
#[warn(dead_code)]
use chrono::{DateTime, Datelike, Local};

struct FeriaStruct<'m> {
    month: usize,
    day: isize,
    festival: &'m str,
}

static FERIAE: [FeriaStruct; 13] = [
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
];

struct MensStruct<'m> {
    nominis: &'m str,
    nomines: &'m str,
    nomunculus: &'m str,
    nones: isize,
    ides: isize,
    finis: isize,
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

struct Arabic2RomanStruct<'a> {
    tabulatus: isize,
    romanum: &'a str,
    romanum_duo: Option<&'a str>,
}

static NUMERI: [Arabic2RomanStruct; 14] = [
    Arabic2RomanStruct {
        tabulatus: 5000000,
        romanum: "?̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 1000000,
        romanum: "M̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 500000,
        romanum: "D̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 100000,
        romanum: "C̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 50000,
        romanum: "L̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 10000,
        romanum: "X̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 5000,
        romanum: "V̄",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 1000,
        romanum: "M",
        romanum_duo: Some("Ī"), // required to not output MX̄ when we want ĪX̄
    },
    Arabic2RomanStruct {
        tabulatus: 500,
        romanum: "D",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 100,
        romanum: "C",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 50,
        romanum: "L",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 10,
        romanum: "X",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 5,
        romanum: "V",
        romanum_duo: None,
    },
    Arabic2RomanStruct {
        tabulatus: 1,
        romanum: "I",
        romanum_duo: None,
    },
];

fn itera(repeat: isize, numeral: &str) -> String {
    return if repeat > 0 {
        numeral.to_owned() + &itera(repeat - 1, numeral)
    } else {
        "".to_string()
    };
}

fn is_leap_year(year: isize) -> bool {
    if year % 4 == 0 {
        if (year % 100) == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    } else {
        return false;
    }
}

fn arabic_to_roman(num: isize) -> String {
    let mut inscriptum: String = "".to_string();
    if num != 0 {
        for (index, basis) in NUMERI.iter().enumerate() {
            let tabulatus = num / basis.tabulatus; // Truncating division since both values are ints
            if tabulatus > 0 {
                let offset = if basis.tabulatus.to_string().contains("5") {
                    1
                } else {
                    0
                };
                inscriptum =
                    if num >= NUMERI[index - 1].tabulatus - NUMERI[index + offset].tabulatus {
                        ({
                            if 4000 <= num && num < 10000 {
                                NUMERI[index + offset].romanum_duo.unwrap()
                            } else {
                                NUMERI[index + offset].romanum
                            }
                        })
                        .to_string()
                            + NUMERI[index - 1].romanum
                            + &arabic_to_roman(
                                (num - (NUMERI[index - 1].tabulatus
                                    - NUMERI[index + offset].tabulatus))
                                    % basis.tabulatus,
                            )
                    } else {
                        itera(tabulatus, basis.romanum).to_owned()
                            + &arabic_to_roman(
                                (num - NUMERI[index - 1].tabulatus).rem_euclid(basis.tabulatus),
                            )
                    };
                break;
            }
        }
    }
    return inscriptum;
}

fn is_a_festival_day(month: usize, day: isize) -> String {
    let mut festival: String = " ".to_string();
    for (_index, festival_day) in FERIAE.iter().enumerate() {
        if festival_day.month == month && festival_day.day == day {
            festival = festival_day.festival.to_string();
            break;
        }
    }
    return festival;
}

fn kalendarium(date_time: DateTime<Local>) -> String {
    let month: usize = date_time.month() as usize;
    let day: isize = date_time.day() as isize;
    let year: isize = date_time.year() as isize;

    let leap_day: isize = if day > 25 { 1 } else { 0 };

    let annus: String;
    annus = if year < 0 {
        arabic_to_roman((753 + 1 - year) as isize)
    } else {
        arabic_to_roman((753 + year) as isize)
    };
    let dies: String;
    dies = if day == 1 {
        // The First of the Month, the Kalends
        "Kalendae ".to_owned() + MENSES[month - 1].nominis
    } else if day < MENSES[month - 1].nones {
        // Days leading up to the Nones
        if MENSES[month - 1].nones - day < 2 {
            "prīdiē Nōnās ".to_owned() + MENSES[month - 1].nomines
        } else {
            "ante diem ".to_owned()
                + &arabic_to_roman((1 + MENSES[month - 1].nones - day) as isize)
                + " Nōn. "
                + MENSES[month - 1].nomunculus
        }
    } else if day == MENSES[month - 1].nones {
        // The Nones herself
        "Nōnae ".to_owned() + MENSES[month - 1].nominis
    } else if day < MENSES[month - 1].ides {
        // Days leading up to the Ides
        if MENSES[month - 1].ides - day < 2 {
            "prīdiē Īdūs {}".to_owned() + MENSES[month - 1].nomines
        } else {
            "ante diem ".to_owned()
                + &arabic_to_roman((1 + MENSES[month - 1].ides - day) as isize)
                + " Īd. "
                + MENSES[month - 1].nomunculus
        }
    } else if day == MENSES[month - 1].ides {
        // The Ides herself
        "Īdūs ".to_owned() + MENSES[month - 1].nominis
    } else {
        if is_leap_year(year) && day == 25 {
            "ante diem bis VI Kal. Mart.".to_owned()
        } else if (MENSES[month - 1].finis + leap_day) - day < 2 {
            "prīdiē Kalendae {}".to_owned() + MENSES[month].nomines
        } else {
            "ante diem ".to_owned()
                + &arabic_to_roman((2 + (MENSES[month - 1].finis + leap_day) - day) as isize)
                + " Kal. "
                + MENSES[month].nomunculus
        }
    };
    // println!("{} {} a.u.c.", dies, annus);
    return dies + " " + &annus + " a.u.c. " + &is_a_festival_day(month, day);
}

fn main() {
    /*
        println!("27 {}", arabic_to_roman(27));
        println!("59 {}", arabic_to_roman(59));
        println!("99 {}", arabic_to_roman(99));
        println!("501 {}", arabic_to_roman(501));
        println!("999 {}", arabic_to_roman(999));
        println!("1010 {}", arabic_to_roman(1010));
        println!("1444 {}", arabic_to_roman(1444));
        println!("6666 {}", arabic_to_roman(6666));
        println!("9872 {}", arabic_to_roman(9872));
        println!("14389 {}", arabic_to_roman(14389));
        println!("59437 {}", arabic_to_roman(59437));
    */
    //let naive = NaiveDateTime::parse_from_str("2023-12-16T23:59:59", "%Y-%m-%dT%H:%M:%S").unwrap();
    // let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let date_time: DateTime<Local> = chrono::offset::Local::now();
    println!("{}", kalendarium(date_time));
}

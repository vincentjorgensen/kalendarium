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

pub fn ad_romanum(num: isize) -> String {
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
                            + &ad_romanum(
                                (num - (NUMERI[index - 1].tabulatus
                                    - NUMERI[index + offset].tabulatus))
                                    % basis.tabulatus,
                            )
                    } else {
                        itera(tabulatus, basis.romanum).to_owned()
                            + &ad_romanum(
                                (num - NUMERI[index - 1].tabulatus).rem_euclid(basis.tabulatus),
                            )
                    };
                break;
            }
        }
    }
    return inscriptum;
}

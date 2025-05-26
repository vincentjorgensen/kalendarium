//! # kalendarium Roman numeral implementation
//!
//! A library for converting Arabic numerals to Classical Attic numerals
//!
//! TODO: implement Apostrophus (vs Vinculum) for numbers greater than 5000
//!       https://en.wikipedia.org/wiki/Roman_numerals#Other_additive_forms
//! TODO: implement Additive (vs Subtractive) for -4 and -9
//!       e.g. XIIII and MDCCCC for 14 an 1900 respectively
//!
//! ## License
//!
//! GNU GPL 3

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

use crate::{OutOfRangeError, Result};
use core::fmt;

/// A Roman numeral
///
/// Values from 0 to 4,999,9999 are supported
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RomanNumeral(pub u32);

impl RomanNumeral {
    /// Creates a ``RomanNumeral`` for any value in range.
    /// Requires ``value`` to be less than 5,000,000. 0 (ZERO) is acceptable.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    //     let answer: RomanNumeral = RomanNumeral::new(42).unwrap();
    //     assert_eq!("XLII", answer.to_uppercase());
    ///
    pub const fn new(value: u32) -> Result<Self, OutOfRangeError> {
        if value <= 4_999_999 {
            // SAFETY: 0 <= value <= 4,999,999
            Ok(RomanNumeral(value))
        } else {
            Err(OutOfRangeError)
        }
    }

    /// Return the value of this ``RomanNumeral`` as a ``u32``.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!(answer.as_u32(), 42_u32);
    ///
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Converts a ``RomanNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(616)?;
    ///    assert_eq!("DCXVI", answer.to_uppercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_uppercase(&self) -> String {
        let mut out = String::new();
        if self.0 == 0 {
            out.push_str(&"N".to_string());
        } else {
            out.push_str(&Self::arabic_to_roman(self.0, true));
        }
        out
    }
    ///
    /// Converts a ``RomanNumeral`` to a lowercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(616)?;
    ///    assert_eq!("dcxvi", answer.to_lowercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_lowercase(self) -> String {
        let mut out = String::new();
        if self.0 == 0 {
            out.push_str(&"N".to_string());
        } else {
            out.push_str(&Self::arabic_to_roman(self.0, false));
        }
        out
    }

    /// Helper function to repeat a character, needed for II, XX, CCC, etc.
    fn repeat(total: u8, character: &str) -> String {
        let mut count = total;
        let mut out = String::new();
        while count > 0 {
            out.push_str(character);
            count -= 1;
        }
        out
    }

    /// Helper function to convert an Arabic numeral into a Roman one
    ///
    /// This looks complicated and I know there are more concise solutions that
    /// I've seen in other libraries.
    ///
    /// I am using this one because I derived it during Covid lockdown in 2020-2021
    /// without consulting any other solutions. What's fun about it now is that I
    /// no longer am sure how exactly it works anymore, and I find beauty in knowing
    /// that at one point my brain solved this problem in this way and now, though
    /// I no longer intuit the solution, it remains a unique creation of my mind.
    ///
    /// In the original rust code, I painstakingly used Latin works for many of the terms.
    /// Here, I have rendered then in English, though I have left the name of the struct
    /// which contains all the conversion as its Latin name, NUMERI, the nominative plural
    /// of the word for number.
    fn arabic_to_roman(num: u32, uppercase: bool) -> String {
        let mut out: String = String::new();
        if num != 0 {
            for (index, basis) in NUMERI.iter().enumerate() {
                let arabic = num / basis.arabic; // Truncating division since both values are ints
                if arabic > 0 {
                    let offset = if basis.arabic.to_string().contains("5") {
                        1
                    } else {
                        0
                    };
                    if num >= NUMERI[index - 1].arabic - NUMERI[index + offset].arabic {
                        if 4000 <= num && num < 10000 {
                            if uppercase {
                                out.push_str(&"I̅".to_string());
                            } else {
                                out.push_str(&"i̅".to_string());
                            }
                        } else {
                            if uppercase {
                                out.push_str(NUMERI[index + offset].u_latin);
                            } else {
                                out.push_str(NUMERI[index + offset].l_latin);
                            }
                        }
                        out.push_str(NUMERI[index - 1].u_latin);
                        out.push_str(&Self::arabic_to_roman(
                            (num - (NUMERI[index - 1].arabic - NUMERI[index + offset].arabic))
                                % basis.arabic,
                            uppercase,
                        ));
                    } else {
                        if uppercase {
                            out.push_str(&Self::repeat(
                                u8::try_from(arabic).unwrap(),
                                basis.u_latin,
                            ));
                        } else {
                            out.push_str(&Self::repeat(
                                u8::try_from(arabic).unwrap(),
                                basis.l_latin,
                            ));
                        }
                        out.push_str(&Self::arabic_to_roman(
                            u32::try_from(
                                (isize::try_from(num).unwrap()
                                    - isize::try_from(NUMERI[index - 1].arabic).unwrap())
                                .rem_euclid(isize::try_from(basis.arabic).unwrap()),
                            )
                            .unwrap(),
                            uppercase,
                        ));
                    }
                    break;
                }
            }
        }
        out
    }
}
#[cfg(feature = "std")]
impl fmt::Display for RomanNumeral {
    /// Converts a ``RomanNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!("ΜΒ'", answer.to_string());
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_uppercase())
    }
}

/// Struct for holding the conversion values
struct Arabic2RomanStruct<'a> {
    arabic: u32,
    u_latin: &'a str,
    l_latin: &'a str,
}

/// Array of conversions for arabic numerals to Latin numerals
static NUMERI: [Arabic2RomanStruct; 13] = [
    Arabic2RomanStruct {
        arabic: 1000000,
        u_latin: "M̅",
        l_latin: "m̅",
    },
    Arabic2RomanStruct {
        arabic: 500000,
        u_latin: "D̅",
        l_latin: "d̅",
    },
    Arabic2RomanStruct {
        arabic: 100000,
        u_latin: "C̅",
        l_latin: "c̅",
    },
    Arabic2RomanStruct {
        arabic: 50000,
        u_latin: "L̅",
        l_latin: "l̅",
    },
    Arabic2RomanStruct {
        arabic: 10000,
        u_latin: "X̅",
        l_latin: "x̅",
    },
    Arabic2RomanStruct {
        arabic: 5000,
        u_latin: "V̅",
        l_latin: "u̅",
    },
    Arabic2RomanStruct {
        arabic: 1000,
        u_latin: "M",
        l_latin: "m",
    },
    Arabic2RomanStruct {
        arabic: 500,
        u_latin: "D",
        l_latin: "d",
    },
    Arabic2RomanStruct {
        arabic: 100,
        u_latin: "C",
        l_latin: "c",
    },
    Arabic2RomanStruct {
        arabic: 50,
        u_latin: "L",
        l_latin: "l",
    },
    Arabic2RomanStruct {
        arabic: 10,
        u_latin: "X",
        l_latin: "x",
    },
    Arabic2RomanStruct {
        arabic: 5,
        u_latin: "V",
        l_latin: "u",
    },
    Arabic2RomanStruct {
        arabic: 1,
        u_latin: "I",
        l_latin: "i",
    },
];

impl TryFrom<u8> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u8``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u8) -> Result<Self, OutOfRangeError> {
        Self::new(u32::from(value))
    }
}

impl TryFrom<u16> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u16``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u16) -> Result<Self, OutOfRangeError> {
        Self::new(u32::from(value))
    }
}

impl TryFrom<u32> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u32``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u32) -> Result<Self, OutOfRangeError> {
        Self::new(value)
    }
}

impl TryFrom<u64> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u64``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u64) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<u128> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u128``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u128) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<usize> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``usize``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: usize) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i8> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i8``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i8) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i16> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i16``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i16) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i32> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i32``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i32) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i64> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i64``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i64) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i128> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i128``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i128) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

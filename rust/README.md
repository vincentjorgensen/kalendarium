# kalendarium

A Rust library for converting dates to the Kalendarium Romanum, the calendar
used by the ancient Romans. Unlike a modern calendar, days are displayed as the
number of days until either the Kalends, the Nones, or the Ides.  For a 31 day
month, occur on the 1st, 7th, and 15th of the month.

For example, May 26th, 2025 would appear like this:

```
ante diem VII Kal. Iun. MMDCCLXXVIII a.u.c. diēs Lūnae
```

Meaning: there are 7 days until the Kalends of June  (counting inclusively),
it's 2778 years since the founding of Rome, and it's a Monday.

There is also a second library for converting Arabic numerals (1,2,3...) Roman
numerals numerals (I, II, III ...) that handles very large numbers.  Integers
between 0 and 999,999 (inclusive) are supported. Any number beyond this range
will return an ``OutOfRangeError``.

Both uppercase and lowercase Roman numerals are supported. 

## Example usages

### Display the date

Contruct a calendar date with the year, month, and day.

```rust
use kalendarium::Kalendarium;

let kal: Kalendarium = Kalendarium::new("2025", "5", "24").unwrap();
assert_eq!(kal.to_str, "ante diem IX Kal. Iun. MMDCCLXXVIII a.u.c. diēs Sāturnī");
```

### Create Roman numerals

```rust
use kalendarium::RomanNumeral;

let num = RomanNumeral::new(616)?;
assert_eq!(num.to_string(), "DCXVI");

let num: RomanNumeral = 999_999.try_into().unwrap();
println!("{}", num);  // C̅M̅X̅C̅I̅X̅CMXCIX
```

## License

GPL-3.0


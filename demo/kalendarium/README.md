# Demo code
This is the initial stand-alone app I wrote that demonstrates how it works in
the nominal case: Today's date.

The code is there to convert large numbers from arabic to roman numerals.

Additionally, the date conversion logic is present for leapyears.

## Testing
```bash
cargo build
./target/debug/kalendarium
```

For example, today, April 2nd, 2023, results in the following output:
```
ante diem IV Nōn. Apr. MMDCCLXXVI a.u.c.
```

That is, four days (counting inclusively) until the Nones of April (April 5th). 
In modern times, we count exclusively, that is, we skip today, but the Romans
counted the current day as well.

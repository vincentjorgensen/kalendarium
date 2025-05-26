============
 kalendarium
============

.. image:: https://img.shields.io/badge/License-GPL%203.0-green.svg
   :target: https://www.gnu.org/licenses/gpl-3.0.en.html
   :alt: License: GPL-3.0

A Rust library for converting dates to the Kalendarium Romanum, the calendar
used by the ancient Romans. Unlike a modern calendar, days are displayed as the
number of days until either the Kalends, the Nones, or the Ides.  For a 31 day
month, occur on the 1st, 7th, and 15th of the month.

There is also a second library for converting Arabic numerals (1,2,3...) Roman
numerals numerals (I, II, III ...) that handles very large numbers.  Integers
between 0 and 999,999 (inclusive) are supported. Any number beyond this range
will return an ``OutOfRangeError``.

Both uppercase and lowercase Roman numerals are supported. 

Example usage
=============

.. code-block:: rust

   use kalendarium::{RomanNumeral, Kalendarium};

   fn main() {
      let num = RomanNumeral::new(616).unwrap();
      println!("{}", num); // DCXVI
      assert_eq!(num.to_string(), "DCXVI");

      let kal: Kalendarium = Kalendarium::new("2025", "5", "24").unwrap();
      assert_eq!(kal.to_str, "ante diem IX Kal. Iun. MMDCCLXXVIII a.u.c. diēs Sāturnī");
   }

License
=======

`GPL-3.0`__

__ main/LICENSE

Contribution
------------

Any contribution intentionally submitted for inclusion in this project shall be
licensed as above without additional terms or conditions.


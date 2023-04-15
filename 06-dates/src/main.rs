/*
 * Read 2 dates into a date range, then validate the object
 * to ensure min <= max.
 */

use std::io::{self, Write};
use chrono::{NaiveDate, ParseError};

struct DateRange {
    min_date: NaiveDate,
    max_date: NaiveDate,
}

impl DateRange {
    fn new(min_date: NaiveDate, max_date: NaiveDate) -> Self {
        Self { min_date, max_date }
    }

    fn validate(&self) -> bool {
        self.min_date <= self.max_date
    }
}

fn read_date() -> Result<NaiveDate, ParseError> {
    print!("Enter a date (yyyy-mm-dd): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d")
}

fn main() {
    let min_date = loop {
        match read_date() {
            Ok(date) => break date,
            Err(_) => println!("Invalid date format. Try again."),
        }
    };

    let max_date = loop {
        match read_date() {
            Ok(date) => break date,
            Err(_) => println!("Invalid date format. Try again."),
        }
    };

    let date_range = DateRange::new(min_date, max_date);

    if date_range.validate() {
        println!("Date range is valid.");
    } else {
        println!("Invalid date range! Min date is greater than max date.");
    }
}

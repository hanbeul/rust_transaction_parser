use std::error::Error;
use std::io;
use std::process;
use chrono::prelude::*;
use rusty_money::{Money, iso};

use std::cmp::Ordering;

#[derive(Debug)]
struct Transaction<'a> {
    date: chrono::NaiveDate,
    name: String,
    amount: rusty_money::Money<'a, iso::Currency>,
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut transactions: Vec<Transaction> = vec![];
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;

        let t = Transaction {
            date: NaiveDate::parse_from_str(&record[0], "%m/%d/%Y")?,
            name: record[2].to_string(),
            amount: Money::from_str(&record[5], iso::USD)?,
        };
        //println!("{:?}", t);
        transactions.push(t);
    }

    transactions.sort_by(|a, b| {
        if b.amount > a.amount {
            Ordering::Less
        } else if b.amount < a.amount {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    println!("{:?}", transactions);
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

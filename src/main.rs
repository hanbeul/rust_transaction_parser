use std::error::Error;
use std::io;
use std::process;
use std::convert::From;
use chrono::prelude::*;
use rusty_money::{Money, iso};

use std::cmp::Ordering;

#[derive(Debug)]
struct Transaction<'a> {
    date: chrono::NaiveDate,
    name: String,
    amount: rusty_money::Money<'a, iso::Currency>,
}

#[derive(Debug)]
struct CapOneTransaction {
    date: String,
    description: String,
    amount: String,
}

impl From<CapOneTransaction> for Transaction<'_>{
    fn from(t: CapOneTransaction) -> Self {
        Transaction {
            date: NaiveDate::parse_from_str(&t.date, "%m/%d/%Y").unwrap(),
            name: t.description,
            amount: Money::from_str(&t.amount, iso::USD).unwrap(),
        }
    }
}

fn test() -> Result<(), Box<dyn Error>> {
    let c1 = CapOneTransaction {
        date: String::from("05/30/1992"),
        description: String::from("This is a transaction description"),
        amount: String::from("99.99"),
    };

    let t = Transaction::from(c1);
    println!("The converted transaction is: {:?}!", t);
    Ok(())
}

fn example() -> Result<(), Box<dyn Error>> {
    // Read files in current dir? 
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            println!("{:?}", path.extension());
        }
    }


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
    if let Err(err) = test() {
        println!("error running test: {}", err);
        process::exit(1);
    }
}

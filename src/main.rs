use std::error::Error;
use std::io;
use std::process;
use std::convert::From;
use std::cmp::Ordering;
use rusty_money::{Money, iso};
use chrono::prelude::*;

mod Types;
use crate::Types::Transaction;
use crate::Types::CapOneTransaction;

fn convert() -> Result<(), Box<dyn Error>> {
  let c1 = CapOneTransaction {
    date: String::from("05/30/1992"),
    description: String::from("This is a transaction description"),
    amount: String::from("99.99"),
  };

  let t: Transaction = c1.into();
  println!("The converted transaction is: {:?}!", t);
  Ok(())
}

fn parse_transactions<'a, T>(mut rdr: csv::Reader<T>) -> Result<Vec<Transaction<'a>>, Box<dyn Error>> where T: std::io::Read {
  let mut transactions: Vec<Transaction> = vec![];
  for result in rdr.records() {
    // The iterator yields Result<StringRecord, Error>, so we check the
    // error here.
    let record = result?;

    let t = Transaction {
      date: NaiveDate::parse_from_str(&record[0], "%m/%d/%Y")?,
      name: record[2].to_string(),
      amount: Money::from_str(&record[5], iso::USD)?,
    };
    println!("{:?}", t);
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
  Ok(transactions)
}

fn read_from_file() -> Result<(), Box<dyn Error>> {
  // Read files in current dir? 
  for entry in std::fs::read_dir(".")? {
    let entry = entry?;
    let path = entry.path();
    if !path.is_dir() {
      println!("{:?}", path.extension());
    }
  }

  let file = std::fs::File::open("chase.csv")?;
  let rdr = csv::Reader::from_reader(file);

  let transactions = parse_transactions(rdr);
  println!("{:?}", transactions);
  Ok(())
}

fn read_from_stdin() -> Result<(), Box<dyn Error>> {
  // Build the CSV reader and iterate over each record.
  let rdr = csv::Reader::from_reader(io::stdin());

  let transactions = parse_transactions(rdr);
  println!("{:?}", transactions);
  Ok(())
}

fn main() {
  if let Err(err) = read_from_stdin() {
    println!("error running test: {}", err);
    process::exit(1);
  }
}

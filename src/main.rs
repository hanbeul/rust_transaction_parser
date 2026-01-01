use std::error::Error;
use std::io;
use std::process;
use std::convert::From;
use std::cmp::Ordering;
use rusty_money::{Money, iso};
use chrono::prelude::*;

mod Types;
use crate::Types::Transaction;
use crate::Types::VentureXTransaction;

//fn convert() -> Result<(), Box<dyn Error>> {
//  let c1 = CapOneTransaction {
//    date: String::from("05/30/1992"),
//    description: String::from("This is a transaction description"),
//    amount: String::from("99.99"),
//  };
//
//  let t: Transaction = c1.into();
//  println!("The converted transaction is: {:?}!", t);
//  Ok(())
//}

fn print_type<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}

fn parse_transactions<'a, T>(rdr: &mut csv::Reader<T>) -> Result<Vec<Transaction<'a>>, Box<dyn Error>> where T: std::io::Read {
  let mut transactions: Vec<Transaction> = vec![];
  let mut source: &str = "";

  // Read the first record (headers)
  // Determine what type of csv based off headers
  let headers = rdr.headers()?;
  let header_vec: Vec<String> = headers
    .iter()
    .map(|header| header.to_string())
    .collect();
  println!("{:?}", headers);
  if headers.iter().any(|field| field == "Memo") {
    source = "chase";
  } else if headers.iter().any(|field| field == "Account Number") {
    source = "checking";
  } else if headers.iter().any(|field| field == "Card No.") {
    source = "venture";
  }

  println!("source: {:?}", source);

  if source == "chase" {
    for result in rdr.records() {
      // The iterator yields Result<StringRecord, Error>, so we check the
      // error here.
      let record = result?;

      let t = Transaction {
        date: NaiveDate::parse_from_str(&record[0], "%m/%d/%Y")?,
        name: record[2].to_string(),
        amount: Money::from_str(&record[5], iso::USD)?,
        source: source.to_string(),
      };

      println!("{:?}", t);
      transactions.push(t);
    }
  } else if source == "venture" {
    for result in rdr.records() {
      // The iterator yields Result<StringRecord, Error>, so we check the
      // error here.
      let record = result?;

      let mut amount: Money<iso::Currency>;
      if &record[5] != "" {
        amount = Money::from_str(&record[5], iso::USD)? * -1;
      } else {
        amount = Money::from_str(&record[6], iso::USD)?;
      }

      let t = Transaction {
        date: NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?,
        name: record[3].to_string(),
        //amount: Money::from_str(&record[5], iso::USD)?,
        amount,
        source: source.to_string(),
      };
      println!("{:?}", t);
      transactions.push(t);
    }
  } else if source == "checking" {
    for result in rdr.records() {
      // The iterator yields Result<StringRecord, Error>, so we check the
      // error here.
      let record = result?;

      let transaction_type = &record[3];

      let mut amount: Money<iso::Currency>;
      if transaction_type == "Debit" {
        amount = Money::from_str(&record[4], iso::USD)? * -1;
      } else {
        amount = Money::from_str(&record[4], iso::USD)?;
      }

      let t = Transaction {
        date: NaiveDate::parse_from_str(&record[2], "%m/%d/%Y")?,
        name: record[1].to_string(),
        amount,
        source: source.to_string(),
      };
      println!("{:?}", t);
      transactions.push(t);
    }
  } else {
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
  let mut transactions: Vec<Transaction> = vec![];

  // Read files in current dir? 
  for entry in std::fs::read_dir("data")? {
    let entry = entry?;
    let path = entry.path();
    if let Some(extension) = path.extension() {
      if extension == "csv" && !path.is_dir(){
        println!("{:?}", path.file_name().unwrap());

        let file = std::fs::File::open(&path)?;
        let mut rdr = csv::Reader::from_reader(file);

        transactions.extend(parse_transactions(&mut rdr)?);
      }
    }
  }
  println!("{:?}", transactions);

  //let file = std::fs::File::open("data/2025-12-30_Checking...4181.csv")?;
  //let mut rdr = csv::Reader::from_reader(file);

  //let transactions = parse_transactions(&mut rdr);
  //println!("{:?}", transactions);
  Ok(())
}

fn read_from_stdin() -> Result<(), Box<dyn Error>> {
  // Build the CSV reader and iterate over each record.
  let mut rdr = csv::Reader::from_reader(io::stdin());

  let transactions = parse_transactions(&mut rdr);
  println!("{:?}", transactions);
  Ok(())
}

fn write_output(transactions: Vec<Transaction>) {
}

fn main() {
  if let Err(err) = read_from_file() {
    println!("error running test: {}", err);
    process::exit(1);
  }
}

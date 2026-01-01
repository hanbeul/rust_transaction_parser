use chrono::prelude::*;
use rusty_money::{Money, iso};
use serde::{Serialize, Deserialize};

// Generic Transaction
#[derive(Debug)]
pub struct Transaction<'a> {
  pub date: chrono::NaiveDate,
  pub name: String,
  pub amount: rusty_money::Money<'a, iso::Currency>,
  pub source: String,
}

#[derive(Debug)]
pub struct VentureXTransaction {
  pub transaction_date: String,
  pub posted_date: String,
  pub card_no: f64,
  pub description: String,
  pub category: String,
  pub debit: f64,
  pub credit: f64,
}

//impl From<VentureXTransaction> for Transaction<'_>{
//  fn from(t: VentureXTransaction) -> Self {
//    Transaction {
//      date: NaiveDate::parse_from_str(&t.tran, "%Y-%m-%d").unwrap(),
//      name: t.description,
//      amount: Money::from_str(&t.amount, iso::USD).unwrap(),
//    }
//  }
//}

//#[derive(Debug)]
//pub struct SapphireTransaction {
//  pub account_number: i64,
//  pub transaction_description: String,
//  pub transaction_date: String,
//  pub transaction_type:String,
//  pub transaction_amount:String,
//  pub balance:String,
//
//  pub date: String,
//  pub description: String,
//  pub amount: String,
//}
//
//impl From<SapphireTransaction> for Transaction<'_>{
//  fn from(t: SapphireTransaction) -> Self {
//    Transaction {
//      date: NaiveDate::parse_from_str(&t.date, "%m/%d/%Y").unwrap(),
//      name: t.description,
//      amount: Money::from_str(&t.amount, iso::USD).unwrap(),
//    }
//  }
//}

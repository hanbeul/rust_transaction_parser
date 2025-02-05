use chrono::prelude::*;
use rusty_money::{Money, iso};

#[derive(Debug)]
pub struct Transaction<'a> {
  pub date: chrono::NaiveDate,
  pub name: String,
  pub amount: rusty_money::Money<'a, iso::Currency>,
}

#[derive(Debug)]
pub struct CapOneTransaction {
  pub date: String,
  pub description: String,
  pub amount: String,
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

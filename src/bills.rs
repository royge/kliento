use crate::traits::{Counter, Extraction};
use calamine::{DataType, Range};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::iter::Iterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bill {
    pub account_number: String,
    pub amount: f64,
    pub due_date: String,
    pub period: String,
}

impl Bill {
    fn new() -> Bill {
        Bill {
            account_number: "".to_string(),
            amount: 0.0,
            due_date: "".to_string(),
            period: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bills {
    items: Vec<Bill>,

    pub grace_period: usize,
}

impl Extraction<Bill> for Bills {
    fn extract(&mut self, sheet: Range<DataType>) -> Result<(), Box<dyn Error>> {
        let today: DateTime<Local> = Local::now();

        for row in sheet.rows() {
            let mut data: Bill = Bill::new();

            for (index, cell) in row.iter().enumerate() {
                // NOTE: Prioritize dates to avoid unnecessary processing of invalid dates.
                if index == 2 {
                    let mut due_date = cell.to_string();
                    match chrono::NaiveDate::parse_from_str(&due_date, "%m/%d/%Y") {
                        Ok(x) => {
                            if today.date_naive() <= x {
                                due_date = x.format("%d-%m-%Y").to_string();
                            }
                        }
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    };

                    data.due_date = due_date;
                }
                if index == 3 {
                    let period = cell.to_string();
                    let period = match chrono::NaiveDate::parse_from_str(&period, "%m%Y") {
                        Ok(x) => x.format("%m%Y").to_string(),
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    };

                    data.period = period;
                }
                if index == 0 {
                    data.account_number = cell.to_string();
                }
                if index == 1 {
                    data.amount = cell.to_string().replace(",", "").parse::<f64>().unwrap();
                }
            }

            self.items.push(data);
        }

        Ok(())
    }
}

impl Counter<Bill> for Bills {
    fn count(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for Bills {
    type Item = Bill;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Bills {
    pub fn new() -> Bills {
        Bills {
            items: Vec::new(),
            grace_period: 0,
        }
    }
}

pub fn extract_valid_bills_from(sheet: Range<DataType>) -> Vec<Bill> {
    let mut row_data: Vec<Bill> = Vec::new();

    let today: DateTime<Local> = Local::now();

    for row in sheet.rows() {
        let mut data: Bill = Bill {
            account_number: "".to_string(),
            amount: 0.0,
            due_date: "".to_string(),
            period: "".to_string(),
        };

        let mut valid = false;
        for (index, cell) in row.iter().enumerate() {
            if index == 0 {
                data.account_number = cell.to_string();
            }
            if index == 1 {
                data.amount = cell.to_string().parse::<f64>().unwrap();
            }
            if index == 2 {
                let due_date = cell.to_string();
                let due_date = chrono::NaiveDate::parse_from_str(&due_date, "%m/%d/%Y").unwrap();

                if today.date_naive() <= due_date {
                    data.due_date = due_date.format("%d-%m-%Y").to_string();

                    // Override period with due date.
                    data.period = due_date.format("%m-%Y").to_string();

                    valid = true;
                }
            }
        }
        if valid {
            row_data.push(data);
        }
    }
    row_data
}

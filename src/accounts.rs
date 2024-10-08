use crate::traits::{Counter, Extraction};
use calamine::{DataType, Range};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::iter::Iterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    items: Vec<Account>,
}

impl Extraction<Account> for Accounts {
    fn extract(&mut self, sheet: Range<DataType>) -> Result<(), Box<dyn Error>> {
        for row in sheet.rows() {
            let mut data: Account = Account { id: "".to_string() };

            for (index, cell) in row.iter().enumerate() {
                if index == 0 {
                    data.id = cell.to_string();
                }
            }
            self.items.push(data);
        }

        Ok(())
    }
}

impl Counter<Account> for Accounts {
    fn count(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for Accounts {
    type Item = Account;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Accounts {
    pub fn new() -> Accounts {
        Accounts { items: Vec::new() }
    }
}

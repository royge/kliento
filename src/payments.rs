use crate::traits::{Counter, Extraction};
use calamine::{DataType, Range};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub pin: String,
    pub or_date: String,
    pub or_number: String,
    pub pin_hash: String,
}

impl Payment {
    fn new() -> Payment {
        Payment {
            pin: "".to_string(),
            or_date: "".to_string(),
            or_number: "".to_string(),
            pin_hash: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payments {
    items: Vec<Payment>,
}

impl Extraction<Payment> for Payments {
    fn extract(&mut self, sheet: Range<DataType>) -> Result<(), Box<dyn Error>> {
        for row in sheet.rows() {
            let mut data: Payment = Payment::new();

            for (index, cell) in row.iter().enumerate() {
                if index == 0 {
                    data.pin = cell.to_string();
                }
            }
            self.items.push(data);
        }

        Ok(())
    }
}

impl Counter<Payment> for Payments {
    fn count(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for Payments {
    type Item = Payment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Payments {
    pub fn new() -> Payments {
        Payments { items: Vec::new() }
    }
}

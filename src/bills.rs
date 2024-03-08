use calamine::{DataType, Range};
use chrono::prelude::*;
use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bill {
    pub account_number: String,
    pub amount: f64,
    pub due_date: String,
    pub period: String,
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

pub struct Callbacks {
    pub on_upload: fn(total_size: usize, batch_size: usize, iteration: usize),
    pub on_error: fn(error: String),
    pub on_success: fn(),
}

pub struct Config {
    pub url: String,
    pub token: String,
    pub batch_size: usize,
    pub timeout: u64,
    pub callbacks: Callbacks,
}

pub fn upload_bills(bills: Vec<Bill>, config: &Config) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let len = bills.len();

    if len == 0 {
        return Err("No valid bill info found.".to_string().into());
    }

    let capacity: usize;

    if len >= config.batch_size {
        capacity = config.batch_size;
    } else {
        capacity = len;
    }

    let mut data = Vec::with_capacity(capacity);

    for (index, bill) in bills.iter().enumerate() {
        data.push(bill);

        if data.capacity() == data.len() || index == len - 1 {
            let body = serde_json::to_string(&data).unwrap();
            let authorization = format!("Bearer {}", config.token);

            let iteration = get_iteration(&config.batch_size, &index);

            (config.callbacks.on_upload)(len, config.batch_size, iteration);

            let response = client
                .post(config.url.clone())
                .header(AUTHORIZATION, authorization)
                .body(body)
                .timeout(time::Duration::from_secs(config.timeout))
                .send()
                .unwrap();

            (config.callbacks.on_success)();

            data.clear();

            if !response.status().is_success() {
                let error = format!("Error uploading bill info: {}", response.text().unwrap());
                return Err(error.to_string().into());
            }
        }
    }

    Ok(())
}

pub fn get_upload_summary(
    total_size: usize,
    batch_size: usize,
    iteration: usize,
) -> (usize, usize) {
    let mut current = batch_size;
    let remaining = total_size + batch_size - (iteration * batch_size);

    if remaining < batch_size {
        current = remaining;
    }

    (current, remaining)
}

fn get_iteration(batch_size: &usize, index: &usize) -> usize {
    let position = index + 1;

    let mut iteration = position / batch_size;
    if position % batch_size != 0 {
        iteration += 1;
    }

    iteration
}

#[test]
fn test_get_upload_summary() {
    let (current, remaining) = get_upload_summary(13, 5, 1);
    assert_eq!(current, 5);
    assert_eq!(remaining, 13);

    let (current, remaining) = get_upload_summary(13, 5, 2);
    assert_eq!(current, 5);
    assert_eq!(remaining, 8);

    let (current, remaining) = get_upload_summary(13, 5, 3);
    assert_eq!(current, 3);
    assert_eq!(remaining, 3);
}

#[test]
fn test_get_iteration() {
    let iteration = get_iteration(&5, &4);
    assert_eq!(iteration, 1);

    let iteration = get_iteration(&5, &9);
    assert_eq!(iteration, 2);

    let iteration = get_iteration(&5, &12);
    assert_eq!(iteration, 3);

    let iteration = get_iteration(&5, &16);
    assert_eq!(iteration, 4);
}

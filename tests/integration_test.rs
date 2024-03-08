use calamine::{open_workbook, Reader, Xlsx};
use kliento::bills::{
    extract_valid_bills_from,
    upload_bills,
    Bill,
    Config,
    Callbacks,
};
use kliento::auth::{
    login,
    Credentials,
};
use std::env::var;

#[test]
fn test_extract_valid_bills_from() {
    let mut excel: Xlsx<_> = open_workbook("test.xlsx").unwrap();
    let sheet = excel.worksheet_range("Sheet1").unwrap();

    let bills = extract_valid_bills_from(sheet);

    // NOTE: Check the test.xlsx file for the expected values.
    assert_eq!(bills.len(), 4);
}

#[test]
fn test_login() {
    let auth_url = var("AUTH_URL").unwrap();
    let client_id = var("DEMO_CLIENT_ID").unwrap();
    let client_secret = var("DEMO_CLIENT_SECRET").unwrap();

    let token = login(
        auth_url.to_string(),
        &Credentials {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            grant_type: "client_credentials".to_string(),
        },
    );
    assert_eq!(token.contains("."), true);
}

#[test]
fn test_upload_bills() {
    let bills = vec![
        Bill {
            account_number: "123456789".to_string(),
            amount: 153.45,
            due_date: "28-02-2024".to_string(),
            period: "02-2024".to_string(),
        },
        Bill {
            account_number: "987654321".to_string(),
            amount: 513.21,
            due_date: "24-03-2024".to_string(),
            period: "03-2024".to_string(),
        },
        Bill {
            account_number: "887654321".to_string(),
            amount: 513.21,
            due_date: "22-01-2024".to_string(),
            period: "01-2024".to_string(),
        },
    ];

    let auth_url = var("AUTH_URL").unwrap();
    let client_id = var("DEMO_CLIENT_ID").unwrap();
    let client_secret = var("DEMO_CLIENT_SECRET").unwrap();
    let upload_url = var("UPLOAD_URL").unwrap();

    let token = login(
        auth_url.to_string(),
        &Credentials {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            grant_type: "client_credentials".to_string(),
        },
    );

    let result = upload_bills(
        bills,
        &Config {
            url: upload_url.to_string(),
            token,
            batch_size: 500,
            timeout: 60,
            callbacks: Callbacks {
                on_upload: |_total_size, _batch_size, _iteration| println!("Uploading bill info."),
                on_error: |error| println!("Error uploading bill info: {}", error),
                on_success: || println!("Successfully uploaded bill info."),
            },
        },
    );
    assert_eq!(result.unwrap(), ());
}

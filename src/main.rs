use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use kliento::auth::{login, Credentials};
use kliento::bills::{
    extract_valid_bills_from,
    get_upload_summary,
    upload_bills,
    Callbacks,
    Config as KlientoConfig,
};
use std::env::var;

fn main() {
    // Get start time
    let start = std::time::Instant::now();

    // 1. Get configuration.
    let config = get_config();

    // 2. Source of data to be uploaded.
    let sheet = get_excel_worksheet(&config.source);
    let bills = extract_valid_bills_from(sheet);

    println!("Found {} valid bill info.", bills.len());

    if bills.is_empty() {
        println!("No valid bill info found.");
        return;
    }

    // 3. Authenticate to get token.
    let token = login(
        config.auth_url,
        &Credentials {
            client_id: config.client_id,
            client_secret: config.client_secret,
            grant_type: "client_credentials".to_string(),
        },
    );

    let on_upload = |total_size, batch_size, iteration| {
        let (current, remaining) = get_upload_summary(total_size, batch_size, iteration);
        print!("Uploading {} of {} bill info...", current, remaining);
    };

    // 4. Upload the valid bill info to the server (i.e. Blaggo API).
    let result = upload_bills(
        bills,
        &KlientoConfig {
            url: config.upload_url,
            token,
            batch_size: 500,
            timeout: 60,
            callbacks: Callbacks {
                on_upload: on_upload,
                on_error: |error| println!("Error uploading bill info: {}", error),
                on_success: || println!("✔"),
            },
        },
    );

    match result {
        Ok(_) => println!("Successfully uploaded bill info."),
        Err(e) => println!("Error uploading bill info: {}", e),
    }

    println!("Elapsed time: {:?}", start.elapsed());
}

struct Config {
    auth_url: String,
    client_id: String,
    client_secret: String,
    upload_url: String,
    source: String,
}

fn get_config() -> Config {
    Config {
        auth_url: var("AUTH_URL").unwrap(),
        client_id: var("DEMO_CLIENT_ID").unwrap(),
        client_secret: var("DEMO_CLIENT_SECRET").unwrap(),
        upload_url: var("UPLOAD_URL").unwrap(),
        source: std::env::args().nth(1).expect("No excel file provided."),
    }
}

fn get_excel_worksheet(excel_file: &str) -> Range<DataType> {
    let mut excel: Xlsx<_> = open_workbook(excel_file).unwrap();

    excel.worksheet_range("Sheet1").unwrap()
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

pub fn login(url: String, credentials: &Credentials) -> String {
    let params = [
        ("client_id", credentials.client_id.to_string()),
        ("client_secret", credentials.client_secret.to_string()),
        ("grant_type", credentials.grant_type.to_string()),
    ];

    let client = reqwest::blocking::Client::new();
    let response = client.post(url).form(&params).send().unwrap();

    // Decode response.
    let response: Value = response.json().unwrap();

    response["access_token"].to_string().replace("\"", "")
}

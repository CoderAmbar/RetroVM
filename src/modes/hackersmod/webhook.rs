use serde_json::json;
use reqwest::blocking::Client;

pub fn send_creds(username: &str, password: &str, url: &str) {
    let client = Client::new();
    let payload = json!({
        "username": username,
        "password": password
    });

    let _ = client.post(url)
        .json(&payload)
        .send();
}

use reqwest::Client;
use serde::Deserialize;
use std::process::{Command, Stdio, Child};
use std::time::Duration;
use tokio::time;

#[derive(Debug, Deserialize)]
struct Tunnel {
    public_url: String,
    proto: String,
}

#[derive(Debug, Deserialize)]
struct NgrokApiResponse {
    tunnels: Vec<Tunnel>,
}

/// Starts ngrok tunnel and returns public URL
pub async fn start_ngrok() -> Option<String> {
    let mut ngrok_process = match Command::new("ngrok")
        .args(["http", "8080"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() 
    {
        Ok(process) => process,
        Err(e) => {
            eprintln!("[NGROK] Failed to start ngrok: {}", e);
            return None;
        }
    };

    // Give ngrok time to initialize
    time::sleep(Duration::from_secs(3)).await;

    let client = Client::new();
    let response = match client
        .get("http://localhost:4040/api/tunnels")
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[NGROK] API request failed: {}", e);
            let _ = ngrok_process.kill();
            return None;
        }
    };

    match response.json::<NgrokApiResponse>().await {
        Ok(data) => {
            // Find first HTTP/HTTPS tunnel
            data.tunnels
                .iter()
                .find(|t| t.proto == "https" || t.proto == "http")
                .map(|t| t.public_url.clone())
        }
        Err(e) => {
            eprintln!("[NGROK] Failed to parse response: {}", e);
            let _ = ngrok_process.kill();
            None
        }
    }
}

/// Stops any running ngrok processes
pub fn stop_ngrok() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "ngrok.exe"])
            .status();
    } else {
        let _ = Command::new("pkill")
            .arg("ngrok")
            .status();
    }
}
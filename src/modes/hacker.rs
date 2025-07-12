use macroquad::prelude::*;
use std::time::{Duration, Instant};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use ::rand::Rng;
use ::rand::thread_rng;

static START_TIME: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

pub struct HackerMode {
    pub triggered: bool,
    terminal_lines: Vec<String>,
    current_line_index: usize,
    char_index: usize,
    input_buffer: String,
    last_typing_time: Instant,
    blink_on: bool,
    music_sink: Option<Arc<Sink>>,
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
            music_sink: None,
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
            "Bypassing firewall...".to_string(),
            "Injecting payload...".to_string(),
            "Loading quantum exploit module...".to_string(),
            "Access granted. Welcome back, Commander.".to_string(),
        ];
        *START_TIME.lock().unwrap() = Some(Instant::now());
        self.last_typing_time = Instant::now();
        self.play_music();
    }

    pub fn draw_hacker_ui(&mut self) {
        clear_background(BLACK);

        // Early return if not triggered or no lines to render
        if !self.triggered || self.terminal_lines.is_empty() {
            return;
        }

        // Clone only the lines we need to render
        let lines_to_render = self.terminal_lines.clone();
        let mut rendered_lines = vec![];

        // Safely handle line rendering
        for i in 0..lines_to_render.len() {
            if i < self.current_line_index {
                rendered_lines.push(lines_to_render[i].clone());
            } else if i == self.current_line_index {
                let full = &lines_to_render[i];
                let partial: String = full.chars().take(self.char_index).collect();
                rendered_lines.push(partial);

                // Handle typing animation
                if self.last_typing_time.elapsed() >= Duration::from_millis(10) {
                    self.char_index = self.char_index.saturating_add(1);
                    self.last_typing_time = Instant::now();

                    if self.char_index >= full.chars().count() {
                        self.char_index = 0;
                        self.current_line_index = self.current_line_index.saturating_add(1);

                        // Random glitch effect
                        if thread_rng().gen_ratio(1, 10) {
                            rendered_lines.push("@##$%&*^GL!TCH~ERROR###".to_string());
                        }
                    }
                }
            }
        }

        // Render all lines
        let mut y = 20.0;
        for line in &rendered_lines {
            draw_text_ex(
                line,
                20.0,
                y,
                TextParams {
                    font: None,
                    font_size: 20,
                    color: GREEN,
                    ..Default::default()
                },
            );
            y += 24.0;
        }

        // Draw separator
        draw_line(20.0, y + 10.0, screen_width() - 20.0, y + 10.0, 1.0, GREEN);

        // Handle blinking cursor
        if self.last_typing_time.elapsed() >= Duration::from_millis(500) {
            self.blink_on = !self.blink_on;
            self.last_typing_time = Instant::now();
        }

        // Draw input prompt
        let cursor = if self.blink_on { "█" } else { " " };
        let prompt = format!("$ {}{}", self.input_buffer, cursor);
        draw_text_ex(
            &prompt,
            20.0,
            y + 30.0,
            TextParams {
                font: None,
                font_size: 20,
                color: GREEN,
                ..Default::default()
            },
        );

        // Handle input
        self.handle_input();
    }

    fn handle_input(&mut self) {
        // Process character input
        while let Some(key) = get_char_pressed() {
            self.input_buffer.push(key);
        }

        // Handle backspace
        if is_key_pressed(KeyCode::Backspace) {
            self.input_buffer.pop();
        }

        // Handle enter key
        if is_key_pressed(KeyCode::Enter) {
            let input = self.input_buffer.trim().to_lowercase();
            self.process_command(&input);
            self.input_buffer.clear();
            self.current_line_index = self.terminal_lines.len().saturating_sub(1);
            self.char_index = 0;
        }
    }

    fn process_command(&mut self, input: &str) {
        self.terminal_lines.push(format!("$ {}", input));
        
        match input {
            "help" => self.terminal_lines.push("Available: help, clear, ls, cat secret.txt, whoami, exit".to_string()),
            "clear" => {
                self.terminal_lines.clear();
                self.current_line_index = 0;
                self.char_index = 0;
            }
            "ls" => self.terminal_lines.push("Documents  Downloads  secrets  hack.exe".to_string()),
            "cat secret.txt" => {
                self.terminal_lines.extend(vec![
                    "Reading secret.txt...".to_string(),
                    "████████████████████████████████████".to_string(),
                    "Project: Retro-chan_AI".to_string(),
                    "Status: ACTIVE - OBSERVING SUBJECTS".to_string(),
                    "Note: Never trust a smiling ASCII face :)".to_string(),
                    "Location: ///ERROR///ACCESS DENIED///".to_string(),
                    "████████████████████████████████████".to_string(),
                    "👁️ RETRO-CHAN IS WATCHING 👁️".to_string(),
                ]);
            },
            "whoami" => self.terminal_lines.push("💀 you are root. all systems obey you.".to_string()),
            "exit" => {
                self.terminal_lines.extend(vec![
                    "Session terminated.".to_string(),
                    "Rebooting...".to_string()
                ]);
                if let Some(sink) = self.music_sink.take() {
                    sink.stop();
                }
                self.triggered = false;
            }
            _ => self.terminal_lines.push("Unknown command. Type `help` for options.".to_string()),
        }
    }

    fn play_music(&mut self) {
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(file) = File::open("assets/hacker_music.mp3") {
                let sink = Sink::try_new(&stream_handle).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
                sink.append(source);
                sink.play();

                self.music_sink = Some(Arc::new(sink));
                std::mem::forget(stream); // prevent drop
            }
        }
    }
}
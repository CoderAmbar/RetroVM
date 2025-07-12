use macroquad::prelude::*;
use macroquad::audio::*;
use crate::modes::floppy_disk::FloppyDiskGame;
use crate::modes::chess_GAME::{ChessGame, GameStatus};
use crate::modes::math_question::MathQuestion;
use crate::modes::notepad::Notepad;
use crate::modes::chatbot::Chatbot;
use crate::modes::hacker::HackerMode;
use std::fs::{File, remove_file};
use std::process::Command;
use chrono::Utc;

pub enum AppState {
    Booting,
    Desktop,
    Chatbot(Chatbot),
    FloppyDisk(FloppyDiskGame),
    Chess(ChessGame),
    MathQuestion(MathQuestion),
    Vedic(MathQuestion),
    HackerMode(HackerMode, MathQuestion),
    Notepad(Notepad),
    PasswordScreen,
    WelcomeScreen,
    VirusMode,
}

pub struct NormalMode {
    pub state: AppState,
    boot_start: f64,
    anime_effect_timer: f32,
    floppy_button_rect: Rect,
    chess_button_rect: Rect,
    vedic_button_rect: Rect,
    notepad_button_rect: Rect,
    chatbot_button_rect: Rect,
    password_input: String,
    virus_text: String,
    virus_sound: Option<Sound>,
    welcome_timer: f32,
    esc_pressed: bool,
}

impl NormalMode {
    pub async fn new() -> Self {
        Self {
            state: AppState::Booting,
            boot_start: get_time(),
            anime_effect_timer: 8.0,
            chatbot_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 60.0, 200.0, 50.0),
            floppy_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0, 200.0, 50.0),
            chess_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 70.0, 200.0, 50.0),
            vedic_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 140.0, 200.0, 50.0),
            notepad_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 210.0, 200.0, 50.0),
            password_input: String::new(),
            virus_text: String::new(),
            virus_sound: None,
            welcome_timer: 0.0,
            esc_pressed: false,
        }
    }

    pub async fn update(&mut self) -> bool {
        let dt = get_frame_time();
        self.anime_effect_timer += dt;

        self.esc_pressed = is_key_pressed(KeyCode::Escape);

        let mut next_state = None;

        match std::mem::replace(&mut self.state, AppState::Booting) {
            AppState::Booting => {
                if get_time() - self.boot_start > 3.0 {
                    next_state = Some(AppState::PasswordScreen);
                } else {
                    next_state = Some(AppState::Booting);
                }
            }

            AppState::PasswordScreen => {
                while let Some(c) = get_char_pressed() {
                    if c == '\u{8}' && !self.password_input.is_empty() {
                        self.password_input.pop();
                    } else if c.is_ascii() && !c.is_control() {
                        self.password_input.push(c);
                    }
                }

                if is_key_pressed(KeyCode::Enter) {
                    if self.password_input.trim() == "hola amigos!" {
                        next_state = Some(AppState::WelcomeScreen);
                    } else {
                        self.activate_virus().await;
                        next_state = Some(AppState::VirusMode);
                    }
                } else {
                    next_state = Some(AppState::PasswordScreen);
                }
            }

            AppState::WelcomeScreen => {
                self.welcome_timer += dt;
                if self.welcome_timer > 2.0 {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::WelcomeScreen);
                }
            }

            AppState::VirusMode => {
                while let Some(c) = get_char_pressed() {
                    if c == '\u{8}' && !self.virus_text.is_empty() {
                        self.virus_text.pop();
                    } else if c.is_ascii() && !c.is_control() {
                        self.virus_text.push(c);
                    }
                }

                if self.virus_text.trim().to_lowercase() == "stop" {
                    let _ = remove_file("VIRUS.txt");
                    if let Some(sound) = &self.virus_sound {
                        stop_sound(sound);
                    }
                    std::process::exit(0);
                    return true;
                }
                
                next_state = Some(AppState::VirusMode);
            }

            AppState::Desktop => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse = mouse_position();
                    if self.floppy_button_rect.contains(vec2(mouse.0, mouse.1)) {
                        next_state = Some(AppState::FloppyDisk(FloppyDiskGame::new().await));
                    } else if self.chess_button_rect.contains(vec2(mouse.0, mouse.1)) {
                        next_state = Some(AppState::Chess(ChessGame::new().await));
                    } else if self.vedic_button_rect.contains(vec2(mouse.0, mouse.1)) {
                        next_state = Some(AppState::Vedic(MathQuestion::new(0)));
                    } else if self.notepad_button_rect.contains(vec2(mouse.0, mouse.1)) {
                        next_state = Some(AppState::Notepad(Notepad::new()));
                    }  else if self.chatbot_button_rect.contains(vec2(mouse.0, mouse.1)) {
                            next_state = Some(AppState::Chatbot(Chatbot::new()));
                    } else {
                        next_state = Some(AppState::Desktop);
                    }
                } else if self.esc_pressed {
                    return true;
                } else {
                    next_state = Some(AppState::Desktop);
                }
            }

            AppState::Chatbot(mut chatbot) => {
                if self.esc_pressed || chatbot.update().await {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::Chatbot(chatbot));
                }
            }

            AppState::Chess(mut game) => {
                match game.update() {
                    GameStatus::Checkmate => next_state = Some(AppState::Desktop),
                    GameStatus::Exit => return true,
                    _ => {
                        if self.esc_pressed {
                            next_state = Some(AppState::Desktop);
                        } else {
                            next_state = Some(AppState::Chess(game));
                        }
                    }
                }
            }

            AppState::MathQuestion(mut question) => {
                if question.update() {
                    next_state = Some(AppState::Desktop);
                } else if let Some(hacker_mode) = question.hacker_mode.take() {
                    next_state = Some(AppState::HackerMode(hacker_mode, question));
                } else if self.esc_pressed {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::MathQuestion(question));
                }
            }

            AppState::HackerMode(mut hacker_mode, question) => {
                hacker_mode.update();
                next_state = if self.esc_pressed || !hacker_mode.triggered {
                    Some(AppState::MathQuestion(question))
                } else {
                    Some(AppState::HackerMode(hacker_mode, question))
                };
            }

            AppState::Notepad(mut notepad) => {
                let should_exit = notepad.update();
                next_state = if self.esc_pressed || should_exit {
                    Some(AppState::Desktop)
                } else {
                    Some(AppState::Notepad(notepad))
                };
            }

            AppState::FloppyDisk(mut game) => {
                if is_key_pressed(KeyCode::R) {
                    next_state = Some(AppState::FloppyDisk(FloppyDiskGame::new().await));
                } else {
                    game.update(dt);
                    
                    if self.esc_pressed {
                        next_state = Some(AppState::Desktop);
                    } else {
                        next_state = Some(AppState::FloppyDisk(game));
                    }
                }
            }

            AppState::Vedic(mut question) => {
                if question.update() {
                    next_state = Some(AppState::Desktop);
                } else if self.esc_pressed {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::Vedic(question));
                }
            }

            other => next_state = Some(other),
        }

        if let Some(new_state) = next_state {
            self.state = new_state;
        }

        false
    }

    async fn activate_virus(&mut self) {
        let _ = std::fs::write("VIRUS.txt", "VIRUS ACTIVATED! Type 'stop' to exit");
        
        self.virus_sound = Some(load_sound("assets/hell.mp3").await.unwrap());
        play_sound(
            self.virus_sound.as_ref().unwrap(),
            PlaySoundParams { looped: true, volume: 0.6 }
        );

        std::thread::spawn(|| {
            for _ in 0..10 {
                let _ = Command::new("notepad").spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        });
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        match &mut self.state {
            AppState::Booting => self.draw_booting(),
            AppState::PasswordScreen => {
                draw_text("🔐 Enter access code:", 20.0, 80.0, 28.0, GREEN);
                draw_text(&self.password_input, 20.0, 120.0, 24.0, GREEN);
                
                if (get_time() * 2.0).sin() > 0.0 {
                    let cursor_x = 20.0 + measure_text(&self.password_input, None, 24, 1.0).width;
                    draw_rectangle(cursor_x, 100.0, 2.0, 24.0, GREEN);
                }
            }
            AppState::WelcomeScreen => {
                draw_text("kaise ho theek ho!1", screen_width()/2.0 - 100.0, screen_height()/2.0, 30.0, GREEN);
            }
            AppState::VirusMode => {
                clear_background(RED);
                draw_text("☠️ VIRUS ACTIVATED ☠️", 40.0, 100.0, 32.0, YELLOW);
                draw_text("Type 'stop' to exit...", 40.0, 150.0, 24.0, WHITE);
                draw_text(&self.virus_text, 40.0, 200.0, 24.0, WHITE);
            }
            AppState::Desktop => self.draw_desktop(),
            AppState::FloppyDisk(game) => game.draw(),
            AppState::Chess(game) => game.draw(),
            AppState::MathQuestion(question) => question.draw(),
            AppState::Vedic(question) => question.draw(),
            AppState::HackerMode(hacker_mode, _) => hacker_mode.draw_hacker_ui(),
            AppState::Notepad(notepad) => notepad.draw(),
            AppState::Chatbot(chatbot) => {
                chatbot.draw();
            }
        }
    }

    fn draw_booting(&self) {
        let pulse = 0.5 + 0.5 * (self.anime_effect_timer * 2.0).sin();
        let spinner_chars = ["◜", "◝", "◞", "◟"];
        let spinner_idx = ((self.anime_effect_timer * 5.0) as usize) % spinner_chars.len();

        draw_text(
            "SYSTEM BOOTING...", 
            screen_width() / 2.0 - 100.0, 
            screen_height() / 2.0 - 30.0, 
            30.0, 
            WHITE
        );
        
        draw_text(
            spinner_chars[spinner_idx],
            screen_width() / 2.0 - 15.0,
            screen_height() / 2.0 + 10.0,
            60.0,
            Color::from_rgba(255, (255.0 * pulse) as u8, 100, 255)
        );
    }

    fn draw_desktop(&self) {
        clear_background(Color::from_rgba(20, 20, 40, 255));
        draw_text("RETRO VM DESKTOP", screen_width() / 2.0 - 150.0, 100.0, 40.0, WHITE);

        let mouse = mouse_position();
        let draw_button = |rect: &Rect, label: &str, base_color: Color, hover_color: Color| {
            let hovered = rect.contains(vec2(mouse.0, mouse.1));
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, if hovered { hover_color } else { base_color });
            draw_text(label, rect.x + 15.0, rect.y + 35.0, 30.0, WHITE);
        };

        draw_button(&self.floppy_button_rect, "💾 FLOPPY DISK", PURPLE, DARKPURPLE);
        draw_button(&self.chess_button_rect, "♟️ CHESS GAME", GREEN, DARKGREEN);
        draw_button(&self.vedic_button_rect, "🧮 VEDIC MATH", BLUE, DARKBLUE);
        draw_button(&self.notepad_button_rect, "📝 NOTEPAD", GRAY, DARKGRAY);
        draw_button(&self.chatbot_button_rect, "🤖 CHATBOT", ORANGE, Color::new(0.7, 0.4, 0.0, 1.0));
    }
}
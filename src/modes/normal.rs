use macroquad::prelude::*;
use crate::modes::floppy_disk::FloppyDiskGame;
use crate::modes::chess_GAME::{ChessGame, GameStatus};
use crate::modes::math_question::MathQuestion;
use crate::modes::hacker::HackerMode;
pub enum AppState {
    Booting,
    Desktop,
    FloppyDisk(FloppyDiskGame),
    Chess(ChessGame),
    MathQuestion(MathQuestion),
    Vedic(MathQuestion),
    HackerMode(HackerMode, MathQuestion), // Now stores both states
}

pub struct NormalMode {
    pub state: AppState,
    boot_start: f64,
    anime_effect_timer: f32,
    floppy_button_rect: Rect,
    chess_button_rect: Rect,
    vedic_button_rect: Rect,
}

impl NormalMode {
    pub fn new() -> Self {
        Self {
            state: AppState::Booting,
            boot_start: get_time(),
            anime_effect_timer: 8.0,
            floppy_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0, 200.0, 50.0),
            chess_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 70.0, 200.0, 50.0),
            vedic_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 140.0, 200.0, 50.0),
        }
    }

    // In normal.rs
    pub async fn update(&mut self) -> bool {
        let dt = get_frame_time();
        self.anime_effect_timer += dt;

        // Create a temporary variable to hold the next state
        let mut next_state = None;

        // Use a match statement that doesn't borrow self.state
        match std::mem::replace(&mut self.state, AppState::Booting) {
            AppState::Booting => {
                if get_time() - self.boot_start > 3.0 {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::Booting);
                }
            }
            
            AppState::Chess(mut game) => {
                match game.update() {
                    GameStatus::Checkmate => {
                        next_state = Some(AppState::Desktop);
                    }
                    GameStatus::Exit => {
                        return true;
                    }
                    _ => {
                        next_state = Some(AppState::Chess(game));
                    }
                }
            }
            
            // In normal.rs, modify the MathQuestion and HackerMode match arms:

            AppState::MathQuestion(mut question) => {
                if question.update() {
                    next_state = Some(AppState::Desktop);
                } else if let Some(hacker_mode) = question.hacker_mode.take() {
                    // Store the MathQuestion state before entering Hacker mode
                    next_state = Some(AppState::HackerMode(hacker_mode, question));
                } else {
                    next_state = Some(AppState::MathQuestion(question));
                }
            }

            AppState::HackerMode(mut hacker_mode, question) => {
                hacker_mode.update();
                
                if is_key_pressed(KeyCode::Escape) || !hacker_mode.triggered {
                    // Return to the original MathQuestion state
                    next_state = Some(AppState::MathQuestion(question));
                } else {
                    next_state = Some(AppState::HackerMode(hacker_mode, question));
                }
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
                    } else {
                        next_state = Some(AppState::Desktop);
                    }
                } else if is_key_pressed(KeyCode::Escape) {
                    return true;
                } else {
                    next_state = Some(AppState::Desktop);
                }
            }
            
            AppState::FloppyDisk(mut game) => {
                if is_key_pressed(KeyCode::R) {
                    next_state = Some(AppState::FloppyDisk(FloppyDiskGame::new().await));
                } else {
                    game.update(dt);
                    
                    if is_key_pressed(KeyCode::Escape) {
                        next_state = Some(AppState::Desktop);
                    } else {
                        next_state = Some(AppState::FloppyDisk(game));
                    }
                }
            }
            
            AppState::Vedic(mut question) => {
                if question.update() {
                    next_state = Some(AppState::Desktop);
                } else {
                    next_state = Some(AppState::Vedic(question));
                }
            }
        }

        // Restore the state (or set the new one)
        if let Some(new_state) = next_state {
            self.state = new_state;
        }

        false
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        match &mut self.state {
            AppState::Booting => self.draw_booting(),
            AppState::Desktop => self.draw_desktop(),
            AppState::FloppyDisk(game) => game.draw(),
            AppState::Chess(game) => game.draw(),
            AppState::MathQuestion(question) => question.draw(),
            AppState::Vedic(question) => question.draw(),
            AppState::HackerMode(hacker_mode, _) => {
                hacker_mode.draw_hacker_ui();
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

        // Floppy Disk Button
        let mouse = mouse_position();
        let floppy_hovered = self.floppy_button_rect.contains(vec2(mouse.0, mouse.1));
        let floppy_color = if floppy_hovered { DARKPURPLE } else { PURPLE };

        draw_rectangle(
            self.floppy_button_rect.x,
            self.floppy_button_rect.y,
            self.floppy_button_rect.w,
            self.floppy_button_rect.h,
            floppy_color,
        );

        draw_text(
            "💾 FLOPPY DISK",
            self.floppy_button_rect.x + 15.0,
            self.floppy_button_rect.y + 35.0,
            30.0,
            WHITE,
        );

        // Chess Button
        let chess_hovered = self.chess_button_rect.contains(vec2(mouse.0, mouse.1));
        let chess_color = if chess_hovered { DARKGREEN } else { GREEN };

        draw_rectangle(
            self.chess_button_rect.x,
            self.chess_button_rect.y,
            self.chess_button_rect.w,
            self.chess_button_rect.h,
            chess_color,
        );

        draw_text(
            "♟️ CHESS GAME",
            self.chess_button_rect.x + 15.0,
            self.chess_button_rect.y + 35.0,
            30.0,
            WHITE,
        );

        // Vedic Math Button
        let vedic_hovered = self.vedic_button_rect.contains(vec2(mouse.0, mouse.1));
        let vedic_color = if vedic_hovered { DARKBLUE } else { BLUE };

        draw_rectangle(
            self.vedic_button_rect.x,
            self.vedic_button_rect.y,
            self.vedic_button_rect.w,
            self.vedic_button_rect.h,
            vedic_color,
        );

        draw_text(
            "🧮 VEDIC MATH",
            self.vedic_button_rect.x + 15.0,
            self.vedic_button_rect.y + 35.0,
            30.0,
            WHITE,
        );
    }
}
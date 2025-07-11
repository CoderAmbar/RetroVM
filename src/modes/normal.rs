use macroquad::prelude::*;
use crate::modes::floppy_disk::FloppyDiskGame;
use crate::modes::chess_GAME::{ChessGame};

pub enum AppState {
    Booting,
    Desktop,
    FloppyDisk(FloppyDiskGame),
    Chess(ChessGame),
}

pub struct NormalMode {
    state: AppState,
    boot_start: f64,
    anime_effect_timer: f32,
    floppy_button_rect: Rect,
    chess_button_rect: Rect,
}

impl NormalMode {
    pub fn new() -> Self {
        Self {
            state: AppState::Booting,
            boot_start: get_time(),
            anime_effect_timer: 8.0,
            floppy_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0, 200.0, 50.0),
            chess_button_rect: Rect::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 70.0, 200.0, 50.0),
        }
    }

    pub async fn update(&mut self) {
        let dt = get_frame_time();
        self.anime_effect_timer += dt;

        match &mut self.state {
            AppState::Booting => {
                if get_time() - self.boot_start > 3.0 {
                    self.state = AppState::Desktop;
                }

                let pulse = 0.5 + 0.5 * (self.anime_effect_timer * 2.0).sin();
                let spinner_chars = ["◜", "◝", "◞", "◟"];
                let spinner_idx = ((self.anime_effect_timer * 5.0) as usize) % spinner_chars.len();

                clear_background(BLACK);
                draw_text("SYSTEM BOOTING...", screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 30.0, 30.0, WHITE);
                draw_text(spinner_chars[spinner_idx], screen_width() / 2.0, screen_height() / 2.0 + 10.0, 60.0, Color::from_rgba(255, (255.0 * pulse) as u8, 100, 255));
            }

            AppState::Desktop => {
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

                draw_text("💾 FLOPPY DISK",
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

                draw_text("♟️ CHESS GAME",
                    self.chess_button_rect.x + 15.0,
                    self.chess_button_rect.y + 35.0,
                    30.0,
                    WHITE,
                );

                if floppy_hovered && is_mouse_button_pressed(MouseButton::Left) {
                    self.state = AppState::FloppyDisk(FloppyDiskGame::new().await);
                }

                if chess_hovered && is_mouse_button_pressed(MouseButton::Left) {
                    self.state = AppState::Chess(ChessGame::new().await);
                }
            }

            AppState::FloppyDisk(game) => {
                clear_background(DARKGRAY);
                game.update(dt).await;
                game.draw();

                if is_key_pressed(KeyCode::Escape) {
                    self.state = AppState::Desktop;
                }
            }

            AppState::Chess(game) => {
                clear_background(DARKGRAY);
                game.update();
                game.draw();

                if is_key_pressed(KeyCode::Escape) {
                    self.state = AppState::Desktop;
                }
            }
        }
    }
}
mod modes;

use macroquad::prelude::*;
use modes::{NormalMode, ghost_an::GhostAnimation};

enum AppState {
    Menu,
    Normal(NormalMode),
    Ghost(GhostAnimation),
}
fn draw_button(label: &str, pos: Vec2, size: Vec2) -> bool {
    let mouse = mouse_position().into();
    let hovered = Rect::new(pos.x, pos.y, size.x, size.y).contains(mouse);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let color = if hovered {
        GRAY
    } else {
        DARKGRAY
    };

    draw_rectangle(pos.x, pos.y, size.x, size.y, color);
    draw_text(
        label,
        pos.x + 20.0,
        pos.y + size.y / 2.0 + 8.0,
        24.0,
        WHITE,
    );

    clicked
}

#[macroquad::main("Retro VM")]
async fn main() {
    let mut state = AppState::Menu;
    loop {
        clear_background(BLACK);
        // Global Tab key to go back to menu from any state
        if is_key_pressed(KeyCode::Tab) {
            state = AppState::Menu;
        }
        if is_key_pressed(KeyCode::Escape) {
            state = AppState::Normal(NormalMode::new());
        }
        match &mut state {
            AppState::Menu => {
                // Draw menu UI
                let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
                let button_size = vec2(200.0, 50.0);
                let spacing = 20.0;
                // Ghost Mode Button
                let ghost_btn_pos = screen_center - vec2(button_size.x / 2.0, button_size.y + spacing);
                if draw_button("👻 Ghost Mode", ghost_btn_pos, button_size) {
                    state = AppState::Ghost(GhostAnimation::new().await);
                }
                // Normal Mode Button
                let normal_btn_pos = screen_center - vec2(button_size.x / 2.0, -spacing);
                if draw_button("🖥 Normal Mode", normal_btn_pos, button_size) {
                    state = AppState::Normal(NormalMode::new());
                }
            }
            AppState::Normal(normal) => {
                normal.update().await;
                // Allow switching to Ghost mode with 'G' key
                if is_key_pressed(KeyCode::G) {
                    state = AppState::Ghost(GhostAnimation::new().await);
                }
            }
            AppState::Ghost(ghost) => {
                ghost.update();
                ghost.draw();
            }
        }
        next_frame().await;
    }
}

mod modes;

use macroquad::prelude::*;
use modes::{NormalMode, ghost_an::GhostAnimation, ghost::GhostMode};

enum AppState {
    Menu,
    Normal(NormalMode),
    Ghost(GhostState),  // Combined ghost state
}

enum GhostState {
    Animation(GhostAnimation),
    Manager(GhostMode),
}

struct MenuTab {
    label: &'static str,
    position: Vec2,
    size: Vec2,
    active: bool,
}

impl MenuTab {
    fn new(label: &'static str, position: Vec2, size: Vec2) -> Self {
        MenuTab {
            label,
            position,
            size,
            active: false,
        }
    }

    fn draw(&self) -> bool {
        let mouse = mouse_position().into();
        let hovered = Rect::new(self.position.x, self.position.y, self.size.x, self.size.y).contains(mouse);
        let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

        let color = if self.active {
            DARKGRAY
        } else if hovered {
            GRAY
        } else {
            Color::new(0.1, 0.1, 0.1, 1.0)
        };

        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, color);
        draw_text(
            self.label,
            self.position.x + 20.0,
            self.position.y + self.size.y / 2.0 + 8.0,
            24.0,
            WHITE,
        );

        clicked
    }
}

#[macroquad::main("Retro VM")]
async fn main() {
    let mut state = AppState::Menu;
    let mut shift_d_pressed = false;
    
    loop {
        clear_background(BLACK);
        
        // Global key shortcuts
        if is_key_pressed(KeyCode::Tab) {
            state = AppState::Menu;
        }
        

        match &mut state {
            AppState::Menu => {
                let screen_width = screen_width();
                let tab_width = screen_width / 2.0; // Only two tabs now
                let tab_height = 60.0;
                
                // Create tabs - only Normal and Ghost now
                let mut tabs = vec![
                    MenuTab::new("🖥 Normal Mode", vec2(0.0, 0.0), vec2(tab_width, tab_height)),
                    MenuTab::new("👻 Ghost Mode", vec2(tab_width, 0.0), vec2(tab_width, tab_height)),
                ];
                
                // Highlight active tab
                match &state {
                    AppState::Normal(_) => tabs[0].active = true,
                    AppState::Ghost(_) => tabs[1].active = true,
                    _ => {}
                }
                
                // Draw tabs and handle clicks
                for (i, tab) in tabs.iter_mut().enumerate() {
                    if tab.draw() {
                        match i {
                            0 => state = AppState::Normal(NormalMode::new().await),
                            1 => state = AppState::Ghost(GhostState::Animation(GhostAnimation::new().await)),
                            _ => {}
                        }
                    }
                }
                
                // Draw content area below tabs
                let content_area = Rect::new(
                    0.0,
                    tab_height,
                    screen_width,
                    screen_height() - tab_height
                );
                
                draw_rectangle(
                    content_area.x,
                    content_area.y,
                    content_area.w,
                    content_area.h,
                    Color::new(0.05, 0.05, 0.05, 1.0)
                );
                
                // Draw instructions
                let instructions = match &state {
                    AppState::Normal(_) => "Normal Mode - Standard VM interface\n\nPress Tab to return to menu",
                    AppState::Ghost(_) => "Ghost Mode - Press Shift+D to unlock advanced features\n\nCurrently showing: Ghost Animation",
                    AppState::Menu => "Select a mode from the tabs above",
                };
                
                draw_text(
                    instructions,
                    content_area.x + 20.0,
                    content_area.y + 40.0,
                    24.0,
                    LIGHTGRAY,
                );
            }
            
            AppState::Normal(normal) => {
                if normal.update().await {
                    // If update returns true, we should exit back to menu
                    state = AppState::Menu;
                } else {
                    normal.draw();
                }
            }
            
            AppState::Ghost(ghost_state) => {
                match ghost_state {
                    GhostState::Animation(ghost_anim) => {
                        // Check for shift+D to switch to manager mode
                        if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::D) {
                            *ghost_state = GhostState::Manager(GhostMode::new());
                        } else {
                            ghost_anim.update();
                            ghost_anim.draw();
                        }
                    }
                    GhostState::Manager(ghost_mode) => {
                        ghost_mode.update().await;
                        ghost_mode.draw();
                    }
                }
            }
        }

        next_frame().await;
    }
}
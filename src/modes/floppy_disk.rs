use macroquad::prelude::*;
use macroquad::texture::FilterMode;
use macroquad::rand::ChooseRandom;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WALL_THICKNESS: f32 = 6.0;
const MARGIN: f32 = 20.0;
const HUD_HEIGHT: f32 = 40.0;
const COLS: usize = 16;
const ROWS: usize = 12;

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // top, right, bottom, left
}

impl Cell {
    fn new() -> Self {
        Self {
            visited: false,
            walls: [true; 4],
        }
    }
}

struct Wall {
    rect: Rect,
}

impl Wall {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { rect: Rect::new(x, y, w, h) }
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW);
    }
}

struct Player {
    pos: Vec2,
    speed: f32,
    texture: Texture2D,
    size: Vec2,
}

impl Player {
    async fn new() -> Self {
        let texture = load_texture("assets/floppy.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        let cell_width = (WINDOW_WIDTH - 2.0 * MARGIN) / COLS as f32;
        let cell_height = (WINDOW_HEIGHT - 2.0 * MARGIN - HUD_HEIGHT) / ROWS as f32;
        let start_x = MARGIN + cell_width / 2.0 - 16.0;
        let start_y = MARGIN + HUD_HEIGHT + cell_height / 2.0 - 16.0;

        Self {
            pos: vec2(start_x, start_y),
            speed: 200.0,
            texture,
            size: vec2(32.0, 32.0),
        }
    }

    fn update(&mut self, dt: f32, walls: &[Wall]) {
        let mut dir = vec2(0.0, 0.0);
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { dir.x += 1.0; }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { dir.x -= 1.0; }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { dir.y -= 1.0; }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { dir.y += 1.0; }

        if dir != Vec2::ZERO {
            let new_pos = self.pos + dir.normalize() * self.speed * dt;

            let padding = 6.0;
            let future_rect = Rect::new(
                new_pos.x + padding / 2.0,
                new_pos.y + padding / 2.0,
                self.size.x - padding,
                self.size.y - padding,
            );

            if !walls.iter().any(|w| w.rect().overlaps(&future_rect)) {
                self.pos = new_pos;
            }
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );

        let padding = 6.0;
        draw_rectangle_lines(
            self.pos.x + padding / 2.0,
            self.pos.y + padding / 2.0,
            self.size.x - padding,
            self.size.y - padding,
            1.0,
            RED,
        );
    }
}

fn load_random_maze_walls() -> Vec<Wall> {
    let mut grid = vec![vec![Cell::new(); COLS]; ROWS];
    let mut walls = vec![];
    let mut stack = vec![];
    let mut current = (0, 0);
    grid[0][0].visited = true;

    while let Some(_) = Some(current) {
        let (x, y) = current;

        let mut neighbors = vec![];
        if y > 0 && !grid[y - 1][x].visited { neighbors.push((x, y - 1, 0)); }
        if x < COLS - 1 && !grid[y][x + 1].visited { neighbors.push((x + 1, y, 1)); }
        if y < ROWS - 1 && !grid[y + 1][x].visited { neighbors.push((x, y + 1, 2)); }
        if x > 0 && !grid[y][x - 1].visited { neighbors.push((x - 1, y, 3)); }

        if !neighbors.is_empty() {
            let &(nx, ny, dir) = neighbors.choose().unwrap();
            grid[y][x].walls[dir] = false;
            grid[ny][nx].walls[(dir + 2) % 4] = false;
            stack.push(current);
            grid[ny][nx].visited = true;
            current = (nx, ny);
        } else if let Some(prev) = stack.pop() {
            current = prev;
        } else {
            break;
        }
    }

    let cell_width = (WINDOW_WIDTH - 2.0 * MARGIN) / COLS as f32;
    let cell_height = (WINDOW_HEIGHT - 2.0 * MARGIN - HUD_HEIGHT) / ROWS as f32;

    for y in 0..ROWS {
        for x in 0..COLS {
            let cx = MARGIN + x as f32 * cell_width;
            let cy = MARGIN + HUD_HEIGHT + y as f32 * cell_height;
            let cell = grid[y][x];

            if cell.walls[0] {
                walls.push(Wall::new(cx, cy, cell_width, WALL_THICKNESS));
            }
            if cell.walls[1] {
                walls.push(Wall::new(cx + cell_width - WALL_THICKNESS, cy, WALL_THICKNESS, cell_height));
            }
            if cell.walls[2] {
                walls.push(Wall::new(cx, cy + cell_height - WALL_THICKNESS, cell_width, WALL_THICKNESS));
            }
            if cell.walls[3] {
                walls.push(Wall::new(cx, cy, WALL_THICKNESS, cell_height));
            }
        }
    }

    walls
}

pub struct FloppyDiskGame {
    player: Player,
    walls: Vec<Wall>,
    cpu_texture: Texture2D,
    cpu_size: Vec2,
    cpu_pos: Vec2,
    time_left: f32,
    game_won: bool,
    game_over: bool,
    grace_timer: f32,
}

impl FloppyDiskGame {
    pub async fn new() -> Self {
        let player = Player::new().await;
        let cpu_texture = load_texture("assets/cpu.png").await.unwrap();
        
        Self {
            player,
            walls: load_random_maze_walls(),
            cpu_texture,
            cpu_size: vec2(48.0, 48.0),
            cpu_pos: vec2(WINDOW_WIDTH - MARGIN - 58.0, WINDOW_HEIGHT - MARGIN - 58.0),
            time_left: 180.0,
            game_won: false,
            game_over: false,
            grace_timer: 1.0,
        }
    }

    pub async fn update(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::R) {
            *self = Self::new().await;
        }

        if !self.game_won && !self.game_over {
            self.time_left -= dt;
            if self.grace_timer > 0.0 {
                self.grace_timer -= dt;
                self.player.update(dt, &[]);
            } else {
                self.player.update(dt, &self.walls);
            }
        }

        let player_rect = Rect::new(self.player.pos.x, self.player.pos.y, self.player.size.x, self.player.size.y);
        let cpu_rect = Rect::new(self.cpu_pos.x, self.cpu_pos.y, self.cpu_size.x, self.cpu_size.y);
        if player_rect.overlaps(&cpu_rect) {
            self.game_won = true;
        }
        if self.time_left <= 0.0 && !self.game_won {
            self.game_over = true;
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        
        for wall in &self.walls {
            wall.draw();
        }
        
        self.player.draw();

        draw_texture_ex(
            &self.cpu_texture,
            self.cpu_pos.x,
            self.cpu_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.cpu_size),
                ..Default::default()
            },
        );

        if self.game_won {
            draw_text("✅ Delivered to CPU!", 180.0, 50.0, 28.0, YELLOW);
        } else if self.game_over {
            draw_text("💀 Disk Corrupted! Game Over.", 100.0, screen_height() / 2.0, 30.0, RED);
        } else {
            let timer_color = if self.time_left < 10.0 { RED } else { GREEN };
            draw_text(&format!("Time Left: {:.1}s", self.time_left), 10.0, 30.0, 24.0, timer_color);

            if self.time_left < 10.0 {
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(255, 0, 0, 80));
                draw_text(
                    "⚠️ DISK IS CORRUPTING!",
                    screen_width() / 2.0 - 170.0,
                    screen_height() / 2.0 - 40.0,
                    36.0,
                    ORANGE,
                );
            }
        }

        draw_text("Press R to Restart", screen_width() - 200.0, 30.0, 20.0, GRAY);
    }
}

#[macroquad::main("Floppy Disk Courier")]
async fn main() {
    let mut game = FloppyDiskGame::new().await;

    loop {
        let dt = get_frame_time();

        game.update(dt).await;
        game.draw();

        next_frame().await;
    }
}
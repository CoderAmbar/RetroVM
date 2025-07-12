pub mod normal;
pub use normal::NormalMode;
pub mod floppy_disk;
pub mod ghost_an;
pub mod chess_GAME;
pub mod math_question;
pub mod ghost;
pub mod hacker;
pub trait ModeUI {
    fn ui(&mut self, ctx: &eframe::egui::Context);
}

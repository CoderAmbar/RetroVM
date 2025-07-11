use super::ModeUI;
use eframe::egui;

#[derive(Default)]
pub struct HackerMode;

impl ModeUI for HackerMode {
    fn ui(&mut self, ctx: &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Normal Mode Active");
        });
    }
}
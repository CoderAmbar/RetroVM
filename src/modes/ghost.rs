use super::ModeUI;
use eframe::egui;
use std::path::PathBuf;
use std::process::Command;

pub struct GhostMode {
    kali_iso_path: PathBuf,
    ram_gb: u8,
    cpu_cores: u8,
    qemu_path: PathBuf,
    is_vm_running: bool,
    vm_process: Option<std::process::Child>,
    last_error: Option<String>,
    show_animation: bool,
    animation_timer: u32,
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

impl ModeUI for GhostMode {
    fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_animation {
                self.render_animation(ui);
            } else {
                self.render_header(ui);
                self.render_configuration(ui);
                self.render_actions(ui);
                self.render_status(ui);
                self.render_help(ui);
            }
        });
    }
}

impl GhostMode {
    // Animation Screen
    fn render_animation(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("👻 Ghost Mode Activated");
            ui.add_space(20.0);
            
            // Simple ghost animation using text
            self.animation_timer += 1;
            let frame = (self.animation_timer / 15) % 6;
            let ghost = match frame {
                0 => r#"
                    .-.
                   (o o)
                   | O |
                    \_/
                "#,
                1 => r#"
                    .-.
                   (o o)
                   | ^ |
                    \_/
                "#,
                2 => r#"
                    .-.
                   (o o)
                   | O |
                    \_/
                "#,
                3 => r#"
                    .-.
                   (o o)
                   | - |
                    \_/
                "#,
                4 => r#"
                    .-.
                   (o o)
                   | O |
                    \_/
                "#,
                _ => r#"
                    .-.
                   (o o)
                   | ~ |
                    \_/
                "#,
            };
            
            ui.label(egui::RichText::new(ghost).monospace().size(24.0));
            ui.add_space(20.0);
            
            if ui.button("🚀 Enter VM Controls").clicked() {
                self.show_animation = false;
            }
            
            ui.add_space(10.0);
            ui.label("Ghost mode provides a secure VM environment");
            ui.label("for running Kali Linux with QEMU");
        });
    }

    // UI Sections
    fn render_header(&self, ui: &mut egui::Ui) {
        ui.heading("👻 Ghost Mode - Kali Linux VM");
    }

    fn render_configuration(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("vm_config_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label("QEMU Path:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.qemu_path.to_string_lossy());
                    if ui.button("📂").clicked() {
                        self.pick_qemu_path();
                    }
                });
                ui.end_row();

                ui.label("Kali ISO:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.kali_iso_path.to_string_lossy());
                    if ui.button("📂").clicked() {
                        self.pick_iso_path();
                    }
                });
                ui.end_row();

                ui.label("RAM (GB):");
                ui.add(egui::Slider::new(&mut self.ram_gb, 2..=32));
                ui.end_row();

                ui.label("CPU Cores:");
                ui.add(egui::Slider::new(&mut self.cpu_cores, 1..=8));
                ui.end_row();
            });
    }

    fn render_actions(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🚀 Launch VM").clicked() {
                self.launch_vm();
            }

            if self.is_vm_running {
                if ui.button("☠️ Terminate").clicked() {
                    self.kill_vm();
                }
            }

            if ui.button("👻 Back to Animation").clicked() {
                self.show_animation = true;
                self.animation_timer = 0;
            }
        });
    }

    fn render_status(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        if let Some(err) = &self.last_error {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
        }

        if self.is_vm_running {
            ui.colored_label(
                egui::Color32::GREEN,
                format!("● VM Active (PID: {})", self.vm_process.as_ref().unwrap().id()),
            );
        } else {
            ui.colored_label(egui::Color32::RED, "○ VM Inactive");
        }
    }

    fn render_help(&self, ui: &mut egui::Ui) {
        ui.collapsing("💡 Help", |ui| {
            ui.label("1. Download Kali ISO from https://www.kali.org/get-kali/");

            #[cfg(target_os = "windows")]
            {
                ui.label("2. Install QEMU from https://qemu.weilnetz.de/w64/");
                ui.label("3. Default path: C:\\Program Files\\qemu\\qemu-system-x86_64.exe");
                ui.label("4. WHPX often fails — this uses software fallback (TCG)");
            }

            #[cfg(target_os = "linux")]
            {
                ui.label("2. Install QEMU/KVM: sudo apt install qemu-kvm");
            }
        });
    }

    // File Pickers
    fn pick_qemu_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("QEMU", &["exe"])
            .pick_file()
        {
            self.qemu_path = path;
        }
    }

    fn pick_iso_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("ISO", &["iso"])
            .pick_file()
        {
            self.kali_iso_path = path;
        }
    }

    // VM Launch
    fn launch_vm(&mut self) {
        if self.is_vm_running {
            self.last_error = Some("VM is already running".into());
            return;
        }

        let qemu_exe = if self.qemu_path.exists() {
            self.qemu_path.clone()
        } else {
            Self::default_qemu_path()
        };

        let mut cmd = Command::new(qemu_exe);
        cmd.args(&[
            "-m", &format!("{}", (self.ram_gb as u32) * 1024),
            "-smp", &format!("{}", self.cpu_cores),
            "-cdrom", self.kali_iso_path.to_str().unwrap(),
            "-boot", "d",
            "-vga", "virtio",
            "-net", "nic",
            "-net", "user",
            "-cpu", "qemu64",
            "-accel", "tcg",
            "-display", "sdl",
        ]);

        match cmd.spawn() {
            Ok(child) => {
                self.vm_process = Some(child);
                self.is_vm_running = true;
                self.last_error = None;
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to launch QEMU: {}", e));
                self.is_vm_running = false;
            }
        }

        println!("Executing: {:?}", cmd);
    }

    fn kill_vm(&mut self) {
        if let Some(mut child) = self.vm_process.take() {
            if let Err(e) = child.kill() {
                self.last_error = Some(format!("Failed to kill QEMU process: {}", e));
            } else {
                self.is_vm_running = false;
                self.last_error = None;
            }
        }
    }

    fn default_qemu_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(r"C:\Program Files\qemu\qemu-system-x86_64.exe")
        }
        #[cfg(target_os = "linux")]
        {
            PathBuf::from("qemu-system-x86_64")
        }
    }
}

impl Default for GhostMode {
    fn default() -> Self {
        Self {
            kali_iso_path: PathBuf::from(r"assets/kali-linux-2025.2-installer-amd64.iso"),
            ram_gb: 4,
            cpu_cores: 2,
            qemu_path: GhostMode::default_qemu_path(),
            is_vm_running: false,
            vm_process: None,
            last_error: None,
            show_animation: true,
            animation_timer: 0,
        }
    }
}
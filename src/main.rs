mod project;

use eframe::{CreationContext, App};
use project::Project;

struct Main {
    project: Option<Project>,
}

impl Main {
    pub fn new(_: &CreationContext) -> Self {
        Self {
            project: None,
        }
    }
}

impl App for Main {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Manifest");
        });
    }
}

fn main() {
    eframe::run_native("Manifest", eframe::NativeOptions::default(), Box::new(|cc| Box::new(Main::new(cc)))).unwrap();
}

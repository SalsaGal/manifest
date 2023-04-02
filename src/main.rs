mod project;

use eframe::{CreationContext, App};
use project::Project;

struct Main {
    project: Project,
}

impl Main {
    pub fn new(_: &CreationContext) -> Self {
        Self {
            project: Project::default(),
        }
    }
}

impl App for Main {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Manifest");
            if ui.button("New File").clicked() {
                self.project = Project::default();
            }

            macro_rules! text_field {
                ($($label: expr => $field: ident),*) => {
                    $(
                        ui.label($label);
                        ui.text_edit_singleline(&mut self.project.header.$field);
                    )*
                };
            }

            text_field!(
                "Name:" => name,
                "Genre:" => genre,
                "Level author:" => level_author,
                "Song author:" => song_author
            );
        });
    }
}

fn main() {
    eframe::run_native("Manifest", eframe::NativeOptions::default(), Box::new(|cc| Box::new(Main::new(cc)))).unwrap();
}

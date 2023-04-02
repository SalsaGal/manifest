mod project;

use std::num::NonZeroU16;

use eframe::{CreationContext, App};
use egui::DragValue;
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
                ($($label: expr => $field: ident),*$(,)?) => {
                    $(
                        ui.label($label);
                        ui.text_edit_singleline(&mut self.project.header.$field);
                    )*
                };
            }
            macro_rules! number_field {
                ($($label: expr => $field: ident: $type: ty),*$(,)?) => {
                    $(
                        ui.label($label);
                        let mut new_value = self.project.header.$field.get();
                        ui.add(DragValue::new(&mut new_value));
                        if let Some(value) = <$type>::new(new_value) {
                            self.project.header.$field = value;
                        }
                    )*
                };
            }

            text_field!(
                "Name:" => name,
                "Genre:" => genre,
                "Level author:" => level_author,
                "Song author:" => song_author,
            );

            number_field!("BPM:" => bpm: NonZeroU16);
            ui.checkbox(&mut self.project.manual_offset, "Manual offset");
            if self.project.manual_offset {
                number_field!("Offset:" => offset: NonZeroU16);
            }
        });
    }
}

fn main() {
    eframe::run_native("Manifest", eframe::NativeOptions::default(), Box::new(|cc| Box::new(Main::new(cc)))).unwrap();
}

mod options;
mod project;
mod shape;

use std::{fs::File, io::Write, num::NonZeroU16};

use eframe::{App, CreationContext};
use egui::{DragValue, ScrollArea};
use options::{Options, OptionsMenu};
use project::Project;
use rfd::FileDialog;
use shape::Shape;

struct Main {
    options: Options,
    options_menu: Option<OptionsMenu>,
    project: Project,
}

impl Main {
    pub fn new(ctx: &CreationContext) -> Self {
        let options = Options::load();
        ctx.egui_ctx.set_visuals(match options.dark_theme {
            true => egui::Visuals::dark(),
            false => egui::Visuals::light(),
        });

        Self {
            options,
            options_menu: None,
            project: Project {
                shapes: vec![
                    Shape {
                        pos: egui::Vec2::new(0.0, 0.0),
                        size: 0.0,
                        ty: shape::ShapeType::Triangle,
                        color: 0,
                    },
                    Shape {
                        pos: egui::Vec2::new(4.0, 3.0),
                        size: 1.0,
                        ty: shape::ShapeType::Square,
                        color: 2,
                    },
                    Shape {
                        pos: egui::Vec2::new(8.0, 7.0),
                        size: 2.0,
                        ty: shape::ShapeType::Circle,
                        color: 3,
                    },
                ],
                ..Default::default()
            },
        }
    }
}

impl App for Main {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Some(menu) = &mut self.options_menu {
            menu.ui(ctx);
            if menu.to_close {
                self.options_menu = None;
                self.options = Options::load();
            }
        } else {
            egui::SidePanel::left("control_panel").show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Manifest");
                    if ui.button("New File").clicked() {
                        self.project = Project::default();
                    }
                    if ui.button("Export").clicked() {
                        if let Some(mut path) =
                            FileDialog::new().add_filter("json", &["json"]).save_file()
                        {
                            if path.extension().is_none() {
                                path.set_extension("json");
                            }
                            let json = json::stringify_pretty(self.project.as_json(), 4);
                            let mut file = File::create(path).unwrap();
                            write!(file, "{json}").unwrap();
                        }
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
                    ui.checkbox(&mut self.project.header.manual_offset, "Manual offset");
                    if self.project.header.manual_offset {
                        number_field!("Offset:" => offset: NonZeroU16);
                    }

                    ui.collapsing("Color table", |ui| {
                        for i in 0..16 {
                            ui.label(format!("Color {i}"));
                            ui.color_edit_button_srgb(&mut self.project.header.color_table[i]);
                        }
                    });

                    if ui.button("Options").clicked() {
                        self.options_menu = Some(OptionsMenu::new(self.options.clone()));
                    }
                });
            });
            egui::SidePanel::right("shapes").show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {});
            });
            egui::TopBottomPanel::bottom("steps").show(ctx, |ui| {
                ScrollArea::horizontal().show(ui, |ui| ui.horizontal(|ui| {}))
            });
            egui::CentralPanel::default().show(ctx, |ui| self.project.draw(ui, None));
        }
    }
}

fn main() {
    eframe::run_native(
        "Manifest",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(Main::new(cc))),
    )
    .unwrap();
}

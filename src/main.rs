mod options;
mod project;
mod shape;

use std::{fs::File, io::Write, num::NonZeroU16};

use eframe::{App, CreationContext};
use egui::{DragValue, Key, ScrollArea, Vec2};
use options::{Options, OptionsMenu};
use project::Project;
use rfd::FileDialog;
use shape::Shape;

const BUILD_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct Main {
    options: Options,
    options_menu: Option<OptionsMenu>,
    project: Project,

    selected_shape: usize,
}

impl Main {
    pub fn new(ctx: &CreationContext) -> Self {
        let options = Options::load();
        ctx.egui_ctx.set_visuals(match options.dark_theme {
            true => egui::Visuals::dark(),
            false => egui::Visuals::light(),
        });

        Self::default()
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
                        if let Some(current_shape) =
                            self.project.shapes.get_mut(self.selected_shape)
                        {
                            for i in 0..16 {
                                ui.radio_value(&mut current_shape.color, i, format!("Color {i}"));
                                ui.color_edit_button_srgb(&mut self.project.header.color_table[i]);
                            }
                        }
                    });

                    if ui.button("Options").clicked() {
                        self.options_menu = Some(OptionsMenu::new(self.options.clone()));
                    }

                    ui.small(BUILD_VERSION);
                });
            });
            egui::SidePanel::right("shapes").show(ctx, |ui| {
                if ui.button("Remove shape").clicked() && !self.project.shapes.is_empty() {
                    self.project.shapes.remove(self.selected_shape);
                    self.selected_shape = self.selected_shape.saturating_sub(1);
                }
                ScrollArea::vertical().show(ui, |ui| {
                    let width = ui.available_size_before_wrap().x;
                    for i in 0..self.project.shapes.len() {
                        if self
                            .project
                            .draw(ui, Some(Vec2::splat(width)), i + 1)
                            .clicked()
                        {
                            self.selected_shape = i;
                        }
                    }
                });
            });
            egui::TopBottomPanel::bottom("steps").show(ctx, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    if let Some(shape) = self.project.shapes.get_mut(self.selected_shape) {
                        ui.horizontal(|ui| match &shape.moves {
                            Some(moves) => {}
                            None => {
                                if ui.button("Add Sequence").clicked() {
                                    shape.moves = Some(vec![]);
                                }
                            }
                        });
                    }
                })
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if ctx.memory(|mem| mem.focus().is_none()) {
                    ui.input(|input| {
                        if let Some(shape) = self.project.shapes.get_mut(self.selected_shape) {
                            if input.key_pressed(Key::ArrowUp) {
                                shape.pos.y = (shape.pos.y - 1.0).max(0.0);
                            }
                            if input.key_pressed(Key::ArrowDown) {
                                shape.pos.y = (shape.pos.y + 1.0).min(14.0);
                            }
                            if input.key_pressed(Key::ArrowLeft) {
                                shape.pos.x = (shape.pos.x - 1.0).max(0.0);
                            }
                            if input.key_pressed(Key::ArrowRight) {
                                shape.pos.x = (shape.pos.x + 1.0).min(14.0);
                            }
                            if input.key_pressed(Key::A) {
                                shape.size = (shape.size - 1.0).max(0.0);
                            }
                            if input.key_pressed(Key::S) {
                                shape.size = (shape.size + 1.0).min(7.0);
                            }
                            if input.key_pressed(Key::Enter) {
                                self.selected_shape =
                                    (self.selected_shape + 1) % self.project.shapes.len();
                            }
                        }
                        let to_add = if input.key_pressed(Key::Z) {
                            Some(Shape {
                                ty: shape::ShapeType::Circle,
                                color: self
                                    .project
                                    .shapes
                                    .get(self.selected_shape)
                                    .map(|shape| shape.color)
                                    .unwrap_or_default(),
                                ..Default::default()
                            })
                        } else if input.key_pressed(Key::X) {
                            Some(Shape {
                                ty: shape::ShapeType::Square,
                                color: self
                                    .project
                                    .shapes
                                    .get(self.selected_shape)
                                    .map(|shape| shape.color)
                                    .unwrap_or_default(),
                                ..Default::default()
                            })
                        } else if input.key_pressed(Key::C) {
                            Some(Shape {
                                ty: shape::ShapeType::Triangle,
                                color: self
                                    .project
                                    .shapes
                                    .get(self.selected_shape)
                                    .map(|shape| shape.color)
                                    .unwrap_or_default(),
                                ..Default::default()
                            })
                        } else {
                            None
                        };
                        if let Some(to_add) = to_add {
                            if self.project.shapes.len() <= self.selected_shape {
                                self.selected_shape = self.project.shapes.len();
                                self.project.shapes.push(to_add);
                            } else {
                                self.selected_shape += 1;
                                self.project.shapes.insert(self.selected_shape, to_add);
                            }
                        }
                    });
                }
                self.project.draw(ui, None, self.selected_shape + 1)
            });
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

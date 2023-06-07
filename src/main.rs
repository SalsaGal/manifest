mod options;
mod project;

use std::{fs::File, io::Write, num::NonZeroU16};

use eframe::{emath::RectTransform, epaint::RectShape, App, CreationContext};
use egui::{DragValue, Pos2, Rect, ScrollArea};
use glam::uvec2;
use options::{Options, OptionsMenu};
use project::{Project, Shape};
use rfd::FileDialog;

struct Main {
    options: Options,
    options_menu: Option<OptionsMenu>,
    project: Project,
}

impl Main {
    pub fn new(_: &CreationContext) -> Self {
        Self {
            options: Options::load(),
            options_menu: None,
            project: Project {
                shapes: vec![
                    Shape {
                        pos: egui::Vec2::new(0.0, 0.0),
                        size: 0.0,
                        ty: project::ShapeType::Triangle,
                    },
                    Shape {
                        pos: egui::Vec2::new(4.0, 3.0),
                        size: 1.0,
                        ty: project::ShapeType::Square,
                    },
                    Shape {
                        pos: egui::Vec2::new(8.0, 7.0),
                        size: 2.0,
                        ty: project::ShapeType::Circle,
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
            egui::TopBottomPanel::bottom("shapes").show(ctx, |ui| {
                ScrollArea::horizontal().show(ui, |ui| ui.horizontal(|ui| {}))
            });
            egui::SidePanel::left("control_panel").show(ctx, |ui| {
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

                if ui.button("Options").clicked() {
                    self.options_menu = Some(OptionsMenu::new(self.options.clone()));
                }
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                let (mut response, painter) = ui.allocate_painter(
                    ui.available_size_before_wrap(),
                    egui::Sense::click_and_drag(),
                );

                let to_screen = RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions() * 17.0),
                    response.rect,
                );

                response.mark_changed();

                let shapes = self
                    .project
                    .shapes
                    .iter()
                    .map(|shape| shape.as_egui_shape(to_screen));
                painter.extend(shapes);
                painter.extend((0..15 * 15).map(|i| uvec2(i % 15, i / 15)).map(|pos| {
                    egui::Shape::Rect(RectShape::stroke(
                        Rect::from_min_max(
                            to_screen * Pos2::new(pos.x as f32, pos.y as f32),
                            to_screen * Pos2::new((pos.x + 1) as f32, (pos.y + 1) as f32),
                        ),
                        egui::Rounding::none(),
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ))
                }));

                response
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

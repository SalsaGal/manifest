use std::{
    collections::HashMap,
    num::{NonZeroU16, NonZeroU8},
};

use eframe::{emath::RectTransform, epaint::RectShape};
use egui::{vec2, Pos2, Rect, Vec2};
use glam::uvec2;
use json::{object::Object, Array, JsonValue};

use crate::shape::{Shape, ShapeType};

#[derive(Debug, Default)]
pub struct Project {
    pub header: Header,
    pub shapes: Vec<Shape>,
}

impl Project {
    pub fn draw(
        &self,
        ui: &mut egui::Ui,
        bounds: Option<Vec2>,
        shape_count: usize,
    ) -> egui::Response {
        let (mut response, painter) = ui.allocate_painter(
            bounds.unwrap_or_else(|| ui.available_size_before_wrap()),
            egui::Sense::click_and_drag(),
        );

        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions() * 17.0),
            response.rect,
        );

        response.mark_changed();

        let shapes = self
            .shapes
            .iter()
            .take(shape_count)
            .flat_map(|shape| {
                let mut to_ret = vec![shape];
                to_ret.extend(shape.auto_shape.iter());
                to_ret
            })
            .map(|shape| shape.as_egui_shape(to_screen, &self.header.color_table));
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
    }

    pub fn from_json(json: JsonValue) -> Option<Self> {
        let header = &json[0].entries().collect::<HashMap<_, _>>();
        macro_rules! header_item {
            ($i: ident) => {
                header[stringify!($i)].to_string()
            };
        }

        let shapes = json
            .members()
            .skip(1)
            .map(Shape::from_json)
            .collect::<Vec<_>>();

        Some(Self {
            header: Header {
                name: header_item!(name),
                song_author: header_item!(song_author),
                level_author: header_item!(level_author),
                genre: header_item!(genre),
                bpm: NonZeroU16::new(header["bpm"].as_u16()?)?,
                bg_color: header["bg_color"].as_u8()?,
                background_effect: header["background_effect"].to_string(),
                ..Default::default()
            },
            shapes,
        })
    }

    pub fn as_json(&self) -> Array {
        let mut to_ret: Vec<JsonValue> = vec![self.header.as_json().into()];
        to_ret.extend(self.shapes.iter().map(Shape::as_json).map(JsonValue::from));
        to_ret
    }
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub genre: String,
    pub level_author: String,
    pub song_author: String,
    pub bpm: NonZeroU16,
    pub offset: NonZeroU16,
    // This is here so that the offet can be stored even if its temporarily deactivated
    pub manual_offset: bool,
    pub time_signature_top: NonZeroU8,
    pub time_signature_bottom: NonZeroU8,
    pub bg_color: u8,
    pub background_effect: String,
    pub color_table: [[u8; 3]; 16],
}

impl Header {
    fn as_json(&self) -> Object {
        // TODO Could be easier to just store as an Object to begin with
        // so no cloning is necessary when creating the object here
        let mut to_ret = Object::with_capacity(11);
        macro_rules! add_str {
            {$($i: ident),*} => {
                $(
                    to_ret.insert(stringify!($i), JsonValue::String(self.$i.clone()));
                )*
            };
        }
        macro_rules! add_num {
            {$($i: ident),*} => {
                $(
                    to_ret.insert(stringify!($i), JsonValue::Number(self.$i.get().into()));
                )*
            };
        }
        add_str!(name, genre, level_author, song_author, background_effect);
        add_num!(bpm, offset, time_signature_top, time_signature_bottom);
        to_ret.insert("bg_color", self.bg_color.into());
        to_ret.insert(
            "color_table",
            self.color_table
                .iter()
                .map(|color| format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2]))
                .collect::<Vec<_>>()
                .into(),
        );
        to_ret
    }
}

#[cfg(test)]
#[test]
fn header_as_json() {
    let header = Header {
        name: "Test".to_owned(),
        genre: "Maybe".to_owned(),
        ..Default::default()
    };

    assert_eq!(header.as_json().pretty(4), "{\n    \"name\": \"Test\",\n    \"genre\": \"Maybe\",\n    \"level_author\": \"Anonymous\",\n    \"song_author\": \"Anonymous\",\n    \"background_effect\": \"none\",\n    \"bpm\": 120,\n    \"offset\": 32,\n    \"time_signature_top\": 4,\n    \"time_signature_bottom\": 4,\n    \"bg_color\": 15,\n    \"color_table\": [\n        \"#FFFFFF\",\n        \"#0000FF\",\n        \"#00FF00\",\n        \"#00FFFF\",\n        \"#FF0000\",\n        \"#FF00FF\",\n        \"#FF6600\",\n        \"#AAAAAA\",\n        \"#666666\",\n        \"#6666FF\",\n        \"#66FF66\",\n        \"#66FFFF\",\n        \"#FF6666\",\n        \"#FF66FF\",\n        \"#FFFF22\",\n        \"#000000\"\n    ]\n}");
}

impl Default for Header {
    fn default() -> Self {
        let time_signature_top = NonZeroU8::new(4).unwrap();
        let time_signature_bottom = NonZeroU8::new(4).unwrap();

        Self {
            name: "Untitled".to_owned(),
            genre: "Unknown".to_owned(),
            level_author: "Anonymous".to_owned(),
            song_author: "Anonymous".to_owned(),
            bpm: NonZeroU16::new(120).unwrap(),
            offset: NonZeroU16::new(
                u16::from(time_signature_top.get()) * u16::from(time_signature_bottom.get()) * 2,
            )
            .unwrap(),
            manual_offset: false,
            time_signature_top,
            time_signature_bottom,
            bg_color: 15,
            background_effect: "none".to_owned(),
            color_table: [
                [0xFF, 0xFF, 0xFF],
                [0x00, 0x00, 0xFF],
                [0x00, 0xFF, 0x00],
                [0x00, 0xFF, 0xFF],
                [0xFF, 0x00, 0x00],
                [0xFF, 0x00, 0xFF],
                [0xFF, 0x66, 0x00],
                [0xAA, 0xAA, 0xAA],
                [0x66, 0x66, 0x66],
                [0x66, 0x66, 0xFF],
                [0x66, 0xFF, 0x66],
                [0x66, 0xFF, 0xFF],
                [0xFF, 0x66, 0x66],
                [0xFF, 0x66, 0xFF],
                [0xFF, 0xFF, 0x22],
                [0x00, 0x00, 0x00],
            ],
        }
    }
}

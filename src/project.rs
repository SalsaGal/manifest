use std::num::{NonZeroU16, NonZeroU8};

use eframe::{emath::RectTransform, epaint::CircleShape};
use egui::{Color32, Pos2, Vec2};
use glam::UVec2;
use json::{object::Object, Array, JsonValue};

#[derive(Debug, Default)]
pub struct Project {
    pub header: Header,
    pub shapes: Vec<Shape>,
    // TODO Should be an Option on Header::offset
    pub manual_offset: bool,
}

impl Project {
    pub fn as_json(&self) -> Array {
        vec![self.header.as_json().into()]
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
    pub time_signature_top: NonZeroU8,
    pub time_signature_bottom: NonZeroU8,
    pub bg_color: u8,
    pub background_effect: String,
    pub color_table: [String; 16],
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

    assert_eq!(header.as_json().pretty(4), "{\n    \"name\": \"Test\",\n    \"genre\": \"Maybe\",\n    \"level_author\": \"Anonymous\",\n    \"song_author\": \"Anonymous\",\n    \"background_effect\": \"none\",\n    \"bpm\": 120,\n    \"offset\": 32,\n    \"time_signature_top\": 4,\n    \"time_signature_bottom\": 4,\n    \"bg_color\": 15\n}");
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
                time_signature_top.get() as u16 * time_signature_bottom.get() as u16 * 2,
            )
            .unwrap(),
            time_signature_top,
            time_signature_bottom,
            bg_color: 15,
            background_effect: "none".to_owned(),
            color_table: [
                "#FFF".to_owned(),
                "#00F".to_owned(),
                "#0F0".to_owned(),
                "#0FF".to_owned(),
                "#F00".to_owned(),
                "#F0F".to_owned(),
                "#F60".to_owned(),
                "#AAA".to_owned(),
                "#666".to_owned(),
                "#66F".to_owned(),
                "#6F6".to_owned(),
                "#6FF".to_owned(),
                "#F66".to_owned(),
                "#F6F".to_owned(),
                "#FF2".to_owned(),
                "#000".to_owned(),
            ],
        }
    }
}

#[derive(Debug)]
pub struct Shape {
    pub pos: Vec2,
    pub size: f32,
    pub ty: ShapeType,
}

impl Shape {
    pub fn as_egui_shape(&self, transform: RectTransform) -> egui::Shape {
        match self.ty {
            ShapeType::Circle => egui::Shape::Circle(CircleShape::filled(
                transform * (Pos2::new(0.5, 0.5) + self.pos),
                transform.scale().max_elem() * (self.size + 0.5),
                Color32::RED,
            )),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum ShapeType {
    Circle,
    Square,
    Triangle,
}

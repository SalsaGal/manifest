use std::num::{NonZeroU16, NonZeroU8};

use json::{object::Object, Array, JsonValue};

use crate::shape::Shape;

#[derive(Debug, Default)]
pub struct Project {
    pub header: Header,
    pub shapes: Vec<Shape>,
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

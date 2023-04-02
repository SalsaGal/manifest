use std::num::{NonZeroU16, NonZeroU8};

pub struct Project {
    pub header: Header,
}

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
            offset: NonZeroU16::new(time_signature_top.get() as u16 * time_signature_bottom.get() as u16 * 2).unwrap(),
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

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

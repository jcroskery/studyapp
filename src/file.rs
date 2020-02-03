use crate::data::Data;

use std::path::PathBuf;
use std::fs::File;
use std::fs;

use gdk::Screen;
use gdk_pixbuf::Pixbuf;
use gio::{Cancellable, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;
use gtk::{Builder, CssProvider, Image, StyleContext};
use serde_json::Value;

pub fn read_file(data: Data, file: PathBuf) -> Data {
    let value: Value = serde_json::from_reader(File::create(file).unwrap()).unwrap();
    data
}
#[derive(Clone)]
pub struct Images {
    pub x: Pixbuf,
    pub check: Pixbuf,
    pub not_completed: Pixbuf,
}
impl Images {
    pub fn new() -> Self {
        Images {
            x: Self::get_pixbuf(include_bytes!("../x.png")),
            check: Self::get_pixbuf(include_bytes!("../check.png")),
            not_completed: Self::get_pixbuf(include_bytes!("../not_completed.png"))
        }
    }
    pub fn to_image(pixbuf: Pixbuf) -> Image {
        Image::new_from_pixbuf(Some(&pixbuf))
    }
    fn get_pixbuf(bytes: &'static [u8]) -> Pixbuf {
        Pixbuf::new_from_stream::<_, Cancellable>(
            &MemoryInputStream::new_from_bytes(&Bytes::from_static(bytes)),
            None,
        ).unwrap()
    }
}
pub fn get_builder() -> Builder {
    let css = CssProvider::new();
    css.load_from_data(&include_str!("../study.css").as_bytes())
        .unwrap_or_default();
    StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
    Builder::new_from_string(include_str!("../window.ui"))
}

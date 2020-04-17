use crate::data::Data;
use crate::data::Row;

use gdk::Screen;
use gdk_pixbuf::Pixbuf;
use gio::{Cancellable, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;
use gtk::{Builder, CssProvider, Image, StyleContext};
use serde_json::Value;
use rand::prelude::*;

pub fn read_file(data: &mut Data, images: &Images) {
    let value: Value = serde_json::from_str(include_str!("../music.json")).unwrap();
    let mut i = 0;
    let mut keys: Vec<&String> = value.as_object().unwrap().keys().collect();
    keys.shuffle(&mut rand::thread_rng());
    for key in keys {
        let definition: String = serde_json::from_value(value[key].clone()).unwrap();
        data.add(Row::new(
            key.to_string(),
            None,
            definition,
            i,
            images.clone(),
        ));
        i += 1;
    }
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
            not_completed: Self::get_pixbuf(include_bytes!("../not_completed.png")),
        }
    }
    pub fn to_image(pixbuf: Pixbuf) -> Image {
        Image::new_from_pixbuf(Some(&pixbuf))
    }
    fn get_pixbuf(bytes: &'static [u8]) -> Pixbuf {
        Pixbuf::new_from_stream::<_, Cancellable>(
            &MemoryInputStream::new_from_bytes(&Bytes::from_static(bytes)),
            None,
        )
        .unwrap()
    }
}
pub fn get_builder() -> Builder {
    let css = CssProvider::new();
    css.load_from_data(&include_str!("../study.css").as_bytes())
        .unwrap_or_default();
    StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
    Builder::new_from_string(include_str!("../window.ui"))
}

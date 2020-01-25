use crate::data::Data;

use std::path::PathBuf;

use gdk::Screen;
use gdk_pixbuf::Pixbuf;
use gio::{Cancellable, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;
use gtk::{Builder, CssProvider, Image, StyleContext};

pub fn read_file(data: Data, file: PathBuf) -> Data {
    data
}
pub struct Resources {
    pub check: Option<Pixbuf>,
    pub x: Option<Pixbuf>,
    pub not_completed: Option<Pixbuf>,
    pub builder: Builder,
}
impl Resources {
    pub fn get_image(pixbuf: Option<Pixbuf>) -> Image {
        if let Some(pixbuf) = pixbuf {
            Image::new_from_pixbuf(Some(&pixbuf))
        } else {
            Image::new()
        }
    }
    fn get_pixbuf(image: &'static [u8]) -> Option<Pixbuf> {
        Pixbuf::new_from_stream::<_, Cancellable>(
            &MemoryInputStream::new_from_bytes(&Bytes::from_static(image)),
            None,
        )
        .ok()
    }
    pub fn new() -> Self {
        let css = CssProvider::new();
        css.load_from_data(&include_str!("../study.css").as_bytes())
            .unwrap_or_default();
        StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
        let builder = Builder::new_from_string(include_str!("../window.ui"));
        Resources {
            check: Self::get_pixbuf(include_bytes!("../check.png")),
            x: Self::get_pixbuf(include_bytes!("../x.png")),
            not_completed: Self::get_pixbuf(include_bytes!("../not_completed.png")),
            builder,
        }
    }
}

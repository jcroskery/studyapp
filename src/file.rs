use crate::data::Data;

use std::path::PathBuf;

use gdk::Screen;
use gio::{Resource, ResourceLookupFlags, Cancellable};
use gtk::prelude::*;
use gtk::{CssProvider, Image, StyleContext};
use gdk_pixbuf::Pixbuf;

pub fn read_file(data: Data, file: PathBuf) -> Data {
    data
}
pub struct Resources {
    pub check: Option<Pixbuf>,
    pub x: Option<Pixbuf>,
    pub not_completed: Option<Pixbuf>,
}
impl Resources {
    pub fn get_image(pixbuf: Option<Pixbuf>) -> Image {
        if let Some(pixbuf) = pixbuf {
            Image::new_from_pixbuf(Some(&pixbuf))
        } else {
            Image::new()
        }
    }
    fn get_pixbuf(gresource: Resource, image_path: &str) -> Option<Pixbuf> {
        if let Ok(input) = gresource.open_stream(image_path, ResourceLookupFlags::empty()) {
            if let Ok(pixbuf) = Pixbuf::new_from_stream::<_, Cancellable>(&input, None) {
                return Some(pixbuf);
            }
        }
        None
    }
    pub fn new(path: &str) -> Self {
        if let Ok(gresource) = Resource::load(path) {
            if let Ok(bytes) =
                gresource.lookup_data("/tk/olmmcc/rusic/rusic.css", ResourceLookupFlags::empty())
            {
                let css = CssProvider::new();
                css.load_from_data(&bytes).unwrap_or_default();
                StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
            }
            Resources {
                check: Self::get_pixbuf(gresource.clone(), "/tk/olmmcc/study/check.png"),
                x: Self::get_pixbuf(gresource.clone(), "/tk/olmmcc/study/x.png"),
                not_completed: Self::get_pixbuf(gresource.clone(), "/tk/olmmcc/study/not_completed.png"),
            }
        } else {
            Resources {
                check: None,
                x: None,
                not_completed: None,
            }
        }
    }
}

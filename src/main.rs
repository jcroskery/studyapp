mod data;
mod file;

use gio::prelude::ApplicationExtManual;
use gio::{ApplicationExt, ApplicationFlags};
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Adjustment, Application, ApplicationWindow, Entry, Label, ListBox, PolicyType, ScrolledWindow,
    Separator, Window,
};

use std::env;

use data::{Data, Row};
use file::Images;

struct App {}

impl App {
    pub fn new(application: &Application) -> Self {
        let images = Images::new();
        let window = ApplicationWindow::new(application);
        window.set_title("Study");
        let builder = file::get_builder();
        let window: Window = builder.get_object("mainWindow").unwrap();
        let mut data = Data::new(builder.clone());
        file::read_file(&mut data, &images, "music.json");
        data.display_selected();
        window.show_all();
        App {}
    }
}

fn main() {
    let application = Application::new(Some("tk.olmmcc.study"), ApplicationFlags::empty())
        .expect("Application initialization failed!");
    application.connect_startup(|application| {
        App::new(application);
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}

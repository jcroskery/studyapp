mod data;
mod file;

use gio::prelude::ApplicationExtManual;
use gio::{ApplicationExt};
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Adjustment, Application, ApplicationWindow, Entry, Label, ListBox, PolicyType, ScrolledWindow,
    Separator, Window,
};

use std::env;

use data::{Data, Row};
use file::Images;

fn main() {
    let application = Application::new(Some("tk.olmmcc.study"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application initialization failed!");
    application.connect_activate(|application| {
        let images = Images::new();
        let window = ApplicationWindow::new(application);
        window.set_title("Study");
        window.set_default_size(500, 500);
        let builder = file::get_builder();
        let window: Window = builder.get_object("mainWindow").unwrap();
        let mut data = Data::new(builder.clone());
        file::read_file(&mut data, &images);
        data.display_selected();
        window.show_all();
    });
    application.run(&env::args().collect::<Vec<_>>());
}

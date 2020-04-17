mod data;
mod file;

use gio::prelude::ApplicationExtManual;
use gio::{ApplicationExt};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Window};

use std::env;

use data::Data;
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
        data.connect_display_selected();
        data.connect_enter_keypress();
        data.connect_refresh();
        data.connect_submit_answer();
        window.show_all();
    });
    application.run(&env::args().collect::<Vec<_>>());
}

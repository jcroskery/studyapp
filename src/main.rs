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
use file::Resources;

struct App {}

impl App {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Study");
        let resources = Resources::new();
        let window: Window = resources.builder.get_object("mainWindow").unwrap();
        let mut data = Data::new(resources.builder.get_object("listbox").unwrap());
        for i in 1..100 {
            data.add(Row::new(
                "doom".to_string(),
                None,
                "A bad thing".to_string(),
                Resources::get_image(resources.not_completed.clone()),
            ), i);
        }
        data.display_selected(resources.builder);
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

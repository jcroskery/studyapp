mod data;
mod file;

use gio::prelude::ApplicationExtManual;
use gio::{ApplicationExt, ApplicationFlags};
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{Adjustment, Application, ApplicationWindow, Image, ListBox, PolicyType, ScrolledWindow};

use std::env;

use data::{Data, Row};
use file::Resources;

struct App {}

impl App {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Study");
        let resources = Resources::new("study.gresource");
        let hbox = gtk::Box::new(Horizontal, 0);
        window.add(&hbox);
        let scroll = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        scroll.set_property_hscrollbar_policy(PolicyType::Never);
        let list = ListBox::new();
        scroll.add(&list);
        hbox.add(&scroll);
        let mut data = Data::new(list);
        for _ in 1..100 {
            data.add(Row::new(
                "doom".to_string(),
                None,
                "A bad thing".to_string(),
                Resources::get_image(resources.check.clone()),
            ));
        }
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

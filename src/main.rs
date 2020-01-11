mod data;
mod file;

use gio::prelude::ApplicationExtManual;
use gio::{ApplicationExt, ApplicationFlags};
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Adjustment, Application, ApplicationWindow, ListBox, PolicyType, ScrolledWindow, Label, Separator, Entry};

use std::env;

use data::{Data, Row};
use file::Resources;

struct App {}

impl App {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Study");
        let resources = Resources::new(application);
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
                Resources::get_image(resources.not_completed.clone()),
            ));
        }
        let hseparator = Separator::new(Vertical);
        hseparator.set_vexpand(true);
        hbox.add(&hseparator);
        let vbox = gtk::Box::new(Vertical, 0);
        hbox.add(&vbox);
        let question = Label::new(Some("The term is forte. What is its meaning?"));
        let entry = Entry::new();
        vbox.add(&question);
        vbox.add(&entry);
        let vseparator = Separator::new(Horizontal);
        vseparator.set_hexpand(true);
        vbox.add(&vseparator);
        let term = Label::new(Some("Term: Forte"));
        let user_definition = Label::new(Some("Your Definition: Quiet"));
        let definition = Label::new(Some("Correct definition: Loud"));
        vbox.add(&term);
        vbox.add(&user_definition);
        vbox.add(&definition);
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

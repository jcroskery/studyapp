use std::collections::HashMap;

use glib::object::Cast;
use glib::GString;
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{Builder, Image, Label, ListBox, ListBoxRow, Widget};

pub struct Data {
    rows: HashMap<i32, Row>,
    scroll: ListBox,
}
#[derive(Clone)]
pub struct Row {
    term: String,
    user_definition: Option<String>,
    definition: String,
    image: Image,
}
impl Row {
    pub fn new(
        term: String,
        user_definition: Option<String>,
        definition: String,
        image: Image,
    ) -> Self {
        Row {
            term,
            user_definition,
            definition,
            image,
        }
    }
    fn get_term(&self) -> String {
        self.term.clone()
    }
    fn get_definition(&self) -> String {
        self.definition.clone()
    }
    fn get_image(&self) -> Image {
        self.image.clone()
    }
}
impl Data {
    pub fn new(scroll: ListBox) -> Self {
        Data {
            rows: HashMap::new(),
            scroll,
        }
    }
    pub fn add(&mut self, row: Row, id: i32) {
        let box_row = ListBoxRow::new();
        let hbox = gtk::Box::new(Horizontal, 10);
        hbox.set_widget_name(&id.to_string());
        let term_label = Label::new(Some(&row.get_term()));
        let colon = Label::new(Some(":"));
        let definition_label = Label::new(Some(&row.get_definition()));
        hbox.add(&term_label);
        hbox.add(&colon);
        hbox.add(&definition_label);
        hbox.add(&row.get_image());
        box_row.add(&hbox);
        self.scroll.add(&box_row);
        self.rows.insert(id, row);
    }
    pub fn display_selected(&self, builder: Builder) {
        let list: ListBox = builder.get_object("listbox").unwrap();
        let term_label: Label = builder.get_object("term").unwrap();
        let rows = self.rows.clone();
        list.connect_row_selected(move |_, row| {
            let gtk_box = row.unwrap().get_children()[0].clone();
            let id = gtk_box.get_widget_name().unwrap_or(GString::from("")).to_string().parse().unwrap();
            let row = rows.get(&id).unwrap();
            term_label.set_text(&row.term);
        });
    }
}

use std::cell::RefCell;

use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{IconSize, Image, Label, ListBox, ListBoxRow};

pub struct Data {
    rows: Vec<Row>,
    selected_row: RefCell<Option<Row>>,
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
    pub fn new(term: String, user_definition: Option<String>, definition: String, image: Image) -> Self {
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
            rows: vec![],
            selected_row: RefCell::new(None),
            scroll,
        }
    }
    pub fn add(&mut self, row: Row) {
        let box_row = ListBoxRow::new();
        let hbox = gtk::Box::new(Horizontal, 10);
        let label = Label::new(Some(&format!(
            "{}: {}",
            row.get_term(),
            row.get_definition()
        )));
        hbox.add(&label);
        hbox.add(&row.get_image());
        box_row.add(&hbox);
        self.scroll.add(&box_row);
        self.rows.push(row);
    }
}

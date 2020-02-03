use std::cell::RefCell;
use std::collections::HashMap;

use glib::object::Cast;
use glib::GString;
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{Builder, Image, Label, ListBox, ListBoxRow, Widget};

use crate::file::Images;

#[derive(Clone, Debug)]
enum State {
    UNANSWERED,
    WRONG,
    CORRECT,
}
pub struct Data {
    rows: HashMap<i32, RefCell<Row>>,
    scroll: ListBox,
    builder: Builder,
}
#[derive(Clone)]
pub struct Row {
    term: String,
    user_definition: Option<String>,
    definition: String,
    box_row: ListBoxRow,
    id: i32,
    state: State,
    images: Images,
}
impl State {
    fn get_image(&self, images: Images) -> Image {
        match self {
            Self::UNANSWERED => {
                Images::to_image(images.not_completed)
            }
            Self::WRONG => {
                Images::to_image(images.x)
            }
            Self::CORRECT => {
                Images::to_image(images.check)
            }
        }
    }
}
impl Row {
    pub fn new(
        term: String,
        user_definition: Option<String>,
        definition: String,
        id: i32,
        images: Images,
    ) -> Self {
        let box_row = ListBoxRow::new();
        let hbox = gtk::Box::new(Horizontal, 10);
        hbox.set_widget_name(&id.to_string());
        let term_label = Label::new(Some(&term));
        let colon = Label::new(Some(":"));
        let definition_label = Label::new(Some(&definition));
        let state = State::UNANSWERED;
        let image = state.get_image(images.clone());
        hbox.add(&term_label);
        hbox.add(&colon);
        hbox.add(&definition_label);
        hbox.add(&image);
        box_row.add(&hbox);
        Row {
            term,
            user_definition,
            definition,
            state,
            id,
            box_row,
            images,
        }
    }
    fn set_correct(&mut self) {
        self.state = State::CORRECT;
        let gtk_row: gtk::Box = Cast::downcast(self.box_row.get_children()[0].clone()).unwrap();
        gtk_row.remove(gtk_row.get_children().last().unwrap());
        let image = &self.state.get_image(self.images.clone());
        gtk_row.add(image);
        image.show();
    }
    fn set_incorrect(&mut self) {
        self.state = State::WRONG;
        let gtk_row: gtk::Box = Cast::downcast(self.box_row.get_children()[0].clone()).unwrap();
        gtk_row.remove(gtk_row.get_children().last().unwrap());
        let image = &self.state.get_image(self.images.clone());
        gtk_row.add(image);
        image.show();
    }
    fn get_term(&self) -> String {
        self.term.clone()
    }
    fn get_definition(&self) -> String {
        self.definition.clone()
    }
}
impl Data {
    pub fn new(builder: Builder) -> Self {
        Data {
            rows: HashMap::new(),
            scroll: builder.get_object("listbox").unwrap(),
            builder,
        }
    }
    pub fn add(&mut self, row: Row) {
        self.scroll.add(&row.box_row);
        self.rows.insert(row.id, RefCell::new(row));
    }
    pub fn display_selected(&self) {
        let list: ListBox = self.builder.get_object("listbox").unwrap();
        let term_label: Label = self.builder.get_object("term").unwrap();
        let your_definition_label: Label = self.builder.get_object("your_definition").unwrap();
        let correct_definition_label: Label = self.builder.get_object("correct_definition").unwrap();
        let definition_label: Label = self.builder.get_object("definition").unwrap();

        let your_box: gtk::Box = self.builder.get_object("your_box").unwrap();
        let correct_box: gtk::Box = self.builder.get_object("correct_box").unwrap();
        let definition_box: gtk::Box = self.builder.get_object("definition_box").unwrap();
        let rows = self.rows.clone();
        list.connect_row_selected(move |_, row| {
            let gtk_box = row.unwrap().get_children()[0].clone();
            let id = gtk_box
                .get_widget_name()
                .unwrap_or(GString::from(""))
                .to_string()
                .parse()
                .unwrap();
            let mut row = rows.get(&id).unwrap().clone().into_inner();
            term_label.set_text(&row.term);
            your_definition_label.set_text(&row.user_definition.clone().unwrap_or_default());
            correct_definition_label.set_text(&row.definition);
            definition_label.set_text(&row.definition);
            match &row.state {
                State::UNANSWERED => {
                    your_box.hide();
                    correct_box.hide();
                    definition_box.hide();
                }
                State::WRONG => {
                    your_box.show();
                    correct_box.show();
                    definition_box.hide();
                }
                State::CORRECT => {
                    your_box.hide();
                    correct_box.hide();
                    definition_box.show();
                }
            }
            row.set_incorrect();
        });
    }
}

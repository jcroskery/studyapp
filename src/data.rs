use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glib::object::Cast;
use glib::GString;
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{Builder, Image, Label, ListBox, ListBoxRow, EntryBuffer};
use rand::prelude::*;

use crate::file::Images;

#[derive(Clone, Debug, PartialEq)]
enum State {
    UNANSWERED,
    WRONG,
    CORRECT,
}
pub struct Data {
    rows: Rc<RefCell<HashMap<i32, Row>>>,
    scroll: ListBox,
    current_row: Rc<RefCell<i32>>,
    builder: Builder,
    correct: Rc<RefCell<i32>>,
    incorrect: Rc<RefCell<i32>>,
    unanswered: Rc<RefCell<i32>>,
}
#[derive(Clone, Debug)]
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
            Self::UNANSWERED => Images::to_image(images.not_completed),
            Self::WRONG => Images::to_image(images.x),
            Self::CORRECT => Images::to_image(images.check),
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
        let definition_label = Label::new(None);
        definition_label.set_max_width_chars(50);
        definition_label.set_line_wrap(true);
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
        let definition_label: Label =
            Cast::downcast(gtk_row.get_children().last().unwrap().clone()).unwrap();
        definition_label.set_text(&self.definition);
        let image = &self.state.get_image(self.images.clone());
        gtk_row.add(image);
        image.show();
    }
    fn set_unanswered(&mut self) {
        self.state = State::UNANSWERED;
        let gtk_row: gtk::Box = Cast::downcast(self.box_row.get_children()[0].clone()).unwrap();
        gtk_row.remove(gtk_row.get_children().last().unwrap());
        let definition_label: Label =
            Cast::downcast(gtk_row.get_children().last().unwrap().clone()).unwrap();
        definition_label.set_text("");
        let image = &self.state.get_image(self.images.clone());
        gtk_row.add(image);
        image.show();
    }
    fn set_incorrect(&mut self) {
        self.state = State::WRONG;
        let gtk_row: gtk::Box = Cast::downcast(self.box_row.get_children()[0].clone()).unwrap();
        gtk_row.remove(gtk_row.get_children().last().unwrap());
        let definition_label: Label =
            Cast::downcast(gtk_row.get_children().last().unwrap().clone()).unwrap();
        definition_label.set_text(&self.definition);
        let image = &self.state.get_image(self.images.clone());
        gtk_row.add(image);
        image.show();
    }
}
impl Data {
    pub fn new(builder: Builder) -> Self {
        Data {
            rows: Rc::new(RefCell::new(HashMap::new())),
            scroll: builder.get_object("listbox").unwrap(),
            current_row: Rc::new(RefCell::new(0)),
            builder,
            correct: Rc::new(RefCell::new(0)), 
            incorrect: Rc::new(RefCell::new(0)), 
            unanswered: Rc::new(RefCell::new(0)),
        }
    }
    pub fn add(&mut self, row: Row) {
        self.scroll.add(&row.box_row);
        self.rows.borrow_mut().insert(row.id, row);
        *self.unanswered.borrow_mut() += 1;
        self.builder.get_object::<Label>("unanswered").unwrap().set_text(&format!("Unanswered: {}", self.unanswered.borrow()));
    }
    pub fn connect_submit_answer(&self) {
        let answer_entry: gtk::Entry = self.builder.get_object("answer").unwrap();
        let enter: gtk::Button = self.builder.get_object("enter").unwrap();
        let current_row = self.current_row.clone();
        let rows = self.rows.clone();
        let your_definition_label: Label = self.builder.get_object("your_definition").unwrap();
        let your_box: gtk::Box = self.builder.get_object("your_box").unwrap();
        let correct_box: gtk::Box = self.builder.get_object("correct_box").unwrap();
        let definition_box: gtk::Box = self.builder.get_object("definition_box").unwrap();
        let list: ListBox = self.builder.get_object("listbox").unwrap();
        let questions_box: gtk::Box = self.builder.get_object("questions_box").unwrap();
        let congrats: Label = self.builder.get_object("congrats").unwrap();
        let unanswered_label: Label = self.builder.get_object("unanswered").unwrap();
        let correct_label: Label = self.builder.get_object("correct").unwrap();
        let incorrect_label: Label = self.builder.get_object("incorrect").unwrap();
        let incorrect = self.incorrect.clone();
        let unanswered = self.unanswered.clone();
        let correct =  self.correct.clone();
        enter.connect_clicked(move |_| {
            let id;
            {
                id = *current_row.borrow();
                let mut hash_map = rows.borrow_mut();
                let mut row = hash_map.get_mut(&id).unwrap();
                if row.state == State::UNANSWERED {
                    let user_definition = format!("{}", answer_entry.get_text().unwrap());
                    set(&unanswered, Some(-1), None, &unanswered_label, "Unanswered");
                    if is_answer_correct(&user_definition, &row.definition) {
                        row.set_correct();
                        your_box.hide();
                        correct_box.hide();
                        definition_box.show();
                        set(&correct, Some(1), None, &correct_label, "Correct");
                    } else {
                        row.user_definition = Some(user_definition.clone());
                        row.set_incorrect();
                        your_definition_label.set_text(&user_definition);
                        your_box.show();
                        correct_box.show();
                        definition_box.hide();
                        set(&incorrect, Some(1), None, &incorrect_label, "Incorrect");
                    }
                    if is_complete(&hash_map) {
                        questions_box.hide();
                        unanswered_label.hide();
                        congrats.set_text("Congratulations! To restart,\nclick the refresh button in the upper right.");
                    }
                } else {
                    return;
                }
            }
            let children = list.get_children();
            if (id + 1) < children.len() as i32 {
                let listboxrow: gtk::ListBoxRow =
                    Cast::downcast(children[(id + 1) as usize].clone()).unwrap();
                list.select_row(Some(&listboxrow));
            }
            answer_entry.set_buffer(&EntryBuffer::new(None));
        });
    }
    pub fn connect_enter_keypress(&self) {
        let answer_entry: gtk::Entry = self.builder.get_object("answer").unwrap();
        let enter: gtk::Button = self.builder.get_object("enter").unwrap();
        answer_entry.connect_activate(move |_| {
            enter.emit_clicked();
        });
    }
    pub fn connect_refresh(&mut self) {
        let unanswered_label: Label = self.builder.get_object("unanswered").unwrap();
        let correct_label: Label = self.builder.get_object("correct").unwrap();
        let incorrect_label: Label = self.builder.get_object("incorrect").unwrap();
        let incorrect = self.incorrect.clone();
        let unanswered = self.unanswered.clone();
        let correct =  self.correct.clone();
        let refresh_button: gtk::Button = self.builder.get_object("refresh").unwrap();
        let rows = self.rows.clone();
        let questions_box: gtk::Box = self.builder.get_object("questions_box").unwrap();
        let congrats: Label = self.builder.get_object("congrats").unwrap();
        let list: ListBox = self.builder.get_object("listbox").unwrap();
        let hash_map = self.rows.clone();
        refresh_button.connect_clicked(move |_| {
            let total;
            {
                total = *correct.borrow() + *incorrect.borrow() + *unanswered.borrow();
            }
            set(&unanswered, None, Some(total), &unanswered_label, "Unanswered");
            set(&incorrect, None, Some(0), &incorrect_label, "Incorrect");
            set(&correct, None, Some(0), &correct_label, "Correct");
            rows.borrow_mut().iter_mut().for_each(|row| row.1.set_unanswered());
            questions_box.show();
            unanswered_label.show();
            congrats.set_text("");
            hash_map.borrow_mut().iter_mut().map(|x| x.1).collect::<Vec<&mut Row>>().shuffle(&mut rand::thread_rng());
            let mut i = -1;
            let mut new_hash_map: HashMap<i32, Row> = hash_map.borrow().iter().map(|x| {i += 1; (i, x.1.clone())}).collect();
            println!("{:?}", new_hash_map.get(&0));
            for i in 0..total {
                let row = &new_hash_map.get(&i).unwrap().box_row;
                list.remove(row);
                let row_contents: &gtk::Container = &Cast::downcast(row.get_children()[0].clone()).unwrap();
                row.remove(row_contents);
                let list_box_row = ListBoxRow::new();
                list_box_row.add(row_contents);
                new_hash_map.get_mut(&i).unwrap().box_row = list_box_row;
            }
            for i in 0..total {
                list.add(&new_hash_map.get(&i).unwrap().box_row);
                new_hash_map.get(&i).unwrap().box_row.show_all();
            }
            *hash_map.borrow_mut() = new_hash_map;
            list.select_row::<ListBoxRow>(None);
            list.select_row(Some(&list.get_row_at_index(0).unwrap()));
        });
    }
    pub fn connect_display_selected(&self) {
        let list: ListBox = self.builder.get_object("listbox").unwrap();
        let term_label: Label = self.builder.get_object("term").unwrap();
        let question: Label = self.builder.get_object("question").unwrap();
        let your_definition_label: Label = self.builder.get_object("your_definition").unwrap();
        let correct_definition_label: Label =
            self.builder.get_object("correct_definition").unwrap();
        let definition_label: Label = self.builder.get_object("definition").unwrap();

        let your_box: gtk::Box = self.builder.get_object("your_box").unwrap();
        let correct_box: gtk::Box = self.builder.get_object("correct_box").unwrap();
        let definition_box: gtk::Box = self.builder.get_object("definition_box").unwrap();
        let rows = self.rows.clone();
        let current_row = self.current_row.clone();
        list.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let gtk_box = row.get_children()[0].clone();
                let id = gtk_box
                    .get_widget_name()
                    .unwrap_or(GString::from(""))
                    .to_string()
                    .parse()
                    .unwrap();
                current_row.replace(id);
                let hash_map = rows.borrow();
                let row = hash_map.get(&id).unwrap();
                term_label.set_text(&row.term);
                question.set_text(&format!("What is the meaning of {}?", row.term));
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
            }
        });
    }
}

fn is_answer_correct(user_definition: &str, definition: &str) -> bool {
    if user_definition.to_lowercase() == definition.to_lowercase() {
        true
    } else {
        false
    }
}

fn set(var: &Rc<RefCell<i32>>, increase: Option<i32>, value: Option<i32>, label: &Label, name: &str) {
    if let Some(increase) = increase {
        *var.borrow_mut() += increase;
    } else if let Some(value) = value {
        *var.borrow_mut() = value;
    }
    label.set_text(&format!("{}: {}", name, var.borrow()));
}

fn is_complete(rows: &HashMap<i32, Row>) -> bool {
    for (_, row) in rows {
        if row.state == State::UNANSWERED {
            return false;
        }
    }
    true
}

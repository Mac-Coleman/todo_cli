use chrono::{Utc, Datelike, Timelike};
use std::fmt::Display;

pub fn fix_input(mut input: &mut String) {
    *input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    *input = input.to_lowercase();
    *input = String::from(input.trim());
}

pub struct Info {
    pub uncompleted: Vec<TodoItem>,
    pub completed: Vec<TodoItem>,
}

impl Info {
    pub fn new() -> Info {
        Info {
            uncompleted: vec![],
            completed: vec![],
        }
    }

    pub fn mark_complete(&mut self, index: usize) {

    }

    pub fn mark_uncomplete(&mut self, index: usize) {

    }
}

pub struct TodoItem {
    pub time: String,
    pub name: String,
    pub desc: String,
    pub finished: Option<String>,
}

impl TodoItem {
    pub fn new(name: String, desc: String) -> TodoItem {
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        TodoItem {
            time: format!("{}—{:02}—{:02} {:02}:{:02}:{:02} {} UTC",
                    now.year_ce().1,
                    now.month(),
                    now.day(),
                    hour,
                    now.minute(),
                    now.second(),
                    if is_pm { "PM" } else { "AM" } 
                ),
            name: name,
            desc: desc,
            finished: None,
        }
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} {} {}", self.name, self.desc, self.time, match &self.finished {Some(value) => value.clone(), None => String::from("Unfinished.")})
    }
}
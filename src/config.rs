use std::{path::Path, fs};
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    bookname: String,
    author: String
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let json_config = include_str!("../assets/book.json");
        serde_json::from_str(json_config).unwrap()
    }


    pub fn update_author(&mut self, author: String) {
        self.author = author
    }

    pub fn update_bookname(&mut self, name: String) {
        self.bookname = name
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let json = serde_json::to_string(self).unwrap();
        fs::write(path, json).unwrap();
    }
}




use inquire::{validator::Validation, Text};
use std::{fs, path::Path};

use crate::config::Config;

pub struct Book;

impl Book {
    pub fn new(name: &str, force: bool) {
        let path = Path::new(name);
        if path.exists() && force == false {
            panic!("`{}` already exist", name)
        } else if path.exists() && force == true {
            fs::remove_dir_all(path).unwrap();
        }

        fs::create_dir_all(format!("./{name}/src")).unwrap();

        let book_name = Text::new("Choose a book name")
            .with_help_message("Will be displayed at the book cover")
            .with_default(name)
            .with_placeholder(name)
            .prompt()
            .expect("Failed to get user input");

        let author = Text::new("Author name")
            .with_validator(|input: &str| {
                match input.len() {
                    0 =>  Ok(Validation::Invalid("Author name is required".into())),
                    _ => Ok(Validation::Valid)
                }
                
            })
            .prompt()
            .expect("Failed to get user input");

        let mut config = Config::new();
        config.update_author(author);
        config.update_bookname(book_name);
        config.save(format!("./{}/book.json", name));
    }
}

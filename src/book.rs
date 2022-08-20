use inquire::Text;
use std::{fs, path::Path};

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

        let mut book_name = Text::new("Choose a book name")
            .with_help_message("Will be displayed at the book cover")
            .with_default(name)
            .with_placeholder(name)
            .prompt()
            .expect("Failed to get user input");

        let mut author = Text::new("Author name")
            .prompt()
            .expect("Failed to get user input");

        let book_config = include_str!("../assets/book.json");

        fs::write(format!("./{}/book.json", name), book_config).unwrap();
    }
}

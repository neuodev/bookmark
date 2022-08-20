use inquire::Text;
use std::{fs, path::Path};

pub struct Book;

impl Book {
    pub fn new(name: &str, force: bool) {
        let path = Path::new(name);

        if path.exists() {
            panic!("'{}' already exist", name)
        }

        fs::create_dir_all(format!("./{name}/src")).unwrap();

        let book_config = include_str!("../assets/book.json");

        let book_name = Text::new("Book name")
            .prompt()
            .expect("Failed to read user input");

        fs::write(format!("./{}/book.json", name), book_config).unwrap();
    }
}

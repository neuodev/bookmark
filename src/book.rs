use std::{fs, path::Path};

pub struct Book;
impl Book {
    pub fn new(name: &str) {
        let path = Path::new(name);

        if path.exists() {
            panic!("'{}' already exist", name)
        }

        fs::create_dir_all(format!("./{name}/src")).unwrap();

        let book_config = include_str!("../assets/book.json");

        fs::write(format!("./{}/book.json", name), book_config).unwrap();
    }
}
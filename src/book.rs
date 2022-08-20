use inquire::{validator::Validation, Text};
use std::{fs, path::Path};
use std::thread;
use crate::config::Config;
use crate::documents::Document;

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
            .with_validator(|input: &str| match input.len() {
                0 => Ok(Validation::Invalid("Author name is required".into())),
                _ => Ok(Validation::Valid),
            })
            .prompt()
            .expect("Failed to get user input");

        let mut config = Config::new();
        config.update_author(author);
        config.update_bookname(book_name);
        config.save(format!("./{}/book.json", name));
    }

    pub fn build() {
        let config_path = Path::new("./book.json");
        if !config_path.exists() {
            panic!("Missing book.json")
        }

        let config = Config::from_file(config_path);

        let dist = Path::new(&config.dist_dir);

        if dist.exists() {
            fs::create_dir(dist).unwrap();
        }

        let mut handlers = vec![];
        for page in config.pages {
            let root = config.root_dir.clone();
            let dist = config.dist_dir.clone();
            let handler = thread::Builder::new().name(page.title).spawn(move || {
                let file = format!("./{}/{}", root, page.path);
                let path = Path::new(&file);
                let doc = Document::from_file(path);
                doc.save(&format!("./{}/{}.html", dist, page.path));

                page.title
            }).unwrap();

            handlers.push(handler);
        }

        for handler in handlers {
            let page = handler.join().unwrap();
            println!("[Done] {}", page);
        }
    }
}

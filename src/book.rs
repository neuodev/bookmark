use crate::config::{Config, Page};
use crate::documents::Document;
use crate::utils::{md_to_html, copy_recursively};
use inquire::{validator::Validation, Text};
use std::thread;
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

        fs::create_dir_all(format!("./{name}/src/assets")).unwrap();

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
            fs::remove_dir_all(dist).unwrap();
        }

        fs::create_dir(dist).unwrap();
        Book::move_assets(&config.dist_dir,  &config.assets_dir);
        let sidebar = Book::make_sidebar(&config.pages, &config.bookname);

        let mut handlers = vec![];
        for page in config.pages {
            let root = config.root_dir.clone();
            let dist = config.dist_dir.clone();
            let sidebar = sidebar.clone();
            let handler = thread::Builder::new()
                .name(page.title.clone())
                .spawn(move || {
                    let file = format!("./{}/{}", root, page.path);
                    let path = Path::new(&file);
                    let doc = Document::from_file(path);
                    let output_path = md_to_html(&format!("./{}/{}", dist, page.path));
                    doc.save(&output_path, &sidebar);

                    page.title
                })
                .unwrap();

            handlers.push(handler);
        }

        for handler in handlers {
            let page = handler.join().unwrap();
            println!("[Done] {}", page);
        }
    }

    /// Move css / js into the output directory
    fn move_assets(dist: &str, assets_dir: &str) {
        let ouput_dir = Path::new(dist);
        if !ouput_dir.exists() {
            fs::create_dir_all(dist).unwrap();
        }
        
        copy_recursively(assets_dir, format!("{dist}/assets")).unwrap();
        
        let css = include_str!("../assets/style.css");
        let path = format!("{}/style.css", dist);

        fs::write(path, css).unwrap();
    }

    fn make_sidebar(pages: &Vec<Page>, title: &str) -> String {
        let list_item = include_str!("../assets/templates/chapter.html").to_string();
        let mut chapters_list = vec![];

        pages.into_iter().for_each(|page| {
            let mut item = list_item.clone();
            let path = md_to_html(&page.path);
            item = item.replace("$href", &path);
            item = item.replace("$text", &page.title);

            chapters_list.push(item)
        });

        let mut sidebar = include_str!("../assets/templates/sidebar.html").to_string();
        sidebar = sidebar.replace("$chapters", &chapters_list.join(""));
        sidebar.replace("$title", title)
    }


}

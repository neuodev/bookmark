mod cli;
mod documents;
mod node;
mod tokens;
mod utils;
mod book;

use book::Book;
use cli::cli;
use documents::Document;

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let book_name = sub_matches.get_one::<String>("name").expect("required");

            Book::new(book_name)
        }   
        _ => {}
    }

    // let document = Document::from_file("./examples/README.md");
    // document.save("./output.html")
}

#![allow(dead_code)]

mod documents;
mod node;
mod tokens;
mod utils;

use documents::Document;

fn main() {
    let document = Document::from_file("./examples/README.md");
    document.save("./output.html")
}

#![allow(dead_code)]

mod documents;
mod node;
mod tokens;
use std::fs;

use documents::Document;

fn main() {
    let document = Document::from_file("./examples/README.md");
}

#![allow(dead_code)]

mod documents;
mod node;
mod tokens;
use std::fs;

use documents::Document;

fn main() {
    Document::from_file("./examples/README.md");
}

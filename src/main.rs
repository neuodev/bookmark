#![allow(dead_code)]

mod node;
mod tokens;

use regex::Regex;
use std::fs;
use tokens::{List, Paragraph};

use crate::tokens::Heading;

fn main() {
    let file = fs::read_to_string("./examples/list.md").unwrap();

    let mut lines = file
        .split("\n")
        .filter(|l| l != &"\r")
        .collect::<Vec<&str>>();

    println!("{:#?}", lines);
    let mut idx = 0;
    while idx < lines.len() {
        let line = lines[idx];

        let (list, i) = List::new(&lines, idx);

        if list.is_some() {
            println!("{:#?} {i}", list.unwrap());
        }
        idx = i;

        idx += 1;
    }
}

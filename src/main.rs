#![allow(dead_code)]

mod node;
mod tokens;

use regex::Regex;
use std::fs;
use tokens::{List, Paragraph};

use crate::tokens::{Heading, CodeBlock};

fn main() {
    let file = fs::read_to_string("./examples/code.md").unwrap();

    let mut lines = file
        .split("\n")
        .filter(|l| l != &"\r")
        .collect::<Vec<&str>>();

    println!("{:#?}", lines);
    let mut idx = 0;
    while idx < lines.len() {
        let (code_block, end_idx) = CodeBlock::new(&lines, idx);

        if code_block.is_some() {
            idx = end_idx;
        } 

        idx += 1;
    }
}

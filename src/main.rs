#![allow(dead_code)]

mod node;
mod documents;
mod tokens;

use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("./examples/quotes.md").unwrap();

    let mut lines = file
        .split("\n")
        .filter(|l| l != &"\r")
        .collect::<Vec<&str>>();

   
    // let mut idx = 0;
    // while idx < lines.len() {
    //     let (quote, end_idx) = Quote::new(&lines, idx);

    //     if quote.is_some() {
    //         idx = end_idx;

    //         println!("{:#?}", quote.unwrap())
    //     } 

    //     idx += 1;
    // }
}

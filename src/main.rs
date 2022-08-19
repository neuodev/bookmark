mod node;
mod tokens;

use regex::Regex;
use std::fs;
use tokens::Paragraph;

use crate::tokens::Heading;

fn main() {
    let file = fs::read_to_string("./README.md").unwrap();

    let lines = file.split("\n");

    let spaces = Regex::new(r"^\s{2,}").unwrap();

    for (idx, line) in lines.enumerate() {
        if let Some(h) = Heading::new(line) {
            println!("{:#?}", h);
        } else if let Some(p) = Paragraph::new(line) {
            println!("{:#?}", p);
        }
        // if let Some(caps) = re.captures(line) {
        //     println!("Re: {:?}", caps);
        // }

        // if let Some(bold) = bold.captures(line) {
        //     println!("Bold match: {:?}", bold);
        // }

        // if let Some(spaces) = spaces.captures(line) {
        //     println!("Spaces: {:?}", spaces);
        // }
    }
}

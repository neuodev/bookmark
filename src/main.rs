mod node;
mod tokens;

use regex::Regex;
use std::fs;

use crate::tokens::Heading;

fn main() {
    let file = fs::read_to_string("./README.md").unwrap();

    let lines = file.split("\n");
    let re = Regex::new(r"_(?P<italic>.*?)_").unwrap();

    let bold = Regex::new(r"\*\*(?P<bold>.*?)\*\*").unwrap();

    let spaces = Regex::new(r"^\s{2,}").unwrap();

    for (idx, line) in lines.enumerate() {

        if line.len() == 0 {
            println!("{} is empty", idx);
            continue;
        }
        println!("{} -> {}", idx, line);
        Heading::new(line);
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

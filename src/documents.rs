use crate::{
    node::Node,
    tokens::{CodeBlock, Heading, LineBreak, List, Quote, Paragraph},
};
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Document {
    name: String,
    nodes: Vec<Node>,
}

impl Document {
    pub fn from_file<P: AsRef<Path>>(path: P) {
        let file = fs::read_to_string(path).unwrap();
        let lines = file.split("\n").collect::<Vec<&str>>();

        let mut nodes = vec![];
        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx];

            if let Some(linebreak) = LineBreak::new(line) {
                nodes.push(Node::LineBreak(linebreak));
            } else if let Some(heading) = Heading::new(line) {
                nodes.push(Node::Heading(heading));
            } else if let (Some(list), curr_idx) = List::new(&lines, idx) {
                nodes.push(Node::List(list));
                idx = curr_idx;
            } else if let (Some(code_block), curr_idx) = CodeBlock::new(&lines, idx) {
                nodes.push(Node::CodeBlock(code_block));
                idx = curr_idx;
            } else if let (Some(quote), curr_idx) = Quote::new(&lines, idx) {
                nodes.push(Node::Quote(quote));
                idx = curr_idx;
            } else if let Some(paragraph) = Paragraph::new(line) {
                nodes.push(Node::Paragraph(paragraph))
            }

            idx += 1;
        }

        println!("{:#?}", nodes)
    }
}

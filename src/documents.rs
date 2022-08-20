use crate::{
    node::Node,
    tokens::{CodeBlock, Heading, LineBreak, List, Paragraph, Quote},
    utils::replace_by_tag,
};
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Document {
    nodes: Vec<Node>,
}

impl Document {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Document {
        let file = fs::read_to_string(path).unwrap();
        let lines = file.split("\n").collect::<Vec<&str>>();

        let mut nodes = vec![];
        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx];

            if let Some(_) = LineBreak::new(line) {
                // Should be ingored
            } else if let Some(heading) = Heading::new(line) {
                nodes.push(Node::Heading(heading));
            } else if let (Some(list), curr_idx) = List::new(&lines, idx) {
                nodes.push(Node::List(list));
                idx = curr_idx - 1;
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

        Document { nodes }
    }

    pub fn into_html(&self) -> String {
        let html_body = self
            .nodes
            .iter()
            .map(|n| n.into_html())
            .collect::<Vec<String>>()
            .join("\n");

        let html_doc = include_str!("../assets/templates/base.html");

        replace_by_tag(html_doc, &html_body, "body")
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let html = self.into_html();

        fs::write(path, html).unwrap();
    }
}

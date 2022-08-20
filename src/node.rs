use crate::tokens::{CodeBlock, Heading, LineBreak, List, Paragraph, Quote};

/// Awrapper enum for all the supported MD blocks
#[derive(Debug)]
pub enum Node {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
    CodeBlock(CodeBlock),
    Quote(Quote),
    LineBreak(LineBreak),
}


impl Node {
    pub fn into_html(&self) -> String {
        match self {
            Node::CodeBlock(code) => code.into_html(),
            Node::Paragraph(p) => p.into_html(),
            Node::List(l) => l.into_html(),
            Node::Quote(q) => q.into_html(),
            Node::Heading(h) => h.into_html(),
            Node::LineBreak(l) => l.into_html(),
        }
    }
}
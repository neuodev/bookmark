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

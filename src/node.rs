use crate::tokens::{
    Heading,
    Paragraph,
    List,
    CodeBlock,
    Quote,
    LineBreak
};

/// Awrapper enum for all the supported MD blocks
pub enum Node {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
    CodeBlock(CodeBlock),
    Quote(Quote),
    LineBreak(LineBreak)
}
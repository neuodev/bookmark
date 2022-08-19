use regex::Regex;

#[derive(Debug)]
pub enum BlockToken {
    H1 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H2 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H3 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H4 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H5 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H6 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    P {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    Bold,
    Italic,
    Anchor,
    Img,
}

#[derive(Debug)]
pub enum InlineToken {
    Link { href: String, text: String },
    Bold(String),
    Italic(String),
}

pub enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingType {
    pub fn new(line: &str) -> Option<HeadingType> {
        let re = Regex::new(r"^(?P<type>#{1,})").unwrap();

        if let Some(caps) = re.captures(line) {
            let number_signs = &caps["type"];

            let h_type = match number_signs.len() {
                1 => HeadingType::H1,
                2 => HeadingType::H2,
                3 => HeadingType::H3,
                4 => HeadingType::H4,
                5 => HeadingType::H5,
                _ => HeadingType::H6,
            };

            return Some(h_type)
        }
        None
    }
}

pub struct Heading {
    h_type: HeadingType,
    text: String,
    inline_tokens: Vec<InlineToken>,
}

impl Heading {
    pub fn new(line: &str) {
        HeadingType::new(line);
    }
}

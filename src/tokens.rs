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
    Link { href: String, text: String }, // reg: \[(?P<text>[^\]]+)\]\((?P<href>[^\]]+)\)
    Bold(String),
    Code(String),
    Italic(String),
}

impl InlineToken {
    fn extract(text: &str) -> Vec<InlineToken> {
        // Match links
        let re_link = Regex::new(r"\[(?P<text>[^\]]+)\]\((?P<href>[^\]]+)\)").unwrap();
        let re_bold = Regex::new(r"\*\*(?P<text>[^\*]+)\*\*").unwrap();
        let re_italic = Regex::new(r"_(?P<text>[^_]+)_").unwrap();
        let re_code = Regex::new(r"`(?P<text>[^`]+)`").unwrap();

        let re_set = [
            (re_link, "link"),
            (re_bold, "bold"),
            (re_italic, "italic"),
            (re_code, "code"),
        ];

        let mut result: Vec<InlineToken> = vec![];

        re_set.into_iter().for_each(|(re, name)| {
            let mut tokens = re
                .captures_iter(text)
                .map(|caps| {
                    let text = (&caps["text"]).to_string();
                    match name {
                        "link" => InlineToken::Link {
                            href: (&caps["href"]).to_string(),
                            text,
                        },
                        "bold" => InlineToken::Bold(text),
                        "italic" => InlineToken::Italic(text),
                        "code" => InlineToken::Code(text),
                        _ => {
                            panic!("unsupported type")
                        }
                    }
                })
                .collect::<Vec<InlineToken>>();

            result.append(&mut tokens)
        });

        println!("{:#?}", result);

        result
    }
}

#[derive(Debug)]
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

            return Some(h_type);
        }
        None
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            HeadingType::H1 => "#",
            HeadingType::H2 => "##",
            HeadingType::H3 => "###",
            HeadingType::H4 => "####",
            HeadingType::H5 => "#####",
            HeadingType::H6 => "######",
        }
    }
}

pub struct Heading {
    h_type: HeadingType,
    text: String,
    inline_tokens: Vec<InlineToken>,
}

impl Heading {
    pub fn new(line: &str) -> Option<Heading> {
        let line = line.trim();
        let h_type = match HeadingType::new(line) {
            Some(h) => h,
            None => return None,
        };

        // Extract text
        let re = Regex::new(r"#{1,6}\s+(?P<text>.+)").unwrap();

        if let Some(caps) = re.captures(line) {
            let text = &caps["text"];
            InlineToken::extract(text);
        }

        None
    }
}

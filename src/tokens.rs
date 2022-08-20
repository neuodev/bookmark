use regex::{Captures, Regex};

#[derive(Debug)]
pub enum InlineToken {
    Link {
        href: String,
        text: String,
        raw: String,
    },
    Bold {
        value: String,
        raw: String,
    },
    Code {
        value: String,
        raw: String,
    },
    Italic {
        value: String,
        raw: String,
    },
    Image {
        alt: String,
        src: String,
        raw: String,
    },
}

impl InlineToken {
    fn extract(text: &String) -> Vec<InlineToken> {
        // Match any one of these
        let re_set = [
            r"\[(?P<link_text>[^\]]+)\]\((?P<href>[^\]]+)\)", // Link
            r"\*\*(?P<bold>[^\*]+)\*\*",                      // Bold text
            r"_(?P<italic>[^_]+)_",                           // Italic text
            r"`(?P<code>[^`]+)`",                             // Inline code
            r"!\[(?P<alt>[^\]]+)\]\((?P<src>[^\]]+)\)",       // Image
        ];

        let re = Regex::new(&re_set.join("|")).unwrap();

        let tokens = re
            .captures_iter(text)
            .map(|caps| {
                let raw = caps[0].to_string();

                let href = InlineToken::get_name(&caps, "href");
                let link_text = InlineToken::get_name(&caps, "link_text");
                let bold = InlineToken::get_name(&caps, "bold");
                let italic = InlineToken::get_name(&caps, "italic");
                let code = InlineToken::get_name(&caps, "code");
                let img_src = InlineToken::get_name(&caps, "src");
                let img_alt = InlineToken::get_name(&caps, "alt");

                if href.is_some() && link_text.is_some() {
                    InlineToken::Link {
                        href: href.unwrap(),
                        text: link_text.unwrap(),
                        raw,
                    }
                } else if img_src.is_some() && img_alt.is_some() {
                    InlineToken::Image {
                        src: img_src.unwrap(),
                        alt: img_alt.unwrap(),
                        raw,
                    }
                } else if bold.is_some() {
                    InlineToken::Bold {
                        value: bold.unwrap(),
                        raw,
                    }
                } else if italic.is_some() {
                    InlineToken::Italic {
                        value: italic.unwrap(),
                        raw,
                    }
                } else if code.is_some() {
                    InlineToken::Code {
                        value: code.unwrap(),
                        raw,
                    }
                } else {
                    // Should never happen
                    // Regex should never match other names
                    panic!("Regex maching unsupported type")
                }
            })
            .collect();

        tokens
    }

    fn get_name(caps: &Captures, name: &str) -> Option<String> {
        if caps.name(name).is_some() {
            return Some(caps[name].to_string());
        }

        None
    }

    fn get_raw(&self) -> &String {
        match &self {
            InlineToken::Link { raw, .. } => raw,
            InlineToken::Bold { raw, .. } => raw,
            InlineToken::Code { raw, .. } => raw,
            InlineToken::Italic { raw, .. } => raw,
            InlineToken::Image { raw, .. } => raw,
        }
    }

    fn mask_tokens(mut text: String, tokens: &Vec<InlineToken>) -> String {
        tokens.iter().enumerate().for_each(|(idx, token)| {
            text = text.replace(token.get_raw(), &format!("<${}>", idx + 1))
        });

        text
    }

    fn into_html(line: &str, tokens: &Vec<InlineToken>) -> String {
        let mut new_line = line.to_string();
        tokens.into_iter().enumerate().for_each(|(idx, t)| {
            let html: String;
            match t {
                InlineToken::Bold { value, .. } => html = format!("<strong>{}</strong>", value),
                InlineToken::Code { value, .. } => {
                    html = format!("<span class='inline-code'>{}</span>", value)
                }
                InlineToken::Image { alt, src, .. } => {
                    html = format!("<img src='{}' alt='{}'/>", src, alt);
                }
                InlineToken::Italic { value, .. } => {
                    html = format!("<i>{}</i>", value);
                }
                InlineToken::Link { href, text, .. } => {
                    html = format!("<a href='{}'>{}</a>", href, text)
                }
            }
            new_line = new_line.replace(&format!("<${}>", idx + 1), &html);
        });

        new_line
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

    pub fn get_tag_num(&self) -> u8 {
        match self {
            HeadingType::H1 => 1,
            HeadingType::H2 => 2,
            HeadingType::H3 => 3,
            HeadingType::H4 => 4,
            HeadingType::H5 => 5,
            _ => 6,
        }
    }
}

#[derive(Debug)]
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
            let mut text = (&caps["text"]).to_string();
            let inline_tokens = InlineToken::extract(&mut text);

            return Some(Heading {
                h_type,
                text: InlineToken::mask_tokens(text, &inline_tokens),
                inline_tokens,
            });
        }

        None
    }

    pub fn into_html(&self) -> String {
        let tag = self.h_type.get_tag_num();
        let text = InlineToken::into_html(&self.text, &self.inline_tokens);
        format!("<h{}>{}</h{}>", tag, text, tag)
    }
}

#[derive(Debug)]
pub struct Paragraph {
    text: String,
    inline_tokens: Vec<InlineToken>,
}
/// Should match any text without in special tokens (ex: # or *)
impl Paragraph {
    pub fn new(line: &str) -> Option<Self> {
        let mut text = line.trim().to_string();
        if text.len() == 0 {
            return None;
        }

        let inline_tokens = InlineToken::extract(&mut text);
        let text = InlineToken::mask_tokens(text, &inline_tokens);

        Some(Paragraph {
            text,
            inline_tokens,
        })
    }

    pub fn into_html(&self) -> String {
        let text = InlineToken::into_html(&self.text, &self.inline_tokens);
        format!("<p>{}</p>", text,)
    }
}

/// Support for <ol> </ol> or <ul> </ul>
#[derive(Debug, PartialEq, Eq)]
pub enum ListType {
    Ordered,
    Unordered,
}

/// Internal representation of <li> tag
#[derive(Debug)]
pub struct ListItem {
    text: String,
    inline_tokens: Vec<InlineToken>,
}

impl ListItem {
    /// Create new list item
    fn new(line: &str) -> Self {
        let text = line.trim().to_string();
        let inline_tokens = InlineToken::extract(&text);
        let text = InlineToken::mask_tokens(text, &inline_tokens);
        let text = ListItem::trim(&text);
        Self {
            text,
            inline_tokens,
        }
    }

    /// Remove list markers like `* | - | 1. | 2.`
    fn trim(line: &str) -> String {
        let list_type = List::get_list_type(line).unwrap();
        match list_type {
            ListType::Unordered => line.replace("-", "").trim().into(),
            ListType::Ordered => {
                // Match every line that starts with a number 1. | 2. | ....
                let re = Regex::new(r"^(?P<idx>[0-9]+)\.").unwrap();
                let caps = re.captures(line).unwrap();
                let raw_token = &caps[0];
                line.replace(raw_token, "").trim().into()
            }
        }
    }

    pub fn into_html(&self) -> String {
        let text = InlineToken::into_html(&self.text, &self.inline_tokens);
        format!("<li>{text}</li>")
    }
}

#[derive(Debug)]
/// Collection of list items
pub struct List {
    list_type: ListType,
    items: Vec<ListItem>,
}

impl List {
    pub fn new(lines: &Vec<&str>, mut idx: usize) -> (Option<Self>, usize) {
        let mut items = vec![];
        let mut list_type = ListType::Ordered;

        loop {
            if idx >= lines.len() {
                break;
            }

            let line = lines[idx];

            if line.trim().len() == 0 {
                idx += 1;
                continue;
            }

            list_type = match List::get_list_type(&line) {
                Some(t) => t,
                None => break,
            };

            let item = ListItem::new(&line);
            items.push(item);

            idx += 1;
        }
        if items.len() != 0 {
            return (
                Some(Self {
                    list_type,
                    items: items,
                }),
                idx,
            );
        }

        (None, idx)
    }

    fn get_list_type(line: &str) -> Option<ListType> {
        let line = line.trim();
        // Match every line that starts with a number 1. | 2. | ....
        let re = Regex::new(r"^(?P<idx>[0-9]+)\.").unwrap();

        match line {
            line if line.starts_with("- ") => Some(ListType::Unordered),
            line if line.starts_with("* ") => Some(ListType::Unordered),
            line if re.captures(line).is_some() => Some(ListType::Ordered),
            _ => None,
        }
    }

    pub fn into_html(&self) -> String {
        let list_items = self
            .items
            .iter()
            .map(|list_item| list_item.into_html())
            .collect::<Vec<String>>()
            .join("\n");

        let mut tag = "ol";
        if self.list_type == ListType::Unordered {
            tag = "ul"
        }

        format!("<{}>{}</{}>", tag, list_items, tag)
    }
}

#[derive(Debug)]
pub struct CodeBlock {
    lang: String,
    lines: Vec<String>,
}

impl CodeBlock {
    pub fn new(lines: &Vec<&str>, idx: usize) -> (Option<Self>, usize) {
        let line = lines[idx];
        // Regex to match the first line in a code block
        let re = Regex::new(r"`{3}(?P<lang>[^\n]+)").unwrap();

        let caps = match re.captures(line) {
            Some(c) => c,
            None => return (None, idx),
        };

        let mut end_idx = idx;

        loop {
            end_idx += 1;
            if end_idx >= lines.len() {
                break;
            }

            if lines[end_idx] == "```" {
                break;
            }
        }

        let code_lines = (&lines[idx + 1..end_idx])
            .into_iter()
            .map(|l| l.to_string())
            .collect();

        (
            Some(CodeBlock {
                lang: caps["lang"].to_string(),
                lines: code_lines,
            }),
            end_idx,
        )
    }

    pub fn into_html(&self) -> String {
        let code = self.lines.join("<br/>");

        format!("<code class={}>{}</code>", self.lang, code)
    }
}

#[derive(Debug)]
pub struct QuoteLine {
    text: String,
    inline_tokens: Vec<InlineToken>,
}

impl QuoteLine {
    fn new(line: &str) -> Self {
        let line = line.trim().to_string();
        let inline_tokens = InlineToken::extract(&line);
        let text = InlineToken::mask_tokens(line, &inline_tokens);
        QuoteLine {
            text,
            inline_tokens,
        }
    }

    pub fn into_html(&self) -> String {
        let quote = InlineToken::into_html(&self.text, &self.inline_tokens);

        format!("<p>{}<p>", quote)
    }
}

#[derive(Debug)]
pub struct Quote {
    lines: Vec<QuoteLine>,
}

impl Quote {
    pub fn new(lines: &Vec<&str>, idx: usize) -> (Option<Self>, usize) {
        let line = lines[idx].trim();

        if !line.starts_with(">") {
            return (None, idx);
        }

        let mut end_idx = idx;
        loop {
            end_idx += 1;
            if end_idx >= lines.len() || !lines[end_idx].starts_with(">") {
                break;
            }
        }

        let quote_lines = lines[idx..end_idx]
            .into_iter()
            .map(|&l| QuoteLine::new(l))
            .collect();
        (Some(Quote { lines: quote_lines }), end_idx)
    }

    pub fn into_html(&self) -> String {
        let quote = self
            .lines
            .iter()
            .map(|l| l.into_html())
            .collect::<Vec<String>>()
            .join("\r");

        format!("<quote>{}<quote>", quote)
    }
}

#[derive(Debug)]
pub struct LineBreak;

impl LineBreak {
    pub fn new(line: &str) -> Option<LineBreak> {
        if line.trim().len() == 0 {
            return Some(LineBreak);
        }
        None
    }

    pub fn into_html(&self) -> String {
        "<br/>".into()
    }
}

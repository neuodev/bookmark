use regex::Regex;

pub fn replace_by_tag(doc: &str, by: &str, tag: &str) -> String {
    let re = Regex::new(&format!(r"<{t}>(?P<text>[\s\S]*)</{t}>", t=tag)).unwrap();

    let caps = re.captures(doc).expect("Tag not found");

    let text = &caps["text"];

    doc.replace(text, by)
}
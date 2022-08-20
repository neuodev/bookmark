use regex::Regex;

pub fn replace_by_tag(doc: &str, tag: &str) {
    let re = Regex::new(&format!(r"<{t}>(?P<text>[^<]*)<\/{t}>", t=tag)).unwrap();

    let caps = re.captures(doc).expect("Tag not found");

    println!("Caps: {:#?}", caps);
}
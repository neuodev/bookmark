use std::path::Path;

use regex::Regex;

pub fn replace_by_tag(doc: &str, by: &str, tag: &str) -> String {
    let re = Regex::new(&format!(r"<{t}>(?P<text>[\s\S]*)</{t}>", t = tag)).unwrap();

    let caps = re.captures(doc).expect("Tag not found");

    let text = &caps["text"];

    doc.replace(text, by)
}


pub fn md_to_html(file: &str) -> String {
    let path = Path::new(file);
    
    let ext = path.extension().unwrap();

    if ext != "md" {
        panic!("{} is not a markdown file", file)
    }

    let mut parts = file.split(".").collect::<Vec<&str>>();
    let last_idx  = parts.len() - 1;
    let html_ext = parts[last_idx].replace("md", "html");
    parts[last_idx] = html_ext.as_str();

    parts.join(".")
}

#[cfg(test)]
mod test {
    use super::md_to_html;

    #[test]
    fn it_change_md_extensxion_to_html() {
        let paths = [
            ("README.md", "README.html"),
            ("./src/rust.md", "./src/rust.html"),
            ("./src/file.md.md", "./src/file.md.html")
        ];

        paths.into_iter().for_each(|(input, output)| {
            assert_eq!(md_to_html(input), output);
        });
    }
}
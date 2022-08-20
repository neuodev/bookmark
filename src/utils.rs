use std::{path::Path, fs, io};

use regex::Regex;

pub fn replace_html(doc: &str, content: &str, sidebar: &str) -> String {
    let re_content = Regex::new(r#"<div class="content">(?P<text>[\s\S][^<]*?)</div>"#).unwrap();
    let re_sidebar = Regex::new(r#"<div class="sidebar">(?P<text>[\s\S][^<]*?)</div>"#).unwrap();

    let content_caps = re_content.captures(doc).expect("Tag not found");
    let sidebar_caps = re_sidebar.captures(doc).expect("Tag not found");

    let content_placeholder = &content_caps["text"];
    let sidebar_placholder = &sidebar_caps["text"];

    let doc = doc.replace(content_placeholder, content);
    doc.replace(sidebar_placholder, sidebar)
}

pub fn md_to_html(file: &str) -> String {
    let path = Path::new(file);

    let ext = path.extension().unwrap();

    if ext != "md" {
        panic!("{} is not a markdown file", file)
    }

    let mut parts = file.split(".").collect::<Vec<&str>>();
    let last_idx = parts.len() - 1;
    let html_ext = parts[last_idx].replace("md", "html");
    parts[last_idx] = html_ext.as_str();

    parts.join(".")
}

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::md_to_html;

    #[test]
    fn it_change_md_extensxion_to_html() {
        let paths = [
            ("README.md", "README.html"),
            ("./src/rust.md", "./src/rust.html"),
            ("./src/file.md.md", "./src/file.md.html"),
        ];

        paths.into_iter().for_each(|(input, output)| {
            assert_eq!(md_to_html(input), output);
        });
    }
}

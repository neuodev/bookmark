use std::path::Path;

pub struct Config<P: AsRef<Path>> {
    path: P
}

impl<P: AsRef<Path>> Config<P> {
    fn new(path: P) -> Self {
        Self { path }
    }
}
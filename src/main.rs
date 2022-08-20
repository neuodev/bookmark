mod book;
mod cli;
mod config;
mod documents;
mod node;
mod tokens;
mod utils;

use book::Book;
use cli::{Action, Cli};
use documents::Document;

fn main() {
    let cli = Cli::new();

    match cli.get_command() {
        Action::NewBook { name, force } => Book::new(&name, force),
    }
}

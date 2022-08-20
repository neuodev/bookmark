use clap::{arg, Command};

pub struct Cli;

impl Cli {
    pub fn new() -> Command<'static> {
        Command::new("bookmark")
            .about("Convert markdown to production ready websites")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("new")
                    .about("Start new book")
                    .arg(arg!(<name> "Name of the book"))
                    .arg_required_else_help(true),
            )
    }

}  

use clap::{arg, Arg, ArgMatches, Command};

pub struct Cli {
    args: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        let app = Command::new("bookmark")
            .about("Convert markdown to production ready websites")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("new")
                    .about("Start new book")
                    .arg(arg!(<name> "Name of the book"))
                    .arg_required_else_help(true)
                    .arg(
                        Arg::with_name("force")
                            .short('f')
                            .long("force")
                            .takes_value(false)
                            .help("Overwrite existing book with the same name"),
                    )
                    .arg_required_else_help(false),
            )
            .subcommand(
                Command::new("build")
                    .about("Combile markdown into HTML")
                    .subcommand_required(false),
            );
        let args = app.get_matches();
        Cli { args }
    }

    pub fn get_command(&self) -> Action {
        match self.args.subcommand() {
            Some(("new", sub_matches)) => {
                let book_name = sub_matches.get_one::<String>("name").expect("required");
                let force = sub_matches.is_present("force");
                Action::NewBook {
                    name: book_name.into(),
                    force,
                }
            }
            Some(("build", _)) => Action::Build,
            _ => todo!(),
        }
    }
}

pub enum Action {
    NewBook { name: String, force: bool },
    Build,
}

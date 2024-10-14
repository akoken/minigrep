use clap::{Arg, ArgAction, Command};

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub ignore_case: bool,
}

pub fn parse_args() -> Config {
    let matches = Command::new("minigrep")
        .version("1.0")
        .author("Abdurrahman Alp Köken")
        .about("Searches for a pattern in a file")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("file")
                .help("The file to search in")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("ignore_case")
                .short('i')
                .long("ignore-case")
                .help("Ignore case during search")
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    // Get the values from the matches
    let pattern = matches.get_one::<String>("pattern").unwrap().to_string();
    let filename = matches.get_one::<String>("file").unwrap().to_string();
    let ignore_case = matches.get_flag("ignore_case");

    Config {
        pattern,
        filename,
        ignore_case,
    }
}

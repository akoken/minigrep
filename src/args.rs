use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub ignore_case: bool,
    pub line_number: bool,
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
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("line_number")
                .short('l')
                .long("line-number")
                .help("Show line numbers")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap().to_string();
    let filename = matches.get_one::<String>("file").unwrap().to_string();
    let ignore_case = matches.get_flag("ignore_case");
    let line_number = matches.get_flag("line_number");

    Config {
        pattern,
        filename,
        ignore_case,
        line_number,
    }
}

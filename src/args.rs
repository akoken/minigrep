use clap::{Arg, ArgAction, Command};

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub ignore_case: bool,
    pub line_number: bool,
    pub color_mode: ColorMode,
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
                .short('n')
                .long("line-number")
                .help("Show line numbers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .help("Control colorization: auto, always, never")
                .value_parser(["auto", "always", "never"])
                .default_value("auto"),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .expect("pattern argument is required")
        .to_string();
    let filename = matches
        .get_one::<String>("file")
        .expect("file argument is required")
        .to_string();
    let ignore_case = matches.get_flag("ignore_case");
    let line_number = matches.get_flag("line_number");

    let color_mode = match matches.get_one::<String>("color").map(|s| s.as_str()) {
        Some("always") => ColorMode::Always,
        Some("never") => ColorMode::Never,
        _ => ColorMode::Auto,
    };

    Config {
        pattern,
        filename,
        ignore_case,
        line_number,
        color_mode,
    }
}

mod args;

use args::Config;
use std::cmp::PartialEq;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let config = args::parse_args();

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results: Vec<SearchResult> = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };

    for line in results {
        if config.line_number {
            println!("{}: {}", line.line_number, line.line_text);
        } else {
            println!("{}", line.line_text);
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct SearchResult {
    line_number: u32,
    line_text: String,
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number && self.line_text == other.line_text
    }
}

pub fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(index, line)| SearchResult {
            line_number: (index + 1) as u32,
            line_text: line.to_string(),
        })
        .collect()
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|(index, line)| SearchResult {
            line_number: (index + 1) as u32,
            line_text: line.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let expected = vec![SearchResult {
            line_number: 2,
            line_text: "safe, fast, productive.".to_string(),
        }];
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![
            SearchResult {
                line_number: 1,
                line_text: "Rust:".to_string(),
            },
            SearchResult {
                line_number: 4,
                line_text: "Trust me.".to_string(),
            },
        ];
        assert_eq!(expected, search_case_insensitive(query, contents));
    }
}

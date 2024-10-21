mod args;

use args::Config;
use colored::*;
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
    let results = search(&config.pattern, &contents, config.ignore_case);

    for line in results {
        if config.line_number {
            println!(
                "{}: {}",
                line.line_number.to_string().blue(),
                line.line_text
            );
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

pub fn search(query: &str, contents: &str, ignore_case: bool) -> Vec<SearchResult> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let matches = if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(query)
            };

            if matches {
                let colored_line = if ignore_case {
                    let mut colored_line = line.to_string();
                    for (start, part) in line.to_lowercase().match_indices(&query.to_lowercase()) {
                        let end = start + part.len();
                        let colored_part = &line[start..end].red().to_string();
                        colored_line.replace_range(start..end, colored_part);
                    }
                    colored_line
                } else {
                    line.replace(query, &query.red().to_string())
                };

                Some(SearchResult {
                    line_number: (index + 1) as u32,
                    line_text: colored_line,
                })
            } else {
                None
            }
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
            line_text: "safe, fast, pro".to_string() + &"duct".red().to_string() + "ive.",
        }];
        assert_eq!(expected, search(query, contents, false));
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
                line_text: "Rust".red().to_string() + ":",
            },
            SearchResult {
                line_number: 4,
                line_text: "T".to_string() + &"rust".red().to_string() + " me.",
            },
        ];
        assert_eq!(expected, search(query, contents, true));
    }
}

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
    let query_to_search = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line_to_search = if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if line_to_search.contains(&query_to_search) {
                let mut colored_line = String::new();
                let mut last_match_end = 0;

                let matches = line_to_search.match_indices(&query_to_search);

                for (start, _) in matches {
                    colored_line.push_str(&line[last_match_end..start]);

                    let original_text = &line[start..start + query.len()];
                    colored_line.push_str(&original_text.red().to_string());

                    last_match_end = start + query.len();
                }

                colored_line.push_str(&line[last_match_end..]);

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

    fn strip_color_codes(s: &str) -> String {
        s.replace("\x1b[31m", "").replace("\x1b[0m", "")
    }

    #[test]
    fn test_case_sensitive_colorization() {
        let query = "Warning";
        let contents = "Warning: This is a test.\nAnother line with Warning.\nFinal line.";
        let ignore_case = false;

        let results = search(query, contents, ignore_case);

        assert_eq!(results.len(), 2);

        let expected_line1 = format!("{}: This is a test.", "Warning".red());
        let expected_line2 = format!("Another line with {}.", "Warning".red());

        assert_eq!(results[0].line_text, expected_line1);
        assert_eq!(results[1].line_text, expected_line2);
    }

    #[test]
    fn test_case_insensitive_colorization() {
        let query = "warning";
        let contents = "Warning: This is a test.\nAnother line with warning.\nFinal line.";
        let ignore_case = true;

        let results = search(query, contents, ignore_case);

        assert_eq!(results.len(), 2);

        let expected_line1 = format!("{}: This is a test.", "Warning".red());
        let expected_line2 = format!("Another line with {}.", "warning".red());

        assert_eq!(results[0].line_text, expected_line1);
        assert_eq!(results[1].line_text, expected_line2);
    }

    #[test]
    fn test_no_colorization_in_non_matching_line() {
        let query = "Error";
        let contents = "Warning: This is a test.\nAnother line with Warning.\nFinal line.";
        let ignore_case = true;

        let results = search(query, contents, ignore_case);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_colorization_strip() {
        let query = "Warning";
        let contents = "Warning: Check the warning levels.\nIgnore warnings on final line.";
        let ignore_case = false;

        let results = search(query, contents, ignore_case);

        for result in results {
            let stripped_text = strip_color_codes(&result.line_text);
            assert!(stripped_text.contains(query));
            assert!(!stripped_text.contains("\x1b["));
        }
    }
}

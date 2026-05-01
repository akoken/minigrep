mod args;

use args::{ColorMode, Config};
use colored::*;
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
            let num_str = if should_colorize(config.color_mode) {
                line.line_number.to_string().blue().to_string()
            } else {
                line.line_number.to_string()
            };
            println!(
                "{}: {}",
                num_str,
                colorize_line(
                    &line.line_text,
                    &config.pattern,
                    config.ignore_case,
                    config.color_mode
                )
            );
        } else {
            println!(
                "{}",
                colorize_line(
                    &line.line_text,
                    &config.pattern,
                    config.ignore_case,
                    config.color_mode
                )
            );
        }
    }

    Ok(())
}

fn should_colorize(color_mode: ColorMode) -> bool {
    match color_mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    }
}

pub struct SearchResult {
    pub line_number: u32,
    pub line_text: String,
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
                Some(SearchResult {
                    line_number: (index + 1) as u32,
                    line_text: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn colorize_line(line: &str, query: &str, ignore_case: bool, color_mode: ColorMode) -> String {
    if !should_colorize(color_mode) {
        return line.to_string();
    }

    let query_to_search = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };

    let line_to_search = if ignore_case {
        &line.to_lowercase()
    } else {
        line
    };

    if !line_to_search.contains(query_to_search) {
        return line.to_string();
    }

    let mut colored_line = String::new();
    let mut last_match_end = 0;

    for (start, _) in line_to_search.match_indices(query_to_search) {
        colored_line.push_str(&line[last_match_end..start]);

        let original_text = &line[start..start + query.len()];
        colored_line.push_str(&format!("\x1b[31m{}\x1b[0m", original_text));

        last_match_end = start + query.len();
    }

    colored_line.push_str(&line[last_match_end..]);

    colored_line
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi_codes(s: &str) -> String {
        const ANSI_ESCAPE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap());
        ANSI_ESCAPE.replace_all(s, "").to_string()
    }

    fn make_colored(text: &str) -> String {
        format!("\x1b[31m{}\x1b[0m", text)
    }

    #[test]
    fn test_case_sensitive_search() {
        let query = "Warning";
        let contents = "Warning: This is a test.\nAnother line with Warning.\nFinal line.";

        let results = search(query, contents, false);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_text, "Warning: This is a test.");
        assert_eq!(results[1].line_text, "Another line with Warning.");
    }

    #[test]
    fn test_case_insensitive_search() {
        let query = "hello";
        let contents = "HELLO world\nHello there\nhElLo mIxEd\nno match here";

        let results = search(query, contents, true);
        assert_eq!(results.len(), 3);

        let results_sensitive = search(query, contents, false);
        assert_eq!(results_sensitive.len(), 0);
    }

    #[test]
    fn test_colorization_with_colors_enabled() {
        let query = "Warning";
        let contents = "Warning: This is a test.\nAnother line with Warning.";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 2);

        let colored0 = colorize_line(&results[0].line_text, query, false, ColorMode::Always);
        let colored1 = colorize_line(&results[1].line_text, query, false, ColorMode::Always);

        assert!(colored0.contains("Warning"));
        assert!(colored1.contains("Warning"));

        let stripped0 = strip_ansi_codes(&colored0);
        let stripped1 = strip_ansi_codes(&colored1);
        assert_eq!(stripped0, "Warning: This is a test.");
        assert_eq!(stripped1, "Another line with Warning.");
    }

    #[test]
    fn test_colorization_with_colors_disabled() {
        let query = "Warning";
        let contents = "Warning: This is a test.";

        let results = search(query, contents, false);
        let colored = colorize_line(&results[0].line_text, query, false, ColorMode::Never);
        assert_eq!(colored, "Warning: This is a test.");
    }

    #[test]
    fn test_colorization_preserves_original_case() {
        let query = "warning";
        let contents = "Warning: test\nWARNING: test2\nWaRnInG: test3";

        let results = search(query, contents, true);
        assert_eq!(results.len(), 3);

        let colored0 = colorize_line(&results[0].line_text, query, true, ColorMode::Always);
        let stripped0 = strip_ansi_codes(&colored0);
        assert_eq!(stripped0, "Warning: test");

        let colored2 = colorize_line(&results[2].line_text, query, true, ColorMode::Always);
        let stripped2 = strip_ansi_codes(&colored2);
        assert_eq!(stripped2, "WaRnInG: test3");
    }

    #[test]
    fn test_colorization_no_match() {
        let query = "Error";
        let contents = "Warning: This is a test.\nAnother line.";

        let results = search(query, contents, true);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_multiple_matches_on_same_line() {
        let query = "the";
        let contents = "The quick brown the fox jumps over the lazy dog.";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 1);

        let colored = colorize_line(&results[0].line_text, query, false, ColorMode::Always);
        let stripped = strip_ansi_codes(&colored);
        assert_eq!(stripped, "The quick brown the fox jumps over the lazy dog.");

        let results_ci = search(query, contents, true);
        assert_eq!(results_ci.len(), 1);
    }

    #[test]
    fn test_empty_pattern() {
        let query = "";
        let contents = "line one\nline two";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_file_with_no_matches() {
        let query = "zzzznonexistent";
        let contents = "hello world\nfoo bar";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_unicode_search() {
        let query = "café";
        let contents = "I love café latte.\nRegular coffee here.";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 1);
        assert!(results[0].line_text.contains("café"));
    }

    #[test]
    fn test_unicode_case_insensitive() {
        let query = "café";
        let contents = "CAFÉ is great.\nI love café.";

        let results = search(query, contents, true);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_line_numbers() {
        let contents = "first line\nsecond line with test\nthird line\ntest again";
        let query = "test";

        let results = search(query, contents, false);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 2);
        assert_eq!(results[1].line_number, 4);
    }

    #[test]
    fn test_expected_color_format() {
        let query = "Warning";
        let contents = "Warning: test";

        let results = search(query, contents, false);
        let colored = colorize_line(&results[0].line_text, query, false, ColorMode::Always);

        let expected = make_colored("Warning") + ": test";
        assert_eq!(colored, expected);
    }

    #[test]
    fn test_expected_color_format_case_insensitive() {
        let query = "warning";
        let contents = "Warning: test";

        let results = search(query, contents, true);
        let colored = colorize_line(&results[0].line_text, query, true, ColorMode::Always);

        let expected = make_colored("Warning") + ": test";
        assert_eq!(colored, expected);
    }

    #[test]
    fn test_expected_color_format_multiple_matches() {
        let query = "the";
        let contents = "the cat and the dog";

        let results = search(query, contents, false);
        let colored = colorize_line(&results[0].line_text, query, false, ColorMode::Always);

        let expected = make_colored("the") + " cat and " + &make_colored("the") + " dog";
        assert_eq!(colored, expected);
    }
}

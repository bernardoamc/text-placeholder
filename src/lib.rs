//! # A flexible text template engine
//!
//! ## Overview
//! Create templates with named placeholders within it.
//!
//! Placeholders are defined by default following the handlebars syntax, but can be overriden
//! with specific boundaries.
//!
//! ## Example
//!
//!     use text_placeholder::Template;
//!     use std::collections::HashMap;
//!
//!     let default_template = Template::new("Hello {{first}} {{second}}!");
//!
//!     let mut table = HashMap::new();
//!     table.insert("first", "text");
//!     table.insert("second", "placeholder");
//!
//!     assert_eq!(default_template.fill_in(&table), "Hello text placeholder!");
//!
//!     // We can also specify our own boundaries:
//!
//!     let custom_template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");
//!
//!     assert_eq!(default_template.fill_in(&table), "Hello text placeholder!");

mod parser;
use parser::{Parser, Token};

use std::collections::HashMap;

const DEFAULT_START_PLACEHOLDER: &str = "{{";
const DEFAULT_END_PLACEHOLDER: &str = "}}";

/// A template is composed of tokens, which in turn can represent plain text
/// or a named placeholder.
pub struct Template<'t> {
    tokens: Vec<Token<'t>>,
}

impl<'t> Template<'t> {
    /// Generates a Template with boundaries specified by the handlebars syntax,
    /// this means that within the string `"hello {{key}}"` we will have `key`
    /// as a named placeholder.
    pub fn new(text: &'t str) -> Self {
        Self {
            tokens: Parser::new(text, DEFAULT_START_PLACEHOLDER, DEFAULT_END_PLACEHOLDER).parse(),
        }
    }

    /// Generates a Template with boundaries specified by the `start` and `end`
    /// arguments
    pub fn new_with_placeholder(text: &'t str, start: &'t str, end: &'t str) -> Self {
        Self {
            tokens: Parser::new(text, start, end).parse(),
        }
    }

    /// Generates a string using the `replacements` HashMap to replace
    /// our named placeholders within our Template.
    pub fn fill_in(&self, replacements: &HashMap<&str, &str>) -> String {
        let mut result = String::new();

        for segment in &self.tokens {
            match segment {
                Token::Text(s) => result.push_str(s),
                Token::Placeholder(s) => {
                    let entry = replacements.get(s);
                    match entry {
                        Some(value) => result.push_str(value),
                        _ => {}
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Template;
    use std::collections::HashMap;

    #[test]
    fn test_single_fill_in_default_no_replacements() {
        let table = HashMap::new();

        assert_eq!(Template::new("hello world").fill_in(&table), "hello world");
    }

    #[test]
    fn test_single_fill_in_default_start() {
        let mut table = HashMap::new();
        table.insert("placeholder", "hello");

        assert_eq!(
            Template::new("{{placeholder}} world").fill_in(&table),
            "hello world"
        );
    }

    #[test]
    fn test_single_fill_in_default_middle() {
        let mut table = HashMap::new();
        table.insert("placeholder", "crazy");

        assert_eq!(
            Template::new("hello {{placeholder}} world").fill_in(&table),
            "hello crazy world"
        );
    }

    #[test]
    fn test_single_fill_in_default_end() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_in(&table),
            "hello world"
        );
    }

    #[test]
    fn test_multiple_fill_in_default() {
        let mut table = HashMap::new();
        table.insert("first", "one");
        table.insert("second", "two");
        table.insert("third", "three");

        assert_eq!(
            Template::new("{{first}} {{second}} {{third}}").fill_in(&table),
            "one two three"
        );
    }

    #[test]
    fn test_single_fill_in_default_incomplete_replacements_start() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder").fill_in(&table),
            "hello {{placeholder"
        );
    }

    #[test]
    fn test_single_fill_in_default_incomplete_replacements_end() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello placeholder}}").fill_in(&table),
            "hello placeholder}}"
        );
    }

    #[test]
    fn test_single_fill_in_default_missing_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_in(&table),
            "hello "
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_no_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new_with_placeholder("hello world", "[", "]").fill_in(&table),
            "hello world"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_start() {
        let mut table = HashMap::new();
        table.insert("placeholder", "hello");

        assert_eq!(
            Template::new_with_placeholder("[placeholder] world", "[", "]").fill_in(&table),
            "hello world"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_middle() {
        let mut table = HashMap::new();
        table.insert("placeholder", "crazy");

        assert_eq!(
            Template::new_with_placeholder("hello [placeholder] world", "[", "]").fill_in(&table),
            "hello crazy world"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_end() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new_with_placeholder("hello [placeholder]", "[", "]").fill_in(&table),
            "hello world"
        );
    }

    #[test]
    fn test_multiple_fill_in_with_placeholder() {
        let mut table = HashMap::new();
        table.insert("first", "one");
        table.insert("second", "two");
        table.insert("third", "three");

        assert_eq!(
            Template::new_with_placeholder("[first] [second] [third]", "[", "]").fill_in(&table),
            "one two three"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_incomplete_replacements_start() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new_with_placeholder("hello [placeholder", "[", "]").fill_in(&table),
            "hello [placeholder"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_incomplete_replacements_end() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new_with_placeholder("hello placeholder]", "[", "]").fill_in(&table),
            "hello placeholder]"
        );
    }

    #[test]
    fn test_single_fill_in_with_placeholder_missing_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new_with_placeholder("hello [placeholder]", "[", "]").fill_in(&table),
            "hello "
        );
    }
}

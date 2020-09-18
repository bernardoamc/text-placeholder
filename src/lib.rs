//! # A flexible text template engine
//!
//! ## Overview
//! Create templates with named placeholders within it.
//!
//! Placeholders are defined by default following the handlebars syntax,
//! but can be overriden with specific boundaries.
//!
//! This library supports passing a `HashMap` or `Struct` as a context
//! in order to replace the specified placeholders.
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
//!     assert_eq!(default_template.fill_with_hashmap(&table), "Hello text placeholder!");
//!
//!     // We can also specify our own boundaries:
//!
//!     let custom_template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");
//!
//!     assert_eq!(default_template.fill_with_hashmap(&table), "Hello text placeholder!");

mod parser;
use parser::{Parser, Token};

mod error;
use error::{Error, Result};

#[cfg(feature = "struct_context")]
extern crate serde_json;
#[cfg(feature = "struct_context")]
use serde::Serialize;

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
    /// arguments. Example:
    ///
    /// Template::new_with_placeholder("Hello [key]!", "[", "]");
    pub fn new_with_placeholder(text: &'t str, start: &'t str, end: &'t str) -> Self {
        Self {
            tokens: Parser::new(text, start, end).parse(),
        }
    }

    /// Fill the template's placeholders using the provided `replacements HashMap`
    /// in order to to infer values for the named placeholders.
    ///
    /// Placeholders without an associated value will be replaced with an empty string.
    ///
    /// For a version that generates an error in case a placeholder is missing see
    /// `fill_with_hashmap_strict`.
    pub fn fill_with_hashmap(&self, replacements: &HashMap<&str, &str>) -> String {
        let mut result = String::new();

        for segment in &self.tokens {
            match segment {
                Token::Text(s) => result.push_str(s),
                Token::Placeholder(s) => match replacements.get(s) {
                    Some(value) => result.push_str(value),
                    _ => {}
                },
            }
        }

        result
    }

    /// Fill the template's placeholders using the provided `replacements HashMap`
    /// in order to to infer values for the named placeholders.
    ///
    /// Placeholders without an associated value will result in a `Error::PlaceholderError`.
    ///
    /// For a version that does not generate an error in case a placeholder is missing see
    /// `fill_with_hashmap`.
    pub fn fill_with_hashmap_strict(&self, replacements: &HashMap<&str, &str>) -> Result<String> {
        let mut result = String::new();

        for segment in &self.tokens {
            match segment {
                Token::Text(s) => result.push_str(s),
                Token::Placeholder(s) => match replacements.get(s) {
                    Some(value) => result.push_str(value),
                    None => {
                        let message = format!("missing value for placeholder named '{}'.", s);
                        return Err(Error::PlaceholderError(message));
                    }
                },
            }
        }

        Ok(result)
    }

    #[cfg(feature = "struct_context")]
    /// Fill the template's placeholders using the provided `replacements struct`
    /// in order to to infer values for the named placeholders. The provided struct
    /// must implement `serde::Serialize`.
    ///
    /// Placeholders without an associated value or with values that cannot be converted
    /// to an str will be replaced with an empty string.
    ///
    /// For a version that generates an error in case a placeholder is missing see
    /// `fill_with_struct_strict`.
    pub fn fill_with_struct<R>(&self, replacements: &R) -> Result<String>
    where
        R: Serialize,
    {
        let mut result = String::new();
        let replacements = serde_json::to_value(replacements)?;

        for segment in &self.tokens {
            match segment {
                Token::Text(s) => result.push_str(s),
                Token::Placeholder(s) => match replacements.get(s) {
                    Some(value) => result.push_str(value.as_str().unwrap_or("")),
                    _ => {}
                },
            }
        }

        Ok(result)
    }

    #[cfg(feature = "struct_context")]
    /// Fill the template's placeholders using the provided `replacements struct`
    /// in order to to infer values for the named placeholders. The provided struct
    /// must implement `serde::Serialize`.
    ///
    /// Placeholders without an associated value or with values that cannot be converted
    /// to an str will result in a `Error::PlaceholderError`.
    ///
    /// For a version that does not generate an error in case a placeholder is missing see
    /// `fill_with_struct`.
    pub fn fill_with_struct_strict<R>(&self, replacements: &R) -> Result<String>
    where
        R: Serialize,
    {
        let mut result = String::new();
        let replacements = serde_json::to_value(replacements)?;

        for segment in &self.tokens {
            match segment {
                Token::Text(s) => result.push_str(s),
                Token::Placeholder(s) => match replacements.get(s) {
                    Some(value) => match value.as_str() {
                        Some(value) => result.push_str(value),
                        None => {
                            let message = format!("missing value for placeholder named '{}'.", s);
                            return Err(Error::PlaceholderError(message));
                        }
                    },
                    None => {
                        let message = format!("missing value for placeholder named '{}'.", s);
                        return Err(Error::PlaceholderError(message));
                    }
                },
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Template;
    use std::collections::HashMap;

    #[cfg(feature = "struct_context")]
    use serde::Serialize;

    // ---------------------
    // | fill_with_hashmap |
    // ---------------------
    #[test]
    fn test_hashmap_no_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new("hello world").fill_with_hashmap(&table),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_replacement_start_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "hello");

        assert_eq!(
            Template::new("{{placeholder}} world").fill_with_hashmap(&table),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_replacement_middle_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "crazy");

        assert_eq!(
            Template::new("hello {{placeholder}} world").fill_with_hashmap(&table),
            "hello crazy world"
        );
    }

    #[test]
    fn test_hashmap_replacement_end_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_with_hashmap(&table),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_multiple_replacements() {
        let mut table = HashMap::new();
        table.insert("first", "one");
        table.insert("second", "two");
        table.insert("third", "three");

        assert_eq!(
            Template::new("{{first}} {{second}} {{third}}").fill_with_hashmap(&table),
            "one two three"
        );
    }

    #[test]
    fn test_hashmap_missing_starting_boundaries() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello placeholder}}").fill_with_hashmap(&table),
            "hello placeholder}}"
        );
    }

    #[test]
    fn test_hashmap_missing_closing_boundaries() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder").fill_with_hashmap(&table),
            "hello {{placeholder"
        );
    }

    #[test]
    fn test_hashmap_missing_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_with_hashmap(&table),
            "hello "
        );
    }

    // ----------------------------
    // | fill_with_hashmap_strict |
    // ----------------------------

    #[test]
    fn test_hashmap_strict_no_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new("hello world")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_strict_replacement_start_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "hello");

        assert_eq!(
            Template::new("{{placeholder}} world")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_strict_replacement_middle_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "crazy");

        assert_eq!(
            Template::new("hello {{placeholder}} world")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello crazy world"
        );
    }

    #[test]
    fn test_hashmap_strict_replacement_end_line() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder}}")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_hashmap_strict_multiple_replacements() {
        let mut table = HashMap::new();
        table.insert("first", "one");
        table.insert("second", "two");
        table.insert("third", "three");

        assert_eq!(
            Template::new("{{first}} {{second}} {{third}}")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "one two three"
        );
    }

    #[test]
    fn test_hashmap_strict_missing_starting_boundaries() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello placeholder}}")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello placeholder}}"
        );
    }

    #[test]
    fn test_hashmap_strict_missing_closing_boundaries() {
        let mut table = HashMap::new();
        table.insert("placeholder", "world");

        assert_eq!(
            Template::new("hello {{placeholder")
                .fill_with_hashmap_strict(&table)
                .unwrap(),
            "hello {{placeholder"
        );
    }

    #[test]
    fn test_hashmap_strict_missing_replacements() {
        let table = HashMap::new();

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_with_hashmap_strict(&table).map_err(|e| e.to_string()),
            Err("Error while replacing placeholder. Reason: missing value for placeholder named 'placeholder'.".to_owned())
        );
    }

    // --------------------
    // | fill_with_struct |
    // --------------------

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_no_replacements() {
        #[derive(Serialize)]
        struct Context {}
        let context = Context {};

        assert_eq!(
            Template::new("hello world")
                .fill_with_struct(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_replacement_start_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "hello".to_string(),
        };

        assert_eq!(
            Template::new("{{placeholder}} world")
                .fill_with_struct(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_replacement_middle_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "crazy".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}} world")
                .fill_with_struct(&context)
                .unwrap(),
            "hello crazy world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_replacement_end_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}}")
                .fill_with_struct(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_multiple_replacements() {
        #[derive(Serialize)]
        struct Context {
            first: String,
            second: String,
            third: String,
        }
        let context = Context {
            first: "one".to_string(),
            second: "two".to_string(),
            third: "three".to_string(),
        };

        assert_eq!(
            Template::new("{{first}} {{second}} {{third}}")
                .fill_with_struct(&context)
                .unwrap(),
            "one two three"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_missing_starting_boundaries() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello placeholder}}")
                .fill_with_struct(&context)
                .unwrap(),
            "hello placeholder}}"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_missing_closing_boundaries() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder")
                .fill_with_struct(&context)
                .unwrap(),
            "hello {{placeholder"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_missing_replacements() {
        #[derive(Serialize)]
        struct Context {
            different: String,
        }
        let context = Context {
            different: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}}")
                .fill_with_struct(&context)
                .unwrap(),
            "hello "
        );
    }

    // ---------------------------
    // | fill_with_struct_strict |
    // ---------------------------

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_no_replacements() {
        #[derive(Serialize)]
        struct Context {}
        let context = Context {};

        assert_eq!(
            Template::new("hello world")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_replacement_start_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "hello".to_string(),
        };

        assert_eq!(
            Template::new("{{placeholder}} world")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_replacement_middle_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "crazy".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}} world")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello crazy world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_replacement_end_line() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}}")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello world"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_multiple_replacements() {
        #[derive(Serialize)]
        struct Context {
            first: String,
            second: String,
            third: String,
        }
        let context = Context {
            first: "one".to_string(),
            second: "two".to_string(),
            third: "three".to_string(),
        };

        assert_eq!(
            Template::new("{{first}} {{second}} {{third}}")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "one two three"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_missing_starting_boundaries() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello placeholder}}")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello placeholder}}"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_missing_closing_boundaries() {
        #[derive(Serialize)]
        struct Context {
            placeholder: String,
        }
        let context = Context {
            placeholder: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder")
                .fill_with_struct_strict(&context)
                .unwrap(),
            "hello {{placeholder"
        );
    }

    #[cfg(feature = "struct_context")]
    #[test]
    fn test_struct_strict_missing_replacements() {
        #[derive(Serialize)]
        struct Context {
            different: String,
        }
        let context = Context {
            different: "world".to_string(),
        };

        assert_eq!(
            Template::new("hello {{placeholder}}").fill_with_struct_strict(&context).map_err(|e| e.to_string()),
            Err("Error while replacing placeholder. Reason: missing value for placeholder named 'placeholder'.".to_owned())
        );
    }
}

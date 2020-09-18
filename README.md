# Text Placeholder

A flexible text template engine that allows templates with named placeholders within it.

Placeholders are defined by default following the handlebars syntax, but can be overriden
with specific boundaries.

In order to provide the context in which our placeholders will be replaced the following
options are available:

- HashMap
- Struct as an optional feature.

## HashMap

As the name implies we can pass a `HashMap` in order to provide the context. The
methods available are:

- `fill_with_hashmap` which replaces empty placeholders with an empty string.
- `fill_with_hashmap_strict` which returns a `Error::PlaceholderError` when:
  - the provided context does not contain the placeholder
  - the provided value for a placeholder cannot be converted to a string

### Example

```rust
use text_placeholder::Template;
use std::collections::HashMap;

let default_template = Template::new("Hello {{first}} {{second}}!");

let mut table = HashMap::new();
table.insert("first", "text");
table.insert("second", "placeholder");

assert_eq!(default_template.fill_with_hashmap(&table), "Hello text placeholder!");

// We can also specify our own boundaries:

let custom_template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");

assert_eq!(default_template.fill_with_hashmap(&table), "Hello text placeholder!");
```

## Struct

This is an optional feature that depends on `serde`. In order to enable it specify in your `Cargo.toml` dependencies the following:

```toml
text_placeholder = { version = "0.2", features = ["struct_context"] }
```

As the name implies we can pass a `Struct` in order that implements the `serde::Serialize` trait in order to provide the context. The methods available are:

- `fill_with_struct` which replaces empty placeholders with an empty string.
- `fill_with_struct_strict` which returns a `Error::PlaceholderError` when:
  - the provided context does not contain the placeholder
  - the provided value for a placeholder cannot be converted to a string

### Example

```rust
use text_placeholder::Template;

#[derive(Serialize)]
struct Context {
    first: String,
    second: String
}

let default_template = Template::new("Hello {{first}} {{second}}!");
let context = Context { first: "text".to_string(), second: "placeholder".to_string() };

assert_eq!(default_template.fill_with_struct(&context), "Hello text placeholder!");

// We can also specify our own boundaries:

let custom_template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");

assert_eq!(default_template.fill_with_struct(&context), "Hello text placeholder!");
```

## Roadmap

- Allow objects that implement trait `std::ops::Index` instead of depending directly on a HashMap.

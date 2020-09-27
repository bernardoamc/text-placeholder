# Text Placeholder

A flexible text template engine that allows templates with named placeholders within it.

```rust
let template = Template::new("Hello {{first}} {{second}}!");
```

Placeholders are defined by default following the handlebars syntax, but can be overriden
with specific boundaries.

```rust
let template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");
```

In order to provide the context in which our placeholders will be replaced the following
options are available:

- HashMap.
- Struct, as an **optional** feature.

## HashMap

The following methods are available with a `HashMap`:

- `fill_with_hashmap`
  - replaces missing placeholders with an empty string.
  - replaces placeholders that cannot be converted to a strint with an empty string.
- `fill_with_hashmap_strict` which returns a `Error::PlaceholderError` when:
  - a placeholder is missing.
  - a placeholder value cannot be converted to a string.

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

Allow structs that implement the `serde::Serialize` trait to be used as context.

This is an optional feature that depends on `serde`. In order to enable it add the following to your `Cargo.toml` file:

```toml
[dependencies]
text_placeholder = { version = "0.3", features = ["struct_context"] }
```

The methods available are:

- `fill_with_struct`
  - replaces missing placeholders with an empty string.
  - replaces placeholders that cannot be converted to a strint with an empty string.
- `fill_with_struct_strict` which returns a `Error::PlaceholderError` when:
  - a placeholder is missing.
  - a placeholder value cannot be converted to a string.

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

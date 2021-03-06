# Text Placeholder

A minimal text template engine that allows named placeholders within your templates.

There are two necessary pieces in order to parse a template:

- Placeholders
- Context

## Placeholders

Placeholders are defined within certain boundaries and will be replaced once the template is parsed.

Let's define a template with placeholders named `first` and `second`:

```rust
let template = Template::new("Hello {{first}} {{second}}!");
```

Templates use the handlebars syntax as boundaries by default, but can be overridden:

```rust
let template = Template::new_with_placeholder("Hello $[first] $[second]!", "$[", "]");
```

## Context

Context is the data structure that will be used to replace your placeholders with real data.

You can think of your placeholder as a key within a `HashMap` or the name of a field within a
`struct`. In fact, these are the two types of context supported by this library:

- HashMap.
- Struct, as an **optional** feature.

### HashMap

Each placeholder should be a `key` with an associated `value` that can be converted into a `str`.

The following methods are available with a `HashMap`:

- `fill_with_hashmap`
  - replaces missing placeholders with an empty string.
  - replaces placeholders that cannot be converted to a strint with an empty string.
- `fill_with_hashmap_strict` which returns a `Error::PlaceholderError` when:
  - a placeholder is missing.
  - a placeholder value cannot be converted to a string.

#### Example

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

### Struct

Allow structs that implement the `serde::Serialize` trait to be used as context.

This is an optional feature that depends on `serde`. In order to enable it add the following to your `Cargo.toml` file:

```toml
[dependencies]
text_placeholder = { version = "0.4", features = ["struct_context"] }
```

Each placeholder should be a `field` in your `struct` with an associated `value` that can be converted into a `str`.

The following methods are available with a `struct`:

- `fill_with_struct`
  - replaces missing placeholders with an empty string.
  - replaces placeholders that cannot be converted to a strint with an empty string.
- `fill_with_struct_strict` which returns a `Error::PlaceholderError` when:
  - a placeholder is missing.
  - a placeholder value cannot be converted to a string.

#### Example

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

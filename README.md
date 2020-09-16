# Text Placeholder

A flexible text template engine that allows templates with named placeholders within it.

Placeholders are defined by default following the handlebars syntax, but can be overriden
with specific boundaries.

In order to provide the context in which our placeholders will be replaced the following
options are available:

- Provide a HashMap and use `fill_with_hashmap` or `fill_with_hashmap_strict`
- Provide a Struct and use `fill_with_struct` or `fill_with_struct_strict`
  - :warning: This is an optional feature that depends on `serde` and `serde_json` :warning:

## Example

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

## Roadmap

- Allow objects that implement trait `std::ops::Index` instead of depending on a HashMap.

_This project is inspired by the awesome [text-template](https://gitlab.com/boeckmann/text-template) repository._

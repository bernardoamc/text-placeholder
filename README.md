# Text Placeholder

A flexible text template engine that allows templates with named placeholders within it.

Placeholders are defined by default following the handlebars syntax, but can be overriden
with specific boundaries.

## Example

```rust
use text_placeholder::Template;
use std::collections::HashMap;

let default_template = Template::new("Hello {{first}} {{second}}!");

let mut table = HashMap::new();
table.insert("first", "text");
table.insert("second", "placeholder");

assert_eq!(default_template.fill_in(&table), "Hello text placeholder!");

// We can also specify our own boundaries:

let custom_template = Template::new_with_placeholder("Hello $[first]] $[second]!", "$[", "]");

assert_eq!(default_template.fill_in(&table), "Hello text placeholder!");
```

## Roadmap

- Allow named arguments to be resolved through a struct.
- Implement a `fill_in` that returns errors when a named argument cannot be found.

_This project is based on the awesome [text-template](https://gitlab.com/boeckmann/text-template) repository._

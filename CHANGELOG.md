# Changelog

## Version 0.1.2 (27/07/2019)

- Imports no longer forced on any file including `#[derive(Tablefy)]`
  - This comes courtesy of an update in the [tablefy_derive](tablefy_derive) crate.
- `get_headers()` and `into_vec()` _(previously `into_row()`)_ now return a vector of strings rather than a `Row`.
- Added `into_string(data)` function in the case that a formatted `String` is all that is required. Helps to avoid dealing with a `Table` struct directly.
- Updated some doc examples to reflect above changes.
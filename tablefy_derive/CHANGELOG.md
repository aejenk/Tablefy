# Changelog

## Version 0.1.1 (27/07/2019)

- Changed functions to return `Vec<String>` instead of `Row`.
  - Serves to abstract the functions from the library.
  - As a result, the responsibility of Row/Cell construction can now 
    be handed off to `tablefy`.

- Functions now locally import the necessary parts of `prettytable`.
  - The benefit of this is that the `use` on files using the `Derive`
    is now no longer necessary. It also avoids forcing a `use`
    if not required.

- Non-Option fields are not explicitly formatted.
  - Before this was done in the `row!` macro. This prepares the macro
    for a future `Debug`/`Pretty` update.
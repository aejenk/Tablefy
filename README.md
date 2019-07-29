# tablefy

[![tablefy](https://img.shields.io/crates/v/tablefy.svg)](https://crates.io/crates/tablefy)
[![tablefy](https://docs.rs/tablefy/badge.svg)](https://docs.rs/crate/tablefy)

To check for updates see [the changelog](CHANGELOG.md).

```rust
use tablefy_derive::Tablefy;
use tablefy::Tablefy;

// This struct now implements the tablefy trait
#[derive(Tablefy)]
pub struct Basic {

    #[header(name = "Hmm... Nice Header")]
    pub something: String,

    #[header(name = "We Have Here!")]
    pub otherthing: i16,

    #[header(name = "Don't You Agree?")]
    pub newthing: i8,

    pub maybe: Option<String>
}

fn main() {
    // Creating a vector of structs...
    let basic = vec![Basic {
        something: String::from("a"),
        otherthing: 2,
        newthing: 3,
        maybe: None
    }, Basic {
        something: String::from("b"),
        otherthing: 3,
        newthing: 4,
        maybe: Some(String::from("x"))
    }, Basic {
        something: String::from("c"),
        otherthing: 5,
        newthing: 8,
        maybe: None
    }];

    // Turning them into a Table struct...
    let table = tablefy::into_table(&basic);

    // Or if you just want the string...
    println!("{}", tablefy::into_string(&basic));
}
```

```rust
+--------------------+---------------+------------------+-------+
| Hmm... Nice Header | We Have Here! | Don't You Agree? | maybe |
+====================+===============+==================+=======+
| a                  | 2             | 3                |       |
+--------------------+---------------+------------------+-------+
| b                  | 3             | 4                | x     |
+--------------------+---------------+------------------+-------+
| c                  | 5             | 8                |       |
+--------------------+---------------+------------------+-------+
```
This crate serves as an extension of [`prettytable`](https://docs.rs/prettytable-rs/0.8.0/prettytable/)
by specifying a `Tablefy` trait in order to turn any struct (whose members implement Display) to turn into
a [`Table`](https://docs.rs/prettytable-rs/0.8.0/prettytable/struct.Table.html) object.

As a result, this means that `prettytable` is a dependency. You won't be able to use this crate without
also adding `prettytable`.

If you'd like to get the full functionality of this crate *(with proc_macros)*, be sure to check out
[tablefy_derive](tablefy_derive)!.

### Future updates
Currently there are two major improvements I have in mind for this crate.

- ~~Fields can be tagged to customize the header name.~~
    - This has now been implemented! Be sure to update `tablefy_derive`.
- Fields can be tagged to print using `{:?}` and `{:#?}` instead of `{}`

License: MPL-2.0

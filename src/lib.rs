//! ```
//! use tablefy_derive::Tablefy;
//! use tablefy::Tablefy;
//! 
//! // This struct now implements the tablefy trait
//! #[derive(Tablefy)]
//! pub struct Basic {
//! 
//!     #[header(name = "Hmm... Nice Header")]
//!     pub something: String,
//! 
//!     #[header(name = "We Have Here!")]
//!     pub otherthing: i16,
//! 
//!     #[header(name = "Don't You Agree?")]
//!     pub newthing: i8,
//! 
//!     pub maybe: Option<String>
//! }
//! 
//! fn main() {
//!     // Creating a vector of structs...
//!     let basic = vec![Basic {
//!         something: String::from("a"),
//!         otherthing: 2,
//!         newthing: 3,
//!         maybe: None
//!     }, Basic {
//!         something: String::from("b"),
//!         otherthing: 3,
//!         newthing: 4,
//!         maybe: Some(String::from("x"))
//!     }, Basic {
//!         something: String::from("c"),
//!         otherthing: 5,
//!         newthing: 8,
//!         maybe: None
//!     }];
//! 
//!     // Turning them into a Table struct...
//!     let table = tablefy::into_table(&basic);
//! 
//!     // Or if you just want the string...
//!     println!("{}", tablefy::into_string(&basic));
//! }
//! ```
//! 
//! ```
//! +--------------------+---------------+------------------+-------+
//! | Hmm... Nice Header | We Have Here! | Don't You Agree? | maybe |
//! +====================+===============+==================+=======+
//! | a                  | 2             | 3                |       |
//! +--------------------+---------------+------------------+-------+
//! | b                  | 3             | 4                | x     |
//! +--------------------+---------------+------------------+-------+
//! | c                  | 5             | 8                |       |
//! +--------------------+---------------+------------------+-------+
//! ```
//! This crate serves as an extension of [`prettytable`](https://docs.rs/prettytable-rs/0.8.0/prettytable/)
//! by specifying a `Tablefy` trait in order to turn any struct (whose members implement Display) to turn into
//! a [`Table`](https://docs.rs/prettytable-rs/0.8.0/prettytable/struct.Table.html) object.
//! 
//! As a result, this means that `prettytable` is a dependency. You won't be able to use this crate without 
//! also adding `prettytable`. 
//! 
//! If you'd like to get the full functionality of this crate *(with proc_macros)*, be sure to check out
//! [tablefy_derive](tablefy_derive)!.
//! 
//! ## Future updates
//! Currently there are two major improvements I have in mind for this crate.
//! 
//! - ~~Fields can be tagged to customize the header name.~~
//!     - This has now been implemented! Be sure to update `tablefy_derive`.
//! - Fields can be tagged to print using `{:?}` and `{:#?}` instead of `{}`

use prettytable::{Table, Row, Cell};

/// The main trait of the library. Has two main functions with which a table can be constructed.
pub trait Tablefy {
    /// Retrieves the headers of the table.
    /// 
    /// If derived, the headers will be the field names of the struct.
    /// Currently custom names aren't supported, but they may be implemented in the future.
    fn get_headers() -> Vec<String>;

    /// Turns the contents of a struct into a vector of strings.
    /// 
    /// If derived, all the contents are saved as a `String`. This is to facilitate displaying
    /// as a full table. However, in order for the derivation to work, all the fields of the struct
    /// must implement the `Display` trait. Otherwise the code won't compile.
    /// 
    /// ```
    /// struct Thing {
    ///     name : String,
    ///     age : i8,
    ///     location : String
    /// }
    /// 
    /// ...
    /// 
    /// 
    /// let items = thing.into_vec();
    /// ```
    fn into_vec(&self) -> Vec<String>;
}

/// Function that turns a vector of Tablefy implementations into a full Table.
/// 
/// This Table comes from [`prettytable`](https://docs.rs/prettytable-rs/0.8.0/prettytable/struct.Table.html), any function supported by that crate will also work here.
/// 
/// ```
/// let table = into_table(servers);
/// 
/// table.printstd();
/// let tablestr = format!("{}", table);
/// ```
pub fn into_table<T : Tablefy> (data: &Vec<T>) -> Table {
    let mut table = Table::new();

    table.set_titles(vec_to_row(T::get_headers()));

    for obj in data {
        table.add_row(vec_to_row(obj.into_vec()));
    }

    table
}

/// Function that turns a vector of Tablefy implementations into a formatted string.
/// 
/// Purpose of this is to abstract the `Table` from the main program. Especially
/// useful if all you need is the string itself.
pub fn into_string<T : Tablefy> (data: &Vec<T>) -> String {
    into_table(data).to_string()
}

/// Converts a vector into a Row.
/// Meant to be used internally to facilitate `Table` construction.
fn vec_to_row(data: Vec<String>) -> Row {
    Row::new(data.iter().map(|s| Cell::new(s)).collect())
}
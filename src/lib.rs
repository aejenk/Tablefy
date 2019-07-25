//! ```
//! use tablefy_derive::Tablefy;
//! use tablefy::Tablefy;
//! use prettytable::{cell, row, Row};
//!
//! // This struct now implements the tablefy trait
//! #[derive(Tablefy)]
//! pub struct Basic {
//!     pub something: String,
//!     pub otherthing: i16,
//!     pub newthing: i8,
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
//!     }];
//!
//!     // Turning them into a Table struct...
//!     let table = tablefy::into_table(basic);
//!
//!     // ...and printing the output! Table implements Display.
//!     println!("{}", table);
//! }
//! ```
//! 
//! ```
//! +-----------+------------+----------+-------+
//! | something | otherthing | newthing | maybe |
//! +===========+============+==========+=======+
//! | a         | 2          | 3        |       |
//! +-----------+------------+----------+-------+
//! | b         | 3          | 4        | x     |
//! +-----------+------------+----------+-------+
//! ```
//! This crate serves as an extension of [`prettytable`](https://docs.rs/prettytable-rs/0.8.0/prettytable/)
//! by specifying a `Tablefy` trait in order to turn any struct (whose members implement Display) to turn into
//! a [`Table`](https://docs.rs/prettytable-rs/0.8.0/prettytable/struct.Table.html) object.
//! 
//! As a result, this means that `prettytable` is a dependency. You won't be able to use this crate without 
//! also adding `prettytable`.
//! 
//! ## Future updates
//! Currently there are two major improvements I have in mind for this crate.
//! 
//! - Fields can be tagged to customize the header name.
//! - Fields can be tagged to print using `{:?}` and `{:#?}` instead of `{}`

use prettytable::{Table, Row};

/// The main trait of the library. Has two main functions with which a table can be constructed.
pub trait Tablefy {
    /// Retrieves the headers of the table.
    /// 
    /// If derived, the headers will be the field names of the struct.
    /// Currently custom names aren't supported, but they may be implemented in the future.
    fn get_headers() -> Row;

    /// Turns the contents of a struct into a row.
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
    /// ```
    fn into_row(&self) -> Row;
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
pub fn into_table<T : Tablefy> (data: Vec<T>) -> Table {
    let mut table = Table::new();

    table.set_titles(T::get_headers());

    for obj in data {
        table.add_row(obj.into_row());
    }

    table
}
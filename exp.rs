#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use tablefy_derive::Tablefy;
mod lib {
    use prettytable::{Row, Table};
    pub trait Tablefy {
        fn get_headers() -> Row;
        fn into_row(&self) -> Row;
    }
    pub fn tablefy<T: Tablefy>(data: Vec<T>) -> String {
        let mut table = Table::new();
        table.set_titles(T::get_headers());
        for obj in data {
            table.add_row(obj.into_row());
        }
        ::alloc::fmt::format(::std::fmt::Arguments::new_v1(
            &[""],
            &match (&table,) {
                (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
            },
        ))
    }
}
use lib::Tablefy;
use prettytable::{cell, row, Row};
pub struct Basic {
    pub something: String,
    pub otherthing: i16,
    pub newthing: i8,
    pub maybe: Option<String>,
}
impl Tablefy for Basic {
    fn get_headers() -> Row {
        return ::prettytable::Row::new(<[_]>::into_vec(box [
            ::prettytable::Cell::new(&"something".to_string()),
            ::prettytable::Cell::new(&"otherthing".to_string()),
            ::prettytable::Cell::new(&"newthing".to_string()),
            ::prettytable::Cell::new(&"maybe".to_string()),
        ]));
    }
    fn into_row(&self) -> Row {
        let something = &self.something;
        let otherthing = &self.otherthing;
        let newthing = &self.newthing;
        let maybe = if let Some(x) = &self.maybe {
            ::alloc::fmt::format(::std::fmt::Arguments::new_v1(
                &[""],
                &match (&x,) {
                    (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                },
            ))
        } else {
            String::new()
        };
        ::prettytable::Row::new(<[_]>::into_vec(box [
            ::prettytable::Cell::new(&something.to_string()),
            ::prettytable::Cell::new(&otherthing.to_string()),
            ::prettytable::Cell::new(&newthing.to_string()),
            ::prettytable::Cell::new(&maybe.to_string()),
        ]))
    }
}
fn main() {
    let bsic = <[_]>::into_vec(box [
        Basic {
            something: String::from("a"),
            otherthing: 2,
            newthing: 3,
            maybe: None,
        },
        Basic {
            something: String::from("b"),
            otherthing: 3,
            newthing: 4,
            maybe: Some("x"),
        },
    ]);
}

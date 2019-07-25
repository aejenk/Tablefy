use tablefy_derive::Tablefy;
mod lib;
use lib::{Tablefy, tablefy};
use prettytable::{cell, row, Row};

#[derive(Tablefy)]
pub struct Basic {
    pub something: String,
    pub otherthing: i16,
    pub newthing: i8,
    pub maybe: Option<String>
}

fn main() {
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
    }];

    let table = tablefy(basic);

    println!("{}", table);
}
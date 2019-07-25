use tablefy_derive::Tablefy;
use tablefy::Tablefy;
use prettytable::{cell, row, Row};

// This struct now implements the tablefy trait
#[derive(Tablefy)]
pub struct Basic {
    pub something: String,
    pub otherthing: i16,
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
    }];

    // Turning them into a Table struct...
    let table = tablefy::into_table(basic);

    // ...and printing the output! Table implements Display.
    println!("{}", table);
}
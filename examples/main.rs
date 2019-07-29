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
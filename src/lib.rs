use prettytable::{Table, Row};

pub trait Tablefy {
    fn get_headers() -> Row;
    fn into_row(&self) -> Row;
}

pub fn tablefy<T : Tablefy> (data: Vec<T>) -> String {
    let mut table = Table::new();

    table.set_titles(T::get_headers());

    for obj in data {
        table.add_row(obj.into_row());
    }

    format!("{}", table)
}
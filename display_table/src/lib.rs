use std::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}

impl Table {
    pub fn new() -> Table {

    }
    pub fn add_row(&mut self, row: &[String]) {

    }
}

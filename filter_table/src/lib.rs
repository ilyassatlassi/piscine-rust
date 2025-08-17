#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Default::default(),
            body: Default::default(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col<T>(&self, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let mut index = 0;
        let mut new_table = Table::new();
        for i in 0..self.headers.len() {
            if filter(self.headers[i].as_str()) {
                index = i;
                new_table.headers.push(self.headers[i].to_string());
            }
        }
        for i in 0..self.body.len() {
            for j in 0..self.headers.len() {
                if j == index {
                    new_table.add_row(&self.body[i][index..j+1]);
                }
            }
        }
        Some(new_table)
    }

    pub fn filter_row<T>(&self, col_name: &str, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let mut index = 0;
        let mut new_table = Table::new();
        for i in 0..self.headers.len() {
            if col_name == self.headers[i] {
                index = i;
            }
            new_table.headers.push(self.headers[i].to_string());
        }
        for i in 0..self.body.len() {
            if filter(&self.body[i][index]) {
                new_table.add_row(&self.body[i][0..self.headers.len()]);
            }
        }
        Some(new_table)
    }
}

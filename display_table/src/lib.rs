use std::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.body.len() == 0 {
            return Ok(());
        }
        let mut max_for_each_column = vec![0; self.headers.len()];
        // let mut length = self.body.len();
        for j in 0..self.headers.len() {
            for i in 0..self.body.len() {
                if max_for_each_column[j] < self.body[i][j].len() {
                    max_for_each_column[j] = self.body[i][j].len();
                    // write!(f, "{:?} ", max_for_each_column)?
                }
            }
        }

        for i in 0..self.headers.len() {
            if i > 0 && i < self.headers.len() - 1 {
                write!(f, " | ")?;
            } else {
                write!(f, "| ")?;
            }

            if max_for_each_column[i] < self.headers[i].len() + 2 {
                max_for_each_column[i] = self.headers[i].len() + 2;
            }
            let mut diff = 0;
            if max_for_each_column[i] > self.headers[i].len() {
                diff = max_for_each_column[i] - self.headers[i].len();
                if diff % 2 == 0 {
                    for _ in 0..diff / 2 {
                        write!(f, " ")?;
                    }

                    write!(f, "{}", self.headers[i])?;

                    for _ in 0..diff / 2 {
                        write!(f, " ")?;
                    }
                } else {
                    write!(f, "{}", self.headers[i])?;

                    for _ in 0..diff {
                        write!(f, " ")?;
                    }
                }
            } else {
                write!(f, "{}", self.headers[i])?;
            }
        }
        writeln!(f, " |")?;
        for row in &self.body {
            for i in 0..row.len() {
                if i > 0 && i < row.len() - 1 {
                    write!(f, " | ")?;
                } else {
                    write!(f, "| ")?;
                }

                if max_for_each_column[i] < row[i].len() {
                    max_for_each_column[i] = row[i].len();
                }
                let mut diff = 0;
                if max_for_each_column[i] > row[i].len() {
                    diff = max_for_each_column[i] - row[i].len();
                    if diff % 2 == 0 {
                        for _ in 0..diff / 2 {
                            write!(f, " ")?;
                        }

                        write!(f, "{}", row[i])?;

                        for _ in 0..diff / 2 {
                            write!(f, " ")?;
                        }
                    } else {
                        write!(f, "{}", row[i])?;

                        for _ in 0..diff {
                            write!(f, " ")?;
                        }
                    }
                } else {
                    write!(f, "{}", row[i])?;
                }
            }

            writeln!(f, " |")?;
        }

                    write!(f, "{:?} ", max_for_each_column)?;
        // writeln!(f, "|---------+-------------+-----------+-------------|")?;
        Ok(())
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            // headers:vec!["Model".to_string(), "Piece N°".to_string(),"In Stock".to_string(),"Description".to_string()  ],
            headers: Default::default(),
            body: Default::default(),
        }
    }
    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

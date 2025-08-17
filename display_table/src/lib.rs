use std::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if  self.headers.len() == 0 {
            return Ok(());
        }

        // Calculate max column widths (including headers)
        let mut max_for_each_column = vec![0; self.headers.len()];
        for j in 0..self.headers.len() {
            max_for_each_column[j] = self.headers[j].len();
            for row in &self.body {
                if max_for_each_column[j] < row[j].len() {
                    max_for_each_column[j] = row[j].len();
                }
            }
          
        }

        // Print headers with centered alignment
        for i in 0..self.headers.len() {
            write!(f, "| ")?;
            let diff = max_for_each_column[i] - self.headers[i].chars().count();

            let left_pad = diff / 2;
            let right_pad = diff - left_pad;

            // Print left padding
            for _ in 0..left_pad {
                write!(f, " ")?;
            }

            // Print header
            write!(f, "{}", self.headers[i])?;

            // Print right padding
            for _ in 0..right_pad {
                write!(f, " ")?;
            }

            write!(f, " ")?; // Extra space before next column
        }
        writeln!(f, "|")?;

        // Print separator line
        write!(f, "|")?;
        for i in 0..self.headers.len() {
            if i > 0 {
                write!(f, "+")?;
            }
            for _ in 0..max_for_each_column[i] + 2 {
                // +2 for padding
                write!(f, "-")?;
            }
        }
        writeln!(f, "|")?;

        // Print body rows with centered alignment
        for row in &self.body {
            for i in 0..row.len() {
                write!(f, "| ")?;
                let diff = max_for_each_column[i] - row[i].len();

                let left_pad = diff / 2;
                let right_pad = diff - left_pad;

                // Print left padding
                for _ in 0..left_pad {
                    write!(f, " ")?;
                }

                // Print cell content
                write!(f, "{}", row[i])?;

                // Print right padding
                for _ in 0..right_pad {
                    write!(f, " ")?;
                }

                write!(f, " ")?; // Extra space before next column
            }
            writeln!(f, "|")?;
        }

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

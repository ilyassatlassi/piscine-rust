#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut mat = Matrix(Default::default());
        for i in 0..slice.len() {
            // println!("{}", i);
            let mut sub_vec = Vec::new();
            for j in 0..slice[i].len() {
                sub_vec.push(slice[i][j]);
            }
            mat.0.push(sub_vec);
        }
        mat
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    //         for i in 0..self.0.len() {
    //             write!(f, "(")?;
    //             for j in 0..self.0[i].len() {
    //                 if j > 0 {
    //                     write!(f, " ")?;
    //                 }
    //                 if j <  self.0[i].len(){
    //                     write!(f, "{}", self.0[i][j])?;
    //                 }
    //             }
    //             if i < self.0.len() - 1 {
    //                 writeln!(f, ")")?;
    //             }else {

    //                 write!(f, ")")?;
    //             }
    //         }
    //         Ok(())
    //     // todo!()
    // }
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     for i in 0..self.0.len() {
    //         write!(f, "(")?;
    //         for j in 0..self.0[i].len() {
    //             if j > 0 {
    //                 write!(f, " ")?;
    //             }
    //             if j <  self.0[i].len(){
    //                 write!(f, "{}", self.0[i][j])?;
    //             }
    //         }
    //         if i < self.0.len() - 1 {
    //             writeln!(f, ")")?;
    //         }else {

    //             write!(f, ")")?;
    //         }
    //     }
    //     Ok(())
    // }
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     writeln!(&self.0[0][0], {})
    // }
}

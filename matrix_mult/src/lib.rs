pub mod matrix;
pub use matrix::*;
use std::{ ops::{Add, Mul}, process::Output};

use lalgebra_scalar::*;

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = Vec::new();
        for i in 0..self.0.len() {
            res.push(self.0[i][n].clone());
        }
        res
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, other: Self) -> Self::Output {
        let row_lenght = self.number_of_rows();
        let col_lenght = other.number_of_cols();
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }
        let mut mul = Matrix::zero(row_lenght, col_lenght);
        for i in 0..row_lenght {
            let row = self.row(i);
            for j in 0..col_lenght {
                let col = other.col(j);
                let mut sum = T::zero();

                for k in 0..row_lenght {
                   sum = sum + row[k].clone() * col[k].clone();
                }
                mul.0[i][j] = sum;
            }
        }
        Some(mul)
    }
}

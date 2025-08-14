use lalgebra_scalar::Scalar;
// use matrix::Matrix;
// use crate::{Matrix, Scalar};
// #[derive(Debug, Eq, PartialEq, Clone)]
// pub struct Matrix<T>(pub Vec<Vec<T>>);

pub mod matrix;
pub use matrix::*;

use std::ops::{Add, Sub};

impl<T: Scalar<Item = T> + Add<Output = T> + std::fmt::Debug> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }
        let mut matrix = Matrix::new();

        for i in 0..self.0.len() {
            if i > 0 {
                matrix.0.push(Vec::new());
            }
            for j in 0..self.0[i].len() {
                let sum = self.0[i][j].clone() + other.0[i][j].clone();
                // println!("{:?}", sum);
                // println!("{}", matrix.0.len());
                matrix.0[i].push(sum);
            }
        }
        Some(matrix)
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }
        let mut matrix = Matrix::new();

        for i in 0..self.0.len() {
            if i > 0 {
                matrix.0.push(Vec::new());
            }
            for j in 0..self.0[i].len() {
                let sum = self.0[i][j].clone() - other.0[i][j].clone();

                matrix.0[i].push(sum);
            }

        }
        Some(matrix)
    }
}

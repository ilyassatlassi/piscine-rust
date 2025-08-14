use lalgebra_scalar::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut res = Matrix(vec![vec![T::zero(); n]; n]);
        for i in 0..n {
            res.0[i][i] = T::one();
        }
        res
    }
}

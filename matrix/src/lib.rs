use lalgebra_scalar::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::from(vec![]))
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
#[allow(dead_code)]
fn main() {
    let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    println!("{:?}", m);
    println!("{:?}", Matrix::<i32>::identity(4));
    println!("{:?}", Matrix::<f64>::zero(3, 4));
}

#[test]
fn zero_property() {
    let matrix: Matrix<u32> = Matrix::zero(3, 4);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::zero(2, 2);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(matrix, expected);
}

#[test]
fn identity_matrix() {
    let matrix: Matrix<u32> = Matrix::identity(2);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::identity(3);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(matrix, expected);
}

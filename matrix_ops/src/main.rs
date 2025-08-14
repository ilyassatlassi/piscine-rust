// Importing Matrix by defining it as a dependency in Cargo.toml
// use matrix::*;
use matrix_ops::*;
// use matrix::Matrix;
fn main() {
	let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);
}

#[test]
fn addition() {
    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
    assert_eq!(matrix + matrix_2, Some(expected));

    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    assert_eq!(matrix + matrix_2, None);
}

#[test]
fn subtraction() {
    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(matrix - matrix_2, Some(expected));

    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    assert_eq!(matrix - matrix_2, None);
}
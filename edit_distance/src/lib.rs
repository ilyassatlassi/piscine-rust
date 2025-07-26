pub fn edit_distance(source: &str, target: &str) -> usize{
let m = source.chars().count() + 1;
let n = target.chars().count() + 1;
let t_source = source.chars().collect::<Vec<char>>();
let t_chars = target.chars().collect::<Vec<char>>();
    let mut matrix = vec![vec![0; n]; m];

// let mut matrix = vec![ vec![0; matrix_lenght]; matrix_lenght];
for i in 1..m {
    matrix[0][i] = i;
}

for j in 1..m {
    matrix[j][0] = j;
}
for i in 1..m {
    for j in 1..n {
       if t_source[i - 1] == t_chars[j -1]{
        matrix[i][j] = matrix[i -1][j-1];
       }else {
        matrix[i][j] = 1 +  matrix[i][j-1].min(matrix[i -1][j].min(matrix[i -1][j-1])) ;
       } 
    }
} 
// println!("{:?}", matrix);
matrix[m - 1][n - 1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(edit_distance("gumbo", "gambol"), 2);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
    }
}
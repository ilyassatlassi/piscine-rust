pub fn sum(a: &[i32]) -> i32 {
    let mut sum = 0;
    for val in a {
        sum += val;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_thirtytwo_tens() {
//         assert_eq!(thirtytwo_tens(), [10; 32]);
//     }

//     #[test]
//     fn test_sum() {
//         assert_eq!(sum((1..=10).collect::<Vec<_>>().as_slice()), (1..=10).sum());
//     }
// }
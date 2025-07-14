pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}
#[cfg(test)]
// mod tests {
    
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
mod tests {
    // use division_and_remainder::*;
    
    use super::*;
    #[test]
    #[should_panic]
    fn test_divide_by_0() {
        divide(40, 0);
    }

    #[test]
    fn test_divide() {
        let (div, rem) = divide(40, 3);
        assert_eq!(div, 13);
        assert_eq!(rem, 1);

        let (div, rem) = divide(389, 39);
        assert_eq!(div, 9);
        assert_eq!(rem, 38);

        let (div, rem) = divide(29, 332);
        assert_eq!(div, 0);
        assert_eq!(rem, 29);
    }
}
pub fn doubtful( s: &mut String ) {
s.push_str("?");
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let mut s = "hello".to_owned();
        let s_copy = s.clone();

        doubtful(&mut s);

        assert_eq!(s, s_copy + "?");
    }
}
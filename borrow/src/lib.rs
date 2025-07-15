pub fn str_len(s:&str ) -> usize {
    s.to_string().chars().count()

}
// #[cfg(test)]
// mod tests {
    
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    // use crate::*;

    #[test]
    fn passes() {
        let inputs = [
            ("hello", 5),
            ("how", 3),
            ("are you", 7),
            ("change", 6),
            ("olá!", 4),
            ("bitteschön", 10),
        ];

        inputs
            .into_iter()
            .for_each(|(s, l)| assert_eq!(l, str_len(s)));
    }
}

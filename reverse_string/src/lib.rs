pub fn rev_str(input: &str) -> String {
    input.to_string().chars().rev().collect()
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_one_word() {
//         println!("{}", rev_str("ilyass"));
//         assert_eq!(rev_str("robot"), "tobor");
//         assert_eq!(rev_str("Ramen"), "nemaR");
//         assert_eq!(rev_str("racecar"), "racecar");
//         assert_eq!(rev_str("drawer"), "reward");
//         assert_eq!(rev_str("子猫"), "猫子");
//         assert_eq!(rev_str(""), "");
//     }

//     #[test]
//     fn test_multiple_words() {
//         assert_eq!(rev_str("I'm hungry!"), "!yrgnuh m'I");
//         assert_eq!(rev_str("Hello, world!"), "!dlrow ,olleH");
//         assert_eq!(
//             rev_str("Hello, my name is Roman"),
//             "namoR si eman ym ,olleH"
//         );
//         assert_eq!(rev_str("I have a nice car!"), "!rac ecin a evah I");
//         assert_eq!(rev_str("How old are You"), "uoY era dlo woH");
//         assert_eq!(
//             rev_str("ex: this is an example água"),
//             "augá elpmaxe na si siht :xe"
//         );
//     }
// }
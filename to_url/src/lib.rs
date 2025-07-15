pub fn to_url(s: &str) -> String {
    let mut add = String::new(); 
    for c in s.chars()  {
        if c == ' ' {
            add.push_str("%20"); 
            continue;
        }
        add.push(c);
    }
      add
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
        assert_eq!(to_url("another search "), "another%20search%20");
    }
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

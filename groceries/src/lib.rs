pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
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
    // use groceries::{at_index, insert};

    #[test]
    fn test_insertions() {
        let mut groceries = Vec::new();
        insert(&mut groceries, "milk".to_owned());
        assert_eq!(groceries, ["milk"]);
        insert(&mut groceries, "bread".to_owned());
        assert_eq!(groceries, ["milk", "bread"]);
    }

    #[test]
    fn test_index() {
        let groceries = vec![
            "milk".to_owned(),
            "bread".to_owned(),
            "water".to_owned(),
            "wine".to_owned(),
        ];
        assert_eq!(at_index(&groceries, 0), "milk");
        assert_eq!(at_index(&groceries, 2), "water");
        assert_eq!(at_index(&groceries, 3), "wine");
    }

    #[test]
    #[should_panic]
    fn test_oob_index() {
        let groceries = vec!["milk".to_owned()];
        at_index(&groceries, 2);
    }
}
use std::collections::HashMap;
pub fn smallest(h: HashMap<&str, i32>) -> i32 {
//  let mut min: <i32 as Example>::MAX;
    let mut min =  i32::MAX;
    for (_, value) in h {
        // if min == 0 {
        // min = *value;
        // }
        if min > value {
            min = value;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let mut f = HashMap::new();
        f.insert("Cat", 12);
        f.insert("Dog", 333);
        f.insert("Elephant", 334);
        f.insert("Gorilla", 14);
        f.insert("Dolphin", 2);

        assert_eq!(2, smallest(f));
    }
    #[test]
    fn test_negative() {
        let mut f = HashMap::new();
        f.insert("Daniel", 41758712);
        f.insert("Ashley", 54551444);
        f.insert("Katie", 575556334);
        f.insert("Roberti", 574148);
        f.insert("Robert", -4);

        assert_eq!(-4, smallest(f));
    }
    #[test]
    fn test_zero() {
        let mut f = HashMap::new();
        f.insert("Mars", 1223);
        f.insert("Jupiter", 33343);
        f.insert("Saturn", 12443334);
        f.insert("Neptune", 14);
        f.insert("Venus", 0);

        assert_eq!(0, smallest(f));
    }
    #[test]
    fn empty() {
        let f = HashMap::new();

        assert_eq!(i32::MAX, smallest(f));
    }
}

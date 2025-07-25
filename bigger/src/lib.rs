use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = 0;
    for(_, value) in h {
if value > res {
res = value;
}
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let f = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);

        assert_eq!(334, bigger(f));
    }

    #[test]
    fn test_long() {
        let f = HashMap::from_iter([
            ("Daniel", 41758712),
            ("Ashley", 54551444),
            ("Katie", 575556334),
            ("Robert", 574148),
        ]);

        assert_eq!(575556334, bigger(f));
    }
}
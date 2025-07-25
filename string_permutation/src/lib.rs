use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.chars().count() != s2.chars().count() {
        return false
    }
    let mut map1 = HashMap::new(); 
    // let mut map2 = HashMap::new(); 
    // let mut permutation = true;
    for char in s1.chars() {
        if map1.contains_key(&char)  {
        map1.insert(char,map1[&char] + 1);
        continue;       
        }
        map1.insert(char, 1);
    }
    for char in s2.chars() {
        if map1.contains_key(&char)  {
            if map1[&char]  == 0{
return false;
            }
        map1.insert(char,map1[&char] - 1);
        }else{
            return false;
        }
    }
    true
    // for (key, value) in m
    // for char in s1.chars() {
    //     if map1.contains_key(&char) && map2.contains_key(&char) {
    //         if map1[&char] != map2[&char] {
    //            permutation = false;
    //          break;
    //         }
    //     }else {

    //            permutation = false;
    //          break;
    //     }

        
    // }
    // permutation
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert!(is_permutation("abcde", "edbca"));
        assert!(!is_permutation("avcde", "edbca"));
        assert!(!is_permutation("cde", "edbca"));
        assert!(is_permutation("code", "deco"));
        assert!(!is_permutation("code", "deeco"));
        assert!(!is_permutation("codde", "deeco"));
    }

    #[test]
    fn test_repeating_characters() {
        assert!(is_permutation("aab", "baa"));
    }

    #[test]
    fn test_one_char() {
        assert!(!is_permutation("a", "b"));
        assert!(is_permutation("a", "a"));
    }

    #[test]
    fn test_empty() {
        assert!(is_permutation("", ""));
    }

    #[test]
    fn test_special_characters() {
        assert!(is_permutation("!#%@", "@%#!"));
    }

    #[test]
    fn test_unicode() {
        assert!(is_permutation("hello♥", "♥oelhl"));
        assert!(!is_permutation("♥", "♥♥"));
    }
}
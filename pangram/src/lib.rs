use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    let mut res = HashMap::new();
    for ch in s.chars() {
        if ch.to_ascii_lowercase().is_ascii_lowercase() {
            res.insert(ch.to_ascii_lowercase(), 4);
        }
    }
    if res.len() == 26 {
       return true 
    }
    false
}

#[test]
fn test_empty_strings() {
    assert!(!is_pangram(""));
    assert!(!is_pangram(" "));
}

#[test]
fn test_is_pangram() {
    assert!(is_pangram("the quick brown fox jumps over the lazy dog"));
    assert!(is_pangram("the_quick_brown_fox_jumps_over_the_lazy_dog"));
    assert!(is_pangram(
        "the 1 quick brown fox jumps over the 2 lazy dogs"
    ));
}
#[test]
fn test_not_pangram() {
    assert!(!is_pangram(
        "a quick movement of the enemy will jeopardize five gunboats"
    ));
    assert!(!is_pangram("the quick brown fish jumps over the lazy dog"));
    assert!(!is_pangram("the quick brown fox jumps over the lay dog"));
    assert!(!is_pangram("7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog"));
    assert!(is_pangram("\"Five quacking Zephyrs jolt my wax bed.\""));
    assert!(is_pangram(
        "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich."
    ));
}
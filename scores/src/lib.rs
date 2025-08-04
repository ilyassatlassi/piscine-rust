fn score(str: &str) -> u64 {
    let mut res = 0;
    for ch in str.chars() {
        if "aeioulnrst".contains(ch.to_ascii_lowercase()) {
            res += 1;
        } else if "dg".contains(ch.to_ascii_lowercase()) {
            res += 2;
        } else if "bcmp".contains(ch.to_ascii_lowercase()) {
            res += 3;
        } else if "fhvwy".contains(ch.to_ascii_lowercase()) {
            res += 4;
        } else if "k".contains(ch.to_ascii_lowercase()) {
            res += 5;
        } else if "jx".contains(ch.to_ascii_lowercase()) {
            res += 8;
        } else if "qz".contains(ch.to_ascii_lowercase()) {
            res += 10;
        }
    }
    res
}
//  'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
//             'd' | 'g' => 2,
//             'b' | 'c' | 'm' | 'p' => 3,
//             'f' | 'h' | 'v' | 'w' | 'y' => 4,
//             'k' => 5,
//             'j' | 'x' => 8,
//             'q' | 'z' => 10,
//             _ => 0,

#[test]
fn test_simple() {
    assert_eq!(score("a"), 1);
    assert_eq!(score("A"), 1);
    assert_eq!(score("h"), 4);
    assert_eq!(score("at"), 2);
    assert_eq!(score("Yes"), 6);
    assert_eq!(score("cellphones"), 17);
}

#[test]
fn test_empty() {
    assert_eq!(score(""), 0);
    assert_eq!(score(" "), 0);
}

#[test]
fn test_special() {
    assert_eq!(score("in Portugal NÃO stands for: no"), 30);
    assert_eq!(score("This is a test, comparação, têm Água?"), 36);
}

#[test]
fn test_long() {
    assert_eq!(score("ThiS is A Test"), 14);
    assert_eq!(score("long sentences are working"), 34);
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}

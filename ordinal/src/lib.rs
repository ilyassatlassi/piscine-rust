pub fn num_to_ordinal(x: u32) -> String {
    let mut res = String::new();
    if x % 100 <= 13 &&  x % 100 >= 11 {
        res.push_str(&x.to_string());
        res.push_str("th");
    } else if x % 10 == 1 {
        res.push_str(&x.to_string());
        res.push_str("st");
    } else if x % 10 == 2 {
        res.push_str(&x.to_string());
        res.push_str("nd");
    } else if x % 10 == 3 {
        res.push_str(&x.to_string());
        res.push_str("rd");
    } else {
        res.push_str(&x.to_string());
        res.push_str("th");
    }
    res
}
#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(0), "0th");
    assert_eq!(num_to_ordinal(1), "1st");
    assert_eq!(num_to_ordinal(11), "11th");
    assert_eq!(num_to_ordinal(12), "12th");
    assert_eq!(num_to_ordinal(13), "13th");
    assert_eq!(num_to_ordinal(14), "14th");
    assert_eq!(num_to_ordinal(22), "22nd");
    assert_eq!(num_to_ordinal(43), "43rd");
    assert_eq!(num_to_ordinal(67), "67th");
    assert_eq!(num_to_ordinal(113), "113th");
    assert_eq!(num_to_ordinal(114), "114th");
    assert_eq!(num_to_ordinal(1901), "1901st");
    assert_eq!(num_to_ordinal(1113), "1113th");
    assert_eq!(num_to_ordinal(11111), "11111th");
}

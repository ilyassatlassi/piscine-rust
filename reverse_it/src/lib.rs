pub fn reverse_it(v: i32) -> String {
    let mut isNegative = false;
    let mut num = v;
    if v < 0 {
        num = -v;
        isNegative = true;
    }
    let mut res: String = num.to_string().chars().rev().collect();
    let num = num.to_string();
    res.push_str(&num);
    if isNegative {
        res.insert(0, '-');
    }
    res
}
#[test]
fn reverse_it_test() {
    assert_eq!("321123", &reverse_it(123));
    assert_eq!("987654321123456789", &reverse_it(123456789));
    assert_eq!("00", &reverse_it(0));
    assert_eq!("-321123", &reverse_it(-123));
    assert_eq!("11", &reverse_it(1));
}

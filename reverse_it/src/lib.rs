pub fn reverse_it(v: i32) -> String {
    let mut is_Negative: bool = false;
    let mut num = v as i64;
    if v < 0 {
        // num = v.unsigned_abs();
        is_Negative = true;
        num = -num;
    } 
    // else {
    //     num = v as u32;
    // }

    let mut res: String = num.to_string().chars().rev().collect();
    let num = num.to_string();
    res.push_str(&num);
    if is_Negative {
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
    assert_eq!("-84638474122147483648", &reverse_it(i32::MIN));
    assert_eq!("74638474122147483647", &reverse_it(i32::MAX));
}

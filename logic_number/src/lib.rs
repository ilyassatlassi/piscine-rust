pub fn number_logic(num: u32) -> bool {
    let mut res = 0;
    let mut a = num;
    while a > 0 {
        let remainder = a % 10;
        a = a / 10;
        res += remainder.pow(num.to_string().len() as u32);
    }
    res == num
}

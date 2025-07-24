// use std::mem::swap;

pub fn lucas_number(n: u32) -> u32 {
    let mut res: u32 = 0;
    let mut first: u32 = 2;
    let mut second: u32 = 1;
    for _ in 1..n {
        res = first + second;
        // swap(&mut first, &mut second);
        first = second;
        second = res;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lucas_number_test() {
        assert_eq!(lucas_number(2), 3);
        assert_eq!(lucas_number(5), 11);
        assert_eq!(lucas_number(10), 123);
        assert_eq!(lucas_number(13), 521);
        assert_eq!(lucas_number(25), 167761);
    }
}
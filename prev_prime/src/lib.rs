pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
       return 0; 
    }
    let mut res = 0;
    for i in 1..nbr {
        if is_prime(nbr - i) {
            res = nbr - i;
            break;
        }
    }
    return res;
}
fn is_prime(nbr: u64) -> bool {
    if  nbr < 2{
        return false;
    }
    for i in 2..nbr  {
        if nbr % i == 0 {
            return false;
        }
    }
    return true;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_prime_test() {
        assert_eq!(0, prev_prime(0));
        assert_eq!(0, prev_prime(2));
        assert_eq!(2, prev_prime(3));
        assert_eq!(3, prev_prime(5));
        assert_eq!(31, prev_prime(34));
        assert_eq!(631, prev_prime(633));
        assert_eq!(478139, prev_prime(478152));
    }
}
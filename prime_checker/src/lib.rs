#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }
    let prime = is_prime(nb);
    if prime.0 {
        return Some(Ok(nb));
    } else if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }
    return Some(Err(PrimeErr::Divider(prime.1)));
}

fn is_prime(nb: u32) -> (bool, u32) {
    if nb == 2 {
        return (true, 0);
    }
    if nb % 2 == 0 {
        return (false, 2);
    }
    // let mut divider = 1;
    let max_divisor = (nb as f64).sqrt() as u32;
    for i in 3..=max_divisor  {
        if nb % i == 0 {
            return (false, i);
        }
    }
    return (true, 0);
}

pub fn first_fifty_even_square() -> Vec<i32> {
    (1..=100).filter(|n|  n % 2 == 0).map(|n| n * n).collect()
    // todo!()
}

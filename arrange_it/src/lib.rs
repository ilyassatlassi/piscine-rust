
pub fn arrange_phrase(phrase: &str) -> String {
    let mut words = Vec::new();
    for word in phrase.split_whitespace() {
        for c in word.chars() {
            if c.is_digit(10) {
            words.push((c.to_digit(10).unwrap(), word.replace(c, "")));
            }
        }
        // if let Some(digit) = word.chars().find_map(|c| c.to_digit(10)) {
        //     words.push((digit, word));
        // }
    };
    words.sort_by_key(|x| x.10);
    words.iter().map(|val| val..to_string()).collect::<Vec<_>>().join(" ")
}

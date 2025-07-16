pub fn first_subword(mut s: String) -> String {
    for (i, c) in s.chars().enumerate() {
        if i > 0  && (c.is_uppercase() || c == '_') {
         return  s[0..i].to_string();
        }
    }
    s
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        let cases = [
            ("helloWorld", "hello"),
            ("helloBigWorld", "hello"),
            ("how_are_you", "how"),
            ("Changeyou", "Changeyou"),
            ("CamelCase", "Camel"),
        ];

        cases
            .into_iter()
            .for_each(|(i, e)| assert_eq!(first_subword(i.to_owned()), e));
    }
}
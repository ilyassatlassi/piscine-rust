pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    if !s.contains(prefix) {
        return None;
    }
    Some(&s[prefix.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prefix() {
        assert_eq!(
            delete_prefix("augusto", "augusto ornelas"),
            Some(" ornelas")
        );

        assert_eq!(delete_prefix("ab", "b"), None);

        assert_eq!(delete_prefix("aa", "ab"), None);

        assert_eq!(delete_prefix("á©", "á©ab"), Some("ab"));
    }
}
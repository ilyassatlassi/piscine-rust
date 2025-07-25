pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    for (i, char) in input.chars().enumerate() {
        if i == 0 {
            res.push(char.to_ascii_uppercase());
            continue;
        }
        res.push(char);
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let mut is_space = false;
    for (i, char) in input.chars().enumerate() {
        if i == 0 || (is_space && char.is_alphabetic()) {
            res.push(char.to_ascii_uppercase());
            is_space = false;
            continue;
        }
        if char.is_ascii_whitespace() {
           is_space = true 
        }
        res.push(char);
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for (i, char) in input.chars().enumerate() {
        if char.is_ascii_lowercase() {
            res.push(char.to_ascii_uppercase());
        } else if char.is_ascii_uppercase() {
            res.push(char.to_ascii_lowercase());
        } else {
            res.push(char);
        }
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("this is working"), "This is working");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("this is a title"), "This Is A Title");
        assert_eq!(
            title_case("hello my\t\tname is carl"),
            "Hello My\t\tName Is Carl"
        );
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("PROgraming"), "proGRAMING");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(title_case(""), "");
        assert_eq!(change_case(""), "");
    }
}

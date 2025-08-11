// use scytale_cipher::*;
pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let mut result = String::new();
    let s_chars: Vec<char> = s.chars().collect::<Vec<char>>();
    for j in 0..letters_per_turn {
        let mut i = j as usize;
        while i < s.len() {
            result.push(s_chars[i]);
            i += letters_per_turn as usize;
        }
    }

    Some(result)
}
fn main() {
    println!(
        "\"sec yCtoadle\" size=2 -> {:?}",
        scytale_decoder("sec yCtoadle".to_string(), 2)
    );

    println!(
        "\"steoca dylCe\" size=4 -> {:?}",
        scytale_decoder("steoca dylCe".to_string(), 4)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_args() {
        assert_eq!(scytale_decoder("".to_string(), 5), None);
        assert_eq!(scytale_decoder("empty test".to_string(), 0), None);
        assert_eq!(scytale_decoder("".to_string(), 0), None);
    }

    #[test]
    fn test_short_nb_letters() {
        assert_eq!(
            scytale_decoder("This is already decoded".to_string(), 100),
            Some("This is already decoded".to_string())
        );
    }

    #[test]
    fn test_short_sentence() {
        assert_eq!(
            scytale_decoder("aebfcgd".to_string(), 2),
            Some("abcdefg".to_string())
        );
    }

    #[test]
    fn test_medium_sentence() {
        assert_eq!(
            scytale_decoder("oenset  daa yt hirne et hfea lflosr".to_string(), 2),
            Some("one day in the forest a three falls".to_string())
        );
    }

    #[test]
    fn test_long_sentence() {
        assert_eq!(
            scytale_decoder(
                "dbtheouoevyigleolepnudtmmwhheaaoegnnurigtsavoteneeosdss".to_string(),
                5
            ),
            Some("doyouwanttobuildhousestogetherandhelpmegivesevenmangoes".to_string())
        );
    }
}

pub fn rot21(input: &str) -> String {
    let mut res = String::new();
    for char in input.chars() {
        if char.is_ascii_alphabetic() {
            if char as u8 >= ('a' as u8) && char as u8 <= ('z' as u8) {
            let new_char = ((char as u8 - ('a' as u8) + 21) % 26) + ('a' as u8);
            // println!("{}", new_char);
            res.push(new_char as char);
        } else if char as u8>= ('A' as u8) && char as u8 <= ('Z' as u8) {
            let new_char = ((char as u8 - ('A' as u8) + 21) % 26) + ('A' as u8);
            res.push(new_char as char);
        } 
        }else {
            res.push(char as char);
        }
        
    }
    res
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rot21_multiple_cases() {
        assert_eq!("ocdn dn v ozno", rot21("this is a test"));
        assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
        assert_eq!(
            "ojj  hpxc    nkvxzn      rjmfn",
            rot21("too  much    spaces      works")
        );
        assert_eq!("mv😋w", rot21("ra😋b"));
        assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

        assert_eq!("VWXY", rot21("ABCD"));
        assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
        assert_eq!("WTZ", rot21("BYE"));
    }
}
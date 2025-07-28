#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    if original.is_empty() || ciphered.is_empty() {
        return
        Err(CipherError {
            expected: ciphered.to_string(),
        });
    }
    let mut cmp = String::new();
    for char in original.chars() {
        if char.is_ascii_lowercase() {
            let rem = 26 - ((char as u8) - b'a');
            cmp.push((b'a' + rem - 1) as char);
        } else if char.is_ascii_uppercase() {
            let rem = 26 - ((char as u8) - b'A');
            cmp.push((b'A' + rem - 1) as char);
        } else {
            cmp.push(char);
        }
    }
    // println!("{}", cmp);
    if cmp == ciphered {
        return Ok(());
    }
    Err(CipherError {
        expected: ciphered.to_string(),
    })
    // todo!()
}
// use cipher::*;

#[test]
fn test_ok_values() {
    assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    assert_eq!(cipher("asdasd", "zhwzhw"), Ok(()));
    assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Ok(()));
}

#[test]
fn test_empty_values() {
    assert_eq!(cipher("", ""), Ok(()));
    assert_eq!(
        cipher("", "1Svool 2dliow!"),
        Err(CipherError {
            expected: "".to_owned()
        })
    );
    assert_eq!(
        cipher("1Hello 2world!", ""),
        Err(CipherError {
            expected: "1Svool 2dliow!".to_owned()
        })
    );
}

#[test]
fn test_errors() {
    assert_eq!(
        cipher("1Hello 2world!", "1svool 2dliow!"),
        Err(CipherError {
            expected: String::from("1Svool 2dliow!")
        })
    );
    assert_eq!(
        cipher("asdasd", "lkdas"),
        Err(CipherError {
            expected: String::from("zhwzhw")
        })
    );
    assert_eq!(
        cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"),
        Err(CipherError {
            expected: String::from("3(/&%hw 32wzh")
        })
    );
}

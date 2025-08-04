pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    let vec_string = text.chars().collect::<Vec<char>>();
    for i in 0..vec_string.len() {
        if i == 0 && "aeiou".contains(vec_string[0]) {
            res.push_str(text);
            res.push_str("ay");
            return res;
        } else if i == 0
            && !"aeiou".contains(vec_string[0])
            && vec_string[1] == 'q'
            && vec_string[2] == 'u'
        {
            if vec_string.len() >= 3 {
                let mut slice = &vec_string[3..];
                let mut rest: String = slice.iter().collect();
                res.push_str(&rest);
                slice = &vec_string[..3];
                rest = slice.iter().collect();
                res.push_str(&rest);
                res.push_str("ay");
                //   res.push_str(&[3..vec_string.len()].iter().collect());
                return res;
            }
        }
        if !"aeiou".contains(vec_string[0]) && "aeiou".contains(vec_string[i]) {
            let mut slice = &vec_string[i..];
            let mut rest: String = slice.iter().collect();
            // println!("{}", rest);
            res.push_str(&rest);
            slice = &vec_string[..i];
            rest = slice.iter().collect();
            // println!("{}", rest);
            res.push_str(&rest);
            res.push_str("ay");
            return res;
        }
    }
    todo!()
}
#[test]
fn test_word_beginning_with_vowel() {
    assert_eq!(pig_latin("apple"), "appleay");
    assert_eq!(pig_latin("ear"), "earay");
    assert_eq!(pig_latin("igloo"), "iglooay");
    assert_eq!(pig_latin("object"), "objectay");
    assert_eq!(pig_latin("under"), "underay");
    assert_eq!(pig_latin("equal"), "equalay");
}

#[test]
fn test_word_beginning_with_consonant() {
    assert_eq!(pig_latin("queen"), "ueenqay");
    assert_eq!(pig_latin("square"), "aresquay");
    assert_eq!(pig_latin("pig"), "igpay");
    assert_eq!(pig_latin("koala"), "oalakay");
    assert_eq!(pig_latin("yellow"), "ellowyay");
    assert_eq!(pig_latin("xenon"), "enonxay");
    assert_eq!(pig_latin("qat"), "atqay");
    assert_eq!(pig_latin("chair"), "airchay");
    assert_eq!(pig_latin("therapy"), "erapythay");
    assert_eq!(pig_latin("thrush"), "ushthray");
    assert_eq!(pig_latin("school"), "oolschay");
    assert_eq!(pig_latin("british"), "itishbray");
}

#[test]
fn test_multiple_words() {
    assert_eq!(pig_latin("apple"), "appleay");
    assert_eq!(pig_latin("ear"), "earay");
    assert_eq!(pig_latin("igloo"), "iglooay");
    assert_eq!(pig_latin("qat"), "atqay");
    assert_eq!(pig_latin("school"), "oolschay");
    assert_eq!(pig_latin("therapy"), "erapythay");
}
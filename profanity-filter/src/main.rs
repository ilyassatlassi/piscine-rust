use profanity_filter::*;
fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", profanity_filter::check_ms(m)));
}

#[test]
fn test_error_ms() {
    ["", "stupid", "you are stupid"]
        .into_iter()
        .for_each(|m| assert_eq!(Err("ERROR: illegal"), profanity_filter::check_ms(m)));
}

#[test]
fn test_ok_ms() {
    [
        "get out of the car",
        "no!",
        "get the werewolf",
        "wait the what...",
    ]
    .into_iter()
    .for_each(|m| assert_eq!(Ok(m), profanity_filter::check_ms(m)));
}
// use profanity_filter::*;

// fn main() {
//     let m0 = Message::new("hello there".to_string(), "toby".to_string());
//     println!("{:?}", check_ms(&m0));

//     let m1 = Message::new("".to_string(), "toby".to_string());
//     println!("{:?}", check_ms(&m1));

//     let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
//     println!("{:?}", check_ms(&m2));

//     let m3 = Message::new("stupid".to_string(), "toby".to_string());
//     println!("{:?}", check_ms(&m3));
// }
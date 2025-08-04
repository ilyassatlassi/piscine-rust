pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    } else if text.chars().all(|ch| !ch.is_ascii_lowercase()) && text.chars().last().unwrap() != '?'{
        return "There is no need to yell, calm down!";
    } else if text.chars().any(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit()) && text.chars().last().unwrap() == '?'  {
        return "Sure.";
    } else if text.chars().last().unwrap() == '?' {
        return "Quiet, I am thinking!";
    } else {
        return "Interesting";
    }

    // todo!()
}

#[test]
fn test_yell() {
    assert_eq!(
        talking("JUST DO IT!"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("1, 2, 3 GO!"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("I LOVE YELLING"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("WJDAGSAF ASVF EVA VA"),
        "There is no need to yell, calm down!"
    );
}

#[test]
fn test_question() {
    assert_eq!(talking("Hello how are you?"), "Sure.");
    assert_eq!(talking("Are you going to be OK?"), "Sure.");
    assert_eq!(talking("7?"), "Sure.");
    assert_eq!(talking("Like 15?"), "Sure.");
}

#[test]
fn test_question_yelling() {
    assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
    assert_eq!(talking("ARE YOU FINISHED?"), "Quiet, I am thinking!");
    assert_eq!(talking("WHAT DID I DO?"), "Quiet, I am thinking!");
    assert_eq!(talking("ARE YOU COMING?"), "Quiet, I am thinking!");
}

#[test]
fn test_interesting() {
    assert_eq!(talking("something"), "Interesting");
    assert_eq!(talking("Wow that's good!"), "Interesting");
    assert_eq!(talking("Run far"), "Interesting");
    assert_eq!(talking("1 2 3 go!"), "Interesting");
    assert_eq!(talking("This is not ? a question."), "Interesting");
}

#[test]
fn test_empty() {
    assert_eq!(talking(""), "Just say something!");
    assert_eq!(talking("										"), "Just say something!");
    assert_eq!(talking("          "), "Just say something!");
}
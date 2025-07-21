pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res = Vec::new();

    if i == 0 {
        return res;
    }

    for j in 0..i {
        let mut space = String::new();
        let mut str = String::new();
        for _ in 0..j +1{
            space.push(' ');
        }
        for _ in 0..j +1 {
            str.push_str(&v);
        }
        res.push(space + &str);
    }
    for j in 0..i - 1 {
        let mut space = String::new();
        let mut str = String::new();
        for _ in 0..i - j - 1 {
            space.push(' ');
        }
        for _ in 0..i - j -1 {
            str.push_str(&v);
        }
        res.push(space + &str);
    }
    res
}


#[test]
fn it_works() {
    // use super::*;
    let data_sets = vec![
        vec![],
        vec![" #"],
        vec![" a", "  aa", " a"],
        vec![
            " >",
            "  >>",
            "   >>>",
            "    >>>>",
            "     >>>>>",
            "    >>>>",
            "   >>>",
            "  >>",
            " >",
        ],
        vec![
            " &",
            "  &&",
            "   &&&",
            "    &&&&",
            "     &&&&&",
            "      &&&&&&",
            "       &&&&&&&",
            "        &&&&&&&&",
            "       &&&&&&&",
            "      &&&&&&",
            "     &&&&&",
            "    &&&&",
            "   &&&",
            "  &&",
            " &",
        ],
    ];
    assert_eq!(inv_pyramid(String::from("#"), 0), data_sets[0]);
    assert_eq!(inv_pyramid(String::from("#"), 1), data_sets[1]);
    assert_eq!(inv_pyramid(String::from("a"), 2), data_sets[2]);
    assert_eq!(inv_pyramid(String::from(">"), 5), data_sets[3]);
    assert_eq!(inv_pyramid(String::from("&"), 8), data_sets[4]);
}
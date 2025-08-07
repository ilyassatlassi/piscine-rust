pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut res = Vec::new();
    for word in &mut s.split_whitespace() {
        if word.ends_with("k") {
            let num = word.replace("k", "");
            let num = (num.parse::<f32>().unwrap() * 1000.) as u32;
            res.push(Box::new(num));
        } else {
            let num = word.parse::<u32>().unwrap() * 1000;
            res.push(Box::new(num));
        }
    }
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res = Vec::new();

    for val in a {
        res.push(*val);
    }
    res
}

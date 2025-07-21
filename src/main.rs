fn main() {
    let a = inv_pyramid(String::from("#"), 1);
    let b = inv_pyramid(String::from("a"), 2);
    let c = inv_pyramid(String::from(">"), 5);
    let d = inv_pyramid(String::from("&"), 8);

    for v in a.iter() {
        println!("{:?}", v);
    }
    for v in b.iter() {
        println!("{:?}", v);
    }
    for v in c.iter() {
        println!("{:?}", v);
    }
    for v in d.iter() {
        println!("{:?}", v);
    }
}
pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res = Vec::new();
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

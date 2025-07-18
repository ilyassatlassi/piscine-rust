pub fn delete_and_backspace(s: &mut String) {
    let mut count = 0;
    let mut new_str = String::new();
    for c in s.chars() {
        if count != 0 && c != '+' {
            count -= 1;
            continue;
        }

        if c == '+' {
            count += 1;
        } else {
            new_str.push(c);
        }
    }

    *s = new_str;
    let mut new_str = String::new();
    count = 0;
    for c in s.chars().rev() {
        if count != 0 && c != '-' {
            // s.remove(i);
            count -= 1;
            continue;
        }

        if c == '-' {
            count += 1;
        } else {
            new_str.push(c);
        }
    }

    *s = new_str
}

pub fn do_operations(v: &mut [String]) {
    for val in v {
        if val.contains('-') {
            match val.split_once('-') {
                Some((x, y)) => *val = (x.parse::<i32>().unwrap() - y.parse::<i32>().unwrap()).to_string(),
                None => println!("test"),
            }
        } else {
            match val.split_once('+') {
                Some((x, y)) => *val = (x.parse::<i32>().unwrap() + y.parse::<i32>().unwrap()).to_string(),
                None => println!("test"),
            }
        }
    }
}
fn main() {
    let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
    let mut b: [String; 4] = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
    ];

    delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", (a, b));
}

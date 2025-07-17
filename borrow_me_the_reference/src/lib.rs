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

    *s = new_str.chars().rev().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
        let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
        let mut a_3 = String::from("pad-rtic+eulqw--+rar");
        let mut a_4 = String::from("--++++");

        delete_and_backspace(&mut a_1);
        delete_and_backspace(&mut a_2);
        delete_and_backspace(&mut a_3);
        delete_and_backspace(&mut a_4);

        assert_eq!(a_1, "borrow");
        assert_eq!(a_2, "help");
        assert_eq!(a_3, "particular");
        assert_eq!(a_4, "");
    }

    #[test]
    fn test_do_operations() {
        let mut b_1 = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "0+0".to_owned(),
            "0-0".to_owned(),
            "10-100".to_owned(),
        ];
        do_operations(&mut b_1);

        assert_eq!(b_1, ["4", "5", "7", "0", "0", "-90"]);
    }
}

// use core::num;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

fn rpn(arg: &str) {
    if arg.is_empty() {
        println!("Error");
        return;
    }
    let mut nums = Vec::new();
    let mut operation = Vec::new();
    for arg in arg.split_whitespace() {
        let mut number = String::new();
        for c in arg.chars() {
            if c.is_ascii_digit() {
                number.push(c);
            } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' {
                operation.push(c);
            } else {
                println!("Error");
                return;
            }
        }
        if !number.is_empty() {
           nums.push(number.parse::<i32>().unwrap()); 
        }
    }
    if nums.len() - 1 != operation.len() {
        println!("Error");
        return;
    }
    let mut res = nums[0];
    for i in 0..operation.len() {
        if operation[i] == '+' {
            res += nums[i + 1]
        }

        if operation[i] == '-' {
            res -= nums[i + 1]
        }

        if operation[i] == '*' {
            res *= nums[i + 1]
        }

        if operation[i] == '/' {
            res /= nums[i + 1]
        }

        if operation[i] == '%' {
            res %= nums[i + 1]
        }
    }
    println!("{}", res)
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../src/test.rs";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn error_tests() {
        assert_eq!("Error\n", run("21 3 2 % 2 3 2 *"));
        assert_eq!("Error\n", run("1 2 3 4 +"));
        assert_eq!("Error\n", run("324   +    1 - 23 "));
        assert_eq!("Error\n", run("32   / 22"));
        assert_eq!("Error\n", run("88 67 dks -"));
    }

    #[test]
    fn simple_tests() {
        assert_eq!("33\n", run("11 22 +"));
        assert_eq!("72\n", run("11016 153 /"));
        assert_eq!("1140\n", run("15 76 *"));
        assert_eq!("-78539698\n", run("23491234 102030932 -"));
    }

    #[test]
    fn complex_tests() {
        assert_eq!("10\n", run("1 2 * 3 * 4 +"));
        assert_eq!("2\n", run("3 1 2 * * 4 %"));
        assert_eq!("0\n", run("5 10 2 / - 50 *"));
    }

    #[test]
    fn with_spaces() {
        assert_eq!("44\n", run("299   255 %"));
        assert_eq!("1\n", run("     1      3 * 2 -"));
    }
}
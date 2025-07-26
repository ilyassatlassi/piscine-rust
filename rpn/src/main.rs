
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
    // let mut operation = Vec::new();
    for arg in arg.split_whitespace() {
        match arg.parse::<i32>() {
            Ok(num) => {
                nums.push(num);
            }
            Err(_) => {
                if "-*/+%".contains(arg) {
                    if nums.len() < 2 {
                        println!("Error");
                        return;
                    }
                    let b = nums.pop().unwrap();
                    let a = nums.pop().unwrap();
                    let result = match arg {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        "%" => a % b,
                        _ => unreachable!(),
                    };
                    nums.push(result);
                } else {
                    println!("Error");
                    return;
                }
            }
        }
    }
    if nums.len() == 1 {
        println!("{}", nums[0]);
    } else {
        println!("Error");
    }
    println!("{}", nums[0])
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}
fn brain_fuck(code: &str) {
    let mut arr = [0u8; 2048];
    let mut index: usize = 0;
    let mut pc = 0; // program counter
    let chars: Vec<char> = code.chars().collect::<Vec<char>>();

    while pc < chars.len() {
        match chars[pc] {
            '+' => arr[index] = arr[index].wrapping_add(1),
            '-' => arr[index] = arr[index].wrapping_sub(1),
            '>' => index += 1,
            '<' => index -= 1,
            '.' => print!("{}", arr[index] as char),
            '[' => {
                if arr[index] == 0 {
                    // Jump forward to matching ']'
                    let mut depth = 1;
                    while depth > 0 {
                        pc += 1;
                        match chars[pc] {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if arr[index] != 0 {
                    // Jump back to matching '['
                    let mut depth = 1;
                    while depth > 0 {
                        pc -= 1;
                        match chars[pc] {
                            '[' => depth -= 1,
                            ']' => depth += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        pc += 1;
    }
}


// static mut arr: [u8; 2048] = [0; 2048];
// static mut index: usize = 0;
// static mut is_loop: bool = false;
// fn brain_fuck(arg: &str) {
//     let mut new_arg = String::new();
//     for ch in arg.chars() {
//         if ch == '[' {
//             unsafe {
//                 is_loop = true;
//             }
//             continue;
//         }

//         if ch == ']' {
//             let new_index = unsafe { index };
//             unsafe {
//                 is_loop = false;
//             }
//             while unsafe { arr[new_index] != 0 } {
//                 // println!("{new_arg}");
//                 brain_fuck(&new_arg);
//             }
//         }

//         if unsafe { is_loop } {
//             new_arg.push(ch);
//             continue;
//         }

//         if ch == '+' {
//             unsafe {
//                 arr[index] += 1;
//             }
//         }

//         if ch == '-' {
//             unsafe {
//                 arr[index] -= 1;
//             }
//         }

//         if ch == '>' {
//             unsafe {
//                 index += 1;
//             }
//         }

//         if ch == '<' {
//             unsafe {
//                 index -= 1;
//             }
//         }
//         if ch == '.' {
//             print!("{}", unsafe { arr[index] as char });
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/brain_fuck/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            // .arg("--manifest-path")
            // .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn nothing_passed() {
        assert_eq!("", run(""));
    }

    #[test]
    fn single_chars() {
        assert_eq!(
            "a",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.")
        );
        assert_eq!(
            "S",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++++++++.")
        );
        assert_eq!(
            "7",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>---------------.")
        );
    }
    #[test]
    fn phrases() {
        assert_eq!(
            "Wow",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>-------------.++++++++++++++++++++++++.++++++++.")
        );
        assert_eq!(
            "Good job!",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+.>+++++++++++..-----------.<<++.>>++++++.+++++.-------------.<<+.")
        );
    }

    #[test]
    fn with_characters_in_middle() {
        assert_eq!("keep going", run("++++++++++[>+>ke+++>+++++++>++++++++++<<<<-]>>>>+++++++e.------p..+++++++++++.<<++.>g>---------.+o+++++++.------i.+++++.-n------.g"));
    }

    #[test]
    fn big_test() {
        assert_eq!(
            "3, 2, 1... Happy New Year",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>-------------------.-------.<++.>++++++.------.<.>+++++.---...<.>++++++++++++++++++++++++++.>---.+++++++++++++++..+++++++++.<<.>++++++.>--------------------.++++++++++++++++++.<<.>+++++++++++.++++++++++++.----.>-----.")
        );
        assert_eq!(
            "To be or not be, that is the question!", 
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++++++++++++++.>+++++++++++.<<++.>>-------------.+++.<<.>>++++++++++.+++.<<.>>----.+.+++++.<<.>++++++++++++++.+++.<++++++++++++.------------.>>.<+++.-------.>.<<.>++++++++.>-.<<.>>+.<-.---.<.>>---.++++.<.>--.+.<++++.>-----.-.<<+.")
        );
    }
}
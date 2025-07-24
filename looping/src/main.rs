use std::io;

fn main() {
    let mut counter = 0;
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        counter += 1;
        if guess== "The letter e\n" {
            println!("Number of trials: {counter}");
            break;
        }
    }
}


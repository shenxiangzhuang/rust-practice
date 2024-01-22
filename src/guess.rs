/// reference: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub fn guess_game() {
    println!("Guess Game Start!");
    let answer: u32 = 42;
    let mut input_str = String::new();

    loop {
        print!("Input a number: ");
        io::stdout().flush().expect("Flush error");
        input_str.clear();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Read input string error");
        let input_num: u32 = input_str
            .trim()
            .parse()
            .expect("Failed to parse input string into u32");
        match input_num.cmp(&answer) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too large"),
            Ordering::Less => println!("Too small"),
        }
    }
}
